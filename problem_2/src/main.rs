const LIMIT: i32 = 4000000;

fn main() {
    println!("{}", fibonacci(1, 2, 0));
}

fn fibonacci(prev: i32, current: i32, sum: i32) -> i32 {
    if current >= LIMIT {
        return sum;
    }

    if (current) % 2 == 0 {
        return fibonacci(current, prev + current, sum + current);
    }

    fibonacci(current, prev + current, sum)
}
