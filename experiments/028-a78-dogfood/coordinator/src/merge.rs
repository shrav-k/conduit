//! The two kinds of path a stopped rebase settles hunk by hunk (order 401b, decision 1).
//!
//! - `spec/CHANGES.md`: a hunk whose ours and theirs each begin with the base's lines becomes
//!   the base's lines, then ours' added lines, then theirs'.
//! - Prose: each `.md` and `.yaml` file under `spec/` but `spec/CHANGES.md` and the fragments,
//!   `engine/CONFORMANCE.md` and `docs/work-orders/*.md`. A hunk whose base is empty keeps ours'
//!   lines and then theirs', each whole, in a `.md` file, and stops in YAML. Any other hunk
//!   merges by words: each side is split at whitespace and both sides' edits against the base
//!   apply, with an unchanged base word between any two, but for both inserting at the hunk's
//!   end, ours first, outside a table row. The result wraps at the widest line of ours' and
//!   theirs' sides, and its first line keeps ours' indent and list marker.
//!
//! Keeping both sides' added lines stops where a line would then belong to another block than
//! on its own side ([`keep_both`]), and a wrapped line never begins with a word that would open
//! a block, nor ends in a backslash ([`layout`]).
//!
//! Anything else stops, naming the path and the hunk's first line, and for an overlap up to five
//! base words on each side. No hunk keeps one side over a changed other.

/// The changelog, which keeps both sides.
pub const CHANGES: &str = "spec/CHANGES.md";
/// The directory of the changelog's fragments.
pub const FRAGMENTS: &str = "spec/changes/";
/// The one file under [`FRAGMENTS`] that is no fragment.
pub const FRAGMENTS_README: &str = "spec/changes/README.md";

/// The most cells the word diff of one side of a hunk may take.
const MOST_CELLS: usize = 4_000_000;

/// How a conflicted path's hunks are settled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// `spec/CHANGES.md`: the base's lines, then ours' added lines, then theirs'.
    Changelog,
    /// A Markdown prose file: an empty base keeps both sides whole, any other hunk merges by
    /// words.
    Markdown,
    /// A YAML prose file: an empty base stops, any other hunk merges by words.
    Yaml,
}

/// How the queue settles a conflicted `path`, or `None` when it stops on it.
pub fn kind_of(path: &str) -> Option<Kind> {
    if path == CHANGES {
        return Some(Kind::Changelog);
    }
    if path.starts_with(FRAGMENTS) && path != FRAGMENTS_README {
        return None;
    }
    if path.starts_with("spec/") {
        if path.ends_with(".md") {
            return Some(Kind::Markdown);
        }
        if path.ends_with(".yaml") {
            return Some(Kind::Yaml);
        }
        return None;
    }
    if path == "engine/CONFORMANCE.md" {
        return Some(Kind::Markdown);
    }
    match path.strip_prefix("docs/work-orders/") {
        Some(name) if !name.contains('/') && name.ends_with(".md") => Some(Kind::Markdown),
        _ => None,
    }
}

/// One conflict hunk of a file git wrote with `merge.conflictstyle=diff3`, each line without
/// its line end.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hunk {
    /// The line of its `<<<<<<< ` marker, counted from 1.
    pub line: usize,
    pub ours: Vec<String>,
    pub base: Vec<String>,
    pub theirs: Vec<String>,
}

impl Hunk {
    /// The hunk's first line, as order 401 names a hunk: ours' first, else the base's, else
    /// theirs'.
    pub fn first_line(&self) -> &str {
        self.ours
            .iter()
            .chain(&self.base)
            .chain(&self.theirs)
            .next()
            .map(String::as_str)
            .unwrap_or("(an empty hunk)")
    }
}

/// How one hunk was settled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum How {
    /// Merged by words.
    ByWords,
    /// Both sides' added lines kept, ours' first.
    KeptBoth,
}

/// A file whose every hunk was settled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settled {
    pub text: String,
    pub by_words: usize,
    pub kept_both: usize,
}

impl Settled {
    /// What was done to `path`, for the ledger's `why`.
    pub fn note(&self, path: &str) -> String {
        let count = |n: usize| format!("{n} hunk{}", if n == 1 { "" } else { "s" });
        let mut parts = Vec::new();
        if self.by_words > 0 {
            parts.push(format!("{} merged by words", count(self.by_words)));
        }
        if self.kept_both > 0 {
            parts.push(format!(
                "{} kept both sides' added lines",
                count(self.kept_both)
            ));
        }
        format!("{path}: {}", parts.join(", "))
    }
}

