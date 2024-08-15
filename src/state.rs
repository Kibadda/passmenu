use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use ratatui::widgets::ListState;
use walkdir::WalkDir;

pub enum Dir {
    Up,
    Down,
    Same,
}

pub struct State {
    pub input: String,
    pub keys: Vec<String>,
    pub filtered_keys: Vec<String>,
    pub list_state: ListState,
}

impl State {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            keys: vec![],
            filtered_keys: vec![],
            list_state: ListState::default(),
        }
    }

    pub fn load_keys(&mut self) {
        let mut keys: Vec<String> = Vec::new();

        let mut home = std::env::var("HOME").unwrap();
        home.push_str("/.password-store/");

        WalkDir::new(home.clone()).into_iter().for_each(|entry| {
            if let Ok(entry) = entry {
                if let Some(path) = entry.path().to_str().unwrap().strip_suffix(".gpg") {
                    keys.push(path.strip_prefix(&home).unwrap().to_owned());
                }
            }
        });

        self.keys = keys;
        self.filter();
    }

    fn filter(&mut self) {
        let matcher = SkimMatcherV2::default();

        self.filtered_keys = self
            .keys
            .clone()
            .into_iter()
            .filter(|key| matcher.fuzzy_match(key, &self.input).is_some())
            .collect();
        self.filtered_keys
            .sort_by_cached_key(|key| -matcher.fuzzy_match(key, &self.input).unwrap());
        self.move_index(Dir::Same);
    }

    pub fn enter_char(&mut self, new_char: char) {
        self.input.insert(self.input.len(), new_char);
        self.filter();
    }

    pub fn delete_char(&mut self) {
        if !self.input.is_empty() {
            self.input = self.input.chars().take(self.input.len() - 1).collect();
            self.filter();
        }
    }

    pub fn move_index(&mut self, dir: Dir) {
        let len = self.filtered_keys.len();

        if len == 0 {
            self.list_state.select(Some(0));
        } else {
            self.list_state.select(match dir {
                Dir::Down => Some(match self.list_state.selected() {
                    Some(i) => {
                        if i >= self.filtered_keys.len() - 1 {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                }),
                Dir::Up => Some(match self.list_state.selected() {
                    Some(i) => {
                        if i == 0 {
                            self.filtered_keys.len() - 1
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                }),
                Dir::Same => Some(match self.list_state.selected() {
                    Some(i) => i.clamp(0, self.filtered_keys.len() - 1),
                    None => 0,
                }),
            })
        }
    }
}
