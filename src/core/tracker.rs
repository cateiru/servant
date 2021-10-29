use chrono::Local;
use csv::{Reader, Writer};
use reqwest;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File, path::Path};
use termion::color;

#[derive(Debug, Deserialize, Serialize)]
struct CSVElement {
    id: String,
    redirect: String,
    date: String,
    secret: String,
}

#[derive(Debug, Deserialize)]
struct CreateRes {
    track_id: String,
    access_key: String,
    redirect_url: String,
}

#[derive(Debug, Deserialize)]
struct HistoryRes {
    unique_id: String,
    ip: String,
    time: String,
}

pub struct Tracker<'a> {
    path: &'a Path,
    api: &'a str,
}

impl<'a> Tracker<'a> {
    pub fn new(path: &'a Path) -> Self {
        Self {
            path: path,
            api: "https://a.cateiru.com",
        }
    }

    pub fn list(&self) -> Result<(), Box<dyn Error>> {
        if self.path.exists() {
            let mut rdr = self.read_csv()?;
            let mut printed_element = false;

            for result in rdr.deserialize() {
                let record: CSVElement = result?;
                printed_element = true;

                let url = format!("{}/{}/", self.api, record.id);
                println!(
                    "🔎 {}{}{}",
                    color::Fg(color::Magenta),
                    record.id,
                    color::Fg(color::Reset),
                );
                println!(
                    "\t📹 Tracking URL: {}{}{}",
                    color::Fg(color::LightBlue),
                    url,
                    color::Fg(color::Reset),
                );
                println!(
                    "\t📆 Date: {}{}{}",
                    color::Fg(color::LightGreen),
                    record.date,
                    color::Fg(color::Reset)
                );
                println!(
                    "\t🔗 Redirect URL: {}{}{}",
                    color::Fg(color::LightBlue),
                    record.redirect,
                    color::Fg(color::Reset)
                );
                println!("");
            }

            if !printed_element {
                println!("📦 List is Empty");
            }
        } else {
            println!("📦 List is Empty");
        }

        Ok(())
    }

    pub fn create(&self, redirect: &str) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/u", self.api);

        let client = reqwest::blocking::Client::new();
        let response = client.post(url).form(&[("redirect", redirect)]).send()?;

        let result = response.json::<CreateRes>()?;
        let now = Local::now();

        let id = result.track_id.clone();

        let svg_element = CSVElement {
            id: result.track_id,
            redirect: redirect.to_string(),
            date: now.to_string(),
            secret: result.access_key,
        };

        self.write_csv(&mut vec![svg_element])?;

        let link = format!("{}/{}/", self.api, id);

        println!(
            "📡 {}Create {}{}",
            color::Fg(color::Magenta),
            id,
            color::Fg(color::Reset)
        );
        println!(
            "\t🔗 Link: {}{}{}",
            color::Fg(color::LightBlue),
            link,
            color::Fg(color::Reset)
        );

        Ok(())
    }

    pub fn delete(&self, id: &str) -> Result<(), Box<dyn Error>> {
        let _secret = self.get_secret(id.to_string());
        if let Ok(secret) = _secret {
            let url = format!("{}/u?id={}&key={}", self.api, id, secret);

            let client = reqwest::blocking::Client::new();
            let res = client.delete(url).send();
            if res.is_ok() {
                self.delete_list(id.to_string())?;
                println!(
                    "🗑 Deleted {}{}{}",
                    color::Fg(color::Magenta),
                    id,
                    color::Fg(color::Reset)
                );
            } else {
                println!(
                    "😵 Sorry, couldn't delete {}{}{}",
                    color::Fg(color::Magenta),
                    id,
                    color::Fg(color::Reset)
                )
            }
        } else {
            println!("id is not found.")
        }

        Ok(())
    }

    pub fn history(&self, id: &str) -> Result<(), Box<dyn Error>> {
        let _secret = self.get_secret(id.to_string());
        if let Ok(secret) = _secret {
            let url = format!("{}/u?id={}&key={}", self.api, id, secret);

            let res = reqwest::blocking::get(url)?;
            let _result = res.json::<Vec<HistoryRes>>();
            if let Ok(result) = _result {
                for element in result {
                    println!(
                        "💿 {}{}{}",
                        color::Fg(color::Magenta),
                        element.unique_id,
                        color::Fg(color::Reset)
                    );
                    println!(
                        "\t💡 IP address: {}{}{}",
                        color::Fg(color::LightGreen),
                        element.ip,
                        color::Fg(color::Reset)
                    );
                    println!(
                        "\t📆 Date: {}{}{}",
                        color::Fg(color::LightGreen),
                        element.time,
                        color::Fg(color::Reset)
                    );
                    println!("");
                }
            } else {
                println!("empty history.")
            }
        } else {
            println!("id is not found.");
        }

        Ok(())
    }

    fn get_secret(&self, id: String) -> Result<String, Box<dyn Error>> {
        let mut secret = String::new();
        let mut flag = false;

        let mut rdr = self.read_csv()?;
        for result in rdr.deserialize() {
            let record: CSVElement = result?;
            if record.id == id {
                secret = record.secret;
                flag = true;
            }
        }
        if flag {
            Ok(secret)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "secret is not fund",
            )))
        }
    }

    fn delete_list(&self, id: String) -> Result<(), Box<dyn Error>> {
        let mut element: Vec<CSVElement> = vec![];

        let mut rdr = self.read_csv()?;
        for result in rdr.deserialize() {
            let record: CSVElement = result?;
            if record.id != id {
                element.push(record);
            }
        }

        self.write_only_csv(&mut element)
    }

    fn read_csv(&self) -> Result<Reader<File>, Box<dyn Error>> {
        return Ok(Reader::from_path(self.path)?);
    }

    fn write_csv(&self, elements: &mut Vec<CSVElement>) -> Result<(), Box<dyn Error>> {
        if self.path.exists() {
            let mut rdr = self.read_csv()?;
            for result in rdr.deserialize() {
                let record: CSVElement = result?;
                elements.push(record);
            }
        }

        self.write_only_csv(elements)
    }

    fn write_only_csv(&self, elements: &mut Vec<CSVElement>) -> Result<(), Box<dyn Error>> {
        let mut wtr = Writer::from_path(self.path)?;

        for element in elements {
            wtr.serialize(element)?;
        }
        wtr.flush()?;
        Ok(())
    }
}
