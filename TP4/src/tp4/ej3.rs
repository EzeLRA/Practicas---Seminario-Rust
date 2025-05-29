use std::borrow::Borrow;
use std::collections::BinaryHeap;

/*
	Estructuras secundarias : Suscripciones , Medios de pago y 
*/

#[derive(PartialEq,Debug,Clone)]
pub enum Suscripciones{
	Basic,
	Clasic,
	Super
}
#[derive(PartialEq,Debug,Clone)]
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

#[derive(PartialEq,Debug,Clone)]
pub struct Suscripcion_activa{
	tipo_suscripcion : Suscripciones,
	costo_mensual : f64,
	duracion_mes : u8,
	fecha_inicio : u64,
	tipo_pago : Medios_de_pago
}

#[derive(PartialEq,Debug,Clone)]
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
	fn upgrade_suscripcion(&mut self){
		if let Some(mut s) = self.get_suscripcion_actual(){
			if s.upgrade(){
				self.suscripcion_anterior = self.suscripcion_actual.clone();
				self.suscripcion_actual = Some(s);
			}
		}
	}
	fn downgrade_suscripcion(&mut self){
		if let Some(mut s) = self.get_suscripcion_actual(){
			self.suscripcion_anterior = self.suscripcion_actual.clone();
			if !s.downgrade(){
				self.suscripcion_actual = None;
			}else{
				self.suscripcion_actual = Some(s);
			}
		}
	}
	fn cancelar_suscripcion(&mut self){
		if self.suscripcion_actual.is_some(){
			self.suscripcion_anterior = self.suscripcion_actual.clone();
			self.suscripcion_actual = None;
		}
	}
}

//Funcion auxiliar para obtener un maximo de un vector (u8)
fn obtener_max<const N:usize>(arr: [u8;N])->Option<u8>{
	let max_res = BinaryHeap::from(arr);
	return max_res.peek().copied();
}

pub trait UsuariosIteratorExt: Iterator + Sized 
{
	fn upgrade_usuario(&mut self,user1:&Usuario);

	fn downgrade_usuario(&mut self,user1:&Usuario);

	fn cancelar_suscripcion_usuario(&mut self,user1:&Usuario);

	fn metodo_pago_mas_usado(&self)->Option<Medios_de_pago>;
	
	fn suscripcion_mas_contratada(&self)->Option<Suscripciones>;
	
	fn metodo_pago_anterior_mas_usado(&self)->Option<Medios_de_pago>;
	
	fn suscripcion_anterior_mas_contratada(&self)->Option<Suscripciones>;
	
}

