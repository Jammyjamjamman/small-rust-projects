fn factorial(val: u64) -> u64 {
    if val <= 1 {
        1
    }
    else {
        val * factorial(val -1)
    }
}

fn main() {
    println!("factorial 10 is {}", factorial(10));
    println!("factorial 13 is {}", factorial(13));
}