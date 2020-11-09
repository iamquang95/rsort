use clap::{App, Arg};
use rsort::{app_config, ArraySorter, SortAlgo};
use std::str::FromStr;

fn main() {
    let matcher = App::new("Sorting algorithms visualization")
        .arg(
            Arg::with_name("algo")
                .long("algo")
                .short("a")
                .help("Sorting algorithm")
                .takes_value(true)
                .possible_values(&SortAlgo::all_algos_str())
                .validator(is_sort_algo)
                .default_value("quick"),
        )
        .arg(
            Arg::with_name("delay")
                .long("delay")
                .short("d")
                .help("Delay between two step in miliseconds")
                .takes_value(true)
                .validator(is_number)
                .default_value("250"),
        )
        .get_matches();

    let v: Vec<u32> = vec![5, 8, 2, 7, 1, 4, 3, 6];
    // let v: Vec<u32> = vec![2, 1];
    let algo = SortAlgo::from_str(
        matcher
            .value_of("algo")
            .unwrap_or(SortAlgo::QuickSort.get_algo_name()),
    )
    .expect("Failed to get algo");

    let delay =
        usize::from_str(matcher.value_of("delay").unwrap_or("250")).expect("Failed to get delay");

    let mut solver = ArraySorter::new(v.clone(), algo, app_config::Config::new(false, delay));
    solver.sort()
}

fn is_sort_algo(s: String) -> Result<(), String> {
    let algo = SortAlgo::from_str(&s);
    match algo {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("{}", err)),
    }
}

fn is_number(s: String) -> Result<(), String> {
    if usize::from_str(&s).is_ok() {
        Ok(())
    } else {
        Err(format!("{} is not a number", &s))
    }
}
