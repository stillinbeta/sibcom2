extern crate minify;

use crate::Value;
use std::fmt::{self, Display};
use std::str::FromStr;

use minify::{minify_css, minify_js};

pub(crate) enum Theme {
    SolarizedDark,
    SolarizedLight,
    Terminal,
}

const SOLARIZED_DARK: &str = "solarized-dark";
const SOLARIZED_LIGHT: &str = "solarized-light";
const TERMINAL: &str = "terminal";

impl FromStr for Theme {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            SOLARIZED_DARK => Ok(Theme::SolarizedDark),
            SOLARIZED_LIGHT => Ok(Theme::SolarizedLight),
            TERMINAL => Ok(Theme::Terminal),
            theme => Err(format!("unknown theme {:?}", theme)),
        }
    }
}

impl Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Theme::SolarizedDark => write!(f, "{}", SOLARIZED_DARK),
            Theme::SolarizedLight => write!(f, "{}", SOLARIZED_LIGHT),
            Theme::Terminal => write!(f, "{}", TERMINAL),
        }
    }
}

macro_rules! container {
        ($tag: expr, $class : expr, $body: expr $(, $args: expr)*) => {
            format!("{}{}{}",
                    format!(r#"<{} class="{}">"#, $tag, $class),
                    format!($body, $($args),*),
                    format!("</{}>", $tag)
            )
        }

    }

macro_rules! div {
    ($class : expr, $body: expr $(, $args: expr)*) => {
        container!("div", $class, $body $(, $args)*)
    };
}

macro_rules! span {
    ($class : expr, $body: expr $(, $args: expr)*) => {
        container!("span", $class, $body $(, $args)*)
    };
}

macro_rules! html_page {
    ($title: expr, $theme: expr, $body: expr) => {
        format!(
            r#"<!DOCTYPE html>
<html>
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title}</title>
<link rel="stylesheet" href="//fonts.googleapis.com/css?family=Inconsolata" type="text/css">
<style type="text/css">
{css}
</style>
</head>
<body class="{theme}">
{body}
</body>
<script type="text/javascript">
{js}
</script>
</html>
"#,
            title = $title,
            css = minify_css!("assets/style.css"),
            js = minify_js!("assets/script.js"),
            theme = $theme,
            body = $body
        )
    };
}

impl Value {
    pub(crate) fn to_html(&self) -> String {
        match self {
            Value::String(s) => span!("string", "\"{}\"", htmlescape::encode_minimal(s)),
            Value::Link(link, display) => span!(
                "link",
                r#""<a href="{0}">{1}</a>""#,
                link.to_string(),
                display
            ),
            Value::Boolean(b) => span!("boolean", "{}", b),
            Value::Number(n) => span!("number", "{}", n),
            Value::Null => span!("null", "null"),
            Value::Sequence(s) => format!(
                "{}{}{}",
                span!("bracket-open", "["),
                div!("bracket-inner", "{}", Self::make_rows(s, |v| v.to_html())),
                span!("bracket-close", "]")
            ),
            Value::Object(s) => format!(
                "{}{}{}",
                span!("brace-open", "{{"),
                div!(
                    "brace-inner",
                    "{}",
                    Self::make_rows(s, |(k, v)| {
                        format!(
                            "{}:\n{}",
                            span!("key", "{}", k.to_html()),
                            span!("value", "{}", v.to_html())
                        )
                    })
                ),
                span!("brace-close", "}}")
            ),
        }
    }

    /// Make rows makes a <div class="row"></div>, but with the commas inside the div
    /// a .join() would put them outside the main
    fn make_rows<T, F>(rows: &[T], f: F) -> String
    where
        F: Fn(&T) -> String,
    {
        let mut iter = rows.iter().peekable();
        let mut buf = String::new();
        // Can't use a for loop because that'd take ownership of the iter
        while let Some(val) = iter.next() {
            let mut row = f(val);
            if iter.peek().is_some() {
                row.push(',')
            }
            buf.push_str(&div!("row", "{}", row));
        }
        buf
    }
}

pub(crate) fn render_page(title: &str, theme: Theme, value: &Value) -> String {
    html_page!(title, theme, value.to_html())
}
