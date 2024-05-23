use std::io::{BufReader, Read};

use crate::pagination::Pagination;
use crate::{print_error_message, text_utils, BookPlugin};

pub struct TxtParser {
    pub lines: Vec<String>,
    pub terminal_size: (u16, u16),
    pagination: Pagination,
}

impl TxtParser {
    pub fn new(terminal_size: (u16, u16)) -> Self {
        TxtParser {
            lines: Vec::new(),
            terminal_size,
            pagination: Pagination::new(),
        }
    }

    pub fn draw(&self) {
        self.pagination.draw(
            &self.lines,
            self.terminal_size,
            self.lines.iter().map(|line| line.len()).max().unwrap_or(0),
        );
    }
}

impl BookPlugin for TxtParser {
    fn render(&mut self, file: std::fs::File, terminal_size: (u16, u16)) -> bool {
        let mut buff = Vec::new();
        let buff_result = BufReader::new(file).read_to_end(&mut buff);
        if buff_result.is_err() {
            print_error_message("There was problem reading file, please try again");
            return false;
        }

        match String::from_utf8(buff) {
            Ok(x) => {
                let lines: Vec<&str> = x.lines().map(|line| line.trim()).collect();
                let folded = text_utils::fold(&lines, terminal_size.0);
                let use_folding = folded.len() <= terminal_size.1 as usize;

                self.lines = if use_folding {
                    folded.iter().map(|x| x.to_string()).collect()
                } else {
                    lines.iter().map(|x| x.to_string()).collect()
                };

                self.draw();
                return self.lines.len() > terminal_size.1 as usize;
            }
            Err(_) => print_error_message("File doesn't contain valid UTF-8 text"),
        }
        return false;
    }

    fn line_up(&mut self) {
        self.pagination.line_up();
        self.draw();
    }

    fn line_down(&mut self) {
        self.pagination
            .line_down(self.lines.len(), self.terminal_size);
        self.draw();
    }

    fn move_right(&mut self) {
        self.pagination.move_right(
            self.lines.iter().map(|line| line.len()).max().unwrap_or(0),
            self.terminal_size,
        );
        self.draw();
    }

    fn move_left(&mut self) {
        self.pagination.move_left();
        self.draw();
    }
}
