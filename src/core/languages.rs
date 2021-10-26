use crate::utils::languages::{languages as lang_list, Language};
use std::str::from_utf8;

/// export versions in all installed languages.
pub fn languages() {
    let languages = lang_list();

    for lang in languages.iter() {
        export_version(&lang);
    }
}

/// export target language version.
pub fn selected_languages(target: &str) {
    let languages = lang_list();
    let mut flag = false;

    for lang in languages.iter() {
        if lang.search(target) {
            export_version(&lang);
            flag = true;
        }
    }

    if !flag {
        println!("🔸 {} is not found", target)
    }
}

/// export version.
fn export_version(lang: &Language) {
    let result = lang.run();

    if let Some(result) = result {
        let version = from_utf8(&result.stdout);
        if let Ok(version) = version {
            println!("🔹 {}", lang.command);
            println!("\t{}", version.replace("\n", "\n\t"));
        }
    } else {
        println!("🔸 {} is not found\n", lang.command);
    }
}
