use text_size::TextRange;

#[allow(unused)]
#[derive(Debug)]
pub struct ParseError {
    pub msg: String,
    pub range: TextRange,
}

impl ParseError {
    pub fn format_with_line_index(&self, index: &line_index::LineIndex) -> String {
        let line_col = index.line_col(self.range.start());
        format!("{}:{}: {}", line_col.line, line_col.col, self.msg)
    }
}
