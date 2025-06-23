
fn main(){

    // let condition = 2 <=2.2; we cant do cz differnt types to fix this
     let condition = (2 as f32) <=2.2;
    println!("The condition is: {}",condition);

    // && and 
    // || or 
    // ! not negation operator
    // order => braces ! && ||
    //

    let food = "cookie";

    if food == "cookie"{
        println!("I like cookies too!");
    }else{
        println!("I dont like cookies too!");
    }

}
