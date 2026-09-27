use rand::seq::SliceRandom;
use rand::rng;
pub struct RandomArray{
}

impl RandomArray{
    pub fn new() -> RandomArray{
        RandomArray{}
    }

    pub fn normalized_array(item_count: usize) -> Box<[f32]>{
        let mut array: Box<[f32]> = vec![0.0; item_count].into_boxed_slice();
        let mut sum: f32 = 0.0;
        for i in 0..item_count{
            array[i] = rand::random_range(0.0..1.0);
            sum += array[i];
        }
        for i in 0..item_count{
            array[i] /= sum;
        }
        array
    }

    pub fn index_array(item_count: usize) -> Vec<usize>{
        let mut random_array: Vec<usize> = (0..item_count).collect();
        random_array.shuffle(&mut rng());
        random_array
    }
}

#[cfg(test)]
mod tests {
    use crate::randomarray::RandomArray;

    #[test]
    fn normalize_array_test() {
        let array = RandomArray::normalized_array(10);
        let sum: f32 = array.iter().sum();
        assert_eq!(sum, 1.0);
    }

    #[test]
    fn index_array_test() {
        let array = RandomArray::index_array(10);
        let sum : usize = array.iter().sum();
        assert_eq!(sum, 45);
    }

}