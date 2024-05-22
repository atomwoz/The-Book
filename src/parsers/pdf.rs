use std::io::{stdout, BufReader, BufWriter, Read, Write};

use crossterm::{
    cursor,
    style::{self, Stylize},
    ExecutableCommand,
};

use crate::{err, BookPlugin};

pub struct PdfParser;
impl BookPlugin for PdfParser {
    fn render(&self, file: std::fs::File, terminal_size: (u16, u16)) {
        let mut buff = Vec::new();
        let buff_result = BufReader::new(file).read_to_end(&mut buff);
        if buff_result.is_err() {
            err("There was problem reading file, please try again");
            return;
        }
        stdout()
            .execute(style::PrintStyledContent(
                "File is being proceeded...".blue(),
            ))
            .unwrap();

        let result = pdf_extract::extract_text_from_mem(&buff);
        match result {
            Ok(text) => {
                stdout().execute(cursor::MoveTo(0, 1)).unwrap();
                println!("{}", text);
            }
            Err(_) => err("There was problem reading file, please try again"),
        }
    }
}
