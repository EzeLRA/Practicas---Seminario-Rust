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
	fecha_inicio : u16,
	tipo_pago : Medios_de_pago
}

#[derive(PartialEq,Debug,Clone)]
pub struct Usuario{
	nombre : String,
	dni : u16 ,
	suscripcion_actual : Option<Suscripcion_activa>
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
	fn set_fecha_inicio(&mut self,f:u16);
	fn get_fecha_inicio(self)->u16;
	fn set_medio(&mut self,m:&Medios_de_pago);
    fn get_medio(&self)->Medios_de_pago;
    fn get_tipo(&self)->Suscripciones;
    fn set_tipo(&mut self,t:Suscripciones);
}
pub trait DatosUsuario{
	fn get_nombre(&self)->String;
	fn get_dni(&self)->u16;
	fn set_nombre(&mut self, nom : &String);
	fn set_dni(&mut self, dni_in : u16);
	fn get_suscripcion(&self)->Option<Suscripcion_activa>;
	fn set_suscripcion(&mut self,s:&Suscripcion_activa);
}
impl DatosUsuario for Usuario{
	fn get_nombre(&self)->String{
		return self.nombre.clone();
	}
	fn get_dni(&self)->u16{
		return self.dni;
	}
	fn set_nombre(&mut self, nom : &String){
		self.nombre = nom.clone();
	}
	fn set_dni(&mut self, dni_in : u16){
		self.dni = dni_in;
	}
	fn get_suscripcion(&self)->Option<Suscripcion_activa>{
		if let Some(s) = self.suscripcion_actual.clone(){
			return Some(s);
		}else{
			return None;
		}
	}
	fn set_suscripcion(&mut self,s:&Suscripcion_activa){
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
	fn set_fecha_inicio(&mut self,f:u16){
		self.fecha_inicio = f;
	}
	fn get_fecha_inicio(self)->u16{
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
	fn crear_suscripcion(tipo:Suscripciones,monto:f64,duracion:u8,fecha_ini : u16,metodo_pago:Medios_de_pago)->Suscripcion_activa{
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
	fn new(nom:&String,dni_in:u16,s:&Suscripcion_activa)->Usuario{
		return Usuario{
			nombre : nom.clone(),
			dni : dni_in,
			suscripcion_actual : Some(s.clone()),
			suscripcion_anterior : None	
		}
	}
	fn upgrade_suscripcion(&mut self){
		if let Some(mut s) = self.get_suscripcion(){
			if s.upgrade(){
				self.suscripcion_anterior = self.suscripcion_actual.clone();
				self.suscripcion_actual = Some(s);
			}
		}
	}
	fn downgrade_suscripcion(&mut self){
		if let Some(mut s) = self.get_suscripcion(){
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

pub trait UsuariosIteratorExt: Iterator<Item = Usuario> + Sized {
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
		if self.clone().next().is_some(){
			let mut res : Option<Medios_de_pago>;

			let mut metodos_cant = [0; 5]; 
			for user in self.clone(){
				if let Some(s) = user.get_suscripcion(){
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
			
			let mut max = -1;
			let mut pos : u8 = 0;
			for i in 0..4 {
				if metodos_cant[i] < max {
					max = metodos_cant[i];
					pos = i as u8;
				}
			}

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
	fn suscripcion_mas_contratado(&self)->Option<Suscripciones>
	where
		Self: Clone,
	{	
		if self.clone().next().is_some(){
			let mut res : Option<Suscripciones>;

			let mut tipos_cant = [0; 3]; 
			for user in self.clone(){
				if let Some(s) = user.get_suscripcion(){
					match s.get_tipo(){
						Suscripciones::Basic => tipos_cant[0] +=1,
						Suscripciones::Clasic => tipos_cant[1] +=1,
						Suscripciones::Super => tipos_cant[2] +=1,
					}
				}
			}
			//Obtener el maximo del array
			
			let mut max = -1;
			let mut pos : u8 = 0;
			for i in 0..4 {
				if tipos_cant[i] < max {
					max = tipos_cant[i];
					pos = i as u8;
				}
			}

			//Retornar segun posicion el tipo de pago con mas cantidad
			
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

impl<I> UsuariosIteratorExt for I
where
    I: Iterator<Item = Usuario>,
{
    // Usa la implementación por defecto del trait
}

