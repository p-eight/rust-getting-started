
fn fibonacci(n : i64) -> i64 {
    if 1 >= n{
        return n;
    }
    else
    {
        return fibonacci(n - 1) + fibonacci(n -2);
    }
}

fn main() {
    println!("printing fibonacci sequence:");
    for n in 0..20
    {
        println!("n : \t {} \t -> \t {}", n, fibonacci(n));
    }
}
