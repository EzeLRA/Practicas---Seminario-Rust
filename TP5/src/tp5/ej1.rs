use std::fmt::{write, Display};
use std::{fs::File,io::{Error,Read,Write}};
use std::path::Path;
//Se debe importar serde para su uso "cargo add serde"
use serde::{Serialize, Deserialize};
use serde_json;

/**
 * 		Extraccion de ejercicio 7 de TP3
**/ 


/*
	Estructuras: Concesionario y Autos
*/

//Enum
#[derive(Debug , Clone, serde::Serialize)]
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
	pub fn es_igual_a(&self, c: &Colores) -> bool {
        match (self, c) {
            (Colores::Rojo, Colores::Rojo) => true,
            (Colores::Azul, Colores::Azul) => true,
            (Colores::Verde, Colores::Verde) => true,
			(Colores::Amarillo, Colores::Amarillo) => true,
			(Colores::Blanco, Colores::Blanco) => true,
			(Colores::Negro, Colores::Negro) => true,
            _ => false
        }
    }
}

//Atributos
#[derive(Debug , Clone , serde::Serialize)]
pub struct Auto{
    marca : String,
    modelo : String,
    anio : u32,
    precio_bruto : f32,
    color : Colores
}

#[derive(Debug,Clone,serde::Serialize)]
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

	//Metodo secundario
	pub fn es_igual_a(&self,a:&Auto)->bool{
		return (self.marca == a.marca)&&(self.modelo == a.modelo)&&(self.anio == a.anio)&&(self.precio_bruto == a.precio_bruto)&&(self.color.es_igual_a(&a.color));
	}

}

impl ConcesionarioAuto{
	//Metodos secundarios
	pub fn es_igual_a(&self,c:&ConcesionarioAuto)->bool{
		return (self.nombre == c.nombre)&&(self.direccion == c.direccion)&&(self.capacidad == c.capacidad);
	}
	//Metodos primarios
	pub fn new(nom:String,dir:String,cant:u32)->ConcesionarioAuto{
		return ConcesionarioAuto{
			nombre : nom,
			direccion : dir,
			capacidad : cant,
			autos:Vec::new()
		}
	}
	//Preserva OwnerShip y agrega repetidos
	pub fn agregar_auto(&mut self,auto:&Auto)->bool{
		if (self.autos.len() as u32) < self.capacidad {
			self.autos.push(auto.clone());
			return true;
		}else{
			return false;
		}
	}
	//Elimina la primer ocurrencia para un auto con las caracteristicas exactas
	pub fn eliminar_auto(&mut self,a1:&Auto)->bool{
		let mut pude = false;
		if !self.autos.is_empty() {
			for i in 0..self.autos.len(){
				if let Some(auto) = self.autos.get(i){
					if auto.es_igual_a(&a1) {
						self.autos.remove(i);
						pude = true;
						break;
					}
				}
			}
		}
		return pude;
	}
	//Se considera que la estructura no tendra un gran impacto en la performance(segun tamanio)
	//Busca un auto con las caracteristicas exactas
	pub fn buscar_auto(&self,a1:&Auto)->Option<Auto>{
		let mut res : Option<Auto> = None;
		if !self.autos.is_empty() {
			for auto in self.autos.clone(){
				if auto.es_igual_a(&a1) {
					res = Some(auto);
					break;
				}
			}
		}
		return res;
	}
}

#[cfg(test)]
mod testing_consecionaria_auto{
	use super::*;

	/*
		Auto
	*/

	#[test]
	fn creacion_auto(){
		let a = Auto::new(String::from("asdf"),String::from("aytuiy"),2023,100432.0,Colores::Rojo);
		assert_eq!(a.es_igual_a(&Auto::new(String::from("asdf"),String::from("aytuiy"),2023,100432.0,Colores::Rojo)),true);
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
		assert_eq!(a.calcular_precio(),125000.0);
	}

	/*
		Concensionaria
	*/

	#[test]
	fn creacion_consecionaria(){
		let conse1 = ConcesionarioAuto::new("asd".to_string(),"tryertw".to_string(),10);
		assert_eq!(conse1.es_igual_a(&ConcesionarioAuto::new("asd".to_string(),"tryertw".to_string(),10)),true);
	}

