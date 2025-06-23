
fn main(){
    test();
    sum(2,3);

    //this is an expression
    let number = {
        let x =3;
        x+1 //explictily not putting  any ; in order to represnt it's returning a value
    };
    println!("{}",number)

}

fn test(){
    println!("test fucntion is called!!!");
}

fn sum(a:i32,b:i32)->i32{
    println!("The sum of {a} and  {b} is {}",a+b);
    let answer = a+b;
    return answer;
}
