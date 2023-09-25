#[derive(Debug)]
struct Token {
    start: &'static str,
    end: &'static str,
    r_start: &'static str,
    r_end: &'static str,
    can_be_nested: bool,
}

const TOKENS: [Token; 6] = [
    Token {
        start: "# ",
        end: "\n",
        r_start: "<h1>",
        r_end: "</h1>\n",
        can_be_nested: false,
    },
    Token {
        start: "## ",
        end: "\n",
        r_start: "<h2>",
        r_end: "</h2>\n",
        can_be_nested: false,
    },
    Token {
        start: "### ",
        end: "\n",
        r_start: "<h3>",
        r_end: "</h3>\n",
        can_be_nested: false,
    },
    Token {
        start: "> ",
        end: "\n",
        r_start: "<blockquote>",
        r_end: "</blockquote>\n",
        can_be_nested: true,
    },
    Token {
        start: "**",
        end: "**",
        r_start: "<strong>",
        r_end: "</strong>",
        can_be_nested: true,
    },
    Token {
        start: "*",
        end: "*",
        r_start: "<em>",
        r_end: "</em>",
        can_be_nested: true,
    },
];

pub fn markdown_to_html(s: &str) -> String {
    convert(s, true)
}

pub fn convert(s: &str, is_root: bool) -> String {
    let mut html = String::new();

    let mut i = 0;
    while i < s.len() {
        match get_token(&s[i..], is_root) {
            Some(t) => {
                i += t.start.len();
                html += t.r_start;

                let content_len = get_content_len(t, &s[i..]);
                html += &convert(&s[i..i + content_len], false);

                html += t.r_end;
                i += content_len + t.end.len();
            }
            None => {
                html.push(s.chars().nth(i).unwrap());
                i += 1;
            }
        }
    }
    html
}

fn get_token(s: &str, is_root: bool) -> Option<&'static Token> {
    TOKENS
        .iter()
        .find(|t| (is_root || t.can_be_nested) && s.starts_with(t.start))
}

fn get_content_len(t: &Token, s: &str) -> usize {
    let mut len = 0;
    while len < s.len() {
        if s[len..].starts_with(t.end) {
            break;
        }
        len += 1;
    }

    len
}
