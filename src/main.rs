use crate::parsers::{PdfParser, TxtParser};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, MouseEventKind},
    style::{self, Stylize},
    terminal::{self, size, Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};
use human_panic::setup_panic;
use std::{
    env, fs,
    io::{self, stderr, stdout, Write},
    path::PathBuf,
};

mod pagination;
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

fn print_header(filename: String, window_size: (u16, u16)) {
    stdout()
        .queue(Clear(ClearType::All))
        .expect("Can't clear console")
        .queue(cursor::MoveTo(
            (window_size.0 - filename.len() as u16) / 2,
            0,
        ))
        .expect("Can't move cursor")
        .queue(style::PrintStyledContent(filename.on_white().black()))
        .expect("Can't write to console");
    println!("\n");
}

fn print_footer(window_size: (u16, u16)) {
    stdout()
        .queue(cursor::MoveTo(0, window_size.1 - 1))
        .expect("Can't move cursor")
        .queue(style::PrintStyledContent(
            "Use arrows or HJKL to move around. Press 'q' to quit"
                .on_white()
                .black(),
        ))
        .expect("Can't write to console");
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
                        Some("txt") => Box::new(TxtParser::new((window_size.0, window_size.1 - 2))),
                        Some("pdf") => {
                            let parser = PdfParser::new((window_size.0, window_size.1 - 2));
                            Box::new(parser)
                        }
                        Some(ext) => {
                            println!("Unsupported file format: {}", ext);
                            todo!()
                        }
                        None => Box::new(TxtParser::new((window_size.0, window_size.1 - 2))),
                    };

                    let mut path = env::current_dir()?;
                    path.push(&name);
                    let system_path = path
                        .canonicalize()
                        .unwrap_or(PathBuf::from(name))
                        .to_string_lossy()
                        .to_string();
                    print_header(system_path, window_size);

                    let paginator_required =
                        operator_fn.render(file, (window_size.0, window_size.1 - 2));
                    if paginator_required {
                        terminal::enable_raw_mode().expect("Failed to enable raw mode");
                        loop {
                            if event::poll(std::time::Duration::from_millis(500)).unwrap() {
                                if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
                                    match code {
                                        KeyCode::Char('q') => break,
                                        KeyCode::Char('j') | KeyCode::Down => {
                                            operator_fn.line_down()
                                        }
                                        KeyCode::Char('k') | KeyCode::Up => operator_fn.line_up(),
                                        KeyCode::Char('l') | KeyCode::Right => {
                                            operator_fn.move_right()
                                        }
                                        KeyCode::Char('h') | KeyCode::Left => {
                                            operator_fn.move_left()
                                        }
                                        KeyCode::PageUp => {
                                            for _ in 0..(window_size.1 / 2) {
                                                operator_fn.line_up()
                                            }
                                        }
                                        KeyCode::PageDown => {
                                            for _ in 0..(window_size.1 / 2) {
                                                operator_fn.line_down()
                                            }
                                        }

                                        _ => (),
                                    }
                                    print_footer(window_size);
                                    stdout().flush().expect("Flushing console failed");
                                }
                            }
                        }
                        terminal::disable_raw_mode().expect("Failed to disable raw mode");
                    }
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
