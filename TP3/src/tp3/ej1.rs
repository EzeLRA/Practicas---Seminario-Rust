/* 
    Estructura Persona
*/

//Atributos
#[derive(Debug,Clone)]
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
    //Metodos getter
    pub fn obtener_edad(&self)-> u32{
        return self.edad;
    }
    pub fn obtener_nombre(&self)->String{
        return self.nombre.clone();
    }
    pub fn obtener_direccion(&self)->String{
        return if let Some(dir) = &self.direccion{dir.to_string()}else{"No identificado".to_string()}
    }
    //Metodos ṕrimarios
    pub fn to_string(&self)->String{
        return format!("{};{};{}", self.obtener_nombre(), self.obtener_edad(), self.obtener_direccion());
    }
    pub fn actualizar_direccion(&mut self , dir_nuevo : Option<String>){
        self.direccion = dir_nuevo;
    }
    //Metodo auxiliar para comparacion
    pub fn igual_a(&self,per1:&Persona)->bool{
        let mut res = false;
        if (self.obtener_nombre() == per1.obtener_nombre()) && 
        (self.obtener_edad() == per1.obtener_edad()) && 
        (self.obtener_direccion() == per1.obtener_direccion()){
            res = true;
        }
        return res;
    }
}

#[cfg(test)]
mod testing_persona{
    use super::Persona;

    #[test]
    fn creacion_persona(){
        let mut persona = Persona::new(String::from("Mario"),23,None );
        //Persona con direccion nula
        assert_eq!( persona.igual_a( &Persona::new(String::from("Mario"),23,None )) , true );
        
        //Persona sin direccion nula
        persona = Persona::new(String::from("Mario"),23,Some(String::from("Av.Entre Rios")));
        assert_eq!( persona.igual_a( &Persona::new(String::from("Mario"),23,Some(String::from("Av.Entre Rios")))) , true );
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