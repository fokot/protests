use fluent::{bundle::FluentBundle, FluentResource};
use unic_langid::LanguageIdentifier;
use std::collections::HashMap;
use std::fs;

pub struct Localizer {
    bundles: HashMap<String, FluentBundle<FluentResource, intl_memoizer::IntlLangMemoizer>>,
}

impl Localizer {
    pub fn new() -> Self {
        let mut bundles = HashMap::new();

        // Load English translations
        let en_lang: LanguageIdentifier = "en-US".parse().unwrap();
        let en_ftl = fs::read_to_string("locales/en-US.ftl").unwrap();
        let en_resource = FluentResource::try_new(en_ftl).unwrap();
        let mut en_bundle = FluentBundle::new(vec![en_lang]);
        en_bundle.add_resource(en_resource).unwrap();
        bundles.insert("en-US".to_string(), en_bundle);

        // // Load Spanish translations
        // let es_lang: LanguageIdentifier = "es-ES".parse().unwrap();
        // let es_ftl = fs::read_to_string("locales/es-ES.ftl").unwrap();
        // let es_resource = FluentResource::try_new(es_ftl).unwrap();
        // let mut es_bundle = FluentBundle::new(vec![es_lang]);
        // es_bundle.add_resource(es_resource).unwrap();
        // bundles.insert("es-ES".to_string(), es_bundle);

        Localizer { bundles }
    }

    pub fn translate(&self, lang: &str, key: &str, args: Option<&fluent::FluentArgs>) -> String {
        if let Some(bundle) = self.bundles.get(lang) {
            let msg = bundle.get_message(key).unwrap();
            let pattern = msg.value().unwrap();
            let mut errors = vec![];
            bundle.format_pattern(pattern, args, &mut errors).to_string()
        } else {
            "Translation not found".to_string()
        }
    }
}