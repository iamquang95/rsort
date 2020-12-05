# rsort

This is a CLI sorting algorithms visualization. 

## Demo

[![rsort Demo](https://asciinema.org/a/377465.svg)](https://asciinema.org/a/377465)

*[(Click for demo video)](https://asciinema.org/a/377465)*


## Usage

```
Sorting algorithms visualization 

USAGE:
    rsort [OPTION]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --algo <algo>              Sorting algorithm [default: quick]  [possible values: insertion, selection, bubble,
                                   gnome, comb, merge, quick]
    -l, --length <array-length>    Length of generated array [default: 15]
    -d, --delay <delay>            Delay between two step in miliseconds [default: 250]
        --max <max-value>          Max value in array [default: 10]
        --min <min-value>          Min value in array [default: 1]
    -s, --seed <random-seed>       Random seed
```