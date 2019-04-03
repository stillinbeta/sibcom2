use crate::Value;

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
    ($title: expr, $body: expr) => {
        format!(
            r#"<!DOCTYPE html>
<html>
<head>
<meta charset="UTF-8">
<title>{}</title>
<link rel="stylesheet" href="//fonts.googleapis.com/css?family=Inconsolata" type="text/css">
<style type="text/css">
{}
</style>
</head>
<body>
{}
</body>
</html>
"#,
            $title,
            include_str!("../../assets/style.css"),
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
            Value::Link(s) => span!(
                "link",
                r#""<a href="https://{0}">{0}</a>""#,
                htmlescape::encode_minimal(s)
            ),
            Value::RelativeLink(s) => span!(
                "link",
                r#""<a href="{0}">{0}</a>""#,
                htmlescape::encode_minimal(s)
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

pub(crate) fn render_page(title: &str, value: &Value) -> String {
    html_page!(title, value.to_html())
}
