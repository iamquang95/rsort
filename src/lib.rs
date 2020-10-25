pub struct ArraySorter {
    arr: Array,
    algo: SortAlgo,
}

impl ArraySorter {

    pub fn new(
        arr: Vec<u32>,
        algo: SortAlgo,
    ) -> ArraySorter {
        ArraySorter {
            arr: Array::new(arr),
            algo,
        }
    }

    pub fn sort(&mut self) {
        self.algo.sort(&mut self.arr)
    }
}

struct Array {
    pub arr: Box<Vec<u32>>
}

impl Array {

    fn new(arr: Vec<u32>) -> Array {
        Array {
            arr: Box::new(arr)
        }
    }

    fn size(&self) -> usize {
        self.arr.len()
    }

    fn print(&self) {
        std::thread::sleep(std::time::Duration::from_millis(500));
        println!("{:?}", self.arr);
    }

    fn swap(&mut self, i: usize, j: usize) {
        assert!(i < self.size() && j < self.size());
        let tmp = self.arr[i];
        self.arr[i] = self.arr[j];
        self.arr[j] = tmp;
    }

}

pub enum SortAlgo {
    InserionSort,
}

impl SortAlgo {
    fn sort(&self, arr: &mut Array) {
        for i in 1..arr.size() {
            let mut j = i;
            while j > 0 && arr.arr[j-1] > arr.arr[j] {
                arr.swap(j, j-1);
                j -= 1;
                arr.print();
            }
        }
    }
}
