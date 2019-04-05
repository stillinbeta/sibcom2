extern crate htmlescape;
extern crate proc_macro2;
extern crate quote;
extern crate rocket;
extern crate serde;
extern crate serde_yaml;

mod handler;
mod html;

pub use handler::BMONHandler;
use quote::quote;
use rocket::http::uri::Uri;
use serde::ser::{SerializeMap, SerializeSeq};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    String(String),
    Link(String, String), // Will be hotlinked if rendered as HTML
    Sequence(Vec<Value>),
    Object(Vec<(Value, Value)>), // Easier to make literals of
    Number(i64),
    Boolean(bool),
    Null,
}

impl From<&str> for Value {
    fn from(source: &str) -> Self {
        Value::String(String::from(source))
    }
}

impl quote::ToTokens for Value {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ts = match self {
            Value::String(s) => quote! { bmon::Value::String(String::from(#s)) },
            Value::Link(u, s) => {
                quote! { bmon::Value::Link(String::from(#u), String::from(#s)) }
            }
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
                quote! { bmon::Value::Object(vec![#((#keys, #values),)*]) }
            }
        };

        tokens.extend(ts);
    }
}

impl serde::Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Value::String(s) => serializer.serialize_str(s),
            Value::Link(s, _) => serializer.serialize_str(&s.to_string()),
            Value::Number(n) => serializer.serialize_i64(*n),
            Value::Boolean(b) => serializer.serialize_bool(*b),
            Value::Null => serializer.serialize_unit(),
            Value::Sequence(s) => {
                let mut seq = serializer.serialize_seq(Some(s.len()))?;
                for e in s {
                    seq.serialize_element(e)?;
                }
                seq.end()
            }
            Value::Object(m) => {
                let mut map = serializer.serialize_map(Some(m.len()))?;
                for (k, v) in m {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
        }
    }
}

impl From<serde_yaml::Value> for Value {
    fn from(source: serde_yaml::Value) -> Self {
        match source {
            serde_yaml::Value::Null => Value::Null,
            serde_yaml::Value::Bool(b) => Value::Boolean(b),
            serde_yaml::Value::Number(n) => {
                let v = match n {
                    _ if n.is_f64() => n.as_f64().unwrap().round() as i64,
                    _ if n.is_u64() => n.as_u64().unwrap() as i64,
                    _ if n.is_i64() => n.as_i64().unwrap(),
                    _ => unreachable!(),
                };
                Value::Number(v)
            }
            serde_yaml::Value::String(s) => {
                if s.starts_with('/') {
                    let _ = Uri::parse(&s).expect("invalid relative URL");
                    Value::Link(s.clone(), s)
                } else if s.contains('/') {
                    let uri = format!("https://{}", s);
                    let _ = Uri::parse(&uri).expect("invalid relative URL");
                    Value::Link(uri, s)
                } else {
                    Value::String(s)
                }
            }
            serde_yaml::Value::Sequence(s) => {
                Value::Sequence(s.into_iter().map(Value::from).collect())
            }
            serde_yaml::Value::Mapping(mapping) => Value::Object(
                mapping
                    .into_iter()
                    .map(|(k, v)| (k.into(), v.into()))
                    .collect(),
            ),
        }
    }
}
