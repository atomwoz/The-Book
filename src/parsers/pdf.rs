use std::io::{stdout, BufReader, Read};

use crossterm::{
    style::{self, Stylize},
    ExecutableCommand,
};

use crate::{pagination::Pagination, text_utils::fold};
use crate::{print_error_message, BookPlugin};

pub struct PdfParser {
    pub lines: Vec<String>,
    pub lines_wrapped: Vec<String>,
    pub terminal_size: (u16, u16),
    line_len: usize,
    pagination: Pagination,
    line_wrapping: bool,
}

impl PdfParser {
    pub fn new(terminal_size: (u16, u16)) -> Self {
        PdfParser {
            lines: Vec::new(),
            lines_wrapped: Vec::new(),
            terminal_size,
            line_len: 0,
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

impl BookPlugin for PdfParser {
    fn render(&mut self, file: std::fs::File, terminal_size: (u16, u16)) -> bool {
        let mut buff = Vec::new();
        let buff_result = BufReader::new(file).read_to_end(&mut buff);
        if buff_result.is_err() {
            print_error_message("There was problem reading file, please try again");
            return false;
        }
        stdout()
            .execute(style::PrintStyledContent(
                "File is being proceeded...".blue(),
            ))
            .unwrap();

        let result = pdf_extract::extract_text_from_mem(&buff);
        match result {
            Ok(text) => {
                let lines = text.lines().collect::<Vec<&str>>();
                self.lines_wrapped.extend(fold(&lines, terminal_size.0));
                self.lines.extend(lines.into_iter().map(|x| x.to_string()));

                self.draw();

                self.line_len = self.lines.iter().map(|line| line.len()).max().unwrap_or(0);
                return self.lines.len() > terminal_size.1 as usize;
            }
            Err(_) => print_error_message("There was problem reading file, please try again"),
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
        self.pagination
            .move_right(self.line_len, self.terminal_size);
        self.draw();
    }

    fn move_left(&mut self) {
        self.pagination.move_left();
        self.draw();
    }

    fn refresh(&self) {
        self.draw();
    }

    fn toggle_line_numbers(&mut self) {
        self.pagination.toggle_line_numbers();
        self.draw();
    }

    fn toggle_line_wrapping(&mut self) {
        self.line_wrapping = !self.line_wrapping;
        self.draw();
    }
}
