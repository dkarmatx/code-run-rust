use std::io::stdin;

fn main() {
    let mut line = String::default();
    stdin().read_line(&mut line).expect("successfull read");

    let mut nums = [0, 0, 0];
    line.split_ascii_whitespace()
        .take(3)
        .map(|x| x.parse::<i32>().unwrap())
        .enumerate()
        .for_each(|(i, v)| {
            nums[i] = v;
        });

    let [a, b, c] = nums;

    let m = match (a < b, b < c, a < c) {
        (true, true, _) => b,
        (false, false, _) => b,

        (true, false, false) => a,
        (false, true, true) => a,

        (true, false, true) => c,
        (false, true, false) => c,
    };

    println!("{m}")
}
