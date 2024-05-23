use std::io::{BufReader, Read};

use crate::pagination::Pagination;
use crate::{print_error_message, text_utils, BookPlugin};

pub struct TxtParser {
    pub lines: Vec<String>,
    pub lines_wrapped: Vec<String>,
    pub terminal_size: (u16, u16),
    pagination: Pagination,
    line_wrapping: bool,
}

impl TxtParser {
    pub fn new(terminal_size: (u16, u16)) -> Self {
        TxtParser {
            lines: Vec::new(),
            lines_wrapped: Vec::new(),
            terminal_size,
            pagination: Pagination::new(),
            line_wrapping: false,
        }
    }

    pub fn draw(&self) {
        let to_draw = if self.line_wrapping {
            &self.lines_wrapped
        } else {
            &self.lines
        };
        self.pagination.draw(&to_draw, self.terminal_size);
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

        let x = String::from_utf8_lossy(&buff);
        let lines: Vec<&str> = x.lines().map(|line| line.trim()).collect();
        let folded = text_utils::fold(&lines, terminal_size.0);

        self.lines = lines.iter().map(|line| line.to_string()).collect();
        self.lines_wrapped = folded;

        self.draw();
        return self.lines.len() > terminal_size.1 as usize;
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

    fn toggle_line_numbers(&mut self) {
        self.pagination.toggle_line_numbers();
    }

    fn refresh(&self) {
        self.draw();
    }
    fn toggle_line_wrapping(&mut self) {
        self.line_wrapping = !self.line_wrapping;
        self.draw();
    }
}
