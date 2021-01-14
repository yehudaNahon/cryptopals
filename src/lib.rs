#![allow(dead_code)]
#![feature(toowned_clone_into)]

use once_cell::sync::Lazy;
use std::{collections::HashSet, str};

fn get_word_score(sentence: &str) -> i32 {
    static WORD_LIST: Lazy<HashSet<&'static str>> = Lazy::new(|| {
        include_str!("english-word-list.txt")
            .split_ascii_whitespace()
            .into_iter()
            .clone()
            .collect()
    });

    let mut sum = 0;
    for word in sentence.split_ascii_whitespace() {
        if WORD_LIST.contains(word) {
            sum += 1;
        }
    }
    sum
}

fn decrypt_single_byte_xor(cipher: &[u8]) -> (String, i32) {
    let mut best_line = String::from("");
    let mut best_score = 0;

    for key in 1..255 {
        let tmp: Vec<u8> = cipher.iter().map(|b| b ^ key).collect();
        let line = str::from_utf8(&tmp).unwrap_or("");
        let score = get_word_score(line);
        if score > best_score {
            line.clone_into(&mut best_line);
            best_score = score;
        }
    }

    (best_line, best_score)
}

fn xor_buffs(buff1: &[u8], buff2: &[u8]) -> Vec<u8> {
    buff1
        .iter()
        .zip(buff2.iter())
        .map(|(byte1, byte2)| byte1 ^ byte2)
        .collect()
}

fn repeating_key_xor(text: &[u8], key: &[u8]) -> Vec<u8> {
    text.iter()
        .zip(key.iter().cycle())
        .map(|(b, k)| b ^ k)
        .collect()
}

#[test]
fn test_repeating_key_xor() {
    let sentence = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let exp_hex = hex::decode(
        "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f").unwrap();
    let cipher = repeating_key_xor(sentence.as_bytes(), b"ICE");
    assert_eq!(cipher, exp_hex);
}

#[test]
fn find_xored_string() {
    let mut best = (String::from(""), 0);
    for cipher in include_str!("challenge1.4.txt").lines() {
        let cipher = hex::decode(cipher).expect("failed parsing hex string");
        let decrypted = decrypt_single_byte_xor(&cipher);
        if decrypted.1 > best.1 {
            best = decrypted;
        }
    }

    assert_eq!(best.0, String::from("Now that the party is jumping\n"));
}

#[test]
fn test_single_key_xor() {
    let cipher =
        hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
            .expect("failed parsing hex string");
    assert_eq!(
        decrypt_single_byte_xor(&cipher).0,
        String::from("Cooking MC's like a pound of bacon")
    );
}

#[test]
fn test_xor_between_buffers() {
    let buff1 = hex::decode("1c0111001f010100061a024b53535009181c").expect("failed parsing");
    let buff2 = hex::decode("686974207468652062756c6c277320657965").expect("failed parsing");
    assert_eq!(
        hex::encode(xor_buffs(&buff1, &buff2)),
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
