fn main() {
    let mut orange = Orange {
        name: "orange".to_string(),
    };
    let mut apple = Apple {
        name: "apple".to_string(),
    };
}

struct Orange {
    name: String,
}

struct Apple {
    name: String,
}

fn print(orange: &Orange, apple: &Apple) {
    println!("orange: {}", orange.name);
    println!("apple: {}", apple.name);
}