enum Part {
    /// A line outside every hunk, with its line end.
    Text(String),
    Hunk(Hunk),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Marker {
    Start,
    Base,
    Middle,
    End,
}

fn marker(line: &str) -> Option<Marker> {
    let labelled = |sign: &str| {
        line.strip_prefix(sign)
            .is_some_and(|rest| rest.is_empty() || rest.starts_with(' '))
    };
    if labelled("<<<<<<<") {
        Some(Marker::Start)
    } else if labelled("|||||||") {
        Some(Marker::Base)
    } else if line == "=======" {
        Some(Marker::Middle)
    } else if labelled(">>>>>>>") {
        Some(Marker::End)
    } else {
        None
    }
}

/// Splits a conflicted file into its lines and its diff3 hunks. A marker out of its place,
/// such as a second `=======` in one hunk, stops: the hunk's sides could not be told apart.
fn parse(text: &str) -> Result<Vec<Part>, String> {
    let mut parts = Vec::new();
    let mut open: Option<(Hunk, Marker)> = None;
    for (index, raw) in text.split_inclusive('\n').enumerate() {
        let number = index + 1;
        let line = raw.strip_suffix('\n').unwrap_or(raw);
        let found = marker(line);
        let Some((hunk, at)) = open.as_mut() else {
            match found {
                Some(Marker::Start) => {
                    let hunk = Hunk {
                        line: number,
                        ours: Vec::new(),
                        base: Vec::new(),
                        theirs: Vec::new(),
                    };
                    open = Some((hunk, Marker::Start));
                }
                _ => parts.push(Part::Text(raw.to_string())),
            }
            continue;
        };
        match (*at, found) {
            (Marker::Start, Some(Marker::Base)) => *at = Marker::Base,
            (Marker::Base, Some(Marker::Middle)) => *at = Marker::Middle,
            (Marker::Middle, Some(Marker::End)) => {
                let (hunk, _) = open.take().expect("a hunk is open");
                parts.push(Part::Hunk(hunk));
            }
            (Marker::Start, Some(Marker::Middle)) => {
                return Err(format!(
                    "its hunk at line {} has no base section, as merge.conflictstyle=diff3 writes",
                    hunk.line
                ));
            }
            (_, Some(_)) => {
                return Err(format!(
                    "its hunk at line {} has a marker out of place at line {number}, so its sides cannot be told apart",
                    hunk.line
                ));
            }
            (_, None) if !raw.ends_with('\n') => {
                return Err(format!(
                    "the file ends inside its hunk at line {}",
                    hunk.line
                ));
            }
            (Marker::Start, None) => hunk.ours.push(line.to_string()),
            (Marker::Base, None) => hunk.base.push(line.to_string()),
            (Marker::Middle, None) => hunk.theirs.push(line.to_string()),
            (Marker::End, None) => unreachable!("a closed hunk is not open"),
        }
    }
    if let Some((hunk, _)) = open {
        return Err(format!(
            "the file ends inside its hunk at line {}",
            hunk.line
        ));
    }
    Ok(parts)
}

/// Settles every hunk of a conflicted file, or says why the first that cannot be settled
/// stops, naming the path and the hunk's line and first line.
pub fn settle_text(path: &str, kind: Kind, text: &str) -> Result<Settled, String> {
    let parts = parse(text).map_err(|why| format!("{path}: {why}"))?;
    let mut lines: Vec<String> = Vec::new();
    let mut settled = Settled {
        text: String::new(),
        by_words: 0,
        kept_both: 0,
    };
    for (index, part) in parts.iter().enumerate() {
        match part {
            Part::Text(raw) => lines.push(raw.clone()),
            Part::Hunk(hunk) => {
                let below = match parts.get(index + 1) {
                    Some(Part::Text(raw)) => Some(raw.trim_end_matches('\n')),
                    Some(Part::Hunk(next)) => Some(next.first_line()),
                    None => None,
                };
                let (merged, how) = resolve(kind, &lines, hunk, below).map_err(|why| {
                    format!(
                        "{path}: its hunk at line {} stops: {why}; the hunk's first line: {}",
                        hunk.line,
                        crate::cut(hunk.first_line(), 200)
                    )
                })?;
                match how {
                    How::ByWords => settled.by_words += 1,
                    How::KeptBoth => settled.kept_both += 1,
                }
                lines.extend(merged.into_iter().map(|line| format!("{line}\n")));
            }
        }
    }
    if settled.by_words + settled.kept_both == 0 {
        return Err(format!("{path}: no conflict hunk in the file"));
    }
    settled.text = lines.concat();
    Ok(settled)
}

/// Settles one hunk, given the settled lines above it and the file's line after it, or says why
/// it stops.
pub fn resolve(
    kind: Kind,
    above: &[String],
    hunk: &Hunk,
    below: Option<&str>,
) -> Result<(Vec<String>, How), String> {
    let Hunk {
        ours, base, theirs, ..
    } = hunk;
    match kind {
        Kind::Changelog => {
            if !(ours.starts_with(base) && theirs.starts_with(base)) {
                return Err("ours and theirs do not each begin with the base's lines".into());
            }
            let context: Vec<String> = above.iter().chain(base).cloned().collect();
            let added = keep_both(
                kind,
                &context,
                &ours[base.len()..],
                &theirs[base.len()..],
                below,
            )?;
            let mut merged = base.clone();
            merged.extend(added);
            Ok((merged, How::KeptBoth))
        }
        Kind::Yaml if base.is_empty() => {
            Err("both sides add lines at one place, which stops in YAML".into())
        }
        Kind::Markdown if base.is_empty() => {
            keep_both(kind, above, ours, theirs, below).map(|merged| (merged, How::KeptBoth))
        }
        _ => word_merge(kind, above, base, ours, theirs).map(|merged| (merged, How::ByWords)),
    }
}

fn indent_of(line: &str) -> usize {
    line.len() - line.trim_start_matches(' ').len()
}

fn is_blank(line: &str) -> bool {
    line.trim().is_empty()
}

/// A Markdown line that opens a block of its own wherever it stands: an ATX heading, a code
/// fence, a block quote, a table row or a list item that is no setext underline or thematic
/// break.
fn opens_block(line: &str) -> bool {
    let rest = line.trim_start_matches(' ');
    is_atx(rest)
        || is_fence(rest)
        || rest.starts_with('>')
        || rest.starts_with('|')
        || (list_marker(rest).is_some() && !is_underline(rest))
}

/// A Markdown line that continues the block above it: not blank, opening no block and no
/// setext underline.
fn continues(line: &str) -> bool {
    !is_blank(line) && !opens_block(line) && !is_underline(line.trim_start_matches(' '))
}

/// The least indent of the lines that are not blank.
fn least_indent(lines: &[String]) -> Option<usize> {
    lines
        .iter()
        .filter(|line| !is_blank(line))
        .map(|line| indent_of(line))
        .min()
}

/// Whether `line` keeps the block it belongs to once it follows `lines` rather than the line it
/// followed on its own side: it opens a block of its own, or follows a blank line on each side,
/// no deeper than any of `lines`, which it could otherwise nest under; or each of `lines`
/// continues the block it continued.
fn keeps_its_block(line: &str, after_blank: bool, lines: &[String]) -> bool {
    let fresh = opens_block(line) || after_blank;
    let own = fresh && least_indent(lines).is_none_or(|least| indent_of(line) <= least);
    own || lines.iter().all(|line| continues(line))
}

/// Keeps ours' added lines and then theirs', each whole, the non-blank lines both begin with
/// once, given the lines above them and the file's line after them. Stops where a line would
/// then belong to another block than on its own side: where ours' lines open or close a code
/// fence, where theirs' first non-blank line would continue a block ours' lines end, or nest
/// under ours' item, and where the line after the hunk would continue theirs' last block, not
/// ours', or a blank line would now part it from the block it continued. In the changelog
/// theirs' first non-blank line must also begin an item, `- `.
fn keep_both(
    kind: Kind,
    above: &[String],
    ours: &[String],
    theirs: &[String],
    below: Option<&str>,
) -> Result<Vec<String>, String> {
    let shared = ours
        .iter()
        .zip(theirs)
        .take_while(|(mine, other)| mine == other && !is_blank(mine))
        .count();
    let (ours_rest, theirs_rest) = (&ours[shared..], &theirs[shared..]);
    let mut merged = ours.to_vec();
    merged.extend_from_slice(theirs_rest);
    if ours_rest.is_empty() || theirs_rest.is_empty() {
        return Ok(merged);
    }
    let before: Vec<String> = above.iter().chain(&ours[..shared]).cloned().collect();
    let fenced = in_fence(&before);
    let after_ours: Vec<String> = before.iter().chain(ours_rest).cloned().collect();
    if in_fence(&after_ours) != fenced {
        return Err(
            "ours' added lines open or close a code fence, so theirs' would fall on its other side"
                .into(),
        );
    }
    if fenced {
        return Ok(merged);
    }
    if let Some(first) = theirs_rest.iter().find(|line| !is_blank(line)) {
        let after_blank = theirs_rest.first().is_some_and(|line| is_blank(line));
        if !keeps_its_block(first, after_blank, ours_rest) {
            return Err(format!(
                "theirs' first added line continues the block above the hunk, which ours' added lines end or nest it under: {}",
                crate::cut(first, 200)
            ));
        }
        if kind == Kind::Changelog && !first.starts_with("- ") {
            return Err(format!(
                "theirs' first added line does not begin an item, `- `, so it would join ours' last: {}",
                crate::cut(first, 200)
            ));
        }
    }
    let ends_blank = |lines: &[String]| lines.last().is_some_and(|line| is_blank(line));
    if let Some(next) = below.filter(|line| !is_blank(line))
        && !keeps_its_block(
            next,
            ends_blank(ours_rest) && ends_blank(theirs_rest),
            theirs_rest,
        )
    {
        return Err(format!(
            "the line after the hunk continues the block that ours' and theirs' added lines each end, so it would leave ours' for theirs': {}",
            crate::cut(next, 200)
        ));
    }
    Ok(merged)
}

/// One side of a hunk read as one run of text lines.
struct Side {
    /// The first line's indent and, in Markdown, its list marker.
    prefix: String,
    table: bool,
    words: Vec<String>,
}

fn list_marker(rest: &str) -> Option<usize> {
    let bullet = rest.starts_with(['-', '*', '+']);
    let digits = rest.chars().take_while(char::is_ascii_digit).count();
    let mark = if bullet {
        1
    } else if (1..=9).contains(&digits) && rest[digits..].starts_with(['.', ')']) {
        digits + 1
    } else {
        return None;
    };
    let after = &rest[mark..];
    if after.is_empty() {
        return Some(mark);
    }
    let spaces = after.chars().take_while(|c| *c == ' ').count();
    (spaces > 0).then_some(mark + spaces)
}

fn is_fence(rest: &str) -> bool {
    rest.starts_with("```") || rest.starts_with("~~~")
}

/// An ATX heading: one to six `#`, then whitespace or the line's end.
fn is_atx(rest: &str) -> bool {
    let hashes = rest.chars().take_while(|c| *c == '#').count();
    (1..=6).contains(&hashes)
        && rest[hashes..]
            .chars()
            .next()
            .is_none_or(char::is_whitespace)
}

/// A setext underline or a thematic break: one sign alone, whitespace aside. A setext
/// underline may be one `=` or one `-`.
fn is_underline(rest: &str) -> bool {
    let squeezed: String = rest.chars().filter(|c| !c.is_whitespace()).collect();
    let run =
        |sign: char, least: usize| squeezed.len() >= least && squeezed.chars().all(|c| c == sign);
    run('=', 1) || run('-', 1) || run('*', 3) || run('_', 3)
}

/// A Markdown heading, a setext underline or a thematic break.
fn is_heading(rest: &str) -> bool {
    is_atx(rest) || is_underline(rest)
}

/// Whether a line after the first of merged words would open a block, or in YAML a comment, a
/// key, a list item or a document marker, where the words it holds continued one.
fn begins_block(kind: Kind, line: &str) -> bool {
    let rest = line.trim_start_matches(' ');
    match kind {
        Kind::Yaml => {
            rest.starts_with('#')
                || yaml_opens(rest)
                || line.starts_with("---")
                || line.starts_with("...")
        }
        _ => {
            list_marker(rest).is_some()
                || rest.starts_with('|')
                || is_heading(rest)
                || rest.starts_with('>')
                || rest.starts_with('<')
                || is_fence(rest)
        }
    }
}

/// A YAML line that opens a key or a list item.
fn yaml_opens(rest: &str) -> bool {
    if rest == "-" || rest.starts_with("- ") || rest == "?" || rest.starts_with("? ") {
        return true;
    }
    let mut previous = ' ';
    for (index, c) in rest.char_indices() {
        if c == '#' && previous == ' ' && index > 0 {
            return false;
        }
        if c == ':'
            && index > 0
            && rest[index + 1..]
                .chars()
                .next()
                .is_none_or(|next| next == ' ' || next == '\t')
        {
            return true;
        }
        previous = c;
    }
    false
}

impl Side {
    fn read(kind: Kind, name: &str, lines: &[String]) -> Result<Self, String> {
        let Some(first) = lines.first() else {
            return Err(format!("{name} is empty"));
        };
        for (index, line) in lines.iter().enumerate() {
            let rest = line.trim_start_matches(' ');
            if line.contains('\r') {
                return Err(format!("{name} holds a carriage return"));
            }
            if rest.trim().is_empty() {
                return Err(format!("{name} holds a blank line"));
            }
            if rest.starts_with('\t') {
                return Err(format!("{name} indents a line with a tab"));
            }
            if is_fence(rest) {
                return Err(format!("{name} holds a code fence"));
            }
            match kind {
                Kind::Yaml
                    if rest.starts_with('#') || rest.contains(" #") || rest.contains("\t#") =>
                {
                    return Err(format!("{name} holds a YAML comment"));
                }
                Kind::Yaml if yaml_opens(rest) => {
                    return Err(format!(
                        "{name} holds a line opening a YAML key or list item"
                    ));
                }
                Kind::Markdown if is_heading(rest) => {
                    return Err(format!("{name} holds a heading or a thematic break"));
                }
                Kind::Markdown if rest.starts_with('>') => {
                    return Err(format!("{name} holds a block quote"));
                }
                Kind::Markdown if line.ends_with("  ") || line.ends_with('\\') => {
                    return Err(format!("{name} holds a hard line break"));
                }
                _ => {}
            }
            if index > 0 && list_marker(rest).is_some() {
                return Err(format!("{name} begins a list item after its first line"));
            }
            if index > 0 && rest.starts_with('|') {
                return Err(format!("{name} begins a table row after its first line"));
            }
        }
        let indent = first.len() - first.trim_start_matches(' ').len();
        let rest = &first[indent..];
        let table = rest.starts_with('|');
        if table && lines.len() > 1 {
            return Err(format!("{name} is a table row with another line after it"));
        }
        let marker = match kind {
            Kind::Markdown => list_marker(rest).unwrap_or(0),
            _ => 0,
        };
        let prefix = first[..indent + marker].to_string();
        let column = prefix.len();
        for (index, line) in lines.iter().enumerate().skip(1) {
            let spaces = line.len() - line.trim_start_matches(' ').len();
            if spaces != column {
                return Err(format!(
                    "line {} of {name} is indented {spaces}, not to column {column}, where its first line's text begins",
                    index + 1
                ));
            }
        }
        let words: Vec<String> = std::iter::once(&first[prefix.len()..])
            .chain(lines[1..].iter().map(String::as_str))
            .flat_map(str::split_whitespace)
            .map(str::to_string)
            .collect();
        if words.is_empty() {
            return Err(format!("{name} has no words"));
        }
        Ok(Self {
            prefix,
            table,
            words,
        })
    }
}

/// Whether the settled lines above leave a Markdown code fence open.
fn in_fence(above: &[String]) -> bool {
    let mut open: Option<(char, usize)> = None;
    for line in above {
        let rest = line.trim();
        let Some(sign) = rest.chars().next().filter(|c| *c == '`' || *c == '~') else {
            continue;
        };
        let run = rest.chars().take_while(|c| *c == sign).count();
        match open {
            Some((open_sign, open_run))
                if sign == open_sign && run >= open_run && rest[run..].trim().is_empty() =>
            {
                open = None
            }
            None if run >= 3 => open = Some((sign, run)),
            _ => {}
        }
    }
    open.is_some()
}

/// A YAML line without its comment: empty for a comment line, else cut before ` #`.
fn yaml_text(line: &str) -> &str {
    let line = line.trim_end();
    if line.trim_start().starts_with('#') {
        return "";
    }
    match line.find(" #").into_iter().chain(line.find("\t#")).min() {
        Some(at) => line[..at].trim_end(),
        None => line,
    }
}

/// The block scalar indicator a YAML line, its comment aside, ends in: `|` or `>`, then any
/// chomping and indentation indicators.
fn block_indicator(text: &str) -> Option<&str> {
    let token = text.rsplit([' ', '\t']).next()?;
    let mut chars = token.chars();
    let sign = chars.next()?;
    ("|>".contains(sign) && chars.all(|c| "+-0123456789".contains(c))).then_some(token)
}

/// Whether `token`, the last of `text`, is a value's: it follows a key's `:`, a list item's
/// `-` or a complex key's `?`, after any tag or anchor.
fn opens_value(text: &str, token: &str) -> bool {
    let mut before = text[..text.len() - token.len()].trim_end();
    if let Some((rest, last)) = before.rsplit_once([' ', '\t'])
        && (last.starts_with('!') || last.starts_with('&'))
    {
        before = rest.trim_end();
    }
    let alone = before.trim_start();
    before.ends_with(':') || alone == "-" || alone == "?" || before.ends_with(" -")
}

const LITERAL: &str =
    "the hunk lies inside a YAML literal block scalar, whose line breaks are its text";
const FOLDED: &str = "the hunk lies inside a YAML folded block scalar, more indented than its text or under an indentation indicator, where its line breaks are its text";

/// Why a YAML hunk indented `indent` lies where its line breaks are text, given the settled
/// lines above: inside a literal block scalar (`|`), or a folded one (`>`) more indented than
/// the block's text or under an indentation indicator. Each line above less indented than all
/// below it, up to column 0, is read, its comment aside. A `|` at the end of any of them stops,
/// whatever precedes it; a `>` counts only as a value's.
fn in_block_scalar(above: &[String], indent: usize) -> Option<&'static str> {
    let mut least = indent;
    let mut next = indent;
    let mut folded: Option<bool> = None;
    for line in above.iter().rev() {
        let text = yaml_text(line);
        if text.trim().is_empty() {
            continue;
        }
        let spaces = indent_of(text);
        if spaces < least {
            match block_indicator(text) {
                Some(token) if token.starts_with('|') => return Some(LITERAL),
                Some(token) if folded.is_none() && opens_value(text, token) => {
                    let indicated = token.chars().any(|c| c.is_ascii_digit());
                    folded = Some(indicated || indent > next);
                }
                _ => {}
            }
            if spaces == 0 {
                break;
            }
            least = spaces;
        }
        next = spaces;
    }
    folded.filter(|stops| *stops).map(|_| FOLDED)
}

