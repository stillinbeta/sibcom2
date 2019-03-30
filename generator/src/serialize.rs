extern crate bmon;
extern crate proc_macro2;
extern crate quote;
extern crate serde_yaml;

use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn value_to_bmon(value: &serde_yaml::Value) -> TokenStream {
    match value {
        serde_yaml::Value::Null => quote!(bmon::Value::Null),
        serde_yaml::Value::Bool(b) => quote!( bmon::Value::Boolean(#b)),
        serde_yaml::Value::Number(n) => {
            let v = match n {
                _ if n.is_f64() => n.as_f64().unwrap().round() as i64,
                _ if n.is_u64() => n.as_u64().unwrap() as i64,
                _ if n.is_i64() => n.as_i64().unwrap(),
                _ => unreachable!(),
            };
            quote!(bmon::Value::Number(#v))
        }
        // TODO: links
        serde_yaml::Value::String(s) => quote! ( bmon::Value::String(#s) ),
        serde_yaml::Value::Sequence(seq) => {
            let tokens = seq.iter().map(value_to_bmon);
            quote!( bmon::Value::Sequence([#(#tokens,)*]))
        }
        serde_yaml::Value::Mapping(mapping) => {
            let keys = mapping.iter().map(|(k, _)| value_to_bmon(k));
            let values = mapping.iter().map(|(_, v)| value_to_bmon(v));
            quote!( bmon::Value::Object([#((#keys, #values),)*]))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn object() {
        let obj = serde_yaml::Value::Mapping(
            vec![
                ("name".into(), "Twilight Sparkle".into()),
                ("age".into(), 23.into()),
                (
                    "parents".into(),
                    serde_yaml::Value::Sequence(
                        vec!["Night Light".into(), "Twilight Velvet".into()]
                            .iter()
                            .cloned()
                            .collect(),
                    ),
                ),
            ]
            .iter()
            .cloned()
            .collect(),
        );

        let expected = quote!(bmon::Value::Object([
            (
                bmon::Value::String("name"),
                bmon::Value::String("Twilight Sparkle")
            ),
            (bmon::Value::String("age"), bmon::Value::Number(23i64)),
            (
                bmon::Value::String("parents"),
                bmon::Value::Sequence([
                    bmon::Value::String("Night Light"),
                    bmon::Value::String("Twilight Velvet"),
                ])
            ),
        ]));

        assert_eq!(expected.to_string(), value_to_bmon(&obj).to_string(),);
    }
}
