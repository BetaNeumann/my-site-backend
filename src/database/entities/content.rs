use regex::Regex;
use crate::database::entities::prelude::*;

const READ_SPEED_MIN: u32 = 150;
const READ_SPEED_MAX: u32 = 250;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    raw: String,
    html: String
}

impl Content {
    pub fn new(content: String) -> Self {
        let html = markdown::to_html(&content);

        Self {
            raw: content,
            html,
        }
    }

    pub fn word_count(&self) -> u32 {
        let word_pattern = Regex::new(r"(?m)(\w+)")
            .expect("Failed to parse regex");

        word_pattern.captures_iter(&self.raw)
            .count()
            .min(u32::MAX as usize) as u32
    }

    pub fn read_time(&self) -> (u32, u32) {
        let word_count = self.word_count();
        (word_count / READ_SPEED_MIN, word_count / READ_SPEED_MAX)
    }
}
