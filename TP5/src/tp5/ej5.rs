use std::fmt::{write, Display};
use std::io;
use std::{fs::{File,OpenOptions}, io::{Error,Read,Write}};
use std::path::Path;
use serde::{Serialize, Deserialize};
use serde_json;

/*
	EXTRACCION DEL EJERCICIO 3 - TP4
	Estructuras secundarias : Suscripciones , Medios de pago y usuarios
*/

#[derive(PartialEq,Debug,Clone,Serialize,Deserialize)]
pub enum Suscripciones{
	Basic,
	Clasic,
	Super
}
#[derive(PartialEq,Debug,Clone,Serialize,Deserialize)]
//No se agrego el tipo de dato que contienen cada dato porque no se piden calculos sobre la misma
pub enum Medios_de_pago{
	Efectivo,
	Mercado_pago,
	Transferencia_bancaria,
	Tarjeta_de_credito,
	Criptomoneda
}

/*
	Estructura primaria : Usuario
*/

#[derive(PartialEq,Debug,Clone,Serialize,Deserialize)]
pub struct Suscripcion_activa{
	tipo_suscripcion : Suscripciones,
	costo_mensual : f64,
	duracion_mes : u8,
	fecha_inicio : u64,
	tipo_pago : Medios_de_pago
}

#[derive(PartialEq,Debug,Clone,Serialize,Deserialize)]
pub struct Usuario{
	nombre : String,
	dni : u64 ,
	suscripcion_actual : Option<Suscripcion_activa>,
	suscripcion_anterior : Option<Suscripcion_activa>
}

/*
	Funcionalidades secundarias : Suscripciones , Medios de pago y usuario
*/

pub trait DatosSuscripcion{
	fn set_costo(&mut self,monto : f64);
	fn get_costo(self)->f64;
	fn set_duracion(&mut self,meses : u8);
	fn get_duracion(self)->u8;
	fn set_fecha_inicio(&mut self,f:u64);
	fn get_fecha_inicio(self)->u64;
	fn set_medio(&mut self,m:&Medios_de_pago);
    fn get_medio(&self)->Medios_de_pago;
    fn get_tipo(&self)->Suscripciones;
    fn set_tipo(&mut self,t:Suscripciones);
}
pub trait DatosUsuario{
	fn get_nombre(&self)->String;
	fn get_dni(&self)->u64;
	fn set_nombre(&mut self, nom : &String);
	fn set_dni(&mut self, dni_in : u64);
	fn get_suscripcion_anterior(&self)->Option<Suscripcion_activa>;
	fn get_suscripcion_actual(&self)->Option<Suscripcion_activa>;
	fn set_suscripcion_actual(&mut self,s:&Suscripcion_activa);
}
impl DatosUsuario for Usuario{
	fn get_nombre(&self)->String{
		return self.nombre.clone();
	}
	fn get_dni(&self)->u64{
		return self.dni;
	}
	fn set_nombre(&mut self, nom : &String){
		self.nombre = nom.clone();
	}
	fn set_dni(&mut self, dni_in : u64){
		self.dni = dni_in;
	}
	fn get_suscripcion_anterior(&self)->Option<Suscripcion_activa>{
		if self.suscripcion_anterior.is_some() {
			return self.suscripcion_anterior.clone();
		}
		return None;
	}
	fn get_suscripcion_actual(&self)->Option<Suscripcion_activa>{
		if self.suscripcion_actual.is_some() {
			return self.suscripcion_actual.clone();
		}
		return None;
	}
	fn set_suscripcion_actual(&mut self,s:&Suscripcion_activa){
		self.suscripcion_anterior = self.suscripcion_actual.clone();
		self.suscripcion_actual = Some(s.clone());
	}
}

impl DatosSuscripcion for Suscripcion_activa {
	fn set_costo(&mut self,monto : f64){
		self.costo_mensual = monto;
	}
	fn get_costo(self)->f64{
		return self.costo_mensual;
	}
	fn set_duracion(&mut self,meses : u8){
		self.duracion_mes = meses;
	}
	fn get_duracion(self)->u8{
		return self.duracion_mes;
	}
	fn set_fecha_inicio(&mut self,f:u64){
		self.fecha_inicio = f;
	}
	fn get_fecha_inicio(self)->u64{
		return self.fecha_inicio;
	}
	fn get_medio(&self)->Medios_de_pago{
		return self.tipo_pago.clone();
	}
	fn set_medio(&mut self, m : &Medios_de_pago) {
		self.tipo_pago = m.clone();
	}
	 fn get_tipo(&self)->Suscripciones{
		return self.tipo_suscripcion.clone();
	}
	 fn set_tipo(&mut self,t:Suscripciones){
		self.tipo_suscripcion = t.clone();
	}
}

