pub struct SubsetFromList {
    set: Vec<i32>,
    element_list: Vec<i32>,
    index_list: Vec<i32>,
    element_count: usize
}

impl SubsetFromList {
    pub fn new(list: Vec<i32>, element_count: usize) -> SubsetFromList {
        let mut set: Vec<i32> = Vec::with_capacity(element_count);
        let element_list: Vec<i32> = list.clone();
        let index_list: Vec<i32> = (0..element_count as i32).collect();
        for i in 0..index_list.len() {
            set.push(element_list[index_list[i] as usize]);
        }
        Self { set, element_list, index_list, element_count}
    }

    pub fn get(&self) -> Vec<i32> {
        self.set.clone()
    }

    pub fn next(&mut self) -> bool {
        if self.element_count == 0 {
            return false;
        }
        let mut i = self.element_count - 1;
        loop {
            self.index_list[i] = self.index_list[i] + 1;
            if self.index_list[i] < self.element_list.len() as i32 - self.element_count as i32 + i as i32 + 1 {
                break;
            }
            if i == 0 {
                return false;
            }
            i = i - 1;
        }
        self.set[i] = self.element_list[self.index_list[i] as usize];
        let mut j = i + 1;
        while j < self.element_count{
            self.index_list[j] = self.index_list[j - 1] + 1;
            self.set[j] = self.element_list[self.index_list[j] as usize];
            j = j + 1;
        }
        true
    }

}

#[cfg(test)]
mod tests {
    use crate::subsetfromlist::SubsetFromList;

    #[test]
    fn next1() {
        let mut subset = SubsetFromList::new(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], 5);
        let first_subset = subset.get();
        assert_eq!(first_subset, vec![10, 20, 30, 40, 50]);
        let mut count = 1;
        while subset.next() {
            count += 1;
        }
        assert_eq!(count, 252);
    }

    #[test]
    fn next2() {
        let mut subset = SubsetFromList::new(vec![9, 8, 2, 12, 7, 16, 17], 3);
        let first_subset = subset.get();
        assert_eq!(first_subset, vec![9, 8, 2]);
        let mut count = 1;
        while subset.next() {
            count += 1;
        }
        assert_eq!(count, 35);
    }

    #[test]
    fn next3() {
        let mut count = 0;
        for i in 0..10 {
            let mut subset = SubsetFromList::new(vec![9, 8, 2, 12, 7, 16, 17, 8, 3], i);
            count += 1;
            while subset.next() {
                count += 1;
            }
        }
        assert_eq!(count, 512);
    }

}