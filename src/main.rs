fn main() {
    fizzbuzz_to(15);
}

fn is_divisible_by(x: u32, y: u32) -> bool {
    if y == 0 {
        return false;
    }

    return x % y == 0;
}

fn fizzbuzz(n: u32) {
    match (is_divisible_by(n, 3), is_divisible_by(n, 5)) {
        (true, true) => println!("fizzbuzz"),
        (true, false) => println!("fizz"),
        (false, true) => println!("buzz"),
        (false, false) => println!("{n}")
    }
}

fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}
