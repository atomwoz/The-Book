use crate::parsers::{PdfParser, TxtParser};
use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal::{self, size, Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};
use human_panic::setup_panic;
use std::{
    env, fs,
    io::{self, stderr, stdout, BufReader, Write},
};

mod parsers;
mod text_utils;

trait BookPlugin {
    fn line_up(&mut self);
    fn line_down(&mut self);
    fn move_right(&mut self);
    fn move_left(&mut self);
    fn render(&mut self, file: fs::File, terminal_size: (u16, u16)) -> bool;
}

const VERSION: &str = "0.0.1";
const STARTUP_MESSAGE: &str = "The BOOK APP v0.0.1 (Simplified BSD License)";

fn get_usage_error(additional_info: &str) -> String {
    format!("Usage: book <file_to_read> [OPTIONS]\n{}", additional_info)
}

fn file_exist_and_readable(file: &fs::File) -> bool {
    file.metadata()
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
}

fn print_error_message(error: &str) {
    eprintln!("\n");
    stderr()
        .execute(style::PrintStyledContent(error.red().bold()))
        .expect("Can't write error");
    stderr().flush().expect("Flushing console failed");
}

fn main() -> io::Result<()> {
    setup_panic!();

    let window_size = terminal::size().expect("Cannot get terminal window size");
    let center_horizontally = |text: &str| (window_size.0 - text.len() as u16) / 2;

    // Initialize console
    stdout()
        .queue(Clear(ClearType::All))?
        .queue(cursor::MoveTo(center_horizontally(STARTUP_MESSAGE), 0))?
        .queue(style::PrintStyledContent(STARTUP_MESSAGE.bold().cyan()))?;
    println!("\n");

    let file_name = env::args().nth(1);
    if let Some(name) = file_name {
        match fs::File::open(&name) {
            Ok(file) => {
                if !file_exist_and_readable(&file) {
                    print_error_message(&get_usage_error(
                        "File does not exist, or it can't be opened",
                    ));
                } else {
                    let mut extension = name.split('.').last();
                    if extension.is_some() && extension.unwrap() == name {
                        extension = None;
                    }
                    let mut operator_fn: Box<dyn BookPlugin> = match extension {
                        Some("txt") => Box::new(TxtParser),
                        Some("pdf") => {
                            let parser = PdfParser::new((window_size.0, window_size.1 - 2));
                            Box::new(parser)
                        }
                        Some(ext) => {
                            println!("Unsupported file format: {}", ext);
                            todo!()
                        }
                        None => Box::new(TxtParser),
                    };
                    operator_fn.render(file, (window_size.0, window_size.1 - 2));
                }
            }
            Err(_) => print_error_message("File does not exist, or it can't be opened"),
        }
    } else {
        print_error_message(&get_usage_error("No file to render provided"));
    }

    stdout().flush().expect("Flushing console failed");
    stderr().flush().expect("Flushing console failed");
    Ok(())
}
