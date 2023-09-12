struct Point<T,U> {
    x:T,
    y:U,
}

impl<T:std::fmt::Debug,U:std::fmt::Debug> Point<T,U>{
    fn printing(&self){
        println!("x: {:?} y: {:?}",self.x,self.y);
    }
}


pub fn run(){
    let p =Point{
        x:12,
        y:56.63,
    };

    p.printing();

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