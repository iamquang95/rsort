use clap::{App, Arg};
use rsort::{app_config, data_generator, ArraySorter, SortAlgo};
use std::str::FromStr;

fn main() {
    let matcher = App::new("Sorting algorithms visualization")
        .usage(
            "rsort [OPTION]"
        )
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
                .default_value("250")
        )
        .arg(
            Arg::with_name("random-seed")
                .long("seed")
                .short("s")
                .help("Random seed")
                .takes_value(true)
                .validator(is_number)
        )
        .arg(
            Arg::with_name("array-length")
                .long("length")
                .short("l")
                .help("Length of generated array")
                .takes_value(true)
                .validator(is_number)
                .default_value("15")
        )
        .arg(
            Arg::with_name("min-value")
                .long("min")
                .help("Min value in array")
                .takes_value(true)
                .validator(is_number)
                .default_value("1"),
        )
        .arg(
            Arg::with_name("max-value")
                .long("max")
                .help("Max value in array")
                .takes_value(true)
                .validator(is_number)
                .default_value("10")
        )
        .get_matches();

    let algo = SortAlgo::from_str(
        matcher
            .value_of("algo")
            .unwrap_or(SortAlgo::QuickSort.get_algo_name()),
    )
    .expect("Failed to get algo");

    let delay =
        usize::from_str(matcher.value_of("delay").unwrap_or("250")).expect("Failed to get delay");

    let app_conf = app_config::Config::new(false, delay);

    let length = usize::from_str(matcher.value_of("array-length").unwrap_or("15")).expect("Failed to get length");
    let min_value = usize::from_str(matcher.value_of("min-value").unwrap_or("1")).expect("Failed to get min value");
    let max_value = usize::from_str(matcher.value_of("max-value").unwrap_or("10")).expect("Failed to get max value");
    let seed = matcher.value_of("max-value").map(|s| u64::from_str(s).expect("Failed to parse seed"));

    let data_generator_conf = data_generator::DataGeneratorConfig::new(length, (min_value, max_value), seed);

    let mut solver = ArraySorter::new(algo, app_conf, data_generator_conf);
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
