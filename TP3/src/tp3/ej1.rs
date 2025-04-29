/* 
    Estructura Persona
*/

//Atributos
pub struct Persona{
    pub nombre : String,
    pub edad : u32,
    pub direccion : String
}

//Metodos
impl Persona{
    pub fn new(nom_in : String , edad_in : u32 , dir_in : String)->Persona{
        return Persona{
            nombre : nom_in,
            edad : edad_in,
            direccion : dir_in
        };
    }
    pub fn to_string(&self)->String{
        return format!("{};{};{}", self.nombre, self.edad, self.direccion);
    }
    pub fn obtener_edad(&self)-> u32{
        return self.edad;
    }
    pub fn actualizar_direccion(&mut self , dir_nuevo : String){
        self.direccion = dir_nuevo;
    }
}