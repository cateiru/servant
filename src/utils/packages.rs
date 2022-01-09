use std::{env, error::Error, path::Path, process::Command};

pub enum PackageManagementType {
    Git,
    Origin,
}

pub enum PackageCheckExistType {
    Which,
    Dir,
}

pub struct Package<'a> {
    pub name: &'a str,

    package_manager_type: PackageManagementType,
    package_check_exist_type: PackageCheckExistType,

    command: Option<&'a str>,
    directory: Option<&'a Path>,

    update_commands: Vec<&'a str>,
}

impl<'a> Package<'a> {
    // exist this command
    pub fn exist(&self) -> bool {
        match self.package_check_exist_type {
            PackageCheckExistType::Which => {
                if let Some(command) = self.command {
                    let result = match cfg!(target_os = "windows") {
                        true => Command::new("cmd")
                            .args(["/C", "where.exe", command])
                            .output(),
                        false => Command::new("sh")
                            .arg("-c")
                            .arg(format!("{} {}", "which", command))
                            .output(),
                    };

                    match result {
                        Ok(result) => result.stderr.len() == 0 && result.stdout.len() != 0,
                        Err(_) => false,
                    }
                } else {
                    false
                }
            }
            PackageCheckExistType::Dir => {
                if let Some(dir) = self.directory {
                    let _home = env::var("HOME");
                    if let Ok(home) = _home {
                        let home_dir = Path::new(&home);
                        let target = home_dir.join(dir);

                        target.exists() && target.is_dir()
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        }
    }

    // run update command
    pub fn update(&self) -> Result<(), Box<dyn Error>> {
        match self.package_manager_type {
            PackageManagementType::Git => {
                for update_command in self.update_commands.iter() {
                    println!("$ {}", update_command);
                    let result = match cfg!(target_os = "windows") {
                        true => Command::new("cmd").args(["/C", update_command]).output()?,
                        false => Command::new("sh").arg("-c").arg(update_command).output()?,
                    };

                    println!("{}", String::from_utf8(result.stdout)?);
                    println!("{}", String::from_utf8(result.stderr)?);
                }
            }
            PackageManagementType::Origin => {
                if let Some(package_command) = self.command {
                    for update_command in self.update_commands.iter() {
                        println!("$ {} {}", package_command, update_command);
                        let result = match cfg!(target_os = "windows") {
                            true => Command::new("cmd")
                                .args(["/C", package_command, update_command])
                                .output()?,
                            false => Command::new("sh")
                                .arg("-c")
                                .arg(format!("{} {}", package_command, update_command))
                                .output()?,
                        };

                        println!("{}", String::from_utf8(result.stdout)?);
                        println!("{}", String::from_utf8(result.stderr)?);
                    }
                }
            }
        }

        Ok(())
    }
}

pub fn get_packages<'a>() -> [Package<'a>; 3] {
    let packages: [Package; 3] = [
        Package {
            name: "homebrew",

            package_check_exist_type: PackageCheckExistType::Which,
            package_manager_type: PackageManagementType::Origin,

            command: Some("brew"),
            directory: None,
            update_commands: vec![
                "update",   // packages update
                "outdated", // check latest versions
                "upgrade",  // self update
                "cleanup",
            ],
        },
        Package {
            name: "SDKMAN!",

            package_check_exist_type: PackageCheckExistType::Which,
            package_manager_type: PackageManagementType::Origin,

            command: Some("sdk"),
            directory: None,
            update_commands: vec!["selfupdate"],
        },
        Package {
            name: "rustup",

            package_check_exist_type: PackageCheckExistType::Which,
            package_manager_type: PackageManagementType::Origin,

            command: Some("rustup"),
            directory: None,
            update_commands: vec!["self update"],
        },
        // Package {
        //     name: "Prezto",

        //     package_check_exist_type: PackageCheckExistType::Dir,
        //     package_manager_type: PackageManagementType::Git,

        //     directory: Some(Path::new(".zprezto")),
        //     command: None,

        //     update_commands: vec![
        //         "cd ~/.zprezto",
        //         "git pull",
        //         "git submodule sync --recursive",
        //         "git submodule update --init --recursive",
        //         "cd $SCRIPT_DIR",
        //     ],
        // },
    ];

    packages
}
