use crate::utils::languages::languages as lang_list;
use std::str::from_utf8;

pub fn languages() {
    let languages = lang_list();

    for lang in languages.iter() {
        let result = lang.run();

        if let Some(result) = result {
            let version = from_utf8(&result.stdout);
            if let Ok(version) = version {
                println!("🔹 {}", lang.command);
                println!("\t{}", version.replace("\n", "\n\t"));
            }
        }
    }
}
