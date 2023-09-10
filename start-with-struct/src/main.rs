struct Person{
    name: String,
    age:u8,
    gender:char,
}

struct Location(f32, f32);

#[derive(Debug)]
struct Marker;

fn main() {
    let p = Person{
        name: "Ebrahim".to_string(),
        age:35,
        gender: 'M',
    };
    println!("{}",p.name);

    let loc = Location(20.43,43.98);
    println!("{} {}",loc.0,loc.1);

    let m = Marker;
    println!("{:?}",m);
}
