use crate::utils::print;
use emoji_searcher::{EmojiDb, EmojiSearcher};
use std::{env, error::Error, fs::File, path::Path, rc::Rc};
use termion::color;

pub fn emoji(query: String) -> Result<(), Box<dyn Error>> {
    let _home = env::var("HOME");
    let path: String;
    if let Ok(home) = _home {
        path = format!("{}/.servant/emoji_db", home);
    } else {
        path = ".servant/emoji_db".to_string();
    }
    let db = db_manager(&Path::new(&path))?;
    let emoji = Emoji::new(db);

    emoji.search(query)
}

pub struct Emoji {
    searcher: EmojiSearcher,
}

impl Emoji {
    pub fn new(db: EmojiDb) -> Self {
        let searcher = EmojiSearcher::new(Rc::new(db));

        Self { searcher: searcher }
    }

    pub fn search(&self, query: String) -> Result<(), Box<dyn Error>> {
        println!(
            "{}Success!{}",
            color::Fg(color::Magenta),
            color::Fg(color::Reset)
        );
        for element in self.searcher.search(query) {
            println!(
                "\t{} : {}{}{}",
                element.emoji,
                color::Fg(color::LightGreen),
                element.matched_tag,
                color::Fg(color::Reset)
            )
        }
        println!("");

        Ok(())
    }
}

pub fn db_manager(db_path: &Path) -> Result<EmojiDb, Box<dyn Error>> {
    if db_path.exists() {
        let mut reader = File::open(db_path)?;
        EmojiDb::from_cache(&mut reader)
    } else {
        // print load text
        print::print_line(&format!(
            "{}Install Emoji Database...{}",
            color::Fg(color::LightGreen),
            color::Fg(color::Reset)
        ))?;
        let db = EmojiDb::from_web()?;
        let mut writer = File::create(db_path)?;
        // delete load text
        print::print_line("\r                                     \r")?;

        db.save(&mut writer)?;

        Ok(db)
    }
}
