extern crate htmlescape;
extern crate proc_macro2;
extern crate quote;
extern crate rocket;

use quote::quote;
use rocket::handler::{Handler, Outcome};
use rocket::http::{Accept, ContentType, Status};
use rocket::{Data, Request, Response};
use std::io::Cursor;

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

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    String(String),
    Link(String), // Will be hotlinked if rendered as HTML
    RelativeLink(String),
    Sequence(Vec<Value>),
    Object(Vec<(Value, Value)>), // Easier to make literals of
    Number(i64),
    Boolean(bool),
    Null,
}

impl quote::ToTokens for Value {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ts = match self {
            Value::String(s) => quote! { bmon::Value::String(String::from(#s)) },
            Value::Link(s) => quote! { bmon::Value::Link(String::from(#s)) },
            Value::RelativeLink(s) => quote! {  bmon::Value::RelativeLink(String::from(#s)) },
            Value::Number(n) => quote! { bmon::Value::Number(#n) },
            Value::Boolean(b) => quote! { bmon::Value::Boolean(#b) },
            Value::Null => quote! { bmon::Value::Null },
            Value::Sequence(s) => {
                let tokens = s.iter().map(|v| quote! { #v });
                quote! {
                    bmon::Value::Sequence(vec![#(#tokens,)*])
                }
            }
            Value::Object(m) => {
                let keys = m.iter().map(|(k, _)| quote! { #k });
                let values = m.iter().map(|(_, v)| quote! { #v });
                quote!(bmon::Value::Object(vec![#((#keys, #values),)*]))
            }
        };

        tokens.extend(ts);
    }
}

impl Value {
    fn to_json(&self) -> String {
        match self {
            Value::String(s) => format!("{:?}", s),
            Value::Link(s) => format!("{:?}", s),
            Value::RelativeLink(s) => format!("{:?}", s),
            Value::Number(n) => format!("{}", n),
            Value::Boolean(b) => format!("{}", b),
            Value::Null => "null".into(),
            Value::Sequence(s) => format!(
                "[{}]",
                s.iter()
                    .map(|v| v.to_json())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            Value::Object(m) => format!(
                "{{{}}}",
                m.iter()
                    .map(|(k, v)| format!("{}:{}", k.to_json(), v.to_json()))
                    .collect::<Vec<String>>()
                    .join(",")
            ),
        }
    }

    fn to_html(&self) -> String {
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
                "{}{}{}\n",
                span!("bracket-open", "["),
                div!("bracket-inner", "{}", make_rows(s, |v| v.to_html())),
                span!("bracket-close", "]")
            ),
            Value::Object(s) => format!(
                "{}{}{}",
                span!("brace-open", "{{"),
                div!(
                    "brace-inner",
                    "{}",
                    make_rows(s, |(k, v)| {
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

#[derive(Clone, Debug)]
pub struct BMONHandler(pub Value, pub &'static str);

impl Handler for BMONHandler {
    fn handle<'r>(&self, req: &'r Request, _data: Data) -> Outcome<'r> {
        let mut response = Response::new();
        response.set_status(Status::Ok);

        match req.accept() {
            Some(v) if *v == Accept::JSON => {
                response.set_header(ContentType::JSON);
                response.set_sized_body(Cursor::new(self.0.to_json()));
            }
            _ => {
                response.set_header(ContentType::HTML);
                response.set_sized_body(Cursor::new(html_page!(self.1, self.0.to_html())));
            }
        };
        Outcome::from(req, response)
    }
}
