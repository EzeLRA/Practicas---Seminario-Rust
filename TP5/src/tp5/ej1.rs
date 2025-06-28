use std::fmt::{write, Display};
use std::io;
use std::{fs::{File,OpenOptions}, io::{Error,Read,Write}};
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
#[derive(Debug , Clone, Serialize, Deserialize)]
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
#[derive(Debug , Clone , Serialize, Deserialize)]
pub struct Auto{
    marca : String,
    modelo : String,
    anio : u32,
    precio_bruto : f32,
    color : Colores
}

#[derive(Debug,Clone,Serialize, Deserialize)]
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
		let a = Auto::new(String::from("gfd"),String::from("mkemf"),1990,100000.0,Colores::Rojo);
		assert_eq!(a.calcular_precio(),120000.0);
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
		let a3 = Auto::new(String::from("BMW"),String::from("ytjyjt"),2000,200000.0,Colores::Blanco);
        let mut conse1 = ConcesionarioAuto::new("asd".to_string(),"tryertw".to_string(),3);
		//Limite de insersiones
		assert_eq!(conse1.get_cantAutos(),0);
		assert_eq!(conse1.agregar_auto(&a1),true);
		assert_eq!(conse1.agregar_auto(&a2),true);
		assert_eq!(conse1.agregar_auto(&a3),true);
		assert_eq!(conse1.agregar_auto(&a2),false);
		assert_eq!(conse1.get_cantAutos(),3);
		
		//Busqueda de auto "a1 y a3"(solo encontrara al unico existente con tales caracteristicas)
		if let Some(a) = conse1.buscar_auto(&a1){
			assert_eq!(a.es_igual_a(&a1),true);
		}else{
			panic!("El auto no fue encontrado en el concesionario");
		}
        if let Some(a) = conse1.buscar_auto(&a3){
			assert_eq!(a.es_igual_a(&a3),true);
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
#[derive(Debug)]
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

#[derive(Debug)]
pub struct error_capacidad(String);
impl Display for error_capacidad{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "La Capacidad de autos para la consecionaria {} fue superada" , self.0)
	}
}

#[derive(Debug)]
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
			Errores::ErrorCapacidad(err) => write!(f,"{}",err),
		    Errores::ErrorIO(err) => write!(f, "Error de E/S al guardar: {}", err),
            Errores::ErrorSerde(err) => write!(f, "Error de serialización: {}", err)
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
	
	fn rescatar_informacion(&self)-> Result<ConcesionarioAuto,Errores>{
		let mut file = File::open(self.path.clone())?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let consecionaria  = serde_json::from_str(&buf)?;
        Ok(consecionaria) 
	}

	fn respaldar_informacion(&self) -> Result<(), Errores> {
        // Apertura/Creación del archivo (Se utiliza OpenOptions para la apertura y edicion de un archivo existente)
        let mut file = if self.existe_archivo() {
        	// Abrir en modo lectura/escritura si existe
        	OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)?
    	} else {
        	// Crear nuevo archivo si no existe
        	File::create(&self.path)?
    	};

        // Serialización de la informacion
        let serializado = serde_json::to_string(&self.informacion)?;

        // Escritura en el archivo
        file.write_all(serializado.as_bytes())?;

        Ok(())
    }
	
	fn validar_insercion(&mut self,auto:&Auto)->Result<(), Errores>{
		if !self.informacion.is_Lleno() {
			self.informacion.agregar_auto(&auto);
		}else{
			return Err(Errores::ErrorCapacidad(error_capacidad(self.informacion.to_string()) ));
		}

		if self.autoguardado{
			self.respaldar_informacion()?;
		}

		Ok(())
	}
	fn validar_eliminacion(&mut self, a:&Auto)->Result<(),Errores>{
		if !self.informacion.is_Vacio() {
			let mut pude = self.informacion.eliminar_auto(&a);
			if !pude {
				return Err(Errores::ErrorBaja(error_baja::Inexistente(self.informacion.to_string())) );
			}
		}else{
			return Err(Errores::ErrorBaja(error_baja::EstructuraVacia(self.informacion.to_string())) );
		}

		if self.autoguardado{
			self.respaldar_informacion()?;
		}
		Ok(())
	}
}

