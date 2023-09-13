pub fn run(){

    let a:Option<i32>;
    a = Some(-35);
    println!("{:?}",a);

    let b:Result<i32, String> = Ok(15);

    println!("{:?}",b)


}