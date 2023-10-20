use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Write},
};

use encoding::{all::GBK, DecoderTrap, EncoderTrap, Encoding};

pub fn read_as_gbk(filename: String) -> Vec<String> {
    let file = File::open(filename).expect("file not found");
    let reader = BufReader::new(&file);
    reader
        .split(b'\n')
        .map(|l| {
            GBK.decode(&l.unwrap(), DecoderTrap::Strict)
                .unwrap()
                .replace("\r", "")
        })
        .collect::<Vec<String>>()
}
pub fn save_as_gbk(filename: String, content: String) {
    let mut file = File::create(filename).expect("create failed");
    file.write_all(
        GBK.encode(&content, EncoderTrap::Strict)
            .unwrap()
            .as_slice(),
    )
    .expect("write failed");
}
