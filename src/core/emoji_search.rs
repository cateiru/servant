use emoji_searcher::{EmojiDb, EmojiSearcher};
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::Path,
    rc::Rc,
};

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
        println!("Searched emoji:");
        for element in self.searcher.search(query) {
            print!("{} ", element.emoji)
        }
        println!("");

        Ok(())
    }
}

pub fn db_manager(db_path: &Path) -> Result<EmojiDb, Box<dyn Error>> {
    if db_path.exists() {
        let mut reader = BufReader::new(File::open(db_path)?);
        EmojiDb::from_cache(&mut reader)
    } else {
        let db = EmojiDb::from_web()?;
        let mut writer = File::create(db_path)?;
        db.save(&mut writer)?;

        Ok(db)
    }
}