impl<'a, I> UsuariosIteratorExt for I
where
    I: Iterator<Item = Usuario> + Clone,
{
    fn upgrade_usuario(&mut self,user1:&Usuario){
		for mut user in self{
			if user == user1.clone() {
				user.upgrade_suscripcion();
				break;
			}
		}
	}

	fn downgrade_usuario(&mut self,user1:&Usuario){
		for mut user in self{
			if user == user1.clone() {
				user.downgrade_suscripcion();
				break;
			}
		}
	}

	fn cancelar_suscripcion_usuario(&mut self,user1:&Usuario){
		for mut user in self{
			if user == user1.clone() {
				user.cancelar_suscripcion();
				break;
			}
		}
	}

	fn metodo_pago_mas_usado(&self)->Option<Medios_de_pago>
	where
		Self: Clone,
	{	
		let mut it = self.clone().peekable();
		if it.peek().is_some(){
			let mut res : Option<Medios_de_pago>;

			let mut metodos_cant = [0; 5]; 
			for user in it{
				if let Some(s) = user.get_suscripcion_actual(){
					match s.get_medio(){
						Medios_de_pago::Efectivo => metodos_cant[0] +=1,
						Medios_de_pago::Mercado_pago => metodos_cant[1] +=1,
						Medios_de_pago::Transferencia_bancaria => metodos_cant[2] +=1,
						Medios_de_pago::Tarjeta_de_credito => metodos_cant[3] +=1,
						Medios_de_pago::Criptomoneda => metodos_cant[4] +=1,
					}
				}
			}
			//Obtener el maximo del array
			
			/*
			let mut max = -1;
			let mut pos : u8 = 0;
			for i in 0..4 {
				if metodos_cant[i] < max {
					max = metodos_cant[i];
					pos = i as u8;
				}
			}
			*/
			let pos = if let Some(s) = obtener_max(metodos_cant) {s}else{5};

			//Retornar segun posicion el tipo de pago con mas cantidad
			
			match pos {
			    0 => res = Some(Medios_de_pago::Efectivo),
			    1 => res = Some(Medios_de_pago::Mercado_pago),
			    2 => res = Some(Medios_de_pago::Transferencia_bancaria),
			    3 => res = Some(Medios_de_pago::Tarjeta_de_credito),
			    4 => res = Some(Medios_de_pago::Criptomoneda),
				_ => res = None,
		   	}
			

			return res;
		}
		return None
	}
	fn suscripcion_mas_contratada(&self)->Option<Suscripciones>
	where
		Self: Clone,
	{	
		let mut it = self.clone().peekable();
		if it.peek().is_some(){
			let mut res : Option<Suscripciones>;

			let mut tipos_cant = [0; 3]; 
			for user in it{
				if let Some(s) = user.get_suscripcion_actual(){
					match s.get_tipo(){
						Suscripciones::Basic => tipos_cant[0] +=1,
						Suscripciones::Clasic => tipos_cant[1] +=1,
						Suscripciones::Super => tipos_cant[2] +=1,
					}
				}
			}
			//Obtener el maximo del array
			
			/*
			let mut max = -1;
			let mut pos : u8 = 0;
			for i in 0..4 {
				if tipos_cant[i] < max {
					max = tipos_cant[i];
					pos = i as u8;
				}
			}
			*/
			let pos = if let Some(s) = obtener_max(tipos_cant) {s}else{5};

			//Retornar segun posicion del tipo de pago con mas cantidad
			
			match pos {
			    0 => res = Some(Suscripciones::Basic),
			    1 => res = Some(Suscripciones::Clasic),
			    2 => res = Some(Suscripciones::Super),
				_ => res = None,
		   	}
			

			return res;
		}
		return None
	}
	fn metodo_pago_anterior_mas_usado(&self)->Option<Medios_de_pago>
	where
		Self: Clone,
	{	
		let mut it = self.clone().peekable();
		if it.peek().is_some(){
			let mut res : Option<Medios_de_pago>;

			let mut metodos_cant = [0; 5]; 
			for user in it{
				if let Some(s) = user.get_suscripcion_anterior(){
					match s.get_medio(){
						Medios_de_pago::Efectivo => metodos_cant[0] +=1,
						Medios_de_pago::Mercado_pago => metodos_cant[1] +=1,
						Medios_de_pago::Transferencia_bancaria => metodos_cant[2] +=1,
						Medios_de_pago::Tarjeta_de_credito => metodos_cant[3] +=1,
						Medios_de_pago::Criptomoneda => metodos_cant[4] +=1,
					}
				}
			}
			//Obtener el maximo del array
			
			/*
			let mut max = -1;
			let mut pos : u8 = 0;
			for i in 0..4 {
				if metodos_cant[i] < max {
					max = metodos_cant[i];
					pos = i as u8;
				}
			}
			*/
			let pos = if let Some(s) = obtener_max(metodos_cant) {s}else{5};

			//Retornar segun posicion el tipo de pago con mas cantidad
			
			match pos {
			    0 => res = Some(Medios_de_pago::Efectivo),
			    1 => res = Some(Medios_de_pago::Mercado_pago),
			    2 => res = Some(Medios_de_pago::Transferencia_bancaria),
			    3 => res = Some(Medios_de_pago::Tarjeta_de_credito),
			    4 => res = Some(Medios_de_pago::Criptomoneda),
				_ => res = None,
		   	}
			

			return res;
		}
		return None
	}
	fn suscripcion_anterior_mas_contratada(&self)->Option<Suscripciones>
	where
		Self: Clone,
	{	
		let mut it = self.clone().peekable();
		if it.peek().is_some(){
			let mut res : Option<Suscripciones>;

			let mut tipos_cant = [0; 3]; 
			for user in it{
				if let Some(s) = user.get_suscripcion_anterior(){
					match s.get_tipo(){
						Suscripciones::Basic => tipos_cant[0] +=1,
						Suscripciones::Clasic => tipos_cant[1] +=1,
						Suscripciones::Super => tipos_cant[2] +=1,
					}
				}
			}
			//Obtener el maximo del array
			
			/*
			let mut max = -1;
			let mut pos : u8 = 0;
			for i in 0..4 {
				if tipos_cant[i] < max {
					max = tipos_cant[i];
					pos = i as u8;
				}
			}
			*/
			let pos = if let Some(s) = obtener_max(tipos_cant) {s}else{5};

			//Retornar segun posicion del tipo de pago con mas cantidad
			
			match pos {
			    0 => res = Some(Suscripciones::Basic),
			    1 => res = Some(Suscripciones::Clasic),
			    2 => res = Some(Suscripciones::Super),
				_ => res = None,
		   	}
			

			return res;
		}
		return None
	}
}

#[cfg(test)]
mod test_ejercicio2{
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
		
		usuario1.upgrade_suscripcion();

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

		usuario1.downgrade_suscripcion();

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
	}

	#[test]
	fn operar_suscripcion_usuarios(){
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
		let mut usuario1 = Usuario::new(&"Dario".to_string() , 
	32124 , 
		&Suscripcion_activa::crear_suscripcion(Suscripciones::Clasic,
			 183.5,
			  7, 
			  120125, 
			  Medios_de_pago::Criptomoneda));
	
		let mut usuarios : Vec<Usuario> = Vec::new();
		usuarios.push(usuario1);
		//Solucionar error de trait
		let res = usuarios.iter().suscripcion_mas_contratada();

		//assert!(usuarios.iter().suscripcion_anterior_mas_contratada().is_none());
		
		//assert!(usuarios.iter().suscripcion_mas_contratada().is_none());
		//assert!(usuarios.into_iter().metodo_pago_anterior_mas_usado().is_none());
		//assert!(usuarios.into_iter().metodo_pago_mas_usado().is_none());
	
	}

}