#[cfg(test)]
mod testing_implementacion_ejercicio1{
use std::env::temp_dir;

use super::*;

    //Test para maximar el coverage
    #[test]
    fn test_error_serializacion(){
        let directorio_temp = std::env::temp_dir();
        let archivo_temp = directorio_temp.join("archivo_prueba.json");
        std::fs::write(&archivo_temp, "contenido basura");

        let mut conse1 = ConcesionarioAuto::new("asd".to_string(),"trrjrjrtw".to_string(),3);

		let mut archivo1 = Archivo_respaldable::new(&conse1, archivo_temp.to_str().unwrap().to_string() ,true);
		match archivo1.rescatar_informacion(){
            Ok(_) => assert!(false),
			Err(e) => assert!(format!("{}",e).contains("Error de serialización")) 
        }
		
    }

    //Test para maximar el coverage
    #[test]
    fn test_error_guardado(){
        let a1 = Auto::new(String::from("asdf"),String::from("aytuiy"),2023,100432.0,Colores::Rojo);

        let mut conse1 = ConcesionarioAuto::new("asd".to_string(),"trrjrjrtw".to_string(),3);

        //Direccion nula
		let mut archivo1 = Archivo_respaldable::new(&conse1, "".to_string(),true);

        //Primera Insercion(Notifica de un error)
		match archivo1.validar_insercion(&a1) {
			Ok(_) => assert!(false),
			Err(e) => assert!(format!("{}",e).contains("Error de E/S al guardar")) 
		}

        //Intento de guardardo forzoso
        match archivo1.respaldar_informacion() {
			Ok(_) => assert!(false),
			Err(e) => assert!(format!("{}",e).contains("Error de E/S al guardar")) 
		}

    }

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
		assert!(archivo1.validar_insercion(&a1).is_ok());

		//Llenado de la consecionaria
		assert!(archivo1.validar_insercion(&a2).is_ok());
		match archivo1.validar_insercion(&a3) {
			Ok(_) => assert!(true),
			Err(e) => assert!(false,"{}",e)
		}
		
