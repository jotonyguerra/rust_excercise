//challenge activity from Linkedin Learn
fn main() {
    println!("Hello, world!");
    //compute average of three numbers
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    // Your code goes here
    let sum: f64 = a as f64 + b + c as f64;
    let average = sum / 3.0;
    println!("Average = {:.1}", average);

    assert_eq!(average, 45.1);
    println!("Test Passed!");
}
