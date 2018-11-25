fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x + 6;
    println!("The value of x is: {}", x);


    let _: u32 = "42".parse().expect("Not a number!");


    let a = 0;
//    let _: u32 = a - 1;
    let _b = 57u8;

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
  }