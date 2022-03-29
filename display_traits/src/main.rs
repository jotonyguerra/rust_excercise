use std::fmt;

//given struct
struct Satellite {
    name: String,
    velocity: f64 //miles per second
}

//Your code here

//This implementation is a modified copy from the rust book on 
//fmt::Display
impl fmt::Display for Satellite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} flying at {} miles per second", self.name, self.velocity)
    }
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };
    println!("Hubble is {}", hubble);
}


//Bonus
/*
Implement the PartialOrd trait to compare satellites

Define new Alititude Trait.


*/