use std::collections::HashMap;


fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    let e = scores.entry(String::from("Yello"));
    e.or_insert(0);

    for (k, v) in scores {
        println!("{}, {}", k, v);
    }
}
