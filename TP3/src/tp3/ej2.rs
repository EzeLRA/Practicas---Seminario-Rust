/* 
    Estructura Rectangulo
*/

//Atributos
#[derive(PartialEq,Debug)]
pub struct Rectangulo{
	pub longitud : f32,
	pub ancho : f32
}

/*
    Metodos
*/

impl Rectangulo{
	pub fn new(lon:f32 , anc:f32)->Rectangulo{
		return Rectangulo{
            longitud : lon,
            ancho : anc
        };
	}

	pub fn calcular_area(&self)->f32{
		return self.longitud*self.ancho;
	}

	pub fn calcular_perimetro(&self)->f32{
		return 2.0*(self.longitud+self.ancho);
	}

	pub fn es_cuadrado(&self)->bool{
		return self.longitud == self.ancho;
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