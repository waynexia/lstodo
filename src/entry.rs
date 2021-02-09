use git2::Oid;

#[derive(Debug)]
pub struct Entry {
    path: String,
    line_number: u64,
    content: String,
    oid: Option<Oid>,
}

impl Entry {
    pub fn new(path: String, line_number: u64, content: String, oid: Option<Oid>) -> Self {
        Self {
            path,
            line_number,
            content,
            oid,
        }
    }
}
