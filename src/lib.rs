use base64;
use hex;

fn xor(buff1: &[u8], buff2: &[u8]) -> Vec<u8> {
    buff1
        .iter()
        .zip(buff2.iter())
        .map(|(byte1, byte2)| byte1 ^ byte2)
        .collect()
}

pub fn xor_buffer(buffer_in: &[u8], key: u8) -> Vec<u8> {
    buffer_in.iter().map(|byte| *byte ^ key).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_between_buffers() {
        let buff1 = hex::decode("1c0111001f010100061a024b53535009181c").expect("failed parsing");
        let buff2 = hex::decode("686974207468652062756c6c277320657965").expect("failed parsing");
        assert_eq!(
            hex::encode(xor(&buff1, &buff2)),
            "746865206b696420646f6e277420706c6179"
        );
    }
    #[test]
    fn test_hex_to_base64() {
        let hex = hex::decode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").expect("Failed parsing hex string");
        assert_eq!(
            base64::encode(hex),
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }
}
