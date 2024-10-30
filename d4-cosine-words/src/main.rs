use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader, Result, Seek, SeekFrom, Write},
    path::Path,
};

use ahash::RandomState;

fn cosine_angle_mut(a: &mut [f64], b: &mut [f64]) -> f64 {
    assert_eq!(a.len(), b.len());

    let mut a_magnitude = a.iter().copied().map(|x| x * x).sum::<f64>().sqrt();
    if a_magnitude <= 0f64 {
        a_magnitude = 1f64;
    }
    a.iter_mut().for_each(|x| *x /= a_magnitude);

    let mut b_magnitude = b.iter().copied().map(|x| x * x).sum::<f64>().sqrt();
    if b_magnitude <= 0f64 {
        b_magnitude = 1f64;
    }
    b.iter_mut().for_each(|x| *x /= b_magnitude);

    a.iter()
        .copied()
        .zip(b.iter().copied())
        .map(|(a, b)| a * b)
        .sum::<f64>()
}

type Dictionary = BTreeMap<u64, Vec<(usize, u32)>>;
fn vectorize_sentence(
    sentence: &str,
    dictionary_meta: &Dictionary,
    dictionary: impl AsRef<Path>,
) -> Result<Vec<f64>> {
    sentence
        .split(' ')
        .map(|word| word_position(word, dictionary_meta, &dictionary))
        .map(|position| position.map(f64::from))
        .collect()
}

fn word_position(
    word: &str,
    dictionary_meta: &Dictionary,
    dictionary: impl AsRef<Path>,
) -> Result<u32> {
    let mut reader = BufReader::new(File::open(dictionary)?);
    let mut string_buffer = String::with_capacity(10);
    let word_hash = RandomState::with_seed(0).hash_one(word);

    if let Some(word_meta) = dictionary_meta.get(&word_hash) {
        for &(byte_position, word_position) in word_meta {
            _ = reader.seek(SeekFrom::Start(byte_position as u64))?;
            _ = reader.read_line(&mut string_buffer)?;
            string_buffer.make_ascii_lowercase();

            if word.trim() == string_buffer.trim() {
                return Ok(word_position);
            }
        }
    }

    Ok(0)
}

fn construct_dictionary(file: impl AsRef<Path>) -> Result<Dictionary> {
    let mut dict = Dictionary::new();

    let mut reader = BufReader::new(File::open(file)?);
    let mut string_buffer = String::with_capacity(10);

    let hasher = RandomState::with_seed(0);
    let mut byte_position = 0;
    let mut word_position = 0;

    loop {
        let read = reader.read_line(&mut string_buffer)?;
        if read == 0 {
            break;
        }

        string_buffer.make_ascii_lowercase();
        let word_hash = hasher.hash_one(string_buffer.trim());
        let word_meta = (byte_position, word_position);

        if let Some(ranges) = dict.get_mut(&word_hash) {
            ranges.push(word_meta);
        } else {
            dict.insert(word_hash, vec![word_meta]);
        }

        word_position += 1;
        byte_position += read;
        string_buffer.clear();
    }

    return Ok(dict);
}

fn main() -> Result<()> {
    let dict_loc = std::path::absolute("./english-words/words_alpha.txt")?;
    println!("Using dictionary {dict_loc:?}");
    let dict = construct_dictionary(&dict_loc)?;

    println!("Provide two sentences for a cosine similarity, using their words and an English dictionary.");

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    // let mut a = String::from("not similar at all");
    // let mut b = String::from("test");
    let mut a = String::new();
    let mut b = String::new();

    print!("Sentence A: ");
    stdout.flush()?;
    stdin.read_line(&mut a)?;
    a.make_ascii_lowercase();
    print!("Sentence B: ");
    stdout.flush()?;
    stdin.read_line(&mut b)?;
    b.make_ascii_lowercase();

    let mut a: Vec<f64> = vectorize_sentence(&a, &dict, &dict_loc)?;
    let mut b: Vec<f64> = vectorize_sentence(&b, &dict, &dict_loc)?;

    if a.len() != b.len() {
        let diff = b.len().max(a.len());
        let small = if a.len() < b.len() { &mut a } else { &mut b };
        small.resize(diff, 0f64);
    }

    let cosine = cosine_angle_mut(&mut a, &mut b);

    println!("Cosine similarity: {cosine:.4}");
    Ok(())
}
