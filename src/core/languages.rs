use crate::utils::languages::{languages as lang_list, Language};
use std::str::from_utf8;

pub fn languages() {
    let languages = lang_list();

    for lang in languages.iter() {
        export_version(&lang);
    }
}

pub fn selected_languages(target: &str) {
    let languages = lang_list();

    for lang in languages.iter() {
        if lang.search(target) {
            export_version(&lang);
        }
    }
}

fn export_version(lang: &Language) {
    let result = lang.run();

    if let Some(result) = result {
        let version = from_utf8(&result.stdout);
        if let Ok(version) = version {
            println!("🔹 {}", lang.command);
            println!("\t{}", version.replace("\n", "\n\t"));
        }
    }
}
