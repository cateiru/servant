use crate::utils::{graph as util_graph, ip_blacklist};
use chrono::{DateTime, Datelike, FixedOffset, Local};
use chrono_tz::{Asia::Tokyo, Tz};
use csv::{Reader, Writer};
use reqwest;
use rustc_serialize::json::Json;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fs::File, path::Path};
use termion::color;
use whois::WhoIs;

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

struct PrintHistory {
    unique_id: String,
    ip: String,
    time: DateTime<FixedOffset>,
}

pub struct Tracker<'a> {
    path: &'a Path,
    api: &'a str,
}

impl<'a> Tracker<'a> {
    pub fn new(path: &'a Path) -> Self {
        Self {
            path: path,
            api: "https://uie.jp",
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

    pub fn history(
        &self,
        id: &str,
        oneline: bool,
        all: bool,
        graph: bool,
    ) -> Result<(), Box<dyn Error>> {
        let mut show_his = History::new(oneline, all);
        let _secret = self.get_secret(id.to_string());
        if let Ok(secret) = _secret {
            let url = format!("{}/u?id={}&key={}", self.api, id, secret);

            let res = reqwest::blocking::get(url)?;
            let _result = res.json::<Vec<HistoryRes>>();
            if let Ok(result) = _result {
                if graph {
                    show_his.print_graph(result)?;
                } else {
                    show_his.print(&result)?;
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

struct History {
    cache: HashMap<String, String>,
    oneline: bool,
    all: bool,
}

impl History {
    pub fn new(oneline: bool, all: bool) -> Self {
        Self {
            cache: HashMap::new(),
            oneline: oneline,
            all: all,
        }
    }

    pub fn print(&mut self, result: &Vec<HistoryRes>) -> Result<(), Box<dyn Error>> {
        let mut his: Vec<PrintHistory> = vec![];

        for element in result {
            his.push(PrintHistory {
                unique_id: element.unique_id.clone(),
                ip: element.ip.clone(),
                time: DateTime::parse_from_rfc3339(&element.time)?,
            })
        }

        his.sort_by(|x, y| y.time.cmp(&x.time));

        for element in his {
            if self.all || self.select_ip(&element.ip) {
                match self.oneline {
                    true => self.history_oneline(&element)?,
                    false => self.history_multiline(&element)?,
                }
            }
        }
        Ok(())
    }

    pub fn print_graph(&mut self, result: Vec<HistoryRes>) -> Result<(), Box<dyn Error>> {
        let mut plot_data: Vec<(DateTime<Tz>, usize)> =
            vec![(Local::now().with_timezone(&Tokyo), 0)];

        for element in result {
            if self.all || self.select_ip(&element.ip) {
                let date = DateTime::parse_from_rfc3339(&element.time)?.with_timezone(&Tokyo);
                let index = plot_data
                    .iter()
                    .position(|x| x.0.month() == date.month() && x.0.day() == date.day());

                if let Some(index) = index {
                    plot_data[index] = (plot_data[index].0, plot_data[index].1 + 1);
                } else {
                    plot_data.push((date, 1));
                }
            }
        }

        plot_data.sort_by(|x, y| y.0.cmp(&x.0));

        util_graph::plot_barchart(plot_data)?;

        Ok(())
    }

    fn whois(&mut self, ip: String) -> Result<(), Box<dyn Error>> {
        println!("\t👤 Whois:");

        let lookup: String = match self.cache.get(&ip) {
            Some(value) => value.clone(),
            None => {
                let mut whois = WhoIs::new(ip.clone());
                let lookup = whois.lookup()?;

                self.cache.insert(ip.clone(), lookup.clone());

                lookup
            }
        };

        let json = &Json::from_str(&lookup)?;

        if let Some(json_object) = json.as_object() {
            for (key, value) in json_object {
                println!(
                    "\t\t▪ {}: {}{}{}",
                    key,
                    color::Fg(color::Red),
                    match *value {
                        Json::String(ref v) => format!("{}", v),

                        _ => break,
                    },
                    color::Fg(color::Reset),
                );
            }
        }

        Ok(())
    }

    fn select_ip(&self, ip: &str) -> bool {
        let result = ip_blacklist::IP_BLACKLIST.iter().position(|&r| r == ip);

        result.is_none()
    }

    fn history_multiline(&mut self, history: &PrintHistory) -> Result<(), Box<dyn Error>> {
        println!(
            "💿 {}{}{}",
            color::Fg(color::Magenta),
            history.unique_id,
            color::Fg(color::Reset)
        );
        println!(
            "\t💡 IP address: {}{}{}",
            color::Fg(color::LightGreen),
            history.ip,
            color::Fg(color::Reset)
        );
        println!(
            "\t📆 Date: {}{}{}",
            color::Fg(color::LightGreen),
            history.time.format("%F (%a) %T"),
            color::Fg(color::Reset)
        );
        self.whois(history.ip.clone())?;

        println!("");

        Ok(())
    }

    fn history_oneline(&self, history: &PrintHistory) -> Result<(), Box<dyn Error>> {
        println!(
            "💿 {}{}{} - {}{}{}",
            color::Fg(color::Magenta),
            history.ip,
            color::Fg(color::Reset),
            color::Fg(color::LightGreen),
            history.time.format("%F (%a) %T"),
            color::Fg(color::Reset)
        );

        Ok(())
    }
}