/*
	Funcionalidades primarias para usuario
*/

impl Suscripcion_activa{
	//Funciones primarias
	fn crear_suscripcion(tipo:Suscripciones,monto:f64,duracion:u8,fecha_ini : u64,metodo_pago:Medios_de_pago)->Suscripcion_activa{
		return Suscripcion_activa{
			tipo_suscripcion : tipo,
			costo_mensual : monto,
			duracion_mes : duracion,
			fecha_inicio : fecha_ini,
			tipo_pago : metodo_pago
		}
	}
	fn upgrade(&mut self)->bool
	{
		match self.get_tipo(){
			Suscripciones::Basic =>	self.set_tipo(Suscripciones::Clasic),
			Suscripciones::Clasic => self.set_tipo(Suscripciones::Super),
			Suscripciones::Super => return false,
		}
		return true;
	}
	fn downgrade(&mut self)->bool
	{
		match self.get_tipo(){
			Suscripciones::Basic =>	return false,
			Suscripciones::Clasic => self.set_tipo(Suscripciones::Basic),
			Suscripciones::Super => self.set_tipo(Suscripciones::Clasic),
		}
		return true;
	}
}

impl Usuario{
	fn new(nom:&String,dni_in:u64,s:&Suscripcion_activa)->Usuario{
		return Usuario{
			nombre : nom.clone(),
			dni : dni_in,
			suscripcion_actual : Some(s.clone()),
			suscripcion_anterior : None	
		}
	}
	fn upgrade_suscripcion(&mut self)->bool{
		if let Some(mut s) = self.get_suscripcion_actual(){
			if s.upgrade(){
				self.suscripcion_anterior = self.suscripcion_actual.clone();
				self.suscripcion_actual = Some(s);
				return true;
			}
		}
		return false;
	}
	fn downgrade_suscripcion(&mut self)->bool{
		if let Some(mut s) = self.get_suscripcion_actual(){
			self.suscripcion_anterior = self.suscripcion_actual.clone();
			if !s.downgrade(){
				self.suscripcion_actual = None;
			}else{
				self.suscripcion_actual = Some(s);
			}
			return true;
		}
		return false;
	}
	fn cancelar_suscripcion(&mut self)->bool{
		if self.suscripcion_actual.is_some(){
			self.suscripcion_anterior = self.suscripcion_actual.clone();
			self.suscripcion_actual = None;
			return true;
		}
		return false;
	}
	fn es_igual_a(&self,u:&Usuario)->bool{
		return (self.nombre == u.nombre)&&(self.dni == u.dni);
	}
}

