use crate::utils::graph as util_graph;
use chrono::{DateTime, Datelike, FixedOffset, Local};
use chrono_tz::{Asia::Tokyo, Tz};
use reqwest;
use rustc_serialize::{json, json::Json};
use serde::{Deserialize, Serialize};
use std::error::Error;
use termion::color;
use whois::WhoIs;

#[derive(Debug, Deserialize)]
struct HistoryResponse {
    unique_id: String,
    ip: String,
    useragent: String,
    time: String,
}

#[derive(Debug, RustcDecodable)]
pub struct UserAgent {
    name: String,
    version: String,
    os: String,
    os_version: String,
    device: String,
    mobile: bool,
    access_id: bool,
    desktop: bool,
    bot: bool,
    url: String,
    string: String,
}

pub struct HistoryElement {
    unique_id: String,
    ip: String,
    user_agent: UserAgent,
    time: DateTime<FixedOffset>,
}

pub struct History {
    histories: Vec<HistoryElement>,
}

impl History {
    pub fn new(api_url: String, id: String, secret: String) -> Result<Self, Box<dyn Error>> {
        let url = format!("{}/u?id={}&key={}", api_url, id, secret);

        let histories = History::get(url)?;

        Ok(Self {
            histories: histories,
        })
    }

    pub fn print_all(&self, is_whois: bool, is_user_agent: bool) -> Result<(), Box<dyn Error>> {
        for history in self.histories.iter() {
            history.print(is_whois, is_user_agent)?;
            println!("");
        }

        Ok(())
    }

    pub fn print_all_oneline(&self) -> Result<(), Box<dyn Error>> {
        for history in self.histories.iter() {
            history.print_oneline()?;
        }

        Ok(())
    }

    pub fn print_by_ip(
        &self,
        target_ip: String,
        is_whois: bool,
        is_user_agent: bool,
    ) -> Result<(), Box<dyn Error>> {
        for history in self.histories.iter() {
            if history.ip == target_ip {
                history.print(is_whois, is_user_agent)?;
                println!("");
            }
        }

        Ok(())
    }

    pub fn print_graph(&self) -> Result<(), Box<dyn Error>> {
        let mut plot_data: Vec<(DateTime<Tz>, usize)> =
            vec![(Local::now().with_timezone(&Tokyo), 0)];

        for element in self.histories.iter() {
            let date = element.time.with_timezone(&Tokyo);
            let index = plot_data
                .iter()
                .position(|x| x.0.month() == date.month() && x.0.day() == date.day());

            if let Some(index) = index {
                plot_data[index] = (plot_data[index].0, plot_data[index].1 + 1);
            } else {
                plot_data.push((date, 1));
            }
        }

        plot_data.sort_by(|x, y| y.0.cmp(&x.0));

        util_graph::plot_barchart(plot_data)?;

        Ok(())
    }

    // Get the url of args and parse response json.
    fn get(url: String) -> Result<Vec<HistoryElement>, Box<dyn Error>> {
        let mut histories: Vec<HistoryElement> = Vec::<HistoryElement>::new();

        let res = reqwest::blocking::get(url)?;
        let _result = res.json::<Vec<HistoryResponse>>();

        if let Ok(result) = _result {
            for resp_history in result {
                let user_agent = History::convert_user_agent(resp_history.useragent)?;
                let history_element = HistoryElement {
                    unique_id: resp_history.unique_id,
                    ip: resp_history.ip,
                    time: DateTime::parse_from_rfc3339(&resp_history.time)?,
                    user_agent: user_agent,
                };
                histories.push(history_element);
            }
        }

        histories.sort_by(|x, y| y.time.cmp(&x.time));

        Ok(histories)
    }

    // Parse the user-agent json text.
    fn convert_user_agent(target: String) -> Result<UserAgent, Box<dyn Error>> {
        let user_agent = json::decode::<UserAgent>(&target)?;

        Ok(user_agent)
    }
}

impl HistoryElement {
    pub fn print(&self, is_whois: bool, is_user_agent: bool) -> Result<(), Box<dyn Error>> {
        println!(
            "💿 {}{}{}",
            color::Fg(color::Magenta),
            self.unique_id,
            color::Fg(color::Reset)
        );
        println!(
            "\t📆 Date: {}{}{}",
            color::Fg(color::LightGreen),
            self.time.format("%F (%a) %T"),
            color::Fg(color::Reset)
        );

        println!("\t👤 UserData");
        println!(
            "\t\t💡 IP address: {}{}{}",
            color::Fg(color::LightGreen),
            self.ip,
            color::Fg(color::Reset)
        );
        println!(
            "\t\t📛 Name: {}{}{}",
            color::Fg(color::LightGreen),
            self.user_agent.name,
            color::Fg(color::Reset),
        );
        if is_user_agent {
            HistoryElement::print_user_agent(&self.user_agent);
        }
        if is_whois {
            HistoryElement::print_whois(self.ip.clone())?;
        }

        Ok(())
    }

    pub fn print_oneline(&self) -> Result<(), Box<dyn Error>> {
        println!(
            "💿 {}{}{} - {}{}{}({}{}{})",
            color::Fg(color::Magenta),
            self.user_agent.name,
            color::Fg(color::Reset),
            color::Fg(color::Magenta),
            self.ip,
            color::Fg(color::Reset),
            color::Fg(color::LightGreen),
            self.time.format("%F (%a) %T"),
            color::Fg(color::Reset)
        );

        Ok(())
    }

    fn print_user_agent(user_agent: &UserAgent) {
        println!(
            "\t\t🖥 OS: {}{}{} - {}{}{}",
            color::Fg(color::LightGreen),
            user_agent.os,
            color::Fg(color::Reset),
            color::Fg(color::LightGreen),
            user_agent.os_version,
            color::Fg(color::Reset),
        );

        let mut device_type = "Unknown";
        if user_agent.mobile {
            device_type = "Mobile"
        } else if user_agent.desktop {
            device_type = "Desktop"
        }
        println!(
            "\t\t⌨ Device: {}{}{} - {}{}{}",
            color::Fg(color::LightGreen),
            user_agent.device,
            color::Fg(color::Reset),
            color::Fg(color::LightGreen),
            device_type,
            color::Fg(color::Reset)
        );
    }

    fn print_whois(ip: String) -> Result<(), Box<dyn Error>> {
        println!("\t👤 Whois:");

        let mut whois = WhoIs::new(ip);
        let lookup = whois.lookup()?;

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
}
