/*
	Estructuras secundarias : Suscripciones , Medios de pago y usuario
*/

#[derive(Debug,Clone)]
pub struct Suscripcion{
	costo_mensual : f64,
	duracion_mes : u8,
	fecha_inicio : u16
}
#[derive(Debug,Clone)]
pub enum Suscripciones{
	Basic(Suscripcion),
	Clasic(Suscripcion),
	Super(Suscripcion)
}
#[derive(Debug,Clone)]
pub enum Medios_de_pago{
	Efectivo,
	Mercado_pago,
	Transferencia_bancaria,
	Tarjeta_de_credito,
	Criptomoneda
}
#[derive(Debug,Clone)]
pub struct Usuario_datos{
	nombre : String,
	dni : u16
}

/*
	Estructura primaria : Suscripcion activa
*/

#[derive(Debug,Clone)]
pub struct Suscripcion_activa{
	usuario : Usuario_datos,
	tipo_suscripcion : Suscripciones,
	tipo_pago : Medios_de_pago
}

// Suscripcion_activa implementa los traits de usuario + medios de pago + suscripcion

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
}

impl DatosSuscripcion for Suscripcion{
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
}

impl Medios_de_pago{
	pub fn es_igual_a(&self, t: &Medios_de_pago) -> bool {
        match (self, t) {
            (Medios_de_pago::Efectivo, Medios_de_pago::Efectivo) => true,
            (Medios_de_pago::Mercado_pago, Medios_de_pago::Mercado_pago) => true,
            (Medios_de_pago::Transferencia_bancaria, Medios_de_pago::Transferencia_bancaria) => true,
            (Medios_de_pago::Tarjeta_de_credito, Medios_de_pago::Tarjeta_de_credito) => true,
            (Medios_de_pago::Criptomoneda, Medios_de_pago::Criptomoneda) => true,
            _ => false,
        }
    }
    pub fn get_medio(&self)->Medios_de_pago{
    	return self.clone()
    }
}