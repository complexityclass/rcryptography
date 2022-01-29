extern crate base64;
extern crate hex;

use to_binary::{BinaryError, BinaryString};

pub fn hex_to_base64(str: &str) -> Option<String> {
    let bytes = hex_to_vec(str)?;
    Some(base64::encode(&bytes))
}

pub fn fixed_xor(lhs_hex: &str, rhs_hex: &str) -> Option<String> {
    if lhs_hex.len() != rhs_hex.len() {
        return None;
    }

    let lhs = hex_to_vec(lhs_hex).unwrap();
    let rhs = hex_to_vec(rhs_hex).unwrap();
    Some(hex::encode(
        lhs.into_iter()
            .zip(rhs.into_iter())
            .map(|(l, r)| l ^ r)
            .collect::<Vec<u8>>(),
    ))
}

pub fn humming_score(lhs: &[u8], rhs: &[u8]) -> f64 {
    humming_distance(lhs, rhs) as f64 / (8 * lhs.len().min(rhs.len())) as f64
}

fn humming_distance(lhs: &[u8], rhs: &[u8]) -> i32 {
    if lhs.len() != rhs.len() {
        return 0;
    }

    let mut dist = 0;
    for (b1, b2) in lhs.into_iter().zip(rhs.into_iter()) {
        let bin_str = BinaryString::from(b1 ^ b2);
        dist += bin_str.to_string().chars().fold(0, |acc, item| match item {
            '0' => acc,
            '1' => acc + 1,
            _ => acc,
        });
    }

    dist
}

fn hex_to_vec(str: &str) -> Option<Vec<u8>> {
    if str.len() % 2 != 0 {
        return None;
    }

    let mut res = vec![];
    res.resize(str.len() / 2, 0u8);
    let chars: Vec<char> = str.chars().collect::<Vec<_>>();

    for i in (0..str.len()).step_by(2) {
        res[i / 2] =
            (chars[i].to_digit(16).unwrap() << 4) as u8 + chars[i + 1].to_digit(16).unwrap() as u8;
    }

    Some(res)
}
