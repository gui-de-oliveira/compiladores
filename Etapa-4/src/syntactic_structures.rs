#[derive(Debug)]
pub struct Symbol {
    pub id: String,
    pub line: usize,
    pub col: usize,
}

impl Symbol {
    pub fn new(id: String, line: usize, col: usize) -> Symbol {
        Symbol { id, line, col }
    }
}
