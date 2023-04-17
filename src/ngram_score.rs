use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;

use unicode_segmentation::UnicodeSegmentation;
use hashbrown::HashMap;
use itertools::Itertools;

const DEFAULT_SCORE : f64 = -13.;

struct Row<'a>
{
    ngram: &'a str,
    count: u64
}

impl<'a> Row<'a>
{
    fn from_line(s: &'a str) -> Option<Self>
    {
        s.split_whitespace().collect_tuple().map(|(ngram, count)| Row { ngram, count: count.parse::<u64>().unwrap() })
    }
}

fn only_uppercase(buf: &str) -> Vec<u8>
{
    let mut rho = Vec::new();
    for c in buf.graphemes(true)
    {
        match c
        {
            "f" => rho.push(0),
            "v" => rho.push(1),
            "T" => rho.push(2),
            "o" => rho.push(3),
            "r" => rho.push(4),
            "k" => rho.push(5),
            "g" => rho.push(6),
            "w" => rho.push(7),
            "h" => rho.push(8),
            "n" => rho.push(9),
            "i" => rho.push(10),
            "j" => rho.push(11),
            "E" => rho.push(12),
            "p" => rho.push(13),
            "x" => rho.push(14),
            "s" => rho.push(15),
            "t" => rho.push(16),
            "b" => rho.push(17),
            "e" => rho.push(18),
            "m" => rho.push(19),
            "l" => rho.push(20),
            "G" => rho.push(21),
            "O" => rho.push(22),
            "d" => rho.push(23),
            "a" => rho.push(24),
            "A" => rho.push(25),
            "y" => rho.push(26),
            "I" => rho.push(27),
            "X" => rho.push(28),
            _ => (),
        }
    }
    rho
}

pub fn score(s: &[u8], ngram: &HashMap<Vec<u8>, f64>) -> f64
{
    let mut rho : f64 = 0.;
    for i in 0..(s.len() - 4 + 1)
    {
        let key = &s[i..i + 4];
        rho += ngram.get(key).unwrap_or(&DEFAULT_SCORE);
    }
    rho
}

pub fn init(path: PathBuf) -> HashMap<Vec<u8>, f64>
{
    let file = File::open(path).expect("you need a file bruh");
    let reader = BufReader::new(file);

    let mut raw: HashMap<Vec<u8>, u64> = HashMap::new();
    let mut total: u64 = 0;

    for line in reader.lines()
    {
        let actual = &line.unwrap();
        let kv = Row::from_line(actual).expect("malformed line");
        raw.insert(only_uppercase(kv.ngram), kv.count);
        total += kv.count;
    }

    let rho: HashMap<Vec<u8>, f64> = raw.into_iter().map(|(key, value)| (key, (value as f64 / total as f64).log10())).collect();
    rho
}
