use std::io::{BufReader, Read};

use crate::{print_error_message, text_utils, BookPlugin};

pub struct TxtParser;
impl BookPlugin for TxtParser {
    fn render(&mut self, file: std::fs::File, terminal_size: (u16, u16)) -> bool {
        let mut buff = Vec::new();
        let buff_result = BufReader::new(file).read_to_end(&mut buff);
        if buff_result.is_err() {
            print_error_message("There was problem reading file, please try again");
        }
        match String::from_utf8(buff) {
            Ok(x) => {
                let lines: Vec<&str> = x.lines().map(|line| line.trim()).collect();
                let folded = text_utils::fold(&lines, terminal_size.0);
                println!("{}", folded.join("\n"));
            }
            Err(_) => print_error_message("File doesn't contain valid UTF-8 text"),
        }
        return false;
    }

    fn line_up(&mut self) {
        todo!()
    }

    fn line_down(&mut self) {
        todo!()
    }

    fn move_right(&mut self) {
        todo!()
    }

    fn move_left(&mut self) {
        todo!()
    }
}
