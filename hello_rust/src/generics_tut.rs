pub fn run(){
    let x = 10;
    println!("{}",square_int(x));

    let x = 10.4;
    println!("{}",square_float(x));

    let x = 50;
    println!("{}",retutn_generics(x));

    let x = 'a';
    println!("{}",retutn_generics(x));

    let x = 24;
    println!("{}",square_generics(x));

    let x = 24.9;
    println!("{}",square_generics(x));
}

fn square_int (x:i32) -> i32 {
    x * x
}

fn square_float(x:f32) -> f32 {
    x * x
}

fn retutn_generics<T>(x:T) -> T {
    x
}

fn square_generics<T: std::ops::Mul<Output=T> + Copy>(x:T) -> T {
    x * x
}