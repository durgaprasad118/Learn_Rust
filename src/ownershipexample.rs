
fn main(){
    change_val();
    println!("{}",2);
}
fn change_val(){
    let s1= String::from("hello");
    let s2 = s1;
    println!("s2 = {s2}");
}
