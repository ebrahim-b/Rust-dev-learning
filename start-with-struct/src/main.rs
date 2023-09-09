struct Person{
    name: String,
    age:u8,
    gender:char,
}

fn main() {
    let p = Person{
        name: "Ebrahim".to_string(),
        age:35,
        gender: 'M',
    };
    println!("{}",p.name);
}
