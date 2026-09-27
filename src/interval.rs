use crate::tuple::Tuple;

pub struct Interval{
    list: Vec<Tuple>
}

impl Interval {
    pub fn new() -> Interval {
        Interval {
            list: Vec::new()
        }
    }

    pub fn add(&mut self, start:i16, end:i16) {
        self.list.push(Tuple::new(start, end));
    }

    pub fn get_first(&self, index:usize) -> i16{
        return self.list[index].first();
    }

    pub fn get_last(&self, index:usize) -> i16{
        return self.list[index].last();
    }

    pub fn size(&self) -> usize{
        return self.list.len()
    }

}