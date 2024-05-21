use human_panic::setup_panic;
use std::{
    env,
    io::{self, stdout, Write},
};

use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal::{self, size, window_size, Clear, ClearType, WindowSize},
    Command, ExecutableCommand, QueueableCommand,
};

//WILL BE REPLACED WITH TOKIO
use std::fs;

mod mime_parser;

trait BookPlugin {
    fn render(file: fs::File, terminal_info: WindowSize);
}

const _VERSION: &'static str = "0.0.1";
const STARTUP_MESSAGE: &'static str = "The BOOK APP v v0.0.1 (Simplified BSD License)";

fn get_usage_error(additional_info: &str) -> String {
    "Usage: book <file_to_read> [OPTIONS]\n".to_string() + additional_info
}

fn file_exist_and_readable(file_name: &str) -> bool {
    fs::metadata(file_name)
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
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
        Some(x) => {
            println!("Nioch nioch 🐰🐇, {} will be loaded! //TODO", x);
        }
        None => {
            stdout().execute(style::PrintStyledContent(
                get_usage_error("No file to render provided").red().bold(),
            ))?;
        }
    };

    stdout().flush().expect("Flushing on console sucked !!! ");
    Ok(())
}
