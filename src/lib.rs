pub mod app_config;
mod tui;

use ansi_term::Color;
use std::error::Error;
use std::str::FromStr;

pub struct ArraySorter {
    arr: Vec<u32>,
    algo: SortAlgo,
    tui: tui::TermUI,
    total_swap: u32,
    highlight_range: Option<(usize, usize)>,
    config: app_config::Config,
}

impl ArraySorter {
    pub fn new(arr: Vec<u32>, algo: SortAlgo, config: app_config::Config) -> ArraySorter {
        ArraySorter {
            arr,
            algo,
            tui: tui::TermUI::new(),
            total_swap: 0,
            highlight_range: None,
            config,
        }
    }

    pub fn sort(&mut self) {
        let algo = self.algo.clone();

        if !self.config.silent {
            self.tui.write(self.print_arr());
            self.thread_sleep();
        }
        algo.sort(self);

        if !self.config.silent {
            self.tui.write(self.print_arr());
        }
    }

    fn swap(&mut self, i: usize, j: usize) {
        assert!(i < self.size() && j < self.size());
        if !self.config.silent {
            self.tui.write(self.print_swap(i, j));
            self.thread_sleep();
        }

        self.arr.swap(i, j);

        if !self.config.silent {
            self.tui.write(self.print_swap(i, j));
            self.thread_sleep();
        }
        self.total_swap += 1;
    }

    fn set(&mut self, i: usize, val: u32) {
        assert!(i < self.size());
        self.arr[i] = val;
        if !self.config.silent {
            self.tui.write(self.print_set(i));
            self.thread_sleep();
        }
        self.total_swap += 1;
    }

    fn size(&self) -> usize {
        self.arr.len()
    }

    fn thread_sleep(&self) {
        std::thread::sleep(std::time::Duration::from_millis(self.config.delay as u64));
    }

    fn print_swap(&self, i: usize, j: usize) -> String {
        let mut result = String::from("");
        for (idx, item) in self.arr.iter().enumerate() {
            let color = if idx == i || idx == j {
                Color::Yellow
            } else {
                Color::White
            };
            let range_prefix = self.get_range_prefix(idx);
            let s = format!(
                "{}{} {}\r\n",
                range_prefix,
                idx,
                color.paint("▇".repeat(*item as usize))
            );
            result += &s;
        }
        result
    }

    fn print_set(&self, i: usize) -> String {
        let mut result = String::from("");
        for (idx, item) in self.arr.iter().enumerate() {
            let color = if idx == i {
                Color::Yellow
            } else {
                Color::White
            };
            let range_prefix = self.get_range_prefix(idx);
            let s = format!(
                "{}{} {}\r\n",
                range_prefix,
                idx,
                color.paint("▇".repeat(*item as usize))
            );
            result += &s;
        }
        result
    }

    fn print_arr(&self) -> String {
        let mut result = String::from("");
        for (idx, item) in self.arr.iter().enumerate() {
            let color = Color::White;
            let range_prefix = " ";
            let s = format!(
                "{}{} {}\r\n",
                range_prefix,
                idx,
                color.paint("▇".repeat(*item as usize))
            );
            result += &s;
        }
        result
    }

    fn get_range_prefix(&self, idx: usize) -> String {
        String::from(match self.highlight_range {
            None => " ",
            Some((left, right)) => {
                if idx == left {
                    "┌"
                } else if idx == right {
                    "└"
                } else if left < idx && idx < right {
                    "│"
                } else {
                    " "
                }
            }
        })
    }

    fn set_highlight_range(&mut self, left: usize, right: usize) {
        self.highlight_range = Some((left, right));
    }

    fn clear_highlight_range(&mut self) {
        self.highlight_range = None;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseAlgoError {}

impl std::fmt::Display for ParseAlgoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "provided string is not a valid sorting algorithm. Use help command to see the list of supporting algos".fmt(f)
    }
}

impl Error for ParseAlgoError {}

#[derive(Clone, Debug)]
pub enum SortAlgo {
    InsertionSort,
    SelectionSort,
    BubbleSort,
    GnomeSort,
    CombSort,
    MergeSort,
    QuickSort,
}

impl FromStr for SortAlgo {
    type Err = ParseAlgoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower_str = s.to_lowercase();
        let all_algos = SortAlgo::all_algos();
        for algo in all_algos {
            if algo.get_algo_name() == lower_str {
                return Ok(algo);
            }
        }
        Err(ParseAlgoError {})
    }
}

impl SortAlgo {
    pub fn all_algos() -> Vec<SortAlgo> {
        vec![
            SortAlgo::InsertionSort,
            SortAlgo::SelectionSort,
            SortAlgo::BubbleSort,
            SortAlgo::GnomeSort,
            SortAlgo::CombSort,
            SortAlgo::MergeSort,
            SortAlgo::QuickSort,
        ]
    }

    pub fn all_algos_str() -> [&'static str; 7] {
        let mut res: [&str; 7] = [""; 7];
        let all_algos = SortAlgo::all_algos();
        for (place, element) in res.iter_mut().zip(all_algos.iter()) {
            *place = element.get_algo_name();
        }
        res
    }

