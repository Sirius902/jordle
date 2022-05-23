use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
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
    SameRow,
    SameColumn,
    SameRowColumn,
}

fn main() {
    let words = WORDS_FILE.lines().collect::<BTreeSet<_>>();
    for word in words.iter() {
        assert_eq!(word.chars().count(), WORD_LEN);
    }

    let entropy = words
        .par_iter()
        .map(|word| EntropyEntry::new(word, word_entropy(word, &words)))
        .collect::<BTreeSet<_>>();

    for EntropyEntry { word, entropy } in entropy.iter().take(10) {
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
            let same_row = kana::row(g)
                .zip(kana::row(a))
                .map_or(false, |(r1, r2)| r1 == r2);
            let same_column = kana::column(g)
                .zip(kana::column(a))
                .map_or(false, |(c1, c2)| c1 == c2);

            if same_row && same_column {
                tiles[i] = Tile::SameRowColumn;
            } else if same_row {
                tiles[i] = Tile::SameRow;
            } else if same_column {
                tiles[i] = Tile::SameColumn;
            }
        }
    }

    for (i, g) in guess.chars().enumerate() {
        if tiles[i] != Tile::Correct && tiles[i] != Tile::SameRowColumn {
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

struct EntropyEntry<'a> {
    word: &'a str,
    entropy: f64,
}

impl<'a> EntropyEntry<'a> {
    fn new(word: &'a str, entropy: f64) -> Self {
        EntropyEntry { word, entropy }
    }
}

impl Ord for EntropyEntry<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).expect("Failed to compare entropy")
    }
}

impl PartialOrd for EntropyEntry<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.entropy.partial_cmp(&self.entropy)
    }
}

impl PartialEq for EntropyEntry<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.entropy == other.entropy
    }
}

impl Eq for EntropyEntry<'_> {}

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
    fn row_column() {
        assert_eq!(
            word_tiles("げんざい", "けんざい"),
            [SameRowColumn, Correct, Correct, Correct]
        );
        assert_eq!(
            word_tiles("けってい", "かつどう"),
            [SameRow, SameRowColumn, SameRow, SameRow]
        );
        assert_eq!(
            word_tiles("はっけん", "はつげん"),
            [Correct, SameRowColumn, SameRowColumn, Correct]
        );
        assert_eq!(
            word_tiles("とつぜん", "こいぶみ"),
            [SameColumn, Absent, Absent, Absent]
        );
        assert_eq!(
            word_tiles("かいぶつ", "こしょう"),
            [SameRow, SameColumn, Absent, SameColumn]
        );
        assert_eq!(
            word_tiles("だいざい", "たいだん"),
            [SameRowColumn, Correct, SameColumn, Absent]
        );
        assert_eq!(
            word_tiles("だいたい", "たいたい"),
            [SameRowColumn, Correct, Correct, Correct]
        );
    }
}
