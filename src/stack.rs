

fn main(){
    stack_fn();
    heap_fn();
    update_str();
}
fn stack_fn(){
    let a =33;
    let b =23;
    let c = a+b;
    println!("stack function{} {} {}", a,b,c);
}
fn heap_fn(){
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let combo = format!("{} {}", s1,s2);
    println!("heap function{}",combo)
}

fn update_str(){
    let mut s : String = String::from("Initial String");
    println!("Before update:{}",s);
    println!("Before capacity:{}, length:{}, pointer:{:p}",s.capacity(),s.len(),s.as_ptr());
    s.push_str("and some add text");
    println!("after update:{}",s);
    println!("after capacity:{}, length:{}, pointer:{:p}",s.capacity(),s.len(),s.as_ptr());
}
