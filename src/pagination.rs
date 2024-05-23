pub struct Pagination {
    pub current_start_line: usize,
    pub current_start_column: usize,
}

impl Pagination {
    pub fn new() -> Self {
        Pagination {
            current_start_line: 0,
            current_start_column: 0,
        }
    }

    pub fn draw(&self, lines: &[String], terminal_size: (u16, u16), line_len: usize) {
        use crossterm::{
            cursor,
            terminal::{self, ClearType},
            ExecutableCommand,
        };
        use std::io::stdout;

        // Draw the lines from current_start_line to current_start_line + terminal_size.1
        stdout()
            .execute(cursor::MoveTo(0, 1))
            .unwrap()
            .execute(terminal::Clear(ClearType::FromCursorDown))
            .unwrap();
        let end_line = self.current_start_line + terminal_size.1 as usize;
        let end_line = if end_line > lines.len() {
            lines.len()
        } else {
            end_line
        };
        let lines_to_draw = &lines[self.current_start_line..end_line];
        let lines_to_draw = lines_to_draw
            .iter()
            .map(|line| {
                let start_column = self.current_start_column;
                let end_column = start_column + terminal_size.0 as usize;
                if start_column > line.len() {
                    ""
                } else if end_column > line.len() {
                    &line[start_column..]
                } else {
                    &line[start_column..end_column]
                }
            })
            .collect::<Vec<&str>>();
        println!("{}", lines_to_draw.join("\n"));
    }

    pub fn line_up(&mut self) {
        self.current_start_line = if self.current_start_line > 0 {
            self.current_start_line - 1
        } else {
            0
        };
    }

    pub fn line_down(&mut self, lines_len: usize, terminal_size: (u16, u16)) {
        let max_start_line = lines_len.saturating_sub(terminal_size.1 as usize);
        self.current_start_line = (self.current_start_line + 1).min(max_start_line);
    }

    pub fn move_right(&mut self, line_len: usize, terminal_size: (u16, u16)) {
        let max_start_column = line_len.saturating_sub(terminal_size.0 as usize);
        self.current_start_column = (self.current_start_column + 1).min(max_start_column);
    }

    pub fn move_left(&mut self) {
        self.current_start_column = if self.current_start_column > 0 {
            self.current_start_column - 1
        } else {
            0
        };
    }
}
