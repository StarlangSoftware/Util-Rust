pub struct Subset {
    set: Vec<i32>,
    range_end: i32,
    element_count: usize
}

impl Subset {
    pub fn new(range_start: i32, range_end: i32, element_count: usize) -> Subset {
        let mut set: Vec<i32> = Vec::with_capacity(element_count);
        for i in 0..element_count {
            set.push(range_start + i as i32);
        }
        Self { set, range_end, element_count }
    }

    pub fn get(&self) -> Vec<i32> {
        self.set.clone()
    }

    pub fn next(&mut self) -> bool {
        if self.element_count == 0{
            return false;
        }
        let mut i = self.element_count - 1;
        loop {
            self.set[i] = self.set[i] + 1;
            if self.set[i] <= self.range_end - self.element_count as i32 + i as i32 + 1 {
                break;
            }
            if i == 0 {
                return false;
            }
            i = i - 1;
        }
        let mut j = i + 1;
        while j < self.element_count{
            self.set[j] = self.set[j - 1] + 1;
            j = j + 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::subset::Subset;

    #[test]
    fn next1() {
        let mut subset = Subset::new(1, 10, 5);
        let first_subset = subset.get();
        assert_eq!(first_subset, vec![1, 2, 3, 4, 5]);
        let mut count = 1;
        while subset.next() {
            count += 1;
        }
        assert_eq!(count, 252);
    }

    #[test]
    fn next2() {
        let mut subset = Subset::new(1, 20, 3);
        let first_subset = subset.get();
        assert_eq!(first_subset, vec![1, 2, 3]);
        let mut count = 1;
        while subset.next() {
            count += 1;
        }
        assert_eq!(count, 1140);
    }

    #[test]
    fn next3() {
        let mut count = 0;
        for i in 0..=10 {
            let mut subset = Subset::new(1, 10, i);
            count += 1;
            while subset.next() {
                count += 1;
            }
        }
        assert_eq!(count, 1024);
    }

}