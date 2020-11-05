pub struct Config {
    pub silent: bool,
    pub delay: usize,
}

impl Config {
    pub fn new(silent: bool, delay: usize) -> Config {
        Config {
            silent,
            delay,
        }
    }


}
