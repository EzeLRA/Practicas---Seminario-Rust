/* 
    Estructura Rectangulo
*/

//Atributos
#[derive(Debug)]
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
    //Metodos getters
    pub fn obtener_largo(&self)->f32{
        return self.longitud;
    }
    pub fn obtener_ancho(&self)->f32{
        return self.ancho;
    }
    //Metodo de comparacion
    pub fn es_igual_a(&self,r:&Rectangulo)->bool{
        return if(self.obtener_largo() == r.obtener_largo())&&(self.obtener_ancho() == r.obtener_ancho()){true}else{false}
    }
    //Metodos primarios
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
        assert_eq!(r.es_igual_a(&Rectangulo::new(25.0,2.0)), true );
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