pub fn fold(lines: &Vec<&str>, max_line_width: u16) -> Vec<String> {
    let mut folded_lines = Vec::new();
    for line in lines {
        let mut current_line = String::new();
        let mut current_line_width = 0;
        for word in line.split_whitespace() {
            if current_line_width + word.len() as u16 > max_line_width {
                folded_lines.push(current_line.trim_end().to_string());
                current_line = String::new();
                current_line_width = 0;
            }
            current_line.push_str(word);
            current_line.push(' ');
            current_line_width += word.len() as u16 + 1;
        }
        folded_lines.push(current_line.trim_end().to_string());
    }
    folded_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fold() {
        let lines = vec![
            "This is a long line that needs to be folded into multiple lines",
            "This is another line",
            "Short line",
        ];
        let max_line_width = 10;
        let folded_lines = fold(&lines, max_line_width);
        assert_eq!(
            folded_lines,
            vec![
                String::from("This is a"),
                String::from("long line"),
                String::from("that needs"),
                String::from("to be"),
                String::from("folded"),
                String::from("into"),
                String::from("multiple"),
                String::from("lines"),
                String::from("This is"),
                String::from("another"),
                String::from("line"),
                String::from("Short line"),
            ]
        );
    }
}
