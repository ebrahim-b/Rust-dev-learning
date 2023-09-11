pub fn run() {
    let _x:i32 = 12;
    let c:char = 'ف';
    println!("{}",c as i32);

    let name:&str = "Ebrahim";
    println!("{}",name);

    let family:String = "Barati".to_string();
    let full_name = name.to_string() + " " +  &family;

    let full_name_2 = format!("{} {}",name,family);

    println!("{}",full_name);
    println!("{}",full_name_2);

    let mut arr:[i32;3] = [2,95,63];
    println!("{:?}",arr);

    let arr_2 = [5;5];
    println!("{:?}",arr_2);

    let tup = ("ali",1542,12.5,true);
    println!("{:?}",tup);

    for n in arr.iter_mut() {
        *n += 25;
        println!("{}",n);
    }
    println!("{:?}",arr);

    let mut v = Vec::new();
    for i in 0..=10 {
        v.push(i);
    }

    println!("{:?}",v);

    let mat = [[2,5,6,7],[3,5,9,8]];

    println!("{:?}",mat);

}