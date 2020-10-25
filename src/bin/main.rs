use rsort::{Array, SortAlgo};


fn main() {
    let v: Vec<u32> = vec![5, 2, 1, 4, 3];
    let mut arr = Array::new(v);
    let algo = SortAlgo::InserionSort;
    algo.sort(&mut arr);
}