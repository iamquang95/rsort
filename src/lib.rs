pub struct ArraySorter {
    arr: Array,
    algo: SortAlgo,
    tui: TermUI,
    total_swap: u32,
}

impl ArraySorter {
    pub fn new(arr: Vec<u32>, algo: SortAlgo) -> ArraySorter {
        ArraySorter {
            arr: Array::new(arr),
            algo,
            tui: TermUI::new(),
            total_swap: 0,
        }
    }

    pub fn sort(&mut self) {
        let algo = self.algo.clone();
        algo.sort(self)
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.tui.write(self.arr.print_swap(i, j));
        std::thread::sleep(std::time::Duration::from_millis(250));
        self.arr.swap(i, j);
        self.tui.write(self.arr.print_swap(i, j));
        std::thread::sleep(std::time::Duration::from_millis(250));
        self.total_swap += 1;
    }
}

use std::io::{Stdin, Stdout, Write};

struct TermUI {
    stdin: Stdin,
    stdout: Stdout,
}

impl TermUI {
    fn new() -> TermUI {
        TermUI {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
        }
    }

    fn write(&mut self, str: String) {
        write!(
            self.stdout,
            "{}{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            str,
            termion::cursor::Hide
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }
}

use ansi_term::Color;

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

    fn print_swap(&self, i: usize, j: usize) -> String {
        let mut result = String::from("");
        for (idx, item) in self.arr.iter().enumerate() {
            let color = if idx == i || idx == j {
                Color::Yellow
            } else {
                Color::White
            };
            let s = format!("{} {}\r\n", idx, color.paint("▇".repeat(*item as usize)));
            result += &s;
        }
        result
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
    InsertionSort,
    SelectionSort,
    BubbleSort,
    GnomeSort,
}

impl SortAlgo {
    fn sort(&self, arr: &mut ArraySorter) {
        match self {
            SortAlgo::InsertionSort => SortAlgo::insertion_sort(arr),
            SortAlgo::SelectionSort => SortAlgo::selection_srot(arr),
            SortAlgo::BubbleSort => SortAlgo::bubble_sort(arr),
            SortAlgo::GnomeSort => SortAlgo::gnome_sort(arr),
        }
    }

    fn insertion_sort(arr: &mut ArraySorter) {
        for i in 1..arr.arr.size() {
            let mut j = i;
            while j > 0 && arr.arr.arr[j - 1] > arr.arr.arr[j] {
                arr.swap(j, j - 1);
                j -= 1;
            }
        }
    }

    fn selection_srot(arr: &mut ArraySorter) {
        let arr_size = arr.arr.size();
        for i in 0..arr_size {
            let mut save_j = i;
            for j in i + 1..arr_size {
                if arr.arr.arr[j] < arr.arr.arr[save_j] {
                    save_j = j
                }
            }
            if i != save_j {
                arr.swap(i, save_j)
            }
        }
    }

    fn bubble_sort(arr: &mut ArraySorter) {
        let arr_size = arr.arr.size();
        loop {
            let mut swapped = false;
            for i in 1..arr_size {
                if arr.arr.arr[i - 1] > arr.arr.arr[i] {
                    arr.swap(i, i - 1);
                    swapped = true;
                }
            }
            if !swapped {
                break;
            }
        }
    }

    fn gnome_sort(arr: &mut ArraySorter) {
        let arr_size = arr.arr.size();
        let mut i = 0;
        while i < arr_size {
            if i == 0 || arr.arr.arr[i] >= arr.arr.arr[i-1] {
                i += 1
            } else {
                arr.swap(i, i-1);
                i -= 1;
            }
        }
    }
}
