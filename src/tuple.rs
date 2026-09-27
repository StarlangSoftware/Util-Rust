pub struct Tuple {
    first: i16,
    last: i16
}

impl Tuple {
    pub fn new(first: i16, last: i16) -> Tuple {
        Tuple {
            first,
            last
        }
    }

    pub fn first(&self) -> i16 {
        self.first
    }

    pub fn last(&self) -> i16 {
        self.last
    }
}