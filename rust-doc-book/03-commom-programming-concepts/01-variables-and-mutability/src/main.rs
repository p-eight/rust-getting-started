const HOURS_IN_SECONDS: u32 = 60 * 60;

fn main() {
    let mut x = 5;
    println!("The value of mut x is: {}", x);
    x = 6;
    println!("The value of mut x is: {}", x);

    println!("Const HOURS_IN_SECONDS is : {}", HOURS_IN_SECONDS);
    println!("Three * HOURS_IN_SECONDS is : {}", 3 * HOURS_IN_SECONDS);

    let v = 5;
    println!("The value of v is: {}", v);

    let v = v + 1;
    println!("The shadowed value of v is: {}", v);

    {
        let v = v * 2;
        println!("The value of v in the inner scope is: {}", v);
    }

    println!("The shadowed value of v is: {}", v);

    let string = "string";
    println!("The string is: {}", string);

    let string = string.len();
    println!("Now, using the same variable, the string len is: {}", string);
}