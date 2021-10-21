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
        if self.exist_command() {
            let version_option = match self.version_command_type {
                VersionType::OneHyphen => "-version",
                VersionType::TwoHyphen => "--version",
                VersionType::NoHyphen => "version",
            };

            let result = match cfg!(target_os = "windows") {
                true => Command::new("cmd")
                    .args(["/C", self.command, version_option])
                    .output(),
                false => Command::new("sh")
                    .arg("-c")
                    .arg(format!("{} {}", self.command, version_option))
                    .output(),
            };

            match result {
                Ok(result) => Some(result),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    fn exist_command(&self) -> bool {
        let result = match cfg!(target_os = "windows") {
            true => Command::new("cmd")
                .args(["/C", "where.exe", self.command])
                .output(),
            false => Command::new("sh")
                .arg("-c")
                .arg(format!("{} {}", "which", self.command))
                .output(),
        };

        match result {
            Ok(result) => result.stderr.len() == 0 && result.stdout.len() != 0,
            Err(_) => false,
        }
    }
}

pub fn languages() {}

#[cfg(test)]
mod languages_tests {
    use crate::core::languages::{Language, VersionType};
    use std::str::from_utf8;

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
            assert_eq!(false, true);
        }
    }

    #[test]
    fn not_exist_lang() {
        let command = "hoge";
        let keywords = vec!["hogehoge"];
        let version_command_type = VersionType::NoHyphen;

        let lang_unknown = Language {
            command,
            keywords,
            version_command_type,
        };

        let result = lang_unknown.run();

        println!("{:?}", result);

        assert_eq!(result.is_none(), true);
    }
}
