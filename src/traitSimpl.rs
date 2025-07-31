struct Boat {
    speed1: f64,
}
struct Car {
    speed2: f64,
}
trait F {
    fn display_speed(&self) -> bool;
}
impl F for Boat {
    fn display_speed(&self) -> bool {
        if self.speed1 > 4.0 {
            return true;
        }
        false
    }
}

impl F for Car {
    fn display_speed(&self) -> bool {
        if self.speed2 > 12.0 {
            return true;
        }
        false
    }
}
fn main() {
    let a = Car { speed2: 3.0 };
    let b = Boat { speed1: 5.0 };
    println!("{}", a.display_speed());
    println!("{}", b.display_speed());
}
