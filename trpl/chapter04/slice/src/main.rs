fn main() {
    let mut s = String::from("Hello World");
    let wordIndex = first_world(&s[..]);
    println!("{}", wordIndex);

    let s2 = first_world("Wooooo rust");
    println!("{}", s2);
}

// 如果传入参数类型为字符串引用，建议都换成切片
fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    &s[..]
}
