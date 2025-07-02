struct Rect {
    width: u32,
    height: u32,
}
// implementations
impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn peri(&self, num: i32) -> u32 {
        println("{}", num);
        2 * (self.width + self.height)
    }
}
fn main() {
    let rect = Rect {
        width: 10,
        height: 10,
    };
    println!("The area of the rectangle is : {}", rect.area());
    println!("The perimeter of the rectangle is : {}", rect.peri(2));
}
