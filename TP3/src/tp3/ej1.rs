/* 
    Estructura Persona
*/

//Corregir

//Atributos
pub struct Persona{
    nombre : String,
    edad : u32,
    direccion : String
}

//Metodos
pub impl Persona{
    pub fn new(nomIn : String , edadIn : u32 , dirIn : String)->Persona{
        return Persona{
            nombre : nomIn,
            edad : edadIn,
            direccion : dirIn
        };
    }
}