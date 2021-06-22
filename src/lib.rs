#![allow(dead_code)]
#![feature(toowned_clone_into)]
#![feature(map_first_last)]
#![feature(map_into_keys_values)]

use once_cell::sync::Lazy;
use std::str;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    usize,
};

fn letter_freq_score(sentence: &str) -> i64 {
    // static LETTER_FREQ: Lazy<BTreeMap<char, f32>> = Lazy::new(|| include!("letter-freq.txt"));
    let english_freq_order = "ETAOINSHRDLCUMWFGYPBVKJXQZ";

    let mut letter_count = HashMap::new();
    sentence
        .to_uppercase()
        .chars()
        .filter(|ch| ch.is_ascii_alphabetic())
        .for_each(|ch| *letter_count.entry(ch).or_insert(0) += 1);

    // if no letter matches the requirments no need to continue
    if letter_count.is_empty() {
        return 0;
    }

    let mut count_vec: Vec<(_, _)> = letter_count.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));
    let sentence_freq_order: String = count_vec.iter().map(|(ch, _)| **ch).collect();
    // println!("{:?} :: {:?}", count_vec, sentence_freq_order);

    let mut score: i64 = 0;
    // let bounds = min(6, sentence_freq_order.len());
    let bounds = 6;
    for (english, sentence) in english_freq_order
        .as_bytes()
        .chunks(6)
        .zip(sentence_freq_order.as_bytes().chunks(6))
    {
        for ch in sentence.iter() {
            if english.contains(ch) {
                score += 1;
            }
        }
    }
    // let english_top_6 = &english_freq_order[..bounds];
    // let sentence_top_6 = &sentence_freq_order[..bounds];

    // if score > 3 {
    //     println!("{:?} :: {:?} -> {:?}", english_top_6, sentence_top_6, score);
    // }

    /*
    decrease the score for none printable characters. this is to help score human readable text
    above technectly valid english text but with un-readable characters
    */
    // score -= sentence.chars().filter(|ch| ch.is_ascii_control()).count() as i64;

    score
}

fn score_word_match(sentence: &str) -> usize {
    static WORD_LIST: Lazy<HashSet<&'static str>> = Lazy::new(|| {
        include_str!("english-word-list.txt")
            .split_ascii_whitespace()
            .into_iter()
            .clone()
            .collect()
    });

    sentence
        .split_ascii_whitespace()
        .filter(|word| WORD_LIST.contains(word))
        .count()
}

fn xor_buffer(buff: &[u8], key: u8) -> Vec<u8> {
    buff.iter().map(|b| b ^ key).collect()
}

fn break_single_byte_xor(cipher: &[u8]) -> BTreeMap<i64, Vec<String>> {
    let mut options = BTreeMap::new();

    for key in 1..255 {
        let tmp = xor_buffer(cipher, key);
        if let Ok(l) = str::from_utf8(&tmp) {
            options
                .entry(letter_freq_score(l))
                .or_insert(vec![l.to_owned()])
                .push(l.to_owned());
        }
    }

    options
}

fn find_xored_string(ciphers: Vec<Vec<u8>>) -> BTreeMap<i64, Vec<String>> {
    let mut options = BTreeMap::new();

    ciphers
        .iter()
        .for_each(|l| options.append(&mut break_single_byte_xor(&l)));

    options
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

fn break_repeating_key_xor(cipher: &[u8]) {
    let mut distances_map = BTreeMap::new();
    for size in 2..40 {
        if let Ok(distance) = hamming::distance_fast(&cipher[0..size], &cipher[size..size * 2]) {
            distances_map.insert(distance / size as u64, size);
        }
    }

    for _ in 0..4 {
        if let Some((_distance, size)) = distances_map.pop_last() {
            let mut blocks: Vec<Vec<u8>> = Vec::with_capacity(size);
            for chunk in cipher.chunks(size) {
                for (index, byte) in chunk.iter().enumerate() {
                    blocks[index].push(*byte);
                }
            }

            for block in blocks {
                let (key, decypher) = break_single_byte_xor(&block).last_key_value().unwrap();
            }
        }
    }
}

#[test]
fn test_break_repeating_key_xor() {}

#[test]
fn test_hamming() {
    assert_eq!(
        hamming::distance_fast("this is a test".as_bytes(), "wokka wokka!!!".as_bytes()).unwrap(),
        37
    );
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
fn test_find_xored_string() {
    let cipher = include_str!("challenge1.4.txt")
        .lines()
        .map(|l| hex::decode(l).expect("failed parsing hex string"))
        .collect();
    let result = find_xored_string(cipher);
    // println!("{:?}", result);
    let (_score, sentenses) = result.last_key_value().unwrap();

    assert!(sentenses.contains(&String::from("Now that the party is jumping\n")));
}

#[test]
fn test_single_key_xor() {
    let cipher =
        hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
            .expect("failed parsing hex string");
    let result = break_single_byte_xor(&cipher);
    println!("{:?}", result);
    let (_score, sentenses) = result.last_key_value().unwrap();

    assert!(sentenses.contains(&String::from("Cooking MC's like a pound of bacon")));
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
