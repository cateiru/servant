use emoji_searcher::{EmojiDb, EmojiSearcher};
use std::{error::Error, rc::Rc};

pub fn emoji(query: String) -> Result<(), Box<dyn Error>> {
    let emoji_db = EmojiDb::from_web()?;
    let emoji = Emoji::new(emoji_db);

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
