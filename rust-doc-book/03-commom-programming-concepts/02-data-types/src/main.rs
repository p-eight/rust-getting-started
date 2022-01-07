fn main() {
    
    // float numbers

    let f_x = 2.0;

    println!("f_x is a double precision float (64 bits - default) - value {}", f_x);
    
    let f_y : f32 = 3.0;

    println!("f_y is a single precision float (32 bits - use \':f32\') - value {}", f_y);
    // addition
    let sum = 5 + 10;
    println!("The value of sum is: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("The value of product is: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {}", quotient);

    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is: {}", remainder);

    let c = 'z';
    println!("The value of c is: {}", c);

    let z = 'ℤ';
    println!("The value of z is: {}", z);

    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    println!("We can also use tuple as the following \'tup.0\': {}", tup.0);

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);

}
