mod tp3;
use tp3::ej1::Persona;

fn main() {
    let mut persona = Persona::new(String::from("sdfg"),23,String::from("trhtr"));
    println!("{:?}",persona.to_string());
    println!("{}",persona.obtener_edad());
    persona.actualizar_direccion(String::from("asdfg"));
    println!("{:?}",persona.to_string());
}