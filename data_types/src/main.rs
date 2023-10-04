mod array_panic;

use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let x = 2.0;
    let y: f32 = 3.0;
    let z = x + y;
    // print both x and y, and their type
    println!("x + y = {}, and type is {}", z, type_of(z));
    number_play();
    boolean_fun();
    character_fun();
    tuple_fun();
    array_panic::array_panic();
}

fn number_play() {
    // addition
    let sum = 5 + 10;
    // print sum and it's variable type
    println!("sum = {}, and type is {}", sum, type_of(sum));

    // subtraction
    let difference = 95.5 - 4.3;
    // print difference and it's variable type
    println!(
        "difference = {}, and type is {}",
        difference,
        type_of(difference)
    );

    // multiplication
    let product = 4 * 30;
    // print product and it's variable type
    println!("product = {}, and type is {}", product, type_of(product));

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
                            // print quotient, truncated and their's variable types
    println!("quotient = {}, and type is {}", quotient, type_of(quotient));
    println!(
        "truncated = {}, and type is {}",
        truncated,
        type_of(truncated)
    );

    // remainder
    let remainder = 43 % 5;
    // print remainder and it's variable type
    println!(
        "remainder = {}, and type is {}",
        remainder,
        type_of(remainder)
    );
}

fn boolean_fun() {
    let t = true;
    let f: bool = false; // with explicit type annotation

    // print both t and f
    println!("t = {}, and type is {}", t, type_of(t));
    println!("f = {}, and type is {}", f, type_of(f));
}

fn character_fun() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // print all the variables above
    println!("c = {}, and type is {}", c, type_of(c));
    println!("z = {}, and type is {}", z, type_of(z));
    println!(
        "heart_eyed_cat = {}, and type is {}",
        heart_eyed_cat,
        type_of(heart_eyed_cat)
    );
}

fn tuple_fun() {
    let tup = (500, 6.4, 1);

    println!("tup = {:?}, and type is {}", tup, type_of(tup));

    let (x, _y, z) = tup;

    println!("The value of y is: {}", tup.1);
    println!("The value of x is: {x}");
    println!("The value of z is: {z}");
}
