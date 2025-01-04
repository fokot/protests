use fluent::{bundle::FluentBundle, FluentResource};
use unic_langid::LanguageIdentifier;
use std::collections::HashMap;
use std::fs;
use once_cell::sync::Lazy;

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
}

static LOCALIZER: Lazy<Localizer> = Lazy::new(|| {
    println!("Initializing Localizer...");
    Localizer::new()
});

pub fn for_language(lang: String) -> Box<dyn Fn(&str) -> String> {
    let bundle = LOCALIZER.bundles.get(lang.as_str()).expect(&format!("Language '{}' not found", lang));

    Box::new(move |key: &str| {
        let msg = bundle.get_message(key).expect(&format!("Message '{}' in language '{}' not found", key, lang));
        let pattern = msg.value().unwrap();
        let mut errors = vec![];
        bundle.format_pattern(pattern, None, &mut errors).to_string()
    })
}

