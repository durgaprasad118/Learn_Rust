
enum Shape{
    Circle(f64),
    Square(f64),
    Rectangle(f64,f64)
}

//Pattern Matching
fn calc_area(shape:Shape)->f64{
    match shape{
        Shape::Circle(radius)=> 3.14*radius*radius,
        Shape::Square(side) => side *side,
        Shape::Rectangle(l,b) => l*b
    }
}


fn main(){
    let circle = Shape::Circle(3.0);
    let area = calc_area(circle);
    println!("{}",area);
}
