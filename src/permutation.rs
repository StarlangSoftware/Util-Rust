pub struct Permutation {
    a: Vec<i32>,
    n: usize
}

impl Permutation {
    pub fn new(n: usize) -> Self {
        let a: Vec<i32> = (0..n as i32).collect();
        Self { a, n }
    }

    pub fn get(&self) -> Vec<i32> {
        self.a.clone()
    }

    pub fn next(&mut self) -> bool {
        let mut i = self.n - 2;
        while self.a[i] >= self.a[i + 1] {
            if i == 0 {
                return false;
            }
            i = i - 1;
        }
        let mut j = self.n - 1;
        while self.a[i] >= self.a[j] {
            j = j - 1;
        }
        self.a.swap(i, j);
        let mut k = i + 1;
        j = self.n - 1;
        while k < j{
            self.a.swap(k, j);
            k = k + 1;
            j = j - 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::permutation::Permutation;

    #[test]
    fn next1() {
        let mut permutation = Permutation::new(3);
        let first_permutation = permutation.get();
        assert_eq!(first_permutation, vec![0, 1, 2]);
        let mut count = 1;
        while permutation.next() {
            count += 1;
        }
        assert_eq!(count, 6);
    }

    #[test]
    fn next2() {
        let mut permutation = Permutation::new(5);
        let first_permutation = permutation.get();
        assert_eq!(first_permutation, vec![0, 1, 2, 3, 4]);
        let mut count = 1;
        while permutation.next() {
            count += 1;
        }
        assert_eq!(count, 120);
    }

}