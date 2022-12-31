fn main() {
    let mut s = String::from("Hello, ");
    s.push_str("World!");
    // 借用了已经被移动的内容
    // let s2 = s;
    // println!("{}", s)
    let s2 = s.clone();
    println!("{}", s)
}
