pub struct ArraySorter {
    arr: Array,
    algo: SortAlgo,
    total_swap: u32,
}

impl ArraySorter {
    pub fn new(arr: Vec<u32>, algo: SortAlgo) -> ArraySorter {
        ArraySorter {
            arr: Array::new(arr),
            algo,
            total_swap: 0,
        }
    }

    pub fn sort(&mut self) {
        let algo = self.algo.clone();
        algo.sort(self)
    }

    fn swap(&mut self, i: usize, j: usize) {
        std::thread::sleep(std::time::Duration::from_millis(500));
        self.arr.swap(i, j);
        self.arr.print();
        self.total_swap += 1;
    }
}

struct Array {
    pub arr: Box<Vec<u32>>,
}

impl Array {
    fn new(arr: Vec<u32>) -> Array {
        Array { arr: Box::new(arr) }
    }

    fn size(&self) -> usize {
        self.arr.len()
    }

    fn print(&self) {
        println!("{:?}", self.arr);
    }

    fn swap(&mut self, i: usize, j: usize) {
        assert!(i < self.size() && j < self.size());
        let tmp = self.arr[i];
        self.arr[i] = self.arr[j];
        self.arr[j] = tmp;
    }
}

#[derive(Clone)]
pub enum SortAlgo {
    InserionSort,
}

impl SortAlgo {
    fn sort(&self, arr: &mut ArraySorter) {
        for i in 1..arr.arr.size() {
            let mut j = i;
            while j > 0 && arr.arr.arr[j - 1] > arr.arr.arr[j] {
                arr.swap(j, j - 1);
                j -= 1;
            }
        }
    }
}
