use bevy::prelude::*;
use rand::Rng;

#[derive(Resource, Default)]
pub struct WordsResource {
    short_words: Vec<&'static str>,
    long_words: Vec<&'static str>,
    special_chars: Vec<&'static str>,
}

#[derive(Component)]
pub struct TextEnemy {
    pub enemy_entity_id: Entity,
}

impl WordsResource {
    pub fn short_word(&self) -> &'static str {
        let mut thread_rng = rand::thread_rng();
        let index = thread_rng.gen_range(0..self.short_words.len());
        self.short_words[index]
    }

    pub fn long_word(&self) -> &'static str {
        let mut thread_rng = rand::thread_rng();
        let index = thread_rng.gen_range(0..self.long_words.len());
        self.long_words[index]
    }

    pub fn special_word(&self) -> String {
        let mut thread_rng = rand::thread_rng();
        let numbers_of_chars = thread_rng.gen_range(3..7);

        let mut word = String::from("");

        for _ in 0..numbers_of_chars {
            let index = thread_rng.gen_range(0..self.special_chars.len());

            word.push_str(self.special_chars[index]);
        }

        word
    }

    pub fn from_file() -> Self {
        Self {
            short_words: vec!["short", "should", "be", "not", "long"],
            long_words: vec!["longword"],
            special_chars: vec![
                "!", "@", "#", "$", "%", "^", "&", "*", "(", ")", "-", "_", "=", "+", "[", "]",
                "{", "}", ";", ":", "'", "\"", ",", "<", ">", ".", "/", "?", "\\", "|",
            ],
        }
    }
}

pub struct WordsPlugin;
impl Plugin for WordsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WordsResource::from_file());
    }
}
