extern crate bmon;
extern crate proc_macro2;
extern crate serde_yaml;

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

        // let bmon = value_to_bmon(&obj);
        // assert_eq!(expected.to_string(), quote! { #bmon }.to_string(),);
    }
}
