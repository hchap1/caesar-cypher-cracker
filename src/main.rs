use std::fs::read_to_string;
use std::io::{self, Write};
use std::env::args;
use std::thread::sleep;
use std::time::Duration;
use std::collections::HashMap;

fn usize_to_char(idx: usize) -> Option<char> {
    if idx < 26 {
        Some((idx as u8 + b'a') as char)
    } else { None }
}

fn char_to_usize(c: char) -> Option<usize> {
    if c.is_ascii_lowercase() {
        Some((c as usize) - ('a' as usize))
    } else { None }
}

fn main() {
    let mut args = args();
    match args.nth(1) {
        Some(raw) => {
            let animate: bool = args.collect::<Vec<String>>().contains(&String::from("--animate"));
            let encoded_message = raw.to_lowercase();
            let mut dict_hash: HashMap<String, bool> = HashMap::new();
            match read_to_string("dictionary.txt") {
                Ok(dict_words) => {
                    for word in dict_words.lines().map(|x| x.trim().to_string()).collect::<Vec<String>>() {
                        dict_hash.insert(word, true);
                    }
                    let mut max_validity: f32 = 0f32;
                    let mut best_shift: usize = 0;
                    let mut best_decode: String = String::new();
                    for shift in 0..26 {
                        let mut decoded_message: String = String::new();
                        for c in encoded_message.chars() {
                            decoded_message.push(match char_to_usize(c) {
                                Some(idx) => { usize_to_char((idx + shift) % 26).unwrap_or(' ') }
                                None => { c }
                            });
                        }
                        let mut count: f32 = 0f32;
                        let mut valid: f32 = 0f32;
                        for word in decoded_message.split(' ') {
                            count += 1f32;
                            match dict_hash.get(word) {
                                Some(_) => { valid += 1f32; }
                                None => {}
                            }
                        }
                        let validity: f32 = valid / count;
                        if animate {
                            print!("+{}-> {decoded_message}    \r", 26 - shift);
                            let _ = io::stdout().flush();
                            sleep(Duration::from_millis(50));
                        }
                        if validity > max_validity {
                            max_validity = validity;
                            best_shift = 26 - shift;
                            best_decode = decoded_message;
                        }
                    }
                    println!("+{best_shift}-> {best_decode}    ");
                }
                Err(_) => {
                    eprintln!("Error: Unable to access dictionary.txt.");
                }
            }
        }
        None => {
            eprintln!("Error: Expected 1 argument: Message, then flags.");
        }
    }
}
