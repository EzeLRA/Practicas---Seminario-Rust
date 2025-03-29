//Libreria
use std::io::stdin;

fn main() {
    let arr : [&str;5] = ["Banana","Chocolate","Ventilador","Teclado","Televisor"];

    //Lector
    let mut ent = String::new();
    //Lectura en consola
    stdin().read_line(&mut ent).expect("Error");

    let cad:&str = ent.trim(); 

    let mut encontre:bool = false;
    let mut i = 0;
    while (encontre==false)&&(i < 5){
        if arr[i] == cad {
            encontre = true;
        }
        i+=1;
    }


    println!("{}",encontre);
}
