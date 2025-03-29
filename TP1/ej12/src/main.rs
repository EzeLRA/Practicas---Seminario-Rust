fn main() {
    let tup : (&str,[i32;5]) = ("Prueba",[1,2,3,4,5]);
    let mut total:i32 = 0;

    for num in tup.1.iter(){
        total += num;
    }

    println!("{}", tup.0);
    println!("{}",total);
}