/// One side's edit against the base: base words `start..end` become `words`.
#[derive(Debug, Clone)]
struct Edit {
    start: usize,
    end: usize,
    words: Vec<String>,
    ours: bool,
}

/// The edits that turn `base` into `side`, by a longest common subsequence of words.
fn diff(base: &[String], side: &[String], ours: bool) -> Result<Vec<Edit>, String> {
    let mut head = 0;
    while head < base.len() && head < side.len() && base[head] == side[head] {
        head += 1;
    }
    let mut tail = 0;
    while tail < base.len() - head
        && tail < side.len() - head
        && base[base.len() - 1 - tail] == side[side.len() - 1 - tail]
    {
        tail += 1;
    }
    let b = &base[head..base.len() - tail];
    let s = &side[head..side.len() - tail];
    let width = s.len() + 1;
    if (b.len() + 1).saturating_mul(width) > MOST_CELLS {
        return Err("the hunk is too long to merge by words".into());
    }
    let mut common = vec![0u32; (b.len() + 1) * width];
    for i in (0..b.len()).rev() {
        for j in (0..s.len()).rev() {
            common[i * width + j] = if b[i] == s[j] {
                common[(i + 1) * width + j + 1] + 1
            } else {
                common[(i + 1) * width + j].max(common[i * width + j + 1])
            };
        }
    }
    let mut edits = Vec::new();
    let mut open: Option<Edit> = None;
    let (mut i, mut j) = (0, 0);
    while i < b.len() || j < s.len() {
        if i < b.len() && j < s.len() && b[i] == s[j] {
            edits.extend(open.take());
            i += 1;
            j += 1;
            continue;
        }
        let edit = open.get_or_insert(Edit {
            start: head + i,
            end: head + i,
            words: Vec::new(),
            ours,
        });
        if i < b.len() && (j == s.len() || common[(i + 1) * width + j] >= common[i * width + j + 1])
        {
            i += 1;
            edit.end = head + i;
        } else {
            edit.words.push(s[j].clone());
            j += 1;
        }
    }
    edits.extend(open);
    Ok(edits)
}

