fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);

    let _condition = true;

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() - 1 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }


    let _ = a.iter();
    let a = (1..4).rev();

    let _ = a.to_owned();


    let mut s2 = String::from("delight");
    s2.push_str("safe");

    let mut s2 = {
        let s1 = String::from("hello");
        s1
    };
    s2.push_str("3");

    let s1 = String::from("hello");
    let _s2 = s1;

//    println!("{}, world!", s1);


    println!("The value of number is: {}", number);


    let mut s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
    // ... and so is no longer valid here

    s.push_str("can't");

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
    // but i32 is Copy, so it’s okay to still
    // use x afterward
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.