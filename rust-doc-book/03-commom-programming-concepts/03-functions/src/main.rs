fn main() {
    println!("Hello, world!");

    another_function();
    
    println!("function_returning_multiples: {}", function_returning_multiples(3, 5));
    function_explaning();

}

fn another_function() {
    println!("Another function.");
    println!("Need to remember that snake case is the pattern, and if not followed we'll have some warnings.");
}

fn function_returning_multiples (x : i32, y : i32) -> i32 {
    x * y
}

fn function_explaning()
{
    println!("For a function return something, it must end with an expression and not an statement.");
    println!("This text should be returned but until now, and after read only one stack overflow question, I didn't found an easy way to do it.");
}