	#[test]
	fn operatoria_consecionaria(){
		let a1 = Auto::new(String::from("asdf"),String::from("aytuiy"),2023,100432.0,Colores::Rojo);
		let a2 = Auto::new(String::from("BMW"),String::from("ajytjt"),2000,200500.0,Colores::Verde);
		let mut conse1 = ConcesionarioAuto::new("asd".to_string(),"tryertw".to_string(),3);
		//Limite de insersiones
		assert_eq!(conse1.agregar_auto(&a1),true);
		assert_eq!(conse1.agregar_auto(&a1),true);
		assert_eq!(conse1.agregar_auto(&a2),true);
		assert_eq!(conse1.agregar_auto(&a2),false);
		//Borra auto "a1"(primera recurrencia)
		conse1.eliminar_auto(&a1);

		//Busqueda de auto "a1"(solo encontrara al unico existente con tales caracteristicas)
		if let Some(a) = conse1.buscar_auto(&a1){
			assert_eq!(a.es_igual_a(&a1),true);
		}else{
			panic!("El auto no fue encontrado en el concesionario");
		}
		//Borra auto "a1"
		conse1.eliminar_auto(&a1);

		//Busqueda de auto "a1"(ya no lo dispone y no existe otro en la estructura)
		assert_eq!(conse1.buscar_auto(&a1).is_none(),true);
	}
}




/***
 * 		IMPLEMENTACION PARA EL TP5 - Ejercicio 1
**/
impl ConcesionarioAuto{
	fn to_string(&self)->String{
		return self.nombre.clone();
	}
	fn get_cantAutos(&self)->u32{
		return self.autos.len() as u32;
	}
	fn is_Lleno(&self)->bool{
		return self.autos.len() as u32 == self.capacidad;
	}
	fn is_Vacio(&self)->bool{
		return self.autos.is_empty();
	}
}

/*
	Tipos de errores
*/
pub enum error_baja{
	Inexistente(String),
	EstructuraVacia(String)
}

impl Display for error_baja{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self{
			error_baja::Inexistente(val) => write!(f, "No se encontro el auto en la consecionaria {} ",val),
			error_baja::EstructuraVacia(val) => write!(f, "La consecionaria {} no dispone de autos ",val)
		}
	}
}

pub struct error_capacidad(String);
impl Display for error_capacidad{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "La Capacidad de autos para la consecionaria {} fue superada" , self.0)
	}
}

pub enum Errores{
	ErrorBaja(error_baja),
	ErrorCapacidad(error_capacidad),
	ErrorIO(io::Error),
	ErrorSerde(serde_json::Error)
}
impl Display for Errores{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Errores::ErrorBaja(err) => write!(f,"{}",err),
			Errores::ErrorCapacidad(err) => write!(f,"{}",err)
		        Errores::ErrorIO(err) => write!(f, "Error de E/S al guardar: {}", e),
                        Errores::ErrorSerde(err) => write!(f, "Error de serialización: {}", e),
		}
	}
}
//Implementacion para el uso del operador (?)
impl std::error::Error for Errores {}

//Implementacion automatica errores subyacentes
impl From<io::Error> for Errores {
    fn from(err: io::Error) -> Self {
        Errores::ErrorIO(err)
    }
}

impl From<serde_json::Error> for Errores {
    fn from(err: serde_json::Error) -> Self {
        Errores::ErrorSerde(err)
    }
}



//Estructura auxiliar para el manejo de la informacion de los archivos
#[derive(Debug)]
pub struct Archivo_respaldable{
	informacion : ConcesionarioAuto,
	path : String,
	autoguardado : bool 
}

impl Archivo_respaldable{
	fn new(dato:&ConcesionarioAuto,dir:String,estado:bool)->Archivo_respaldable{
		return Archivo_respaldable { informacion: dato.clone(), path: dir , autoguardado : estado};
	}
	fn existe_archivo(&self)->bool{
		return Path::new(&self.path.clone()).exists();
	}
	/*	Generalizar el tipo de error 
	fn respaldar_informacion(&self)->Result<(),Err>{
		if self.existe_archivo(){
			let mut file = File::open(&self.path.clone())?;
			let serializado = serde_json::to_string(&self.informacion)?;
			file.write_all(&serializado.as_bytes());
		}else{
			let mut file = File::create(&self.path.clone())?;
			let serializado = serde_json::to_string(&self.informacion)?;
			file.write_all(&serializado.as_bytes());
		}
	}
	*/
	fn validar_insercion(&mut self,auto:&Auto)->Result<bool, Errores>{
		if !self.informacion.is_Lleno() {
			self.informacion.agregar_auto(&auto);
			return Ok(true);
		}else{
			return Err(Errores::ErrorCapacidad(error_capacidad(self.informacion.to_string()) ));
		}
	}
	fn validar_eliminacion(&mut self, a:&Auto)->Result<bool,Errores>{
		if !self.informacion.is_Vacio() {
			let mut pude = self.informacion.eliminar_auto(&a);
			if pude {return Ok(pude)}else{
				return Err(Errores::ErrorBaja(error_baja::Inexistente(self.informacion.to_string())) );
			}
		}
		return Err(Errores::ErrorBaja(error_baja::EstructuraVacia(self.informacion.to_string())) );
	}
}

