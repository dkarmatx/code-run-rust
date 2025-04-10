use std::fmt::Display;

pub fn seq_print<T: Display>(seq: impl Iterator<Item = T>, sep: &str) {
    let mut sep_ = "";
    for v in seq {
        print!("{sep_}{v}");
        sep_ = sep;
    }
}

pub fn seq_println<T: Display>(seq: impl Iterator<Item = T>, sep: &str) {
    seq_print(seq, sep);
    println!();
}
