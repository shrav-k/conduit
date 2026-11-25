//! Merge messages, `NAME.txt` under the configured `messages` directory.
//!
//! A message is well formed when its first line, the title, is not empty, its second line is
//! blank, a body with a line that is not blank follows, and it ends with a blank line and its
//! trailer lines, at least one, each beginning with a configured prefix.

use std::fs;
use std::path::Path;

/// A well-formed merge message, as `check` read it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    text: String,
    lines: Vec<String>,
    /// The index of the blank line before the trailers.
    blank: usize,
}

impl Message {
    /// Reads and checks the message at `path`.
    pub fn read(path: &Path, trailers: &[String]) -> Result<Self, String> {
        let bytes = fs::read(path)
            .map_err(|error| format!("the message {} cannot be read: {error}", path.display()))?;
        let text = String::from_utf8(bytes)
            .map_err(|_| format!("the message {} is not UTF-8", path.display()))?;
        Self::parse(&text, trailers)
            .map_err(|why| format!("the message {} is not well formed: {why}", path.display()))
    }

    /// Checks a message's text.
    pub fn parse(text: &str, trailers: &[String]) -> Result<Self, String> {
        let content = text.strip_suffix('\n').unwrap_or(text);
        let lines: Vec<String> = content.split('\n').map(str::to_string).collect();
        if lines[0].trim().is_empty() {
            return Err("its first line, the title, is empty".into());
        }
        if lines.len() < 2 || !lines[1].trim().is_empty() {
            return Err("its second line is not blank".into());
        }
        let is_trailer = |line: &str| trailers.iter().any(|prefix| line.starts_with(prefix));
        let mut start = lines.len();
        while start > 2 && is_trailer(&lines[start - 1]) {
            start -= 1;
        }
        if start == lines.len() {
            return Err(format!(
                "it does not end with a trailer line beginning with one of {}",
                trailers
                    .iter()
                    .map(|prefix| format!("{prefix:?}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        let blank = start - 1;
        let no_body = "it has no body: no line that is not blank before its trailers";
        if blank < 2 {
            return Err(no_body.into());
        }
        if !lines[blank].trim().is_empty() {
            return Err("its trailer lines do not follow a blank line".into());
        }
        if lines[2..blank].iter().all(|line| line.trim().is_empty()) {
            return Err(no_body.into());
        }
        Ok(Self {
            text: text.to_string(),
            lines,
            blank,
        })
    }

    /// The whole message, as read.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// The first line.
    pub fn title(&self) -> &str {
        self.lines[0].trim_end()
    }

    /// The body, less the blank line and the trailer lines that end the message.
    pub fn body(&self) -> String {
        self.lines[2..self.blank].join("\n").trim_end().to_string()
    }

    /// The pull request's body: the body, then the footer.
    pub fn pull_request_body(&self, footer: &str) -> String {
        if footer.is_empty() {
            self.body()
        } else {
            format!("{}\n\n{footer}", self.body())
        }
    }

    /// The message after its blank line, trailers kept: the squash merge's body.
    pub fn merge_body(&self) -> String {
        self.lines[2..].join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::Message;

    fn trailers() -> Vec<String> {
        vec!["Co-Authored-By: ".into(), "Session: ".into()]
    }

    #[test]
    fn takes_a_well_formed_message() {
        let text = "Title\n\nBody one.\n\nBody two.\n\nCo-Authored-By: A\nSession: x\n";
        let message = Message::parse(text, &trailers()).unwrap();
        assert_eq!(message.title(), "Title");
        assert_eq!(message.body(), "Body one.\n\nBody two.");
        assert_eq!(
            message.pull_request_body("F"),
            "Body one.\n\nBody two.\n\nF"
        );
        assert_eq!(
            message.merge_body(),
            "Body one.\n\nBody two.\n\nCo-Authored-By: A\nSession: x"
        );
        assert_eq!(message.text(), text);
    }

    #[test]
    fn refuses_each_malformed_message() {
        let cases = [
            ("\n\nBody\n\nSession: x\n", "title"),
            ("Title\nBody\n\nSession: x\n", "second line"),
            ("Title\n\nSession: x\n", "no body"),
            ("Title\n\n\n\nSession: x\n", "no body"),
            ("Title\n\nBody\n", "trailer line"),
            ("Title\n\nBody\nSession: x\n", "follow a blank"),
            ("Title\n\nBody\n\nSession: x\n\n", "trailer line"),
        ];
        for (text, why) in cases {
            let error = Message::parse(text, &trailers()).unwrap_err();
            assert!(error.contains(why), "{text:?}: {error}");
        }
    }
}
