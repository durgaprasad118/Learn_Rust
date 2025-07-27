trait Shape {
    fn to_str(&self) -> String;
}
struct Circle {
    radius: f64,
}
impl Circle {
    pub fn new(radius: f64) -> Circle {
        Circle { radius }
    }
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    pub fn scale(&self, factor: f64) {
        println!("{}", self.radius * factor);
    }
}
impl Shape for Circle {
    fn to_str(&self) -> String {
        format!("Circle is radius of {}", self.radius)
    }
}
fn main() {
    let a = Circle::new(5.0);
    // let b = a.area();
    // println!("{}", b);
    a.scale(2.0);
    let b = a.to_str();
    println!("{}", b);
}