//Funcion auxiliar para obtener un maximo de un vector (u8)
fn obtener_max<const N:usize>(arr: [u8;N])->Option<usize>{
	if arr != [0;N] {
		let mut max = 0;
		arr.iter().enumerate().for_each(|(i,cantidad)| {
			if *cantidad>arr[max] {
				max = i;
			}
		});
		return Some(max);
	}
	return None;
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Plataforma{
	usuarios : Vec<Usuario>
}
impl Plataforma{
	fn new()->Plataforma{
		return Plataforma { usuarios: Vec::new() }
	}
	fn agregar(&mut self,u2:&Usuario)->bool{
		match self.usuarios.iter().find(|us| us.es_igual_a(&u2)) {
            Some(_u) => return false,
            None => self.usuarios.push(u2.clone())
        }
        return true;
	}
	fn eliminar(&mut self,u:&Usuario)->bool{
		let mut pude = false;
		if let Some(pos) = self.usuarios.iter().position(|us| us.es_igual_a(&u) ){
        	self.usuarios.remove(pos);
        	pude = true;
        }
        return pude;
	}
	fn upgrade_usuario(&mut self, usuario: &Usuario) -> bool {
        match self.usuarios.iter_mut().find(|u| u.es_igual_a(&usuario)) {
            Some(u) => return u.upgrade_suscripcion(),
            None => return false,
        }
    }
	fn downgrade_usuario(&mut self, usuario: &Usuario) -> bool {
        match self.usuarios.iter_mut().find(|u| u.es_igual_a(&usuario)) {
            Some(u) => return u.downgrade_suscripcion(),
            None => return false,
        }
    }
	fn cancelar_suscripcion(&mut self, usuario: &Usuario) -> bool {
        match self.usuarios.iter_mut().find(|u| u.es_igual_a(&usuario)) {
            Some(u) => return u.cancelar_suscripcion(),
            None => return false,
        }
    }
	fn metodo_pago_mas_usado(&self)->Option<Medios_de_pago>
	{	
		if !self.usuarios.is_empty(){
			let mut res : Option<Medios_de_pago> = None;

			let mut metodos_cant = [0; 5];
			self.usuarios.iter().for_each(|user| {
				if let Some(s) = user.get_suscripcion_actual(){
					match s.get_medio(){
						Medios_de_pago::Efectivo => metodos_cant[0] +=1,
						Medios_de_pago::Mercado_pago => metodos_cant[1] +=1,
						Medios_de_pago::Transferencia_bancaria => metodos_cant[2] +=1,
						Medios_de_pago::Tarjeta_de_credito => metodos_cant[3] +=1,
						Medios_de_pago::Criptomoneda => metodos_cant[4] +=1,
					}
				}
			});
			
			//Retornar segun posicion el tipo de pago con mas cantidad
			if let Some(pos) = obtener_max(metodos_cant) {
				match pos {
					0 => res = Some(Medios_de_pago::Efectivo),
					1 => res = Some(Medios_de_pago::Mercado_pago),
					2 => res = Some(Medios_de_pago::Transferencia_bancaria),
					3 => res = Some(Medios_de_pago::Tarjeta_de_credito),
					4 => res = Some(Medios_de_pago::Criptomoneda),
					_ => res = None,
				}
			}

			return res;
		}
		return None
	}
	fn suscripcion_mas_contratada(&self)->Option<Suscripciones>
	{	
		if !self.usuarios.is_empty(){
			let mut res : Option<Suscripciones> = None;

			let mut tipos_cant = [0; 3]; 
			self.usuarios.iter().for_each(|user| {
				if let Some(s) = user.get_suscripcion_actual(){
					match s.get_tipo(){
						Suscripciones::Basic => tipos_cant[0] +=1,
						Suscripciones::Clasic => tipos_cant[1] +=1,
						Suscripciones::Super => tipos_cant[2] +=1,
					}
				}
			});
			
			//Retornar segun posicion del tipo de pago con mas cantidad
			if let Some(pos) = obtener_max(tipos_cant) {
				match pos {
					0 => res = Some(Suscripciones::Basic),
					1 => res = Some(Suscripciones::Clasic),
					2 => res = Some(Suscripciones::Super),
					_ => res = None,
				}
			}

			return res;
		}
		return None
	}
	fn metodo_pago_anterior_mas_usado(&self)->Option<Medios_de_pago>
	{	
		if !self.usuarios.is_empty(){
			let mut res : Option<Medios_de_pago> = None;

			let mut metodos_cant = [0; 5]; 
			self.usuarios.iter().for_each(|user| {
				if let Some(s) = user.get_suscripcion_anterior(){
					match s.get_medio(){
						Medios_de_pago::Efectivo => metodos_cant[0] +=1,
						Medios_de_pago::Mercado_pago => metodos_cant[1] +=1,
						Medios_de_pago::Transferencia_bancaria => metodos_cant[2] +=1,
						Medios_de_pago::Tarjeta_de_credito => metodos_cant[3] +=1,
						Medios_de_pago::Criptomoneda => metodos_cant[4] +=1,
					}
				}
			});
			
			//Retornar segun posicion el tipo de pago con mas cantidad
			if let Some(pos) = obtener_max(metodos_cant) {
				match pos {
					0 => res = Some(Medios_de_pago::Efectivo),
					1 => res = Some(Medios_de_pago::Mercado_pago),
					2 => res = Some(Medios_de_pago::Transferencia_bancaria),
					3 => res = Some(Medios_de_pago::Tarjeta_de_credito),
					4 => res = Some(Medios_de_pago::Criptomoneda),
					_ => res = None,
				}
			}

			return res;
		}
		return None
	}
	fn suscripcion_anterior_mas_contratada(&self)->Option<Suscripciones>
	{	
		if !self.usuarios.is_empty(){
			let mut res : Option<Suscripciones> = None;

			let mut tipos_cant = [0; 3]; 
			self.usuarios.iter().for_each(|user| {
				if let Some(s) = user.get_suscripcion_anterior(){
					match s.get_tipo(){
						Suscripciones::Basic => tipos_cant[0] +=1,
						Suscripciones::Clasic => tipos_cant[1] +=1,
						Suscripciones::Super => tipos_cant[2] +=1,
					}
				}
			});
			//Obtener el maximo del array
	
			//Retornar segun posicion del tipo de pago con mas cantidad
			if let Some(pos) = obtener_max(tipos_cant) {
				match pos {
			    0 => res = Some(Suscripciones::Basic),
			    1 => res = Some(Suscripciones::Clasic),
			    2 => res = Some(Suscripciones::Super),
				_ => res = None,
				}
			}		

			return res;
		}
		return None
	}
}

#[cfg(test)]
mod test_ejercicio3{
	use core::panic;
	use super::*;

	#[test]
	fn operar_suscripcion_usuario(){
		let mut usuario1 = Usuario::new(&"Daniel".to_string() , 
		64254 , 
		&Suscripcion_activa::crear_suscripcion(Suscripciones::Basic,
			 123.5,
			  5, 
			  120325, 
			  Medios_de_pago::Transferencia_bancaria));
			
		assert_eq!(usuario1,Usuario::new(&"Daniel".to_string() , 64254 , &Suscripcion_activa::crear_suscripcion(Suscripciones::Basic,123.5,5,120325,Medios_de_pago::Transferencia_bancaria)));
		
		assert!(usuario1.upgrade_suscripcion());

		if let Some(s) = usuario1.get_suscripcion_actual(){
			assert_eq!(s.get_tipo(),Suscripciones::Clasic);
			if let Some(s2) = usuario1.get_suscripcion_anterior(){
				assert_eq!(s2.get_tipo(),Suscripciones::Basic);
			}else{
				panic!("No se registro/actualizo la suscripcion anterior");
			}
		}else{
			panic!("No se registro/actualizo la suscripcion actual");
		}

		assert!(usuario1.downgrade_suscripcion());

		if let Some(s) = usuario1.get_suscripcion_actual(){
			assert_eq!(s.get_tipo(),Suscripciones::Basic);
			if let Some(s2) = usuario1.get_suscripcion_anterior(){
				assert_eq!(s2.get_tipo(),Suscripciones::Clasic);
			}else{
				panic!("No se registro/actualizo la suscripcion anterior");
			}
		}else{
			panic!("No se registro/actualizo la suscripcion actual");
		}

		assert!(usuario1.cancelar_suscripcion());
	}

	#[test]
	fn operar_suscripciones_usuarios(){
		let mut usuario1 = Usuario::new(&"Daniel".to_string() , 
		64254 , 
		&Suscripcion_activa::crear_suscripcion(Suscripciones::Basic,
			 123.5,
			  5, 
			  120325, 
			  Medios_de_pago::Transferencia_bancaria));

		let mut usuario2 = Usuario::new(&"Tobias".to_string() , 
		93843 , 
		&Suscripcion_activa::crear_suscripcion(Suscripciones::Super,
			 225.5,
			  12, 
			  310325, 
			  Medios_de_pago::Transferencia_bancaria));

		let mut usuario3 = Usuario::new(&"Marcos".to_string() , 
		542134 , 
		&Suscripcion_activa::crear_suscripcion(Suscripciones::Basic,
			 103.5,
			  3, 
			  120525, 
			  Medios_de_pago::Efectivo));

		let mut usuario4 = Usuario::new(&"Dario".to_string() , 
	32124 , 
		&Suscripcion_activa::crear_suscripcion(Suscripciones::Clasic,
			 183.5,
			  7, 
			  120125, 
			  Medios_de_pago::Criptomoneda));
	

		//Plataforma vacia

		let mut pl1 = Plataforma::new();

		assert!(pl1.metodo_pago_mas_usado().is_none());
		assert!(pl1.metodo_pago_anterior_mas_usado().is_none());
		assert!(pl1.suscripcion_mas_contratada().is_none());
		assert!(pl1.suscripcion_anterior_mas_contratada().is_none());

		//Plataforma con usuarios

		pl1.agregar(&usuario1);
		pl1.agregar(&usuario2);
		pl1.agregar(&usuario3);
		pl1.agregar(&usuario4);
		
		 
		if let Some(tipo) = pl1.metodo_pago_mas_usado(){
			assert_eq!(tipo,Medios_de_pago::Transferencia_bancaria);
		}else{
			panic!("No hubo un retorno esperado");
		}
		
		if let Some(tipo) = pl1.suscripcion_mas_contratada(){
			assert_eq!(tipo,Suscripciones::Basic);
		}else{
			panic!("No hubo un retorno esperado");
		}
		assert!(pl1.metodo_pago_anterior_mas_usado().is_none());
		assert!(pl1.suscripcion_anterior_mas_contratada().is_none());

		pl1.upgrade_usuario(&usuario1);
		assert!(pl1.upgrade_usuario(&usuario1));
		assert_eq!(pl1.upgrade_usuario(&usuario2),false);

		if let Some(tipo) = pl1.metodo_pago_mas_usado(){
			assert_eq!(tipo,Medios_de_pago::Transferencia_bancaria);
		}else{
			panic!("No hubo un retorno esperado");
		}

		if let Some(tipo) = pl1.metodo_pago_anterior_mas_usado(){
			assert_eq!(tipo,Medios_de_pago::Transferencia_bancaria);
		}else{
			panic!("No hubo un retorno esperado");
		}
		
		if let Some(tipo) = pl1.suscripcion_mas_contratada(){
			assert_eq!(tipo,Suscripciones::Super);
		}else{
			panic!("No hubo un retorno esperado");
		}

		if let Some(tipo) = pl1.suscripcion_anterior_mas_contratada(){
			assert_eq!(tipo,Suscripciones::Clasic);
		}else{
			panic!("No hubo un retorno esperado");
		}

	}

}

/*	
	IMPLEMENTACION EJ5-TP5	-	Se maneja el listado de suscripciones por usuario y el autoguardado se mantiene siempre activado
*/

/*
    Tipos de errores
*/
#[derive(Debug)]
pub enum error_operatoria{
	Existente(String),
    Inexistente(String),
    EstructuraVacia(String)
}

impl Display for error_operatoria{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
        	error_operatoria::Existente(val) => write!(f, "Ya existe el elemento en la estructura {} ",val),
            error_operatoria::Inexistente(val) => write!(f, "No se encontro el elemento en la estructura {} ",val),
            error_operatoria::EstructuraVacia(val) => write!(f, "La estrucutra {} no dispone de elementos ",val)
        }
    }
}

