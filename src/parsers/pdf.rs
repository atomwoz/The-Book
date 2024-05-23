use std::io::{stdout, BufReader, BufWriter, Read, Write};

use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal::{self, ClearType},
    ExecutableCommand,
};

use crate::{print_error_message, text_utils, BookPlugin};

pub struct PdfParser {
    pub lines: Vec<String>,
    pub current_start_line: usize,
    pub current_start_column: usize,
    pub terminal_size: (u16, u16),
}
impl PdfParser {
    pub fn new(terminal_size: (u16, u16)) -> Self {
        PdfParser {
            lines: Vec::new(),
            current_start_line: 0,
            current_start_column: 0,
            terminal_size,
        }
    }
    pub fn draw(&self) {
        //Draw the lines from currebt_start_line to current_start_line + terminal_size.1
        let end_line = self.current_start_line + self.terminal_size.1 as usize;
        let end_line = if end_line > self.lines.len() {
            self.lines.len()
        } else {
            end_line
        };
        let lines_to_draw = &self.lines[self.current_start_line..end_line];
        println!("{}", lines_to_draw.join("\n"));
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
        stdout()
            .execute(cursor::MoveTo(1, 0))
            .unwrap()
            .execute(terminal::Clear(ClearType::All))
            .unwrap();
        match result {
            Ok(text) => {
                let lines = text.lines().collect::<Vec<&str>>();
                self.lines.extend(lines.into_iter().map(|x| x.to_string()));
                self.draw();
                return self.lines.len() > terminal_size.1 as usize;
            }
            Err(_) => print_error_message("There was problem reading file, please try again"),
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
