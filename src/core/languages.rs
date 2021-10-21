use std::process::{Command, Output};

pub enum VersionType {
    OneHyphen,
    TwoHyphen,
    NoHyphen,
}

struct Language<'a> {
    pub command: &'a str,
    keywords: Vec<&'a str>,
    version_command_type: VersionType,
}

impl<'a> Language<'a> {
    pub fn search(&self, lang: &str) -> bool {
        let mut flag = false;
        let target = lang.to_lowercase();

        for key_lang in self.keywords.iter() {
            if *key_lang == target {
                flag = true;
                break;
            }
        }

        flag
    }

    pub fn run(&self) -> Option<Output> {
        let version_option = match self.version_command_type {
            VersionType::OneHyphen => "-version",
            VersionType::TwoHyphen => "--version",
            VersionType::NoHyphen => "version",
        };

        let command = format!("{} {}", self.command, version_option);

        if cfg!(target_os = "windows") {
            None
        } else {
            Some(
                Command::new("sh")
                    .arg("-c")
                    .arg(command)
                    .output()
                    .expect("failed to execute process"),
            )
        }
    }
}

#[cfg(test)]
mod languages_tests {
    use crate::core::languages::{Language, VersionType};
    use std::{process::exit, str::from_utf8};

    #[test]
    fn language_trait() {
        let command = "python";
        let keywords = vec!["python", "py"];
        let version_command_type = VersionType::TwoHyphen;

        let lang_py = Language {
            command,
            keywords,
            version_command_type,
        };

        assert_eq!(lang_py.search("py"), true);
        assert_eq!(lang_py.search("rust"), false);

        let result = lang_py.run();

        if let Some(result) = result {
            println!("{:?}", from_utf8(&result.stdout));

            assert_eq!(from_utf8(&result.stderr), Ok(""));
        } else {
            exit(1)
        }
    }
}
