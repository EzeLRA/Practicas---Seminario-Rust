use std::io::stdin;

fn main() {

    //Lector
    let mut ent = String::new();
    //Lectura en consola
    stdin().read_line(&mut ent).expect("Error");

    let op1:bool = ent.trim().parse().expect("No es un booleano");

    //Lector
    let mut ent = String::new();
    //Lectura en consola
    stdin().read_line(&mut ent).expect("Error");

    let op2:bool = ent.trim().parse().expect("No es un booleano");

    println!("AND = {}", (op1 && op2));
    println!("OR = {}", (op1 || op2));

}
