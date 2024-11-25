use std::fs;
use std::fs::File;
use std::io::{self, BufWriter, Write};

struct Word<'a> {
    word: &'a str,
    token: usize,
    is_last: bool,
}

fn main() {
    let input = "./input.txt";
    let output = "./output.txt";

    let input_string = match fs::read_to_string(input) {
        Ok(string) => string,
        Err(e) => {
            eprintln!("An error occurred: {e}");
            return;
        }
    };

    let mut words = get_words_vec(&input_string);

    let mut words_idx: Vec<usize> = (0..words.len()).collect();
    words_idx.sort_by_key(|&idx| words[idx].word);
    let unique_count = rmv_dup(&mut words, &mut words_idx);

    match file_out(output, &words, &words_idx, unique_count) {
        Ok(()) => {},
        Err(e) => {
            eprintln!("An error occurred: {e}");
            return;
        }
    }
}

fn get_words_vec(input: &str) -> Vec<Word> {
    let mut words = Vec::new();

    let bytes = input.as_bytes();
    let mut start = 0;
    let mut skip = false;

    for (i, &item) in bytes.iter().enumerate() {
        if skip == true { skip = false; continue };
        if item == b'\n' { start += 1; continue };

        if item == b'.' {
            words.push(Word {
                word: &input[start..i],
                token: 0,
                is_last: true,
            });
            start = i + 2;
            skip = true;
        }

        else if item == b' ' {
            words.push(Word {
                word: &input[start..i],
                token: 0,
                is_last: false,
            });
            start = i + 1;
        }

        if start > input.len() { break };
    }
    if let Some(last_word) = words.last_mut() {
        last_word.is_last = false;
    }

    words
}

fn rmv_dup(words: &mut Vec<Word>, words_idx: &mut Vec<usize>) -> usize {
    let mut unique = 0;
    let mut token = 0;

    for i in 0..words_idx.len() {
        let word_idx = words_idx[i];

        if i == 0 || words[word_idx].word != words[words_idx[unique - 1]].word {
            words_idx[unique] = word_idx;
            token += 1;
            words[word_idx].token = token;
            unique += 1;
        }
        else {
            words[word_idx].token = token;
        }
    }

    unique
}

fn file_out(filepath: &str, words: &Vec<Word>, words_idx: &Vec<usize>, unique_count: usize) -> io::Result<()> {
    let mut outfile = BufWriter::new(File::create(filepath)?);
        
    writeln!(outfile, "{}", unique_count)?;

    for i in 0..unique_count {
        writeln!(outfile, "{}", words[words_idx[i]].word)?;
    }

    for word in words {
        write!(outfile, "{} ", word.token)?;
        if word.is_last { writeln!(outfile)? }
    }

    Ok(())
}