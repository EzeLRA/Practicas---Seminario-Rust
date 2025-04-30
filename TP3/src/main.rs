mod tp3;
use tp3::ej1::Persona;
use tp3::ej2::Rectangulo;

fn main() {
    //Ingresar codigo
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
        assert_eq!( persona.to_string() , String::from("Mario;23;no identificado") );
        
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

#[cfg(test)]
mod testing_rectangulo{
    use super::Rectangulo;

    #[test]
    fn creacion_rectangulo(){
        let r = Rectangulo::new(25.0,2.0);
        assert_eq!(r, Rectangulo::new(25.0,2.0) );
    }

    #[test]
    fn retorno_no_negativo(){
        let r = Rectangulo{
            longitud: 25.0,
            ancho: 2.0
        };
        assert_eq!( (r.calcular_area() > 0.0) , true );
        assert_eq!( (r.calcular_perimetro() > 0.0) , true );
    }

    #[test]
    fn calculo_correcto(){
        let r = Rectangulo{
            longitud: 25.0,
            ancho: 2.0
        };
        assert_eq!( (r.calcular_area() == 50.0) , true );
        assert_eq!( (r.calcular_perimetro() == 54.0) , true );
    }


    #[test]
    fn figura_cuadrada(){
        let r = Rectangulo{
            longitud: 2.0,
            ancho: 2.0
        };
        assert_eq!(r.es_cuadrado(), true);
    }

}