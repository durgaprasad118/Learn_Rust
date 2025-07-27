fn get_first_element(list: &[i32]) -> Option<&i32> {
    if list.is_empty() {
        return None;
    }
    Some(&list[0])
}

// let list = [10, 20, 30];
// let ans = get_first_element(&list);
// match ans {
//     Some(x) => println!("{}", x),
//     None => println!("No ele"),
// }

fn get_env(name: &str) -> Option<String> {
    match name {
        "DEBUG" => Some("true".to_string()),
        _ => None,
    }
}

// let x = get_env("TIMEOUT");
// let orcase = x.unwrap_or(String::from("hey no val"));
// println!("{}", orcase);
//
// let discount_percentage_some: Option<f64> = Some(15.0);
// if let Some(x) = discount_percentage_some {
//     println!("Apply this much percentage {}", x)
// }
// let discount_percentage_none: Option<f64> = None;
// if let Some(x) = discount_percentage_none {
//     println!("Apply this much percentage {}", x)
// } else {
//     println!("No disc")
// }

fn main() {
    let user_input: Option<i32> = Some(10);
    // if let Some(x) = user_input {
    //     println!("The trimed string is : {}", x.trim().to_string().len());
    // } else {
    //     println!("This is else case");
    // }
    let doubled: Option<i32> = user_input.map(|x| x * 2);
    println!("{:?}", doubled);
}
