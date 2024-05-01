use crate::wrappers::nibble::Nibble;

#[derive(Clone, Debug)]
pub struct Path(Vec<Nibble>);

impl Path {
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        let mut path = vec![];
        for byte in bytes {
            let (nibble_1, nibble_2) = Nibble::from_byte(byte);
            path.push(nibble_1);
            path.push(nibble_2);
        }
        Path(path)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get_nibble_at(&self, i: usize) -> Nibble {
        self.0[i]
    }

    pub fn suffix(&self, s: usize) -> Path {
        return Path(self.0[s..self.0.len()].to_vec());
    }
    pub fn prefix(&self, e: usize) -> Path {
        return Path(self.0[0..e].to_vec());
    }

    pub fn match_prefix(a: &Self, b: &Self) -> usize {
        let mut i_a = a.0.iter();
        let mut i_b = b.0.iter();
        let mut matched_length = 0;
        loop {
            match (i_a.next(), i_b.next()) {
                (Some(x), Some(y)) => {
                    if x.eq(y) {
                        matched_length += 1;
                    } else {
                        return matched_length;
                    }
                }
                _ => return matched_length,
            }
        }
    }
}
