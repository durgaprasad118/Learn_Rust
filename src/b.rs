enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

fn pat_match_shapes(shape: Shape) -> String {
    use Shape::*;
    match shape {
        Circle(x) => format!("This is cicle and radius is : {}", x),
        Rectangle(a, b) => format!("Rectangle's lenght, and breadth, {} {}", a, b),
        Triangle(a, b, c) => format!("Triangle's sides are {} {} {}", a, b, c),
    }
}
fn calc_perimeters(shape: Shape) -> f64 {
    use Shape::*;
    match shape {
        Circle(x) => std::f64::consts::PI * x * x,
        Rectangle(a, b) => 2.0 * (a + b),
        Triangle(a, b, c) => a + b + c,
    }
}
fn main() {
    let x = Shape::Circle(2.3);
    let y = Shape::Rectangle(3.0, 4.0);
    println!("{}", pat_match_shapes(x));
    println!("{}", calc_perimeters(y));
}
