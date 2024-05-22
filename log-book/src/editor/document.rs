use std::cmp;


pub struct Row {
    string: String,
}

impl From<&str> for Row {
    fn from(s: &str) -> Self {
        Self {string: s.to_string()}
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        self.string.get(start..end).unwrap_or_default().to_string()
    }


}



#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
}

impl Document {
    pub fn open() -> Self {
        let mut rows = Vec::new();
        rows.push(Row.from("Hello, world!"));
        Self {rows}
    }

    pub fn row(&self, index: usize) -> Option<&Row> {            
        self.rows.get(index)            
    }
}