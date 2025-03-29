//Libreria
use std::io::stdin;

fn main(){
    //Variable res
    let res:f32 = 30.2 ; 

    //Lector
    let mut ent = String::new();
    //Lectura en consola
    stdin().read_line(&mut ent).expect("Error");
    
    //Conversion de la entrada
    let num:f32 = ent.trim().parse().expect("No es un flotante");
    
    //Operaciones
    println!("Suma = {}",(res + num));

    println!("Resta = {}",(res - num));

    println!("Multiplicacion = {}",(res * num));

    println!("Division = {}",(res / num));
    
}
