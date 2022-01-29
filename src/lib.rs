pub mod utils;

#[cfg(test)]
mod tests {
    use crate::utils;

    #[test]
    fn test_hex_base64() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let base64 = utils::hex_to_base64(&hex).unwrap();
        assert_eq!(
            base64,
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_owned()
        );
    }

    #[test]
    fn test_fixed_xor() {
        assert_eq!(
            utils::fixed_xor(
                "1c0111001f010100061a024b53535009181c",
                "686974207468652062756c6c277320657965"
            )
            .unwrap(),
            "746865206b696420646f6e277420706c6179"
        );
    }

    #[test]
    fn test_humming_distance() {
        assert_eq!(utils::humming_score(b"ab", b"zb"), 0.25);
    }
}