#[cfg(test)]
mod testing_implementacion_ejercicio1{
use super::*;

	//Se implementa la estructura "Archivo respaldable" como un "influyente" entre el archivo JSON y el struct que se dispone 

	//Inciso a
	#[test]
	fn maxima_capacidad_superada(){
		let a1 = Auto::new(String::from("asdf"),String::from("aytuiy"),2023,100432.0,Colores::Rojo);
		let a2 = Auto::new(String::from("BMW"),String::from("ajytjt"),2000,200500.0,Colores::Verde);
		let a3 = Auto::new(String::from("BMW"),String::from("ajthrth"),2000,250000.0,Colores::Rojo);
		let a4 = Auto::new(String::from("Toyota"),String::from("artjrtjtt"),2000,200000.0,Colores::Azul);
		let mut conse1 = ConcesionarioAuto::new("asd".to_string(),"trrjrjrtw".to_string(),3);

		//No se hacen guardados en el archivo , solo validaciones de modificaciones
		let mut archivo1 = Archivo_respaldable::new(&conse1, "".to_string(),false);

		//Limite de insersiones

		//Primera Insercion(No notifica de un error)
		let r = archivo1.validar_insercion(&a1); 
		match r {
			Ok(mov) => assert!(true),
			Err(e) => {println!("error: {}", e); assert!(false);}
		}

		//Llenado de la consecionaria
		archivo1.validar_insercion(&a2);
		let r = archivo1.validar_insercion(&a3); 
		match r {
			Ok(mov) => assert!(true),
			Err(e) => {println!("error: {}", e); assert!(false);}
		}
		
		//Ultima insercion(Avisa del error)
		/*  	//Descomentar seccion para hacer la validacion 
		let r = archivo1.validar_insercion(&a4); 
		match r {
			Ok(mov) => assert!(true),
			Err(e) => {println!("error: {}", e); assert!(false);}
		}
		*/
		
	}

	//Inciso b
	#[test]
	fn eliminacion_de_un_auto(){
		//Autos
		let a1 = Auto::new(String::from("asdf"),String::from("aytuiy"),2023,100432.0,Colores::Rojo);
		let a2 = Auto::new(String::from("BMW"),String::from("ajytjt"),2000,200500.0,Colores::Verde);
		let a3 = Auto::new(String::from("BMW"),String::from("ajthrth"),2000,250000.0,Colores::Rojo);
		let a4 = Auto::new(String::from("Toyota"),String::from("artjrtjtt"),2000,200000.0,Colores::Azul);
		
		//Consecionaria
		let mut conse1 = ConcesionarioAuto::new("asd".to_string(),"tryertw".to_string(),3);
		
		let mut archivo1 = Archivo_respaldable::new(&conse1, "".to_string(),false);

		archivo1.validar_insercion(&a1);
		archivo1.validar_insercion(&a2);
		archivo1.validar_insercion(&a3);

		//Borra auto "a1"
		match archivo1.validar_eliminacion(&a1) {
			Ok(res) => assert!(true),
			Err(e) => {println!("error: {}", e); assert!(false);}
		}

		//Borra auto "a4" (Error de inexsistencia) ; Descomentar para probar
		/*  
		match archivo1.validar_eliminacion(&a4) {
			Ok(res) => assert!(true),
			Err(e) => {println!("error: {}", e); assert!(false);}
		}
		*/
		archivo1.validar_eliminacion(&a2);
		archivo1.validar_eliminacion(&a3);

		//Borra auto "a4" (Error de estructura vacia) ; Descomentar para probar
		/* 
		match archivo1.validar_eliminacion(&a4) {
			Ok(res) => assert!(true),
			Err(e) => {println!("error: {}", e); assert!(false);}
		}
		*/
		
	}

	
}