#[derive(Debug)]
pub enum Errores{
	ErrorSuscripcion(String),
    ErrorOperatoria(error_operatoria),
    ErrorIO(io::Error),
    ErrorSerde(serde_json::Error)
}

impl Display for Errores{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
        	Errores::ErrorSuscripcion(val) => write!(f, "No dispone de una suscripcion el usuario {} ",val),
            Errores::ErrorOperatoria(err) => write!(f,"{}",err),
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

/*
	Archivo principal
*/

#[derive(Debug)]
pub struct Archivo{
    informacion : Plataforma, //Se considera que la plataforma solo tiene el listado de usuarios con sus suscripciones
    path : String,
    autoguardado : bool 
}

impl Archivo{
	fn new(dato:&Plataforma,dir:&String,estado:bool)->Archivo{
        return Archivo { informacion: dato.clone(), path: dir.clone() , autoguardado : estado};
    }
    fn existe_archivo(&self)->bool{
        return Path::new(&self.path.clone()).exists();
    }
    fn respaldar_informacion(&self) -> Result<(), Errores> {
        // Apertura/Creación del archivo
        let mut file = if self.existe_archivo() {
            // Abrir en modo lectura/escritura si existe
            OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)
            .map_err(Errores::ErrorIO)?
        } else {
            // Crear nuevo archivo si no existe
            File::create(&self.path).map_err(Errores::ErrorIO)?
        };

