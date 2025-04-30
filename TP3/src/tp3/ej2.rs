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