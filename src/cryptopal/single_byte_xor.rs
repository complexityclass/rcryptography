use std::collections::HashMap;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str;

extern crate hex;

pub fn decrypt_single_byte_xor(msg: &str) -> String {
    let english_f = frequency_map();
    let raw_msg = hex::decode(msg).unwrap();
    let mut min_coeff = f64::MAX;
    let mut text = "".to_owned();

    for k in 0..255 {
        let bcandidate = raw_msg.to_vec().iter().map(|b| b ^ k).collect::<Vec<u8>>();
        if let Ok(candidate) = str::from_utf8(&bcandidate) {
            let coeff = compute_fitting_quotient(&english_f, &calculate_frequency(candidate));
            if coeff < min_coeff {
                min_coeff = coeff;
                text = candidate.to_owned();
            }
        }
    }
    text
}

fn compute_fitting_quotient(english: &HashMap<char, f64>, text: &HashMap<char, f64>) -> f64 {
    let sum = english.iter().fold(0f64, |mut acc, (k, v)| {
        acc += (v - text.get(k).unwrap_or(&0f64)).abs();
        acc
    });

    sum / (english.len() as f64)
}

fn calculate_frequency(msg: &str) -> HashMap<char, f64> {
    let mut hm = HashMap::new();
    for ch in msg.chars() {
        *hm.entry(ch).or_insert(0) += 1;
    }

    let msg_len = msg.len() as f64;
    hm.into_iter()
        .map(|(k, v)| (k, ((v as f64) / msg_len) * 100f64))
        .collect()
}

fn frequency_map() -> HashMap<char, f64> {
    let mut hm = HashMap::new();
    if let Ok(lines) = read_lines("src/cryptopal/letter_frequency.txt") {
        for line in lines {
            if let Ok(line) = line {
                let split = line.split(':').collect::<Vec<&str>>();
                hm.insert(
                    split[0].chars().next().unwrap(),
                    split[1].parse::<f64>().unwrap(),
                );
            }
        }
    }
    hm
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
