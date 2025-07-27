enum Traffic {
    Red,
    Yellow,
    Green,
}

fn main() {
    println!("{}", set_timer(Traffic::Yellow));
}

fn set_timer(light: Traffic) -> u32 {
    use Traffic::*;
    match light {
        Red => 30,
        Yellow => 5,
        Green => 45,
    }
}
fn get_light(light: Traffic) -> String {
    use Traffic::*;
    match light {
        Red => String::from("Red bro"),
        Yellow => String::from("yellow bro"),
        Green => String::from("Green bro"),
    }
}
