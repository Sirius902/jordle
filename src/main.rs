use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

mod kana;

const WORDS_FILE: &str = include_str!("../assets/japanese_words.txt");

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
    let words = WORDS_FILE.lines().collect::<HashSet<_>>();
    for word in words.iter() {
        assert_eq!(word.chars().count(), 4);
    }

    let entropy = words
        .par_iter()
        .map(|word| EntropyEntry::new(word, word_entropy(word, &words)))
        .collect::<BinaryHeap<_>>();

    for EntropyEntry { word, entropy } in entropy.iter().take(10) {
        println!("{:.3} {}", entropy, word);
    }
}

fn word_tiles(guess: &str, answer: &str) -> [Tile; 4] {
    let mut tiles = [Tile::Absent; 4];
    let mut chars = answer.chars().collect::<Vec<_>>();

    for (i, (g, a)) in guess.chars().zip(answer.chars()).enumerate() {
        if g == a {
            tiles[i] = Tile::Correct;

            if let Some(found_idx) = chars.iter().position(|c| *c == g) {
                chars.swap_remove(found_idx);
            }
        }
    }

    for (i, g) in guess.chars().enumerate() {
        if let Some(found_idx) = chars.iter().position(|c| *c == g) {
            tiles[i] = Tile::Present;
            chars.swap_remove(found_idx);
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

    tiles
}

fn word_entropy(guess: &str, word_list: &HashSet<&str>) -> f64 {
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
        self.entropy.partial_cmp(&other.entropy)
    }
}

impl PartialEq for EntropyEntry<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.entropy == other.entropy
    }
}

impl Eq for EntropyEntry<'_> {}
