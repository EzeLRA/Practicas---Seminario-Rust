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
            direc = "No identificado"
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

#[cfg(test)]
mod testing_persona{
    use super::Persona;

    #[test]
    fn creacion_persona(){
        let mut persona = Persona::new(String::from("Mario"),23,None );
        //Persona con direccion nula
        assert_eq!( persona , Persona::new(String::from("Mario"),23,None ) );
        
        //Persona sin direccion nula
        persona = Persona::new(String::from("Mario"),23,Some(String::from("Av.Entre Rios")));
        assert_eq!( persona , Persona::new(String::from("Mario"),23,Some(String::from("Av.Entre Rios"))) );
    }

    #[test]
    fn representacion_string(){
        let mut persona = Persona{
            nombre : String::from("Mario"),
            edad : 23 ,
            direccion : None
        };
        //Persona con direccion nula
        assert_eq!( persona.to_string() , String::from("Mario;23;No identificado") );
        
        //Persona sin direccion nula        
        persona.direccion = Some(String::from("Av.Entre Rios"));

        assert_eq!( persona.to_string() , String::from("Mario;23;Av.Entre Rios") );
    }

    #[test]
    fn retorno_edad(){
        let persona = Persona{
            nombre : String::from("Mario"),
            edad : 23 ,
            direccion : None
        };
        assert_eq!( persona.obtener_edad() , 23 );
    }

    #[test]
    fn modificacion_direccion(){
        let mut persona = Persona{
            nombre : String::from("Mario"),
            edad : 24 ,
            direccion : None
        };
        //Persona con direccion nula
        assert_eq!( persona.direccion , None );
        
        //Persona sin direccion nula
        persona.actualizar_direccion( Some(String::from("Av.Corrientes")) );
        assert_eq!( persona.direccion , Some(String::from("Av.Corrientes")) );
    }
}