use rsort::{ArraySorter, SortAlgo};

fn main() {
    let v: Vec<u32> = vec![5, 2, 1, 4, 3];
    let algo = SortAlgo::BubbleSort;
    let mut solver = ArraySorter::new(v.clone(), algo);
    solver.sort();
}