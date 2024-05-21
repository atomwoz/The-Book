use human_panic::setup_panic;
use std::{
    env, fs,
    io::{self, stderr, stdout, BufReader, Write},
    os::windows::fs::FileTypeExt,
};

use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal::{self, size, window_size, Clear, ClearType, WindowSize},
    Command, ExecutableCommand, QueueableCommand,
};

use crate::parsers::TxtParser;

mod mime_parser;
mod parsers;

trait BookPlugin {
    fn render(&self, file: fs::File, terminal_size: (u16, u16));
}

const _VERSION: &'static str = "0.0.1";
const STARTUP_MESSAGE: &'static str = "The BOOK APP v v0.0.1 (Simplified BSD License)";

fn get_usage_error(additional_info: &str) -> String {
    "Usage: book <file_to_read> [OPTIONS]\n".to_string() + additional_info
}

fn file_exist_and_readable(file: fs::File) -> bool {
    file.metadata()
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
}

fn err(error: &str) {
    eprintln!("\n");
    stderr().execute(style::PrintStyledContent(error.red().bold()));
    stderr().flush().expect("Flushing on console sucked !!!");
}

fn main() -> io::Result<()> {
    //Setup panicker
    setup_panic!();

    let window_size = terminal::size().expect("Cannot get terminal window size !!!");
    let center_horizontaly = |text: &str| (window_size.0 - text.len() as u16) / 2;

    //Init console
    stdout()
        .queue(Clear(ClearType::All))
        .expect("Can't initiate terminal properly !!!")
        .queue(cursor::MoveTo(center_horizontaly(STARTUP_MESSAGE), 0))?
        .queue(style::PrintStyledContent(STARTUP_MESSAGE.bold().cyan()))
        .expect("Can't write welcome message");
    println!("\n");

    let file_name = env::args().nth(1);
    match file_name {
        Some(name) => {
            let file = fs::File::open(&name);
            match file {
                Ok(x) => {
                    let extension = name.split('.').nth(1);
                    let operator_fn: Box<dyn BookPlugin> = match extension {
                        Some("txt") => Box::new(TxtParser),
                        Some(_) => todo!(),
                        None => Box::new(TxtParser),
                    };
                    operator_fn.render(x, window_size);
                }
                Err(x) => {
                    stderr().execute(style::PrintStyledContent(
                        get_usage_error(
                            &("File does not exists, or it can't be opened:   ".to_owned()
                                + &x.to_string()),
                        )
                        .red()
                        .bold(),
                    ))?;
                }
            };
        }
        None => {
            stderr().execute(style::PrintStyledContent(
                get_usage_error("No file to render provided").red().bold(),
            ))?;
        }
    };

    stdout().flush().expect("Flushing on console sucked !!! ");
    stderr().flush().expect("Flushing on console sucked !!! ");
    Ok(())
}
