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
	suscripcion : Suscripcion_activa	//Integrar Option<Suscripcion_activa>
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
}
impl Usuario{
	fn new(nom:&String,dni_in:u16,s:&Suscripcion_activa)->Usuario{
		return Usuario{
			nombre : nom.clone(),
			dni : dni_in,
			suscripcion : s.clone()	//Integrar Option<Suscripcion_activa>
		}
	}
	//Implementar cambio de estado de suscripcion (pueda elevar o releevar de tipo)
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
