use ansi_term::Color;

pub struct ArraySorter {
    arr: Vec<u32>,
    algo: SortAlgo,
    tui: TermUI,
    total_swap: u32,
}

impl ArraySorter {
    pub fn new(arr: Vec<u32>, algo: SortAlgo) -> ArraySorter {
        ArraySorter {
            arr,
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
        assert!(i < self.size() && j < self.size());

        self.tui.write(self.print_swap(i, j));
        std::thread::sleep(std::time::Duration::from_millis(250));

        self.arr.swap(i, j);

        self.tui.write(self.print_swap(i, j));
        std::thread::sleep(std::time::Duration::from_millis(250));
        self.total_swap += 1;
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
}

use std::io::{Stdout, Write};

struct TermUI {
    stdout: Stdout,
}

impl TermUI {
    fn new() -> TermUI {
        TermUI {
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

#[derive(Clone)]
pub enum SortAlgo {
    InsertionSort,
    SelectionSort,
    BubbleSort,
    GnomeSort,
    CombSort,
}

impl SortAlgo {
    fn sort(&self, arr: &mut ArraySorter) {
        match self {
            SortAlgo::InsertionSort => SortAlgo::insertion_sort(arr),
            SortAlgo::SelectionSort => SortAlgo::selection_srot(arr),
            SortAlgo::BubbleSort => SortAlgo::bubble_sort(arr),
            SortAlgo::GnomeSort => SortAlgo::gnome_sort(arr),
            SortAlgo::CombSort => SortAlgo::comb_sort(arr),
        }
    }

    fn insertion_sort(arr_sorter: &mut ArraySorter) {
        for i in 1..arr_sorter.size() {
            let mut j = i;
            while j > 0 && arr_sorter.arr[j - 1] > arr_sorter.arr[j] {
                arr_sorter.swap(j, j - 1);
                j -= 1;
            }
        }
    }

    fn selection_srot(arr_sorter: &mut ArraySorter) {
        let arr_size = arr_sorter.size();
        for i in 0..arr_size {
            let mut save_j = i;
            for j in i + 1..arr_size {
                if arr_sorter.arr[j] < arr_sorter.arr[save_j] {
                    save_j = j
                }
            }
            if i != save_j {
                arr_sorter.swap(i, save_j)
            }
        }
    }

    fn bubble_sort(arr_sorter: &mut ArraySorter) {
        let arr_size = arr_sorter.size();
        loop {
            let mut sorted = true;
            for i in 1..arr_size {
                if arr_sorter.arr[i - 1] > arr_sorter.arr[i] {
                    arr_sorter.swap(i, i - 1);
                    sorted = false;
                }
            }
            if sorted {
                break;
            }
        }
    }

    fn gnome_sort(arr_sorter: &mut ArraySorter) {
        let arr_size = arr_sorter.size();
        let mut i = 0;
        while i < arr_size {
            if i == 0 || arr_sorter.arr[i] >= arr_sorter.arr[i - 1] {
                i += 1
            } else {
                arr_sorter.swap(i, i - 1);
                i -= 1;
            }
        }
    }

    fn comb_sort(arr_sorter: &mut ArraySorter) {
        let arr_size = arr_sorter.size();
        let mut gap = arr_size;
        let mut sorted = false;
        let shrink = 1.3;

        while !sorted {
            gap = ((gap as f64) / shrink) as usize;
            if gap <= 1 {
                gap = 1;
                sorted = true
            }
            let mut i = 0;
            while i + gap < arr_size {
                if arr_sorter.arr[i] > arr_sorter.arr[i + gap] {
                    arr_sorter.swap(i, i + gap);
                    sorted = false;
                }
                i += 1
            }
        }
    }
}
