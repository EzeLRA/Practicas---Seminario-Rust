//Libreria
use std::io::stdin;

const CAD:&str = "Palabra";

fn main() {
    //Lector
    let mut ent = String::new();
    //Lectura en consola
    stdin().read_line(&mut ent).expect("Error");

    let _caracter:char = ent.trim().parse().expect("No es un caracter");

    let mut cant = 0;

    for caracteres in CAD.chars(){
        if caracteres == _caracter {
            cant  += 1;
        }
    }

    println!("{}", cant);
}
