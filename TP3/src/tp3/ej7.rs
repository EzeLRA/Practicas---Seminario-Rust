/*
	Estructuras: Concesionario y Autos
*/
//Enum
#[derive(PartialEq, Debug , Clone)]
pub enum Colores{	
	//Primarios
	Rojo,
	Azul,
	Amarillo,
	//Secundarios
	Verde,
	Blanco,
	Negro
}
//Funcionalidad del enum
impl Colores{
	//Determina si es primario o secundario
	pub fn es_primario(&self)->bool{
		matches!(self, Colores::Rojo | Colores::Azul | Colores::Amarillo)
	}
}

//Atributos
#[derive(PartialEq, Debug , Clone)]
pub struct Auto{
    marca : String,
    modelo : String,
    anio : u32,
    precio_bruto : f32,
    color : Colores
}

#[derive(PartialEq, Debug)]
pub struct ConcesionarioAuto{
	nombre : String,
	direccion : String,
	capacidad : u32,
	autos : Vec<Auto>
}

//Metodos
impl Auto{
	
	pub fn new(nom:String,model:String,anio_in:u32,precio:f32,color_in:Colores)->Auto{
		return Auto{
			marca : nom,
			modelo : model,
			anio : anio_in,
			precio_bruto : precio,
			color : color_in
		}
	}

	pub fn calcular_precio(&self)->f32{
		let mut recargo : f32 = 0.0;
		let mut descuento : f32 = 0.0;

		if self.color.es_primario() {
			recargo += (self.precio_bruto * 25.0)/100.0; 
		}else{
			descuento += (self.precio_bruto * 10.0)/100.0;
		}

		if self.marca == "BMW" {
			recargo += (self.precio_bruto * 15.0)/100.0
		}

		if self.anio < 2000 {
			descuento += (self.precio_bruto * 5.0)/100.0;
		}

		return self.precio_bruto + recargo - descuento;
	}

}

impl ConcesionarioAuto{
	pub fn new(nom:String,dir:String,cant:u32)->ConcesionarioAuto{
		return ConcesionarioAuto{
			nombre : nom,
			direccion : dir,
			capacidad : cant,
			autos:Vec::new()
		}
	}
	//Preserva OwnerShip
	pub fn agregar_auto(&mut self,auto:&Auto)->bool{
		if (self.autos.len() as u32) < self.capacidad {
			self.autos.push(auto.clone());
			return true;
		}else{
			return false;
		}
	}
	pub fn eliminar_auto(&mut self,auto:&Auto){
		if !self.autos.is_empty() {
			for i in 1..self.autos.len(){
				if self.autos.get(i) == Some(auto){
					self.autos.remove(i);
					break;
				}
			}
		}
	}
	pub fn buscar_auto(&self,auto:&Auto)->Option<Auto>{
		let mut res : Option<Auto> = None;
		if !self.autos.is_empty() {
			for i in 1..self.autos.len(){
				if self.autos.get(i) == Some(auto){
					res = self.autos.get(i).cloned();
					break;
				}
			}
		}
		return res;
	}
}

#[cfg(test)]
mod testing_consencionaria_auto{
	use crate::tp3::ej7::{Colores,Auto,ConcesionarioAuto};

	//Auto
	#[test]
	fn creacion_auto(){
		let a = Auto::new(String::from("asdf"),String::from("aytuiy"),2023,100432.0,Colores::Rojo);
		assert_eq!(a,Auto::new(String::from("asdf"),String::from("aytuiy"),2023,100432.0,Colores::Rojo));
	}

	#[test]
	fn calculo_precio_auto(){
		//Identificar colores
		let a = Auto::new(String::from("asd"),String::from("aytuiy"),2023,100000.0,Colores::Rojo);
		assert_eq!(a.calcular_precio(),125000.0);
		let a = Auto::new(String::from("asd"),String::from("aytuiy"),2023,100000.0,Colores::Verde);
		assert_eq!(a.calcular_precio(),90000.0);
		//Identificar marca
		let a = Auto::new(String::from("BMW"),String::from("aytuiy"),2023,100000.0,Colores::Verde);
		assert_eq!(a.calcular_precio(),105000.0);
		//Identificar antiguedad
		let a = Auto::new(String::from("asd"),String::from("aytuiy"),2000,100000.0,Colores::Rojo);
		assert_eq!(a.calcular_precio(),120000.0);
	}
}