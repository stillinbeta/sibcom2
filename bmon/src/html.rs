use crate::Value;
use std::fmt::{self, Display};
use std::str::FromStr;

pub(crate) enum Theme {
    SolarizedDark,
    SolarizedLight,
    Terminal,
}

impl FromStr for Theme {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "solarized-dark" => Ok(Theme::SolarizedDark),
            "solarized-light" => Ok(Theme::SolarizedLight),
            "terminal" => Ok(Theme::Terminal),
            theme => Err(format!("unknown theme {:?}", theme)),
        }
    }
}

impl Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Theme::SolarizedDark => write!(f, "solarized-dark"),
            Theme::SolarizedLight => write!(f, "solarized-light"),
            Theme::Terminal => write!(f, "terminal"),
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
<title>{}</title>
<link rel="stylesheet" href="//fonts.googleapis.com/css?family=Inconsolata" type="text/css">
<style type="text/css">
{}
</style>
<script type="text/javascript">
{}
</script>
</head>
<body class="{}">
{}
</body>
</html>
"#,
            $title,
            include_str!("../../assets/style.css"),
            include_str!("../../assets/script.js"),
            $theme,
            $body
        )
    };
}

impl Value {
    pub(crate) fn to_html(&self) -> String {
        match self {
            Value::String(s) => format!(
                r#""{}""#,
                span!("string", "{}", htmlescape::encode_minimal(s))
            ),
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
    fn make_rows<'a, T, F>(rows: &'a [T], f: F) -> String
    where
        F: Fn(&T) -> String,
    {
        let mut iter = rows.iter().peekable();
        let mut buf = String::new();
        // Can't use a for loop because that'd take ownership of the iter
        while iter.peek().is_some() {
            let val = iter.next().unwrap();
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
