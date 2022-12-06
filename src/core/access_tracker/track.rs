use crate::core::access_tracker::history::History;
use crate::core::save::SaveCache;
use chrono::Local;
use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;
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
    save: &'a SaveCache<'a>,
    api: &'a str,
}

impl<'a> Tracker<'a> {
    pub fn new(save: &'a SaveCache) -> Self {
        Self {
            save: save,
            api: "https://s.cateiru.com",
        }
    }

    pub fn list(&self) -> Result<(), Box<dyn Error>> {
        if self.save.path.exists() {
            let mut rdr = self.save.read_csv()?;
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

        self.save.write_csv(&mut vec![svg_element])?;

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
        let _secret = self.save.get_entry::<CSVElement>(&|x| {
            if x.id == id.to_string() {
                Some(x.secret.to_string())
            } else {
                None
            }
        });
        if let Ok(secret) = _secret {
            let url = format!("{}/u?id={}&key={}", self.api, id, secret);

            let client = reqwest::blocking::Client::new();
            let res = client.delete(url).send();
            if res.is_ok() {
                self.save
                    .delete_entity::<CSVElement>(&|x| x.id == id.to_string())?;
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

    pub fn history(&self, id: &str) -> Result<History, Box<dyn Error>> {
        let secret = self.save.get_entry::<CSVElement>(&|x| {
            if x.id == id.to_string() {
                Some(x.secret.to_string())
            } else {
                None
            }
        })?;

        History::new(self.api.to_string(), id.to_string(), secret)
    }
}
