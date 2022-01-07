use csv::{Reader, Writer};
use serde::{de::DeserializeOwned, Serialize};
use std::{env, error::Error, fs::File, path::Path};

pub struct SaveCache<'a> {
    pub path: &'a Path,
}

impl<'a> SaveCache<'a> {
    pub fn new(path: &'a str) -> Self {
        SaveCache {
            path: Path::new(path),
        }
    }

    pub fn current_path(file_name: String) -> Result<String, Box<dyn Error>> {
        let _home = env::var("HOME");
        let path_str: String;
        if let Ok(home) = _home {
            path_str = home.to_string() + "/.servant/" + &file_name;
        } else {
            path_str = ".servant".to_string() + &file_name;
        }

        Ok(path_str)
    }

    pub fn read_csv(&self) -> Result<Reader<File>, Box<dyn Error>> {
        return Ok(Reader::from_path(self.path)?);
    }

    pub fn write_csv<T: Serialize + DeserializeOwned>(
        &self,
        elements: &mut Vec<T>,
    ) -> Result<(), Box<dyn Error>> {
        if self.path.exists() {
            let mut rdr = self.read_csv()?;
            for result in rdr.deserialize() {
                let record: T = result?;
                elements.push(record);
            }
        }

        self.write_only_csv(elements)
    }

    pub fn write_only_csv<T: Serialize>(
        &self,
        elements: &mut Vec<T>,
    ) -> Result<(), Box<dyn Error>> {
        let mut wtr = Writer::from_path(self.path)?;

        for element in elements {
            wtr.serialize(element)?;
        }
        wtr.flush()?;
        Ok(())
    }

    pub fn delete_entity<T: Serialize + DeserializeOwned>(
        &self,
        f: &dyn Fn(&T) -> bool,
    ) -> Result<(), Box<dyn Error>> {
        let mut element: Vec<T> = vec![];

        let mut rdr = self.read_csv()?;
        for result in rdr.deserialize() {
            let record: T = result?;
            if f(&record) {
                element.push(record);
            }
        }

        self.write_only_csv(&mut element)
    }

    pub fn get_entry<T: Serialize + DeserializeOwned>(
        &self,
        f: &dyn Fn(&T) -> Option<String>,
    ) -> Result<String, Box<dyn Error>> {
        let mut buffer = String::new();
        let mut flag = false;

        let mut rdr = self.read_csv()?;
        for result in rdr.deserialize() {
            let record: T = result?;
            match f(&record) {
                Some(e) => {
                    buffer = e;
                    flag = true;
                    break;
                }
                None => {}
            };
        }
        if flag {
            Ok(buffer)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "secret is not fund",
            )))
        }
    }
}
