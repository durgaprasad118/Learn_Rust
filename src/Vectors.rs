fn main() {
    let mut vec = Vec::new();
    let numbers = vec![1, 2, 43];
    for number in numbers {
        println!("{}", number)
    }
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    let mut x = 0;
    while x < vec.len() {
        println!("{}", vec[x]);
        x += 1;
    }
    even_filter(&mut vec);
    println!("{:?}", vec);
    // let arr = even_value_vector(&vec);
    // println!("{:?}", arr);
}
// create a new even_value_vector.
// iterate through original array of items
// push items to it
// return it

fn even_value_vector(vec: Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::new();
    for val in vec {
        if val % 2 == 0 {
            ans.push(val);
        }
    }
    ans
}
//approach 2
//send mutable refresnce
//remvoe the odd thingsy

fn even_filter(vec: &mut Vec<i32>) {
    let mut i = 0;
    while i < vec.len() {
        if vec[i] % 2 != 0 {
            vec.remove(i);
        } else {
            i += 1;
        }
    }
}
