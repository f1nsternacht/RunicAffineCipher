use std::cmp::Ordering;
use std::path::PathBuf;

use hashbrown::HashMap;
use unicode_segmentation::UnicodeSegmentation;
use clap::Parser;

mod ngram_score;

fn r2v(ct: &String) -> Vec<u8>
{
    ct.graphemes(true).map(|x|
        match x
        {
            "ᚠ" => 0,
            "ᚢ" => 1,
            "ᚦ" => 2,
            "ᚩ" => 3,
            "ᚱ" => 4,
            "ᚳ" => 5,
            "ᚷ" => 6,
            "ᚹ" => 7,
            "ᚻ" => 8,
            "ᚾ" => 9,
            "ᛁ" => 10,
            "ᛂ" => 11,
            "ᛇ" => 12,
            "ᛈ" => 13,
            "ᛉ" => 14,
            "ᛋ" => 15,
            "ᛏ" => 16,
            "ᛒ" => 17,
            "ᛖ" => 18,
            "ᛗ" => 19,
            "ᛚ" => 20,
            "ᛝ" => 21,
            "ᛟ" => 22,
            "ᛞ" => 23,
            "ᚪ" => 24,
            "ᚫ" => 25,
            "ᚣ" => 26,
            "ᛡ" => 27,
            "ᛠ" => 28,
            _ => 255,
        }
    ).filter(|x| *x<29).collect()
}

fn v2e(v: &[u8]) -> String
{
    let alphabet = ["f", "v", "T", "o", "r", "k", "g", "w", "h", "n", "i", "j", "E", "p", "x", "s", "t", "b", "e", "m", "l", "G", "O", "d", "a", "A", "y", "I", "X"];
    v.iter().map(|&x| alphabet[x as usize].to_owned()).collect::<Vec<String>>().join("")
}

fn do_math(x: u8, a: u8, b: u8) -> u8
{
    ((a as u64 * (x as u64 + b as u64)) % 29u64) as u8
}

fn affine(ct: &[u8], key_a: u8, key_b: u8) -> Vec<u8>
{
    assert!(key_a >  0, "a needs to be greater than 0");
    assert!(key_a < 29, "a needs to be less than 29");
    assert!(key_b < 29, "b needs to be less than 29");
    ct.iter().map(|&x| do_math(x, key_a, key_b)).collect()
}

struct Score
{
    a: u8,
    b: u8,
    score: f64,
}

impl PartialOrd for Score
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        self.score.partial_cmp(&other.score)
    }
}

impl PartialEq for Score
{
    fn eq(&self, other: &Self) -> bool
    {
        self.score == other.score
    }
}

fn attack(ct: &[u8], ngram: &HashMap<Vec<u8>, f64>) -> Vec<Score>
{
    let mut results: Vec<Score> = Vec::with_capacity(29 * 29);

    for a in 1..29
    {
        for b in 0..29
        {
            let pt = affine(ct, a, b);
            let score = ngram_score::score(&pt, ngram);

            results.push( Score{a, b, score} );
        }
    }
    results.sort_by(|a, b| b.partial_cmp(a).unwrap());

    results
}

/// brute force affine cipher with ngram scoring
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args
{
    /// cipher text to attack
    cipher: String,
    /// number of results to show
    n: usize,

    /// path to ngram statistics
    #[arg(long)]
    ngrams: Option<std::path::PathBuf>,
}

fn main()
{
    let args = Args::parse();
    let ct = r2v(&args.cipher);

    let path = match args.ngrams
    {
        Some(x) => x,
        None => PathBuf::from("frequency/runeglish_quadgrams.txt"),
    };

    let ngram = ngram_score::init(path);
    let scores = attack(&ct, &ngram);

    for score in scores.iter().take(args.n).rev()
    {
        println!("a: {:2} b:{:2} -> {:.4}", score.a, score.b, score.score);
        println!("{}", v2e(&affine(&ct, score.a, score.b)));
    }
}
