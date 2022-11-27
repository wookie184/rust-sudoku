pub struct BitIter {
    n: usize,
}

impl Iterator for BitIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n > 0 {
            let index = self.n.trailing_zeros();
            self.n ^= 1 << index;
            Some(2 << (index - 1))
        } else {
            None
        }
    }
}

pub trait IterBits {
    fn iter_bits(&self) -> BitIter;
}

impl IterBits for usize {
    fn iter_bits(&self) -> BitIter {
        BitIter { n: *self }
    }
}