        // Serialización de la informacion
        let serializado = serde_json::to_string(&self.informacion)
            .map_err(Errores::ErrorSerde)?;

        // Escritura en el archivo
        file.write_all(serializado.as_bytes())
            .map_err(Errores::ErrorIO)?;

        Ok(())
    }
    fn registrar_suscripcion_usuario(&mut self,u:&Usuario)-> Result<(), Errores>{
        if !self.informacion.agregar(&u){
        	return Err(Errores::ErrorOperatoria(error_operatoria::Existente(String::from("Listado de suscripciones"))) );
        }

		if self.autoguardado{
			self.respaldar_informacion()?;
		}

		Ok(())
    }
    fn eliminar_suscripcion_usuario(&mut self,u:&Usuario)-> Result<(), Errores>{
    	if !self.informacion.usuarios.is_empty(){
    		if !self.informacion.eliminar(&u){
    			return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente(String::from("Listado de suscripciones"))) );
    		}
    	}else{
    		return Err(Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(String::from("Listado de suscripciones"))) );
    	}

    	if self.autoguardado{
			self.respaldar_informacion()?;
		}

		Ok(())
    }
    fn upgrade_suscripcion_usuario(&mut self,u:&Usuario)-> Result<(), Errores>{
    	if !self.informacion.usuarios.is_empty(){
    		if let Some(user) = self.informacion.usuarios.iter_mut().find(|us1| us1.es_igual_a(&u)){
    			if !user.upgrade_suscripcion(){
    				return Err(Errores::ErrorSuscripcion(user.get_nombre()));
    			}
    		}else{
    			return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente(String::from("Listado de suscripciones"))) );
    		}
    	}else{
    		return Err(Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(String::from("Listado de suscripciones"))) );
    	}

    	if self.autoguardado{
			self.respaldar_informacion()?;
		}

		Ok(())
    }
    fn downgrade_suscripcion_usuario(&mut self,u:&Usuario)-> Result<(), Errores>{
    	if !self.informacion.usuarios.is_empty(){
    		if let Some(user) = self.informacion.usuarios.iter_mut().find(|us1| us1.es_igual_a(&u)){
    			if !user.downgrade_suscripcion(){
    				return Err(Errores::ErrorSuscripcion(user.get_nombre()));
    			}
    		}else{
    			return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente(String::from("Listado de suscripciones"))) );
    		}
    	}else{
    		return Err(Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(String::from("Listado de suscripciones"))) );
    	}

    	if self.autoguardado{
			self.respaldar_informacion()?;
		}

		Ok(())
    }
    fn cancelar_suscripcion_usuario(&mut self,u:&Usuario)-> Result<(), Errores>{
    	if !self.informacion.usuarios.is_empty(){
    		if let Some(user) = self.informacion.usuarios.iter_mut().find(|us1| us1.es_igual_a(&u)){
    			if !user.cancelar_suscripcion(){
    				return Err(Errores::ErrorSuscripcion(user.get_nombre()));
    			}
    		}else{
    			return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente(String::from("Listado de suscripciones"))) );
    		}
    	}else{
    		return Err(Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(String::from("Listado de suscripciones"))) );
    	}

    	if self.autoguardado{
			self.respaldar_informacion()?;
		}

		Ok(())
    }
    fn retornar_suscripcion_max(&self)->Result<Suscripciones,Errores>{
    	//Apertura(Debe existir el archivo fisico)
        let mut file = File::open(self.path.clone())?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let suscripciones : Plataforma = serde_json::from_str(&buf)?;

        if !suscripciones.usuarios.is_empty(){
    		if let Some(sus) = suscripciones.suscripcion_mas_contratada() {
    			return Ok(sus);
    		}
    	}
    	return Err(Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(String::from("Listado de suscripciones"))) );
    }
    fn retornar_suscripcion_anterior_max(&self)->Result<Suscripciones,Errores>{
    	//Apertura(Debe existir el archivo fisico)
        let mut file = File::open(self.path.clone())?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let suscripciones : Plataforma = serde_json::from_str(&buf)?;

        if !suscripciones.usuarios.is_empty(){
    		if let Some(sus) = suscripciones.suscripcion_anterior_mas_contratada() {
    			return Ok(sus);
    		}
    	}
    	return Err(Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(String::from("Listado de suscripciones"))) );
    }
    fn retornar_medio_pago_max(&self)->Result<Medios_de_pago,Errores>{
    	//Apertura(Debe existir el archivo fisico)
        let mut file = File::open(self.path.clone())?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let suscripciones : Plataforma = serde_json::from_str(&buf)?;

        if !suscripciones.usuarios.is_empty(){
    		if let Some(med) = suscripciones.metodo_pago_mas_usado() {
    			return Ok(med);
    		}
    	}
    	return Err(Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(String::from("Listado de suscripciones"))) );
    }
    fn retornar_medio_pago_anterior_max(&self)->Result<Medios_de_pago,Errores>{
    	//Apertura(Debe existir el archivo fisico)
        let mut file = File::open(self.path.clone())?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let suscripciones : Plataforma = serde_json::from_str(&buf)?;

        if !suscripciones.usuarios.is_empty(){
    		if let Some(med) = suscripciones.metodo_pago_anterior_mas_usado() {
    			return Ok(med);
    		}
    	}
    	return Err(Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(String::from("Listado de suscripciones"))) );
    }
}