		//Ultima insercion(Avisa del error)
		match archivo1.validar_insercion(&a4) {
			Ok(_) => assert!(false,"Aqui tendria que haber fallado") , 
			Err(e) => assert!(format!("{}",e).contains("La Capacidad de autos para la consecionaria")) 
		}
		
		
	}

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
			Ok(_) => assert!(true),
			Err(e) => assert!(false,"Error : {}",e)
		}

		//Borra auto "a4" (Error de inexsistencia)
		
		match archivo1.validar_eliminacion(&a4) {
			Ok(_) => assert!(false) , 
			Err(e) => assert!(format!("{}",e).contains("No se encontro el auto en la consecionaria")) 
		}
		
		archivo1.validar_eliminacion(&a2);
		archivo1.validar_eliminacion(&a3);

		//Borra auto "a4" (Error de estructura vacia)
		
		match archivo1.validar_eliminacion(&a4) {
			Ok(_) => assert!(false) , 
			Err(e) => assert!(format!("{}",e).contains("no dispone de autos"))
		}
		
		
	}

	#[test]
	fn operatoria_archivo_sin_autoguardado(){
		
		//Consecionaria
		let mut conse1 = ConcesionarioAuto::new("asd".to_string(),"tryertw".to_string(),3);
		
		let mut archivo1 = Archivo_respaldable::new(&conse1, "src/concesionaria_info.json".to_string(),false);

		//Guardado directo sin realizar operatorias
		match archivo1.respaldar_informacion(){
			Ok(_) => assert!(true),
			Err(e) => assert!(false,"Error : {}",e)
		}

		/*
			Resultado del archivo(JSON) = {"nombre":"asd","direccion":"tryertw","capacidad":3,"autos":[]}
		*/

		//Autos
		let a1 = Auto::new(String::from("ffds"),String::from("yjyjy"),2023,100432.0,Colores::Azul);
		let a2 = Auto::new(String::from("BMW"),String::from("avcvbvt"),2000,200500.0,Colores::Verde);
		let a3 = Auto::new(String::from("BMW"),String::from("ajmujmuh"),2000,250000.0,Colores::Rojo);
		let a4 = Auto::new(String::from("Toyota"),String::from("arttt"),2000,200000.0,Colores::Azul);
		
		archivo1.validar_insercion(&a1);
		archivo1.validar_insercion(&a2);
		archivo1.validar_insercion(&a3);
		//Error de capacidad
        match archivo1.validar_insercion(&a4) {
			Ok(_) => assert!(false,"Aqui tendria que haber fallado") , 
			Err(e) => assert!(format!("{}",e).contains("La Capacidad de autos para la consecionaria")) 
		}
		 
		//Guardado directo con operatorias realizadas
		match archivo1.respaldar_informacion(){
			Ok(_) => assert!(true),
			Err(e) => assert!(false,"Error : {}",e)
		}
		
		/*
			Resultado del archivo(JSON) = {"nombre":"asd","direccion":"tryertw","capacidad":3,"autos":[{"marca":"ffds","modelo":"yjyjy","anio":2023,"precio_bruto":100432.0,"color":"Azul"},{"marca":"BMW","modelo":"avcvbvt","anio":2000,"precio_bruto":200500.0,"color":"Verde"},{"marca":"BMW","modelo":"ajmujmuh","anio":2000,"precio_bruto":250000.0,"color":"Rojo"}]}
		*/

		archivo1.validar_eliminacion(&a1);
		archivo1.validar_eliminacion(&a3);

		//Guardado directo con operatorias realizadas
		match archivo1.respaldar_informacion(){
			Ok(_) => assert!(true),
			Err(e) => assert!(false,"Error : {}",e)
		}

        //Funcion de lectura de archivo
		match archivo1.rescatar_informacion(){
			Ok(d) => assert!(!d.is_Vacio()),
			Err(e) => assert!(false,"Error : {}",e)
		}	

		/* 
			Resultado del archivo(JSON) = {"nombre":"asd","direccion":"tryertw","capacidad":3,"autos":[{"marca":"BMW","modelo":"avcvbvt","anio":2000,"precio_bruto":200500.0,"color":"Verde"}]}
		*/

	}

	#[test]
	fn operatoria_archivo_con_autoguardado(){
		//Autos
		let a1 = Auto::new(String::from("ffds"),String::from("yjyjy"),2023,100432.0,Colores::Azul);
		let a2 = Auto::new(String::from("BMW"),String::from("avcvbvt"),2000,200500.0,Colores::Verde);
		let a3 = Auto::new(String::from("BMW"),String::from("ajmujmuh"),2000,250000.0,Colores::Rojo);
		let a4 = Auto::new(String::from("Toyota"),String::from("arttt"),2000,200000.0,Colores::Azul);
		
		//Consecionaria
		let mut conse1 = ConcesionarioAuto::new("asd".to_string(),"tryertw".to_string(),3);
		
		let mut archivo1 = Archivo_respaldable::new(&conse1, "src/concesionaria_info.json".to_string(),true);

		//Carga de informacion
		match archivo1.validar_insercion(&a1){
			Ok(_) => assert!(true),
			Err(e) => assert!(false,"Error : {}",e)
		}
		archivo1.validar_insercion(&a2);
		archivo1.validar_insercion(&a3);
		archivo1.validar_insercion(&a4);

		//Baja de informacion
		match archivo1.validar_eliminacion(&a1){
			Ok(_) => assert!(true),
			Err(e) => assert!(false,"Error : {}",e)
		}

		archivo1.validar_eliminacion(&a3);

		//Error de inexistencia
        match archivo1.validar_eliminacion(&a1) {
			Ok(_) => assert!(false) , 
			Err(e) => assert!(format!("{}",e).contains("No se encontro el auto en la consecionaria")) 
		}

	}
	
}
