#![feature(string_replace_in_place)]

use std::sync::LazyLock;

use itertools::Itertools;

const START_DELIM: (&str, &str) = ("<", ">");
const END_DELIM: (&str, &str) = ("</", ">");
const INLINE_DELIM: (&str, &str) = ("<", " />");

static HEADINGS: LazyLock<[(&str, (String, String)); 3]> = LazyLock::new(|| {
    [("# ", "h1"), ("## ", "h2"), ("### ", "h3")].map(|(from, to)| (from, as_element(to)))
});
static BLOCKQUOTE: LazyLock<(&str, (String, String))> =
    LazyLock::new(|| ("> ", as_element("blockquote")));
static BOLD: LazyLock<(&str, (String, String))> = LazyLock::new(|| ("**", as_element("strong")));
static ITALIC: LazyLock<(&str, (String, String))> = LazyLock::new(|| ("*", as_element("em")));
static SEPARATOR: LazyLock<(&str, String)> = LazyLock::new(|| {
    (
        "****",
        format!("{}{}{}", INLINE_DELIM.0, "hr", INLINE_DELIM.1),
    )
});

#[inline]
fn as_element(to: &str) -> (String, String) {
    (
        format!("{}{}{}", START_DELIM.0, to, START_DELIM.1),
        format!("{}{}{}", END_DELIM.0, to, END_DELIM.1),
    )
}

#[inline]
pub fn markdown_to_html(s: &str) -> String {
    s.lines()
        .map(|l| l.trim_ascii())
        .filter(|l| !l.is_empty())
        .peekable()
        .batching(|it| {
            let is_block = |l: &str| {
                l.starts_with(SEPARATOR.0)
                    || l.starts_with(BLOCKQUOTE.0)
                    || HEADINGS.iter().any(|&(h, _)| l.starts_with(h))
            };

            let line = it.next()?;

            if is_block(line) {
                return Some(line.to_owned());
            }

            Some(
                std::iter::once(line)
                    .chain(std::iter::from_fn(|| {
                        it.next_if(|l| !is_block(l) && !l.is_empty())
                            .map(|s| s.trim_ascii_end())
                    }))
                    .join("\n"),
            )
        })
        .map(|mut chunk| {
            let replace = |chunk: &mut String, &(from, ref to): &(&str, (String, String))| {
                chunk.push_str(&to.1);
                chunk.replace_first(from, &to.0);
            };

            if chunk.starts_with(SEPARATOR.0) {
                chunk.replace_first(SEPARATOR.0, &SEPARATOR.1);
            } else if chunk.starts_with(BLOCKQUOTE.0) {
                replace(&mut chunk, &BLOCKQUOTE);
            } else if let Some(v) = HEADINGS.iter().find(|&&(h, _)| chunk.starts_with(h)) {
                replace(&mut chunk, v);
            }

            chunk
        })
        .map(|mut chunk| {
            let mut replace = |&(from, ref to): &(&str, (String, String))| {
                let mut toggle = false;
                while chunk.contains(from) {
                    toggle = !toggle;
                    chunk.replace_first(from, if toggle { &to.0 } else { &to.1 });
                }
            };

            replace(&BOLD);
            replace(&ITALIC);

            chunk
        })
        .join("\n")
}