#[cfg(test)]
mod test_implementacion_ejercicio5{
	use super::*;

	#[test]
	fn operatoria_informacion(){
		//Plataforma y las suscripciones

		//Plataforma vacia
		let pl1 = Plataforma::new();

		//Usuarios con las suscripciones
		let usuario1 = Usuario::new(&"Daniel".to_string() , 
		64254 , 
		&Suscripcion_activa::crear_suscripcion(Suscripciones::Basic,
			 123.5,
			  5, 
			  120325, 
			  Medios_de_pago::Transferencia_bancaria));

		let usuario2 = Usuario::new(&"Tobias".to_string() , 
		93843 , 
		&Suscripcion_activa::crear_suscripcion(Suscripciones::Super,
			 225.5,
			  12, 
			  310325, 
			  Medios_de_pago::Transferencia_bancaria));

		let usuario3 = Usuario::new(&"Marcos".to_string() , 
		542134 , 
		&Suscripcion_activa::crear_suscripcion(Suscripciones::Basic,
			 103.5,
			  3, 
			  120525, 
			  Medios_de_pago::Efectivo));

		let usuario4 = Usuario::new(&"Dario".to_string() , 
	32124 , 
		&Suscripcion_activa::crear_suscripcion(Suscripciones::Clasic,
			 183.5,
			  7, 
			  120125, 
			  Medios_de_pago::Criptomoneda));


		//Archivo (la plafatorma no tiene usuarios)
		let mut archivo1 = Archivo::new(&pl1,&"".to_string(),false);

		//Registro de usuario1
		match archivo1.registrar_suscripcion_usuario(&usuario1){
			Ok(_) => assert!(true),
			Err(e) => {println!("error: {}", e); assert!(false);}
		}

		//Registro de usuario2 y usuario3
		match archivo1.registrar_suscripcion_usuario(&usuario2){
			Ok(_) => assert!(true),
			Err(e) => {println!("error: {}", e); assert!(false);}
		}	
		match archivo1.registrar_suscripcion_usuario(&usuario3){
			Ok(_) => assert!(true),
			Err(e) => {println!("error: {}", e); assert!(false);}
		}

		//Eliminar usuario1
		match archivo1.eliminar_suscripcion_usuario(&usuario1){
			Ok(_) => assert!(true),
			Err(e) => {println!("error: {}", e); assert!(false);}
		}

		//Registro de usuario4
		match archivo1.registrar_suscripcion_usuario(&usuario4){
			Ok(_) => assert!(true),
			Err(e) => {println!("error: {}", e); assert!(false);}
		}

		//Cancelacion de suscripcion usuario2
		match archivo1.cancelar_suscripcion_usuario(&usuario2){
			Ok(_) => assert!(true),
			Err(e) => {println!("error: {}", e); assert!(false);}
		}

		//Upgrade de suscripcion usuario3
		match archivo1.upgrade_suscripcion_usuario(&usuario3){
			Ok(_) => assert!(true),
			Err(e) => {println!("error: {}", e); assert!(false);}
		}

		//Downgrade de suscripcion usuario4
		match archivo1.downgrade_suscripcion_usuario(&usuario4){
			Ok(_) => assert!(true),
			Err(e) => {println!("error: {}", e); assert!(false);}
		}

		//Solo se hacen estas operaciones porque las funciones de estadistica como
		//suscripcion_max y metodopago_max implican abrir el archivo (algo que para este contexto el archivo no esta creado)

	}

