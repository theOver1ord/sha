extern crate sha3;

use sha3::{Digest, Sha3_256};

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {

    let mut hasher = Sha3_256::new();
    let mut hasher_test = Sha3_256::new(); //для тесту, что бы сверяться, что не проебался
    
    //читаем 2 строки из соответствующих файлов, пушим подпись в текст, хэшируем результат
    
    let mut text = File::open("text.txt").expect("file not found");

    let mut contents_text = String::new();
    text.read_to_string(&mut contents_text)
        .expect("something went wrong reading the file");

    let mut sign = File::open("sign.txt").expect("file not found");

    let mut contents_sign = String::new();
    sign.read_to_string(&mut contents_sign)
        .expect("something went wrong reading the file");    
    
    contents_text.push_str(&contents_sign);


    hasher.input(&contents_text);
    hasher_test.input(b"1A2Babc");


    let result = hasher.result();
    let result_test = hasher_test.result();
    
    println!("Result: {:x}", result);
    println!("test result: {:x}", result_test);
//    println!("contents: {}", contents);
}
