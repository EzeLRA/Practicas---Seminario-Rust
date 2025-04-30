/* 
    Estructura Persona
*/

//Atributos
#[derive(PartialEq, Debug)]
pub struct Persona{
    pub nombre : String,
    pub edad : u32,
    pub direccion : Option<String>  //La direccion puede ser nula, por lo que se usa option para manejarlo
}

/*
    Metodos
*/
impl Persona{
    pub fn new(nom_in : String , edad_in : u32 , dir_in : Option<String>)->Persona{
        return Persona{
            nombre : nom_in,
            edad : edad_in,
            direccion : dir_in
        };
    }
    pub fn to_string(&self)->String{
        let direc: &str;
        if let Some(dir) = &self.direccion {
            direc = dir;
        } else {
            direc = "no identificado"
        }
        return format!("{};{};{}", self.nombre, self.edad, direc);
    }
    pub fn obtener_edad(&self)-> u32{
        return self.edad;
    }
    pub fn actualizar_direccion(&mut self , dir_nuevo : Option<String>){
        self.direccion = dir_nuevo;
    }
}