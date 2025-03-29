//Libreria
use std::io::stdin;

fn main() {
    let mut cad:String = String::new();
    
    //Lectura en consola
    stdin().read_line(&mut cad).expect("Error");

    let mut nuevo_str:String = cad.trim().to_string();

    cad = String::new();
    stdin().read_line(&mut cad).expect("Error");

    nuevo_str += &cad;

    println!("{}",(nuevo_str.to_uppercase()));
}
