use fluent::{bundle::FluentBundle, FluentResource};
use unic_langid::LanguageIdentifier;
use std::collections::HashMap;
use std::fs;

pub struct Localizer {
    bundles: HashMap<String, FluentBundle<FluentResource, intl_memoizer::IntlLangMemoizer>>,
}

unsafe impl Send for Localizer {}
unsafe impl Sync for Localizer {}

impl Localizer {
    pub fn new() -> Self {
        let mut bundles = HashMap::new();

        // read all files in locales directory
        for path in fs::read_dir("locales").unwrap() {
            let path = path.unwrap().path();
            if path.is_file() && path.to_str().unwrap().ends_with(".ftl") {
                let file_stem = path.file_stem().unwrap().to_str().unwrap();
                let lang: LanguageIdentifier = file_stem.parse().unwrap();
                let ftl = fs::read_to_string(path).unwrap();
                let resource = FluentResource::try_new(ftl).unwrap();
                let mut bundle = FluentBundle::new(vec![lang.clone()]);
                bundle.add_resource(resource).unwrap();
                bundles.insert(lang.to_string(), bundle);
            }
        }

        Localizer { bundles }
    }

    pub fn translate(&self, lang: &str, key: &str, args: Option<&fluent::FluentArgs>) -> String {
        if let Some(bundle) = self.bundles.get(lang) {
            let msg = bundle.get_message(key).expect(&format!("Message '{}' in language '{}' not found", key, lang));
            let pattern = msg.value().unwrap();
            let mut errors = vec![];
            bundle.format_pattern(pattern, args, &mut errors).to_string()
        } else {
            format!("Translations for language '{}' not found", lang).to_string()
        }
    }
}