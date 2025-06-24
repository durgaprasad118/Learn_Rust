fn main(){
    let mut s1 = String::from("hello");
    let s2 =  &mut s1;
    s2.push_str(" World");
    println!("{}",s1);
}

