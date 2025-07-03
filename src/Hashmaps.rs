use std::collections::HashMap;
fn main() {
    let pairs: Vec<(String, u32)> = vec![(String::from("dp"), 23), (String::from("harkiart"), 33)];

    let grouped_ans = group_values_by_pairs(pairs);

    println!("{:?}", grouped_ans);
    // for (k, v) in grouped_ans {
    //     println!("{} :{:?}", k, v);
    // }
}

fn group_values_by_pairs(vec: Vec<(String, u32)>) -> HashMap<String, u32> {
    let mut mapper = HashMap::new();
    for (k, v) in vec {
        mapper.insert(k, v);
    }
    mapper
}