    pub fn get_algo_name(&self) -> &'static str {
        match self {
            SortAlgo::InsertionSort => "insertion",
            SortAlgo::SelectionSort => "selection",
            SortAlgo::BubbleSort => "bubble",
            SortAlgo::GnomeSort => "gnome",
            SortAlgo::CombSort => "comb",
            SortAlgo::MergeSort => "merge",
            SortAlgo::QuickSort => "quick",
        }
    }

    fn sort(&self, arr: &mut ArraySorter) {
        match self {
            SortAlgo::InsertionSort => SortAlgo::insertion_sort(arr),
            SortAlgo::SelectionSort => SortAlgo::selection_srot(arr),
            SortAlgo::BubbleSort => SortAlgo::bubble_sort(arr),
            SortAlgo::GnomeSort => SortAlgo::gnome_sort(arr),
            SortAlgo::CombSort => SortAlgo::comb_sort(arr),
            SortAlgo::MergeSort => SortAlgo::merge_sort(arr),
            SortAlgo::QuickSort => SortAlgo::quick_sort(arr),
        }
    }

    fn insertion_sort(arr_sorter: &mut ArraySorter) {
        for i in 1..arr_sorter.size() {
            let mut j = i;
            arr_sorter.set_highlight_range(0, i);
            while j > 0 && arr_sorter.arr[j - 1] > arr_sorter.arr[j] {
                arr_sorter.swap(j, j - 1);
                j -= 1;
            }
            arr_sorter.clear_highlight_range();
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

    fn merge_sort(arr_sorter: &mut ArraySorter) {
        fn merge_sort(arr_sorter: &mut ArraySorter, left: usize, right: usize) {
            if left < right {
                let mid = left + (right - left) / 2;
                merge_sort(arr_sorter, left, mid);
                merge_sort(arr_sorter, mid + 1, right);
                merge(arr_sorter, left, mid, right);
            }
        };

        fn merge(arr_sorter: &mut ArraySorter, left: usize, mid: usize, right: usize) {
            let left_arr = arr_sorter.arr[left..=mid].to_vec();
            let mut left_iter = left_arr.iter().peekable();
            let right_arr = arr_sorter.arr[mid + 1..=right].to_vec();
            let mut right_iter = right_arr.iter().peekable();

            arr_sorter.set_highlight_range(left, right);
            for i in left..=right {
                match (left_iter.peek(), right_iter.peek()) {
                    (None, None) => break,
                    (None, Some(_)) => arr_sorter.set(i, *right_iter.next().unwrap()),
                    (Some(_), None) => arr_sorter.set(i, *left_iter.next().unwrap()),
                    (Some(left_val), Some(right_val)) => {
                        if **left_val < **right_val {
                            arr_sorter.set(i, *left_iter.next().unwrap())
                        } else {
                            arr_sorter.set(i, *right_iter.next().unwrap())
                        }
                    }
                }
            }
            arr_sorter.clear_highlight_range();
        };

        let arr_size = arr_sorter.size();
        merge_sort(arr_sorter, 0, arr_size - 1);
    }

    fn quick_sort(arr_sorter: &mut ArraySorter) {
        fn quick_sort(arr_sorter: &mut ArraySorter, left: usize, right: usize) {
            if left < right {
                let p = partition(arr_sorter, left, right);
                if p > 0 {
                    quick_sort(arr_sorter, left, p - 1);
                }
                quick_sort(arr_sorter, p + 1, right);
            }
        }

        fn partition(arr_sorter: &mut ArraySorter, left: usize, right: usize) -> usize {
            let pivot = arr_sorter.arr[right];
            let mut i = left;
            arr_sorter.set_highlight_range(left, right);
            for j in left..=right {
                if arr_sorter.arr[j] < pivot {
                    arr_sorter.swap(i, j);
                    i += 1;
                }
            }
            arr_sorter.swap(i, right);
            arr_sorter.clear_highlight_range();
            i
        }

        let arr_size = arr_sorter.size();
        quick_sort(arr_sorter, 0, arr_size - 1);
    }
}

#[cfg(test)]
mod tests {
    use crate::{ArraySorter, SortAlgo};

    #[macro_export]
    macro_rules! algo_test {
        ($algo_name:ident, $algo:expr) => {
            #[test]
            fn $algo_name() {
                let v: Vec<u32> = vec![5, 8, 2, 7, 1, 4, 3, 6];
                let algo = $algo;
                let mut solver =
                    ArraySorter::new(v.clone(), algo, crate::app_config::Config::new(true, 0));
                solver.sort();
                assert!(is_sorted(&solver));
            }
        };
    }

    fn is_sorted(arr_sorter: &ArraySorter) -> bool {
        for i in 1..arr_sorter.size() {
            if arr_sorter.arr[i] < arr_sorter.arr[i - 1] {
                return false;
            }
        }
        true
    }

    algo_test!(insertion_sort_should_work, SortAlgo::InsertionSort);
    algo_test!(selection_sort_should_work, SortAlgo::SelectionSort);
    algo_test!(bubble_sort_should_work, SortAlgo::BubbleSort);
    algo_test!(gnome_sort_should_work, SortAlgo::GnomeSort);
    algo_test!(comb_sort_should_work, SortAlgo::CombSort);
    algo_test!(merge_sort_should_work, SortAlgo::MergeSort);
    algo_test!(quick_sort_should_work, SortAlgo::QuickSort);
}
