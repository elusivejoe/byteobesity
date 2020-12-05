use std::sync::Arc;

pub struct Piece {
    shared_data: Arc<String>,
    slice_begin: usize,
    slice_end: usize,
}

impl Piece {
    pub fn new(data: &Arc<String>, begin: usize, end: usize) -> Piece {
        Piece {
            shared_data: Arc::clone(data),
            slice_begin: begin,
            slice_end: end,
        }
    }

    pub fn slice(&self) -> &str {
        &self.shared_data[self.slice_begin..self.slice_end]
    }
}
