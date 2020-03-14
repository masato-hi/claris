#[macro_export]
macro_rules! parse_yaml {
    ($x:expr) => {{
        let docs = YamlLoader::load_from_str($x).unwrap();
        if docs.is_empty() {
            panic!("empty yaml entry!");
        }
        let doc = &docs[0];
        doc.clone()
    }};
}
