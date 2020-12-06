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

pub fn split(data: &Arc<String>, pieces: usize) -> Vec<Piece> {
    assert!(pieces > 0);

    let splits = data.split_whitespace();
    let abs_block_len = data.len() / pieces;

    let mut current_block_len = 0;
    let mut total_len = 0;

    let mut abs_lengths = Vec::<usize>::new();

    for num in splits {
        let split_len = if num.len() > 0 { num.len() + 1 } else { 1 };

        total_len += split_len;
        current_block_len += split_len;

        if current_block_len >= abs_block_len || total_len >= data.len() {
            abs_lengths.push(current_block_len);
            current_block_len = 0;
        }
    }

    let mut result = Vec::<Piece>::new();
    let mut prev_idx = 0;

    for len in abs_lengths {
        result.push(Piece::new(&Arc::clone(data), prev_idx, prev_idx + len));
        prev_idx = prev_idx + len;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::split::split;
    use std::sync::Arc;

    #[test]
    fn test_split() {
        let test_buf = "1 2 3 65535 0 10 30 ";
        let expected = split(&Arc::new(String::from(test_buf)), 4);

        assert_eq!(expected.len(), 4);

        assert_eq!(expected[0].slice(), "1 2 3 ");
        assert_eq!(expected[1].slice(), "65535 ");
        assert_eq!(expected[2].slice(), "0 10 ");
        assert_eq!(expected[3].slice(), "30 ");
    }
}
