use crate::{authorization_urls, status, url_facts, version};
use serde_json::json;
use url::Url;

#[test]
fn status_fixtures() {
    let email = format!("{}@example.org", "a".repeat(8));
    let logged = json!({"loggedIn": true, "profile": {"email": email}}).to_string();
    let facts = status(&logged, "");
    assert_eq!(facts["login"], "yes");
    assert_eq!(facts["account_named"], "yes");
    assert_eq!(facts["account_field"], "profile.email");
    for (text, field) in [
        (format!("Signed in\nEmail Address: {email}"), "emailaddress"),
        (format!("Logged in\nA Person: {email}"), "unnamed"),
        (format!("Logged in {email}"), "unnamed"),
        (
            format!("\x1b[32mLogged in\x1b[0m\nEmail: \x1b[1m{email}\x1b[0m"),
            "email",
        ),
    ] {
        let facts = status("", &text);
        assert_eq!(facts["login"], "yes");
        assert_eq!(facts["account_named"], "yes");
        assert_eq!(facts["account_field"], field);
    }
    let facts = status("Logged in using ChatGPT", "");
    assert_eq!(facts["login"], "yes");
    assert_eq!(facts["account_named"], "no");
    assert_eq!(facts["account_field"], serde_json::Value::Null);
    for text in [
        "Not logged in",
        "not signed in",
        "Logged out",
        r#"{"signed_in":false}"#,
    ] {
        assert_eq!(status(text, "")["login"], "no");
    }
    assert_eq!(
        status("Logged in, but now not logged in", "")["login"],
        "no"
    );
    assert_eq!(
        status(r#"{"signedIn":true,"accountID":"fixture"}"#, "")["account_field"],
        "accountID"
    );
    let nested = json!({"signedIn": true, "profiles": [{"userEmail": email}]}).to_string();
    assert_eq!(status(&nested, "")["account_field"], "profiles.0.userEmail");
    for key in ["name@example.org", &"k".repeat(33), "first-last"] {
        let text = json!({key: {"email": email}, "signedIn": true}).to_string();
        assert_eq!(status(&text, "")["account_field"], "unnamed");
    }
    assert_eq!(status("{}", "Logged in")["login"], "unclear");
}

fn authorization(redirect: &str) -> Url {
    let mut url = Url::parse("https://example.org/authorize").unwrap();
    url.query_pairs_mut()
        .append_pair("client_id", &"c".repeat(16))
        .append_pair("response_type", "code")
        .append_pair("redirect_uri", redirect)
        .append_pair("code_challenge_method", "S256")
        .append_pair("code_challenge", &"p".repeat(43))
        .append_pair("state", &"s".repeat(32));
    url
}
fn replace_parameter(url: &mut Url, key: &str, value: &str) {
    let pairs: Vec<_> = url
        .query_pairs()
        .filter(|(k, _)| k != key)
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect();
    url.set_query(None);
    url.query_pairs_mut()
        .extend_pairs(pairs)
        .append_pair(key, value);
}

#[test]
fn url_passes_and_sources() {
    let url = authorization("http://127.0.0.1:12345/callback");
    let (facts, target) = url_facts(url.as_str(), "printed");
    assert_eq!(facts["ifc_049"], json!([]));
    assert_eq!(facts["return"], "redirect");
    assert_eq!(facts["redirect_host"], "127.0.0.1");
    assert_eq!(target, Some(("127.0.0.1".into(), 12345)));
    for host in ["localhost", "[::1]"] {
        let url = authorization(&format!("http://{host}:80/callback"));
        let (facts, target) = url_facts(url.as_str(), "shim");
        assert_eq!(facts["ifc_049"], json!([]));
        assert_eq!(target, Some((host.into(), 80)));
    }
    let page = authorization("https://example.org/code");
    let (facts, target) = url_facts(page.as_str(), "printed");
    assert_eq!(facts["return"], "page");
    assert_eq!(facts["ifc_049"], json!([]));
    assert_eq!(facts["listens"], "not checked");
    assert!(target.is_none());
    let printed = format!("\x1b[32mOpen: <{url}>\x1b[0m\nhttps://example.org/help");
    let sources = authorization_urls(&printed, &format!("{url}\n{page}"));
    assert_eq!(sources.len(), 2);
    assert_eq!(sources[url.as_str()], "both");
    assert_eq!(sources[page.as_str()], "shim");
    for delimiter in ['"', '\'', '<', '>', ')', ']'] {
        assert_eq!(
            authorization_urls(&format!("{url}{delimiter}tail"), "").len(),
            1
        );
    }
}

#[test]
fn each_url_check_fails_alone() {
    let original = authorization("http://127.0.0.1:12345/callback");
    for check in [
        "https",
        "no_userinfo",
        "no_fragment",
        "no_repeated_parameter",
        "client_id",
        "response_type_code",
        "s256",
        "challenge_43",
        "state_ok",
        "redirect_port",
        "redirect_no_query",
        "return",
    ] {
        let mut url = original.clone();
        match check {
            "https" => url.set_scheme("http").unwrap(),
            "no_userinfo" => url.set_username("fixture").unwrap(),
            "no_fragment" => url.set_fragment(Some("fixture")),
            "no_repeated_parameter" => {
                url.query_pairs_mut().append_pair("client_id", "other");
            }
            "client_id" => replace_parameter(&mut url, "client_id", ""),
            "response_type_code" => replace_parameter(&mut url, "response_type", "token"),
            "s256" => replace_parameter(&mut url, "code_challenge_method", "plain"),
            "challenge_43" => replace_parameter(&mut url, "code_challenge", &"p".repeat(42)),
            "state_ok" => replace_parameter(&mut url, "state", &"s".repeat(21)),
            "redirect_port" => {
                replace_parameter(&mut url, "redirect_uri", "http://localhost/callback")
            }
            "redirect_no_query" => replace_parameter(
                &mut url,
                "redirect_uri",
                "http://localhost:12345/callback?fixture=yes",
            ),
            "return" => replace_parameter(&mut url, "redirect_uri", "http://example.org/callback"),
            _ => unreachable!(),
        }
        let (facts, _) = url_facts(url.as_str(), "printed");
        assert_eq!(facts["ifc_049"], json!([check]), "{check}");
    }
    for (key, value, check) in [
        ("state", "s".repeat(22) + "!", "state_ok"),
        ("code_challenge", "p".repeat(42) + "~", "challenge_43"),
    ] {
        let mut url = original.clone();
        replace_parameter(&mut url, key, &value);
        assert_eq!(url_facts(url.as_str(), "printed").0[check], false);
    }
}

#[test]
fn versions_are_tokens_only() {
    assert_eq!(version("1.2.3.preview", ""), "1.2.3.preview");
    assert_eq!(version("fixture 1.2.3 (synthetic)", ""), "1.2.3");
    assert_eq!(version("", "\x1b[32m2.3.4-beta.1\x1b[0m"), "2.3.4-beta.1");
    assert_eq!(version("1.2.3+test", ""), "1.2.3+test");
    assert_eq!(version("v1.2.3 1.2 1.2.3+bad+bad", ""), "unparsed");
}