/// Base words, quoted.
fn quoted(words: &[String]) -> String {
    format!("\"{}\"", words.join(" "))
}

fn meet(base: &[String], start: usize, end: usize) -> String {
    let at = if start == end {
        "one place".to_string()
    } else {
        quoted(&base[start..end])
    };
    let before = &base[start.saturating_sub(5)..start];
    let after = &base[end..(end + 5).min(base.len())];
    let before = if before.is_empty() {
        "at the hunk's start".to_string()
    } else {
        format!("after {}", quoted(before))
    };
    let after = if after.is_empty() {
        "at the hunk's end".to_string()
    } else {
        format!("before {}", quoted(after))
    };
    format!(
        "the two sides' edits have no unchanged base word between them: they meet at {at}, {before}, {after}"
    )
}

/// Merges one hunk by words, or says why it stops.
fn word_merge(
    kind: Kind,
    above: &[String],
    base: &[String],
    ours: &[String],
    theirs: &[String],
) -> Result<Vec<String>, String> {
    let b = Side::read(kind, "the base", base)?;
    let o = Side::read(kind, "ours", ours)?;
    let t = Side::read(kind, "theirs", theirs)?;
    let table = b.table || o.table || t.table;
    if table && !(b.table && o.table && t.table) {
        return Err("a table row on one side is not one on another".into());
    }
    if t.prefix != b.prefix && t.prefix != o.prefix {
        return Err(
            "theirs changes the first line's indent or list marker, and the result keeps ours'"
                .into(),
        );
    }
    match kind {
        Kind::Markdown if in_fence(above) => {
            return Err("the hunk lies inside a code fence".into());
        }
        Kind::Yaml => {
            if let Some(why) = in_block_scalar(above, b.prefix.len()) {
                return Err(why.into());
            }
        }
        _ => {}
    }
    let words = &b.words;
    let end = words.len();
    let mut edits = diff(words, &o.words, true)?;
    edits.extend(diff(words, &t.words, false)?);
    edits.sort_by_key(|edit| (edit.start, edit.end, !edit.ours));
    for pair in edits.windows(2) {
        let (first, second) = (&pair[0], &pair[1]);
        if first.ours == second.ours {
            continue;
        }
        let both_at_end = [first, second]
            .iter()
            .all(|edit| edit.start == end && edit.end == end);
        if both_at_end && !table {
            continue;
        }
        if second.start <= first.end {
            return Err(meet(
                words,
                first.start.min(second.start),
                first.end.max(second.end),
            ));
        }
    }
    let mut merged = Vec::new();
    let mut at = 0;
    for edit in &edits {
        merged.extend_from_slice(&words[at..edit.start.max(at)]);
        merged.extend(edit.words.iter().cloned());
        at = at.max(edit.end);
    }
    merged.extend_from_slice(&words[at..]);
    if merged.is_empty() {
        return Err("the merge leaves no words".into());
    }
    Ok(layout(kind, &o.prefix, table, &merged, ours, theirs))
}

