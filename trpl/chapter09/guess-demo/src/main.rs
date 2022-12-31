pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        // 这里键值对同名，所以简写
        Guess {value}
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let g = Guess::new(101);
    println!("guess value: {}", g.value());    
}
