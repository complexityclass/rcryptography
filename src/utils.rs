extern crate base64;
extern crate hex;

pub fn hex_to_base64(str: &str) -> Option<String> {
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

    Some(base64::encode(&res))
}
