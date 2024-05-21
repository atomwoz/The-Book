use std::io::{BufReader, Read};

use crate::{err, BookPlugin};

pub struct TxtParser;
impl BookPlugin for TxtParser {
    fn render(&self, file: std::fs::File, terminal_size: (u16, u16)) {
        let mut buff = Vec::new();
        let buff_result = BufReader::new(file).read_to_end(&mut buff);
        if buff_result.is_err() {
            err("There was problem reading file, please try again");
        }
        match String::from_utf8(buff) {
            Ok(x) => println!("{}", x),
            Err(_) => err("File doesn't contain valid UTF-8 text"),
        }
    }
}
