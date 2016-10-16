fn sum_of_multiples() -> i32 {
    let mut sum = 0;

    for x in 1..1000 {
        if x % 3 == 0 || x % 5 == 0 {
            sum += x;
        }
    }

    return sum;
}

fn main() {
    println!("{}", sum_of_multiples());
}
