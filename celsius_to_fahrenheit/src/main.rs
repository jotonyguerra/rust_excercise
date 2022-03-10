fn main() {
    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);
    println!("F = {}", fahrenheit_temp);
    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test Passed!");
}

//
fn celsius_to_fahrenheit(c: f64) -> f64 {
    println!("Converting Celsius {}", c);
    let val = ((1.8 * c) + 32.0).into();
    return val;
}