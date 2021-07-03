
    use ammonia::Builder;

    pub fn ammonia(s: &str) -> ::askama::Result<String> {
        Ok(Builder::default()
            .set_tag_attribute_value("img", "loading", "lazy")
            .clean(s)
            .to_string())
    }