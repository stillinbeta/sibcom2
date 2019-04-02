extern crate bmon;
extern crate proc_macro2;
extern crate serde_yaml;

pub(crate) fn value_to_bmon(value: &serde_yaml::Value) -> bmon::Value {
    match value {
        serde_yaml::Value::Null => bmon::Value::Null,
        serde_yaml::Value::Bool(b) => bmon::Value::Boolean(*b),
        serde_yaml::Value::Number(n) => {
            let v = match n {
                _ if n.is_f64() => n.as_f64().unwrap().round() as i64,
                _ if n.is_u64() => n.as_u64().unwrap() as i64,
                _ if n.is_i64() => n.as_i64().unwrap(),
                _ => unreachable!(),
            };
            bmon::Value::Number(v)
        }
        serde_yaml::Value::String(s) if s.starts_with('/') => bmon::Value::RelativeLink(s.clone()),
        serde_yaml::Value::String(s) if s.contains('/') => bmon::Value::Link(s.clone()),
        serde_yaml::Value::String(s) => bmon::Value::String(s.clone()),
        serde_yaml::Value::Sequence(seq) => {
            bmon::Value::Sequence(seq.iter().map(value_to_bmon).collect())
        }
        serde_yaml::Value::Mapping(mapping) => bmon::Value::Object(
            mapping
                .iter()
                .map(|(k, v)| (value_to_bmon(k), value_to_bmon(v)))
                .collect(),
        ),
    }
}

#[cfg(test)]
mod tests {
    extern crate quote;

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

        let expected = quote!(bmon::Value::Object(vec![
            (
                bmon::Value::String(String::from("name")),
                bmon::Value::String(String::from("Twilight Sparkle"))
            ),
            (
                bmon::Value::String(String::from("age")),
                bmon::Value::Number(23i64)
            ),
            (
                bmon::Value::String(String::from("parents")),
                bmon::Value::Sequence(vec![
                    bmon::Value::String(String::from("Night Light")),
                    bmon::Value::String(String::from("Twilight Velvet")),
                ])
            ),
        ]));

        let bmon = value_to_bmon(&obj);
        assert_eq!(expected.to_string(), quote! { #bmon }.to_string(),);
    }
}