	#[test]
	fn operatoria_archivo_suscripciones(){
		//Plataforma y las suscripciones

		//Plataforma vacia
		let pl1 = Plataforma::new();

		//Usuarios con las suscripciones
		let usuario1 = Usuario::new(&"Daniel".to_string() , 
		64254 , 
		&Suscripcion_activa::crear_suscripcion(Suscripciones::Basic,
			 123.5,
			  5, 
			  120325, 
			  Medios_de_pago::Transferencia_bancaria));

		let usuario2 = Usuario::new(&"Tobias".to_string() , 
		93843 , 
		&Suscripcion_activa::crear_suscripcion(Suscripciones::Super,
			 225.5,
			  12, 
			  310325, 
			  Medios_de_pago::Transferencia_bancaria));

		let usuario3 = Usuario::new(&"Marcos".to_string() , 
		542134 , 
		&Suscripcion_activa::crear_suscripcion(Suscripciones::Basic,
			 103.5,
			  3, 
			  120525, 
			  Medios_de_pago::Efectivo));

		let usuario4 = Usuario::new(&"Dario".to_string() , 
	32124 , 
		&Suscripcion_activa::crear_suscripcion(Suscripciones::Clasic,
			 183.5,
			  7, 
			  120125, 
			  Medios_de_pago::Criptomoneda));


		//Archivo (la plafatorma no tiene usuarios)
		let mut archivo1 = Archivo::new(&pl1,&"src/tp5/registro_suscripciones.json".to_string(),true);

		//Registro de usuarios
		match archivo1.registrar_suscripcion_usuario(&usuario1){
			Ok(_) => assert!(true),
			Err(e) => {println!("error: {}", e); assert!(false);}
		}

		match archivo1.registrar_suscripcion_usuario(&usuario2){
			Ok(_) => assert!(true),
			Err(e) => {println!("error: {}", e); assert!(false);}
		}

		match archivo1.registrar_suscripcion_usuario(&usuario3){
			Ok(_) => assert!(true),
			Err(e) => {println!("error: {}", e); assert!(false);}
		}

		match archivo1.registrar_suscripcion_usuario(&usuario4){
			Ok(_) => assert!(true),
			Err(e) => {println!("error: {}", e); assert!(false);}
		}

		//Baja usuario1 y usuario3
		match archivo1.eliminar_suscripcion_usuario(&usuario1){
			Ok(_) => assert!(true),
			Err(e) => {println!("error: {}", e); assert!(false);}
		}

		match archivo1.eliminar_suscripcion_usuario(&usuario3){
			Ok(_) => assert!(true),
			Err(e) => {println!("error: {}", e); assert!(false);}
		}

		//Upgrade usuario4
		match archivo1.upgrade_suscripcion_usuario(&usuario4){
			Ok(_) => assert!(true),
			Err(e) => {println!("error: {}", e); assert!(false);}
		}

		//Suscripcion Max
		match archivo1.retornar_suscripcion_max(){
			Ok(res) => assert!(res == Suscripciones::Super),
			Err(e) => {println!("error: {}", e); assert!(false);}
		}

		//Metodo pago Max
		match archivo1.retornar_medio_pago_max(){
			Ok(res) => assert!(res == Medios_de_pago::Transferencia_bancaria),
			Err(e) => {println!("error: {}", e); assert!(false);}
		}

		//Downgrade usuario2 y usuario4
		match archivo1.downgrade_suscripcion_usuario(&usuario2){
			Ok(_) => assert!(true),
			Err(e) => {println!("error: {}", e); assert!(false);}
		}

		match archivo1.downgrade_suscripcion_usuario(&usuario4){
			Ok(_) => assert!(true),
			Err(e) => {println!("error: {}", e); assert!(false);}
		}

		//Suscripcion anterior Max (luego de que se efecuenten operatorias)
		match archivo1.retornar_suscripcion_anterior_max(){
			Ok(res) => assert!(res == Suscripciones::Super),
			Err(e) => {println!("error: {}", e); assert!(false);}
		}

		//Metodo pago anterior Max (luego de que se efecuenten operatorias)
		match archivo1.retornar_medio_pago_anterior_max(){
			Ok(res) => assert!(res == Medios_de_pago::Transferencia_bancaria),
			Err(e) => {println!("error: {}", e); assert!(false);}
		}

		//Cancelacion de suscripcion usuario2
		match archivo1.cancelar_suscripcion_usuario(&usuario2){
			Ok(_) => assert!(true),
			Err(e) => {println!("error: {}", e); assert!(false);}
		}

	}

}