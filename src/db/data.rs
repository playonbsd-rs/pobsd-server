use std::fmt::Debug;

#[derive(Debug, Clone, Default)]
pub struct Metacritic {
    pub score: usize,
    pub url: String,
}

#[derive(Debug, Clone, Default)]
pub struct GameMetaData {
    pub summary: Option<String>,
    pub story_line: Option<String>,
    pub screenshots: Vec<String>,
    pub cover: Option<String>,
    pub metacritic: Option<Metacritic>,
}

impl GameMetaData {
    pub fn merge(mut self, other: GameMetaData) -> Self {
        if self.summary.is_none() {
            self.summary = other.summary;
        }
        if self.story_line.is_none() {
            self.story_line = other.story_line;
        }
        if self.screenshots.len() < other.screenshots.len() {
            self.screenshots = other.screenshots;
        }
        if self.metacritic.is_none() {
            self.metacritic = other.metacritic;
        }
        if self.cover.is_none() {
            self.cover = other.cover;
        }
        self
    }
}
