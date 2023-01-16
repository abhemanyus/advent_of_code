#[macro_export]
macro_rules! load_file {
    ($file:literal) => {
        std::fs::read_to_string(concat!("data/", $file, ".txt")).unwrap()
    };
}