/// Wraps words at `width`, never before a word `glued` marks: each line and the index of its
/// first word.
fn wrap(prefix: &str, width: usize, words: &[String], glued: &[bool]) -> Vec<(String, usize)> {
    let indent = " ".repeat(prefix.chars().count());
    let mut lines = Vec::new();
    let mut line = prefix.to_string();
    let mut first = 0;
    for (index, word) in words.iter().enumerate() {
        if index == 0 {
            line.push_str(word);
        } else if glued[index] || line.chars().count() + 1 + word.chars().count() <= width {
            line.push(' ');
            line.push_str(word);
        } else {
            let done = std::mem::replace(&mut line, format!("{indent}{word}"));
            lines.push((done, first));
            first = index;
        }
    }
    lines.push((line, first));
    lines
}

/// Lays out merged words: one line for a table row, else wrapped at the widest line of ours'
/// and theirs' sides, the first line after ours' prefix and the others indented to its text.
/// A line after the first never begins with a word that would open a block there, as
/// [`begins_block`] says, and no line but the last ends in a backslash, a Markdown hard line
/// break: such a word stays on the line above, past the width.
fn layout(
    kind: Kind,
    prefix: &str,
    table: bool,
    words: &[String],
    ours: &[String],
    theirs: &[String],
) -> Vec<String> {
    if table {
        return vec![format!("{prefix}{}", words.join(" "))];
    }
    let width = ours
        .iter()
        .chain(theirs)
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);
    let mut glued = vec![false; words.len()];
    loop {
        let lines = wrap(prefix, width, words, &glued);
        let wrong = lines.windows(2).find(|pair| {
            let ((above, _), (line, _)) = (&pair[0], &pair[1]);
            above.ends_with('\\') || begins_block(kind, line)
        });
        match wrong {
            Some(pair) => glued[pair[1].1] = true,
            None => return lines.into_iter().map(|(line, _)| line).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Hunk, Kind, kind_of, resolve, settle_text};

    fn lines(text: &[&str]) -> Vec<String> {
        text.iter().map(|line| line.to_string()).collect()
    }

    fn merge(
        kind: Kind,
        base: &[&str],
        ours: &[&str],
        theirs: &[&str],
    ) -> Result<Vec<String>, String> {
        let hunk = Hunk {
            line: 1,
            ours: lines(ours),
            base: lines(base),
            theirs: lines(theirs),
        };
        resolve(kind, &[], &hunk, None).map(|(merged, _)| merged)
    }

    #[track_caller]
    fn merges(kind: Kind, base: &[&str], ours: &[&str], theirs: &[&str], want: &[&str]) {
        assert_eq!(merge(kind, base, ours, theirs), Ok(lines(want)));
    }

    #[track_caller]
    fn stops(kind: Kind, base: &[&str], ours: &[&str], theirs: &[&str], why: &str) {
        match merge(kind, base, ours, theirs) {
            Ok(merged) => panic!("merged as {merged:?}, not stopped with {why}"),
            Err(error) => assert!(error.contains(why), "{why}: {error}"),
        }
    }

    #[test]
    fn word_merge_cases() {
        use Kind::{Markdown as Md, Yaml};
        let meet = "no unchanged base word between them";
        // Disjoint edits merge, in Markdown and in YAML.
        merges(
            Md,
            &["a b c d e"],
            &["a B c d e"],
            &["a b c D e"],
            &["a B c D e"],
        );
        merges(
            Yaml,
            &["    one two three", "    four five six"],
            &["    ONE two three", "    four five six"],
            &["    one two three", "    four five SIX"],
            &["    ONE two three", "    four five SIX"],
        );
        // Adjacent, overlapping and same-place edits stop.
        stops(Md, &["a b c d e"], &["a B c d e"], &["a b C d e"], meet);
        stops(Md, &["a b c d e"], &["a B C d e"], &["a b C D e"], meet);
        stops(Md, &["a b c d e"], &["a B c d e"], &["a X c d e"], meet);
        stops(
            Md,
            &["a b c d e"],
            &["a x b c d e"],
            &["a y b c d e"],
            "meet at one place",
        );
        stops(
            Md,
            &["a b c"],
            &["x a b c"],
            &["y a b c"],
            "at the hunk's start",
        );
        // Both inserting at the end merges, ours first, but stops in a table row.
        merges(Md, &["a b"], &["a b x"], &["a b y"], &["a b x", "y"]);
        stops(
            Md,
            &["| a | b |"],
            &["| a | b | x |"],
            &["| a | b | y |"],
            "at the hunk's end",
        );
        // The overlap names up to five base words on each side.
        stops(
            Md,
            &["w1 w2 w3 w4 w5 w6 w7 w8 w9 w10 w11 w12 w13"],
            &["w1 w2 w3 w4 w5 w6 OURS w8 w9 w10 w11 w12 w13"],
            &["w1 w2 w3 w4 w5 w6 THEIRS w8 w9 w10 w11 w12 w13"],
            "meet at \"w7\", after \"w2 w3 w4 w5 w6\", before \"w8 w9 w10 w11 w12\"",
        );
        // A table row stays one line, however wide.
        merges(
            Md,
            &["| a | b | c |"],
            &["| a | a longer cell | c |"],
            &["| a | b | another longer cell |"],
            &["| a | a longer cell | another longer cell |"],
        );
        // A list item keeps its marker and its continuation indent, wrapped at the widest line.
        merges(
            Md,
            &["- alpha beta gamma", "  delta epsilon"],
            &["- ALPHA beta gamma", "  delta epsilon"],
            &["- alpha beta gamma", "  delta EPSILON"],
            &["- ALPHA beta gamma", "  delta EPSILON"],
        );
        merges(
            Md,
            &["  1. one two three", "     four five"],
            &["  1. one two three", "     four five"],
            &["  1. one two three", "     four five six seven"],
            &["  1. one two three four", "     five six seven"],
        );
        merges(
            Md,
            &["- one two three four"],
            &["- one two three four five"],
            &["- zero one two three four"],
            &["- zero one two three four", "  five"],
        );
        stops(
            Md,
            &["- one two", "  three"],
            &["- one TWO", "  three"],
            &["- one two", "    THREE"],
            "not to column 2",
        );
        // A blank line, a code fence, a heading or a YAML key stops.
        stops(
            Md,
            &["a b", "c d"],
            &["a B", "", "c d"],
            &["a b", "c D"],
            "blank line",
        );
        stops(Md, &["a b"], &["a B"], &["```", "a b"], "code fence");
        stops(Md, &["a b"], &["a B"], &["# a b"], "heading");
        stops(
            Md,
            &["> a b", "> c d"],
            &["> A b", "> c d"],
            &["> a b", "> c D"],
            "block quote",
        );
        stops(
            Yaml,
            &["  description: a b c"],
            &["  description: A b c"],
            &["  description: a b C"],
            "opening a YAML key",
        );
        stops(
            Yaml,
            &["  - a b c"],
            &["  - A b c"],
            &["  - a b C"],
            "opening a YAML key or list item",
        );
        // An empty base keeps two added list items, or table rows, whole; it stops in YAML.
        merges(
            Md,
            &[],
            &["- main's item"],
            &["- the branch's item"],
            &["- main's item", "- the branch's item"],
        );
        merges(
            Md,
            &[],
            &["| x | 1 |"],
            &["| y | 2 |"],
            &["| x | 1 |", "| y | 2 |"],
        );
        stops(Yaml, &[], &["- id: X-1"], &["- id: X-2"], "stops in YAML");
        // A side that adds a list item or a table row after its first line stops.
        stops(
            Md,
            &["- one two"],
            &["- one two", "- three"],
            &["- one TWO"],
            "begins a list item after its first line",
        );
        stops(
            Md,
            &["| a | b |"],
            &["| a | b |", "| c | d |"],
            &["| a | B |"],
            "begins a table row after its first line",
        );
        // A changed marker on theirs alone stops, since the result keeps ours'.
        stops(
            Md,
            &["- a b c"],
            &["- A b c"],
            &["* a b C"],
            "the result keeps ours'",
        );
        // The changelog keeps the base, then ours' added lines, then theirs'.
        assert_eq!(
            merge(
                Kind::Changelog,
                &["- old"],
                &["- old", "- main's"],
                &["- old", "- branch's"]
            ),
            Ok(lines(&["- old", "- main's", "- branch's"]))
        );
        assert!(merge(Kind::Changelog, &["- old"], &["- OLD"], &["- old", "- x"]).is_err());
        // A wrapped line never begins with a word that would open a block, an ordered or a
        // bullet list item here: the word stays on the line above, past the width.
        merges(
            Md,
            &[
                "Each of these checks prints 1. The",
                "`tr` check reads the file.",
            ],
            &[
                "Each of these three checks prints 1. The",
                "`tr` check reads the file.",
            ],
            &[
                "Each other of these checks prints 1. The",
                "`tr` check reads the file.",
            ],
            &[
                "Each other of these three checks prints 1.",
                "The `tr` check reads the file.",
            ],
        );
        merges(
            Md,
            &["- The ranges run 1 - 5 each"],
            &["- The ranges normally run 1 - 5 each"],
            &["- The integer ranges run 1 - 5 each"],
            &["- The integer ranges normally run 1 -", "  5 each"],
        );
        // Nor does a YAML line begin with one that opens a list item, nor any line end in a
        // backslash, a Markdown hard line break.
        merges(
            Yaml,
            &["    alpha run 1 - 5 each"],
            &["    alpha run normally 1 - 5 each"],
            &["    alpha integer run 1 - 5 each"],
            &["    alpha integer run normally 1 -", "    5 each"],
        );
        merges(
            Md,
            &["aa bb cc\\ dddd"],
            &["aa X bb cc\\ dddd"],
            &["aa bb Y cc\\ dddd"],
            &["aa X bb Y cc\\ dddd"],
        );
        // A YAML comment within a line stops, since a merge could move what it holds.
        stops(
            Yaml,
            &["    one two # three", "    four five"],
            &["    ONE two # three", "    four five"],
            &["    one two # three", "    four FIVE"],
            "YAML comment",
        );
        // Theirs' first added line keeps the block it continued: it may continue one only
        // where ours' added lines continue it too, and it may not nest under ours' item.
        stops(
            Md,
            &[],
            &["- three: main adds a rule."],
            &["  and the branch adds a third line to two."],
            "theirs' first added line continues the block above the hunk",
        );
        stops(
            Md,
            &[],
            &["- three: main adds a rule."],
            &["  - a nested item under two"],
            "theirs' first added line continues the block above the hunk",
        );
        merges(
            Md,
            &[],
            &["  and main adds a line to two."],
            &["  and the branch adds one too."],
            &[
                "  and main adds a line to two.",
                "  and the branch adds one too.",
            ],
        );
        merges(
            Md,
            &[],
            &["  and main adds a line to two."],
            &["- three: the branch adds a rule."],
            &[
                "  and main adds a line to two.",
                "- three: the branch adds a rule.",
            ],
        );
        stops(
            Md,
            &[],
            &["```text", "code"],
            &["- an item"],
            "a code fence",
        );
        // A line after a blank one begins a paragraph of its own, but may not nest under ours'
        // item.
        merges(
            Md,
            &[],
            &["", "Main's paragraph."],
            &["", "The branch's paragraph."],
            &["", "Main's paragraph.", "", "The branch's paragraph."],
        );
        stops(
            Md,
            &[],
            &["- three: main adds a rule."],
            &["", "  a second paragraph of two"],
            "theirs' first added line continues the block above the hunk",
        );
        // In the changelog theirs' first added line must begin an item, ours' having added one.
        assert!(
            merge(
                Kind::Changelog,
                &[],
                &["- Added: main item (DEC-2)."],
                &["  and the branch extends it (DEC-3)."]
            )
            .unwrap_err()
            .contains("theirs' first added line continues the block above the hunk"),
        );
        assert!(
            merge(
                Kind::Changelog,
                &["- old"],
                &["- old", "  and main extends it."],
                &["- old", "  and the branch extends it."]
            )
            .unwrap_err()
            .contains("does not begin an item"),
        );
        assert_eq!(
            merge(
                Kind::Changelog,
                &["- old"],
                &["- old", "  and main extends it."],
                &["- old", "- the branch's"]
            ),
            Ok(lines(&[
                "- old",
                "  and main extends it.",
                "- the branch's"
            ]))
        );
        // A non-blank line both sides add first is kept once; a blank one is kept twice.
        merges(
            Md,
            &[],
            &["- shared", "- ours"],
            &["- shared", "- theirs"],
            &["- shared", "- ours", "- theirs"],
        );
        merges(
            Md,
            &[],
            &["", "- ours"],
            &["", "- theirs"],
            &["", "- ours", "", "- theirs"],
        );
        assert_eq!(
            merge(
                Kind::Changelog,
                &["- old"],
                &["- old", "- Added: shared.", "- main's"],
                &["- old", "- Added: shared.", "- branch's"]
            ),
            Ok(lines(&[
                "- old",
                "- Added: shared.",
                "- main's",
                "- branch's"
            ]))
        );
    }

    #[test]
    fn stops_where_a_merge_would_move_a_line() {
        // The line after the hunk continued ours' last block, and would continue theirs'.
        let below = "- one\n<<<<<<< HEAD\n- main's\n||||||| base\n=======\n- branch's\n>>>>>>> theirs\n  continuation of one\n";
        let stopped = settle_text("spec/a.md", Kind::Markdown, below).unwrap_err();
        assert!(
            stopped.contains("the line after the hunk continues the block"),
            "{stopped}"
        );
        let changelog = below
            .replace("main's", "Added: main's")
            .replace("branch's", "Added: the branch's");
        assert!(
            settle_text("spec/CHANGES.md", Kind::Changelog, &changelog)
                .unwrap_err()
                .contains("the line after the hunk continues the block")
        );
        let blank = "- one\n<<<<<<< HEAD\n- main's\n||||||| base\n=======\n- branch's\n>>>>>>> theirs\n\nA paragraph.\n";
        assert_eq!(
            settle_text("spec/a.md", Kind::Markdown, blank)
                .unwrap()
                .text,
            "- one\n- main's\n- branch's\n\nA paragraph.\n"
        );
        // A paragraph after the hunk stays one of its own where each side ends in a blank line,
        // and stops where only theirs does, since it continued ours' last.
        let paragraphs = "Above.\n<<<<<<< HEAD\n\nMain's paragraph.\n\n||||||| base\n=======\n\nThe branch's paragraph.\n\n>>>>>>> theirs\nBelow.\n";
        assert_eq!(
            settle_text("spec/a.md", Kind::Markdown, paragraphs)
                .unwrap()
                .text,
            "Above.\n\nMain's paragraph.\n\n\nThe branch's paragraph.\n\nBelow.\n"
        );
        let lazy = paragraphs.replace("Main's paragraph.\n\n", "Main's paragraph.\n");
        assert!(
            settle_text("spec/a.md", Kind::Markdown, &lazy)
                .unwrap_err()
                .contains("the line after the hunk continues the block")
        );
        // A YAML hunk inside a literal block scalar stops, however indented, and with a
        // comment after the block's indicator.
        let hunk = "<<<<<<< HEAD\n      alpha gamma\n      delta epsilon zeta\n||||||| base\n      alpha beta gamma\n      delta epsilon zeta\n=======\n      alpha beta gamma\n      delta epsilon ZETA\n>>>>>>> theirs\n";
        for opener in ["  example: |", "  example: | # kept verbatim", "  - |-"] {
            let text = format!("a:\n{opener}\n    first line of the literal\n{hunk}");
            let stopped = settle_text("spec/a.yaml", Kind::Yaml, &text).unwrap_err();
            assert!(
                stopped.contains("literal block scalar"),
                "{opener}: {stopped}"
            );
        }
        let commented = format!(
            "a:\n  example: | # kept verbatim\n{}",
            hunk.replace("      ", "    ")
        );
        assert!(
            settle_text("spec/a.yaml", Kind::Yaml, &commented)
                .unwrap_err()
                .contains("literal block scalar")
        );
        // In a folded one, a hunk more indented than the block's text stops; one at its
        // indent merges.
        let folded = format!("a:\n  example: >-\n    first line of the folded text\n{hunk}");
        assert!(
            settle_text("spec/a.yaml", Kind::Yaml, &folded)
                .unwrap_err()
                .contains("more indented than its text")
        );
        let level = format!("a:\n  example: >-\n{}", hunk.replace("      ", "    "));
        assert_eq!(
            settle_text("spec/a.yaml", Kind::Yaml, &level).unwrap().text,
            "a:\n  example: >-\n    alpha gamma delta\n    epsilon ZETA\n"
        );
        // A plain scalar under a key whose sibling above opens a literal merges.
        let sibling = format!(
            "a:\n  example: |\n    literal\n  other: plain text\n{}",
            hunk.replace("      ", "    ")
        );
        assert!(settle_text("spec/a.yaml", Kind::Yaml, &sibling).is_ok());
    }

    #[test]
    fn settles_a_diff3_file_and_names_a_stop() {
        let text = "# T\n\n<<<<<<< HEAD\nAlpha BETA gamma delta.\n||||||| base\nAlpha beta gamma delta.\n=======\nAlpha beta gamma DELTA.\n>>>>>>> theirs\n\n- one\n<<<<<<< HEAD\n- main's\n||||||| base\n=======\n- branch's\n>>>>>>> theirs\n";
        let settled = settle_text("spec/a.md", Kind::Markdown, text).unwrap();
        assert_eq!(
            settled.text,
            "# T\n\nAlpha BETA gamma DELTA.\n\n- one\n- main's\n- branch's\n"
        );
        assert_eq!((settled.by_words, settled.kept_both), (1, 1));
        let fenced = "```text\n<<<<<<< HEAD\na B c\n||||||| base\na b c\n=======\na b C\n>>>>>>> theirs\n```\n";
        let stopped = settle_text("spec/a.md", Kind::Markdown, fenced).unwrap_err();
        assert!(
            stopped.starts_with("spec/a.md: its hunk at line 2 stops: the hunk lies inside a code fence; the hunk's first line: a B c"),
            "{stopped}"
        );
        let literal = "a:\n  b: |-\n<<<<<<< HEAD\n    x Y z\n||||||| base\n    x y z\n=======\n    X y z\n>>>>>>> theirs\n";
        assert!(
            settle_text("spec/a.yaml", Kind::Yaml, literal)
                .unwrap_err()
                .contains("literal block scalar")
        );
        let doubled = "<<<<<<< HEAD\na\n||||||| base\n=======\nb\n=======\nc\n>>>>>>> theirs\n";
        assert!(
            settle_text("spec/a.md", Kind::Markdown, doubled)
                .unwrap_err()
                .contains("marker out of place")
        );
    }

    #[test]
    fn names_the_paths_it_settles() {
        assert_eq!(kind_of("spec/CHANGES.md"), Some(Kind::Changelog));
        assert_eq!(kind_of("spec/README.md"), Some(Kind::Markdown));
        assert_eq!(kind_of("spec/changes/README.md"), Some(Kind::Markdown));
        assert_eq!(kind_of("spec/changes/400-x.md"), None);
        assert_eq!(kind_of("spec/core/operations.yaml"), Some(Kind::Yaml));
        assert_eq!(kind_of("spec/names.json"), None);
        assert_eq!(kind_of("engine/CONFORMANCE.md"), Some(Kind::Markdown));
        assert_eq!(kind_of("engine/STANDING.md"), None);
        assert_eq!(kind_of("docs/work-orders/401b-x.md"), Some(Kind::Markdown));
        assert_eq!(kind_of("docs/work-orders/sub/x.md"), None);
        assert_eq!(kind_of("conformance/expected-failures.yaml"), None);
    }
}
