
/*
Code Here
 */
//Create a Rectangle Struct
struct Rectangle{
    width: f64,
    height: f64

}

//Rectangle functions
impl Rectangle {

    //return area of Rectangle object
    fn get_area(&self) -> f64 {
        self.width * self.height //no semi colon for implied return
    }

    //scales the dimensions by a scalar amount f64
    fn scale(&mut self, scalar: f64) {
        self.width *= scalar;
        self.height *= scalar;
    }

    //constructor 
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle {
            width,
            height
        }
    }
}


fn main() {
    let mut rect = Rectangle::new(1.2,3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
    println!("Tests passed!");
}
