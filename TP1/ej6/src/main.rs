//Libreria
use std::io::stdin;

fn main() {
    let mut cad:String = String::new();
    
    //Lectura en consola
    stdin().read_line(&mut cad).expect("Error");

    let x:u32 = cad.trim().parse().expect("No es un flotante");
    
    println!("{}",u32::pow(x,2));
}
