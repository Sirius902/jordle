use kana::CloseStatus;
use std::collections::{BTreeSet, HashMap};
use std::hash::Hash;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

mod kana;

const WORDS_FILE: &str = include_str!("../assets/japanese_words.txt");
const WORD_LEN: usize = 4;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Tile {
    Absent,
    Present,
    Correct,
    Consonant,
    Vowel,
    Close,
}

fn main() {
    let words = WORDS_FILE.lines().collect::<BTreeSet<_>>();
    for word in words.iter() {
        assert_eq!(word.chars().count(), WORD_LEN);
    }

    let entropy = {
        let mut es = words
            .par_iter()
            .map(|word| (word, word_entropy(word, &words)))
            .collect::<Vec<_>>();

        es.sort_by(|(_, e1), (_, e2)| e1.partial_cmp(e2).expect("Failed to compare entropy"));

        es
    };

    for (word, entropy) in entropy.iter().rev().take(10) {
        println!("{:.3} {}", entropy, word);
    }
}

fn word_tiles(guess: &str, answer: &str) -> [Tile; WORD_LEN] {
    let mut tiles = [Tile::Absent; WORD_LEN];
    let mut chars = answer.chars().collect::<Vec<_>>();

    for (i, (g, a)) in guess.chars().zip(answer.chars()).enumerate() {
        if g == a {
            tiles[i] = Tile::Correct;

            if let Some(found_idx) = chars.iter().position(|c| *c == g) {
                chars.swap_remove(found_idx);
            }
        }
    }

    for (i, (g, a)) in guess.chars().zip(answer.chars()).enumerate() {
        if tiles[i] == Tile::Absent {
            if let Some(close_status) = kana::close_status(g, a) {
                match close_status {
                    CloseStatus::Close => {
                        tiles[i] = Tile::Close;
                    }
                    CloseStatus::Consonant => {
                        tiles[i] = Tile::Consonant;
                    }
                    CloseStatus::Vowel => {
                        tiles[i] = Tile::Vowel;
                    }
                }
            }
        }
    }

    for (i, g) in guess.chars().enumerate() {
        if tiles[i] != Tile::Correct && tiles[i] != Tile::Close {
            if let Some(found_idx) = chars.iter().position(|c| *c == g) {
                tiles[i] = Tile::Present;
                chars.swap_remove(found_idx);
            }
        }
    }

    tiles
}

fn word_entropy(guess: &str, word_list: &BTreeSet<&str>) -> f64 {
    let mut counter = HashMap::new();

    for word in word_list {
        let tiles = word_tiles(guess, word);
        if let Some(count) = counter.get_mut(&tiles) {
            *count += 1usize;
        } else {
            counter.insert(tiles, 1usize);
        }
    }

    let mut entropy = 0.0f64;
    for count in counter.values() {
        let p = (*count as f64) / (word_list.len() as f64);
        entropy += p * (1.0 / p).log2();
    }

    entropy
}

#[cfg(test)]
mod tests {
    use crate::*;
    use Tile::*;

    #[test]
    fn correct_absent() {
        assert_eq!(word_tiles("ほしがる", "ほしがる"), [Correct; WORD_LEN]);
        assert_eq!(
            word_tiles("かたまる", "かたよる"),
            [Correct, Correct, Absent, Correct]
        );
        assert_eq!(
            word_tiles("かたまる", "かたよる"),
            [Correct, Correct, Absent, Correct]
        );
        assert_eq!(
            word_tiles("かんじん", "かんぱい"),
            [Correct, Correct, Absent, Absent]
        );
        assert_eq!(
            word_tiles("ともしび", "かたかな"),
            [Absent, Absent, Absent, Absent]
        );
    }

    #[test]
    fn correct_present() {
        assert_eq!(word_tiles("かんせい", "せいかん"), [Present; WORD_LEN]);
        assert_eq!(
            word_tiles("せいかい", "かいせい"),
            [Present, Correct, Present, Correct]
        );
        assert_eq!(
            word_tiles("あたなら", "あらたな"),
            [Correct, Present, Present, Present]
        );
    }

    #[test]
    fn consonant_vowel() {
        assert_eq!(
            word_tiles("げんざい", "けんざい"),
            [Close, Correct, Correct, Correct]
        );
        assert_eq!(
            word_tiles("けってい", "かつどう"),
            [Consonant, Close, Consonant, Consonant]
        );
        assert_eq!(
            word_tiles("はっけん", "はつげん"),
            [Correct, Close, Close, Correct]
        );
        assert_eq!(
            word_tiles("とつぜん", "こいぶみ"),
            [Vowel, Absent, Absent, Absent]
        );
        assert_eq!(
            word_tiles("かいぶつ", "こしょう"),
            [Consonant, Vowel, Absent, Vowel]
        );
        assert_eq!(
            word_tiles("だいざい", "たいだん"),
            [Close, Correct, Vowel, Absent]
        );
        assert_eq!(
            word_tiles("だいたい", "たいたい"),
            [Close, Correct, Correct, Correct]
        );
    }
}
