use rsort::{ArraySorter, SortAlgo};

fn main() {
    let v: Vec<u32> = vec![5, 8, 2, 7, 1, 4, 3, 6];
    // let v: Vec<u32> = vec![2, 1];
    let algo = SortAlgo::QuickSort;
    let mut solver = ArraySorter::new(v.clone(), algo);
    solver.sort();
}