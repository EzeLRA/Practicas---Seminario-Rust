/*
    Estructuras base para el sistema
*/

use crate::tp4::ej4::Datos_Persona;

#[derive(PartialEq,Debug,Clone)]
pub struct Blockchain{
    nombre : String,
    prefijo : String
}

impl Blockchain{
    fn new(nom:&String,pre:&String)->Blockchain{
        return Blockchain { nombre: nom.clone(), prefijo: pre.clone() }
    }
}

#[derive(PartialEq,Debug,Clone)]
pub struct Criptomoneda{
    nombre : String,
    prefijo : String,
    blockchains : Vec<Blockchain>
}

impl Criptomoneda{
    fn new(nom:&String,pre:&String)->Criptomoneda{
        return Criptomoneda { nombre: nom.clone(), prefijo: pre.clone(), blockchains: Vec::new()}
    }
    fn agregar_blockchain(&mut self,b:&Blockchain){
        self.blockchains.push(b.clone());
    }
}

#[derive(PartialEq,Debug,Clone)]
pub struct DatosPersona{
    nombre : String,
    apellido : String,
    email : String,
    dni : u64
}

pub trait InformacionPersonal{
    fn get_nombre(&self, datos : &DatosPersona)->String{
        return datos.nombre.clone();
    }
    fn get_apellido(&self, datos: &DatosPersona)->String{
        return datos.apellido.clone();
    }
    fn get_email(&self, datos : &DatosPersona)->String{
        return datos.email.clone();
    }
    fn get_dni(&self, datos : &DatosPersona)->u64{
        return datos.dni;
    }
    fn informacion_correcta(&self, info:&DatosPersona)->bool;
}


#[derive(PartialEq,Debug,Clone)]
pub struct BalancePropio{
    criptomonedas : Vec<Criptomoneda>,
    dinero_fiat : f64
}

impl BalancePropio{
    fn new()->BalancePropio{
        return BalancePropio { criptomonedas: Vec::new() , dinero_fiat: 0.0 }
    }
    fn agregar_criptomoneda(&mut self,c:&Criptomoneda){
        self.criptomonedas.push(c.clone());
    }
    fn fijar_fiat(&mut self,monto:f64){
        self.dinero_fiat = monto;
    }
    fn contabilizar_fiat(&mut self,monto:f64){
        self.dinero_fiat += monto;
    }
    fn descontabilizar_fiat(&mut self,monto:f64){
        self.dinero_fiat -= monto;
    }
    fn get_cant_fiat(&self)->f64{
        return self.dinero_fiat;
    }
}

#[derive(PartialEq,Debug,Clone)]
pub struct Usuario{
    datos : DatosPersona,
    validado : bool,
    balance : BalancePropio
}

impl InformacionPersonal for Usuario{
    fn informacion_correcta(&self, info:&DatosPersona)->bool{
        return &self.datos == info;
    }
}

impl Usuario{
    fn new(nom:&String,ape:&String,mail:&String,dni_in:u64)->Usuario{
        return Usuario { datos: DatosPersona { nombre: nom.clone(), apellido: ape.clone(), email: mail.clone(), dni: dni_in} , validado: false, balance: BalancePropio::new()}
    }
    fn obtener_verificacion(&self)->bool{
        return self.validado;
    }
    fn cambiar_verificacion(&mut self){
        self.validado = !self.validado;
    }
    fn ingresar_monto_fiat(&mut self,monto:f64){
        self.balance.contabilizar_fiat(monto);
    }
    fn retirar_monto_fiat(&mut self,monto:f64){
        if self.balance.get_cant_fiat() > 0.0 {
            self.balance.descontabilizar_fiat(monto);
        }
    }
    fn get_balance_fiat(&self)->f64{
        return self.balance.get_cant_fiat();
    }
}

/*
    Estructura pertenciente al sistema
*/

#[derive(PartialEq,Debug,Clone)]
pub struct Fecha(u8,u8,u64);

#[derive(PartialEq,Debug,Clone)]
pub struct Datos_Ingreso{
    datos_usuario : DatosPersona,
    fecha: Fecha,
    monto : f64
}

impl Datos_Ingreso{
    fn new(datos : &DatosPersona, f : &Fecha , m : f64)->Datos_Ingreso{
        return Datos_Ingreso { datos_usuario: datos.clone(), fecha: f.clone(), monto: m };
    }
    fn get_fecha(&self)->Fecha{
        return self.fecha.clone();
    }
    fn get_monto(&self)->f64{
        return self.monto;
    }
}

impl InformacionPersonal for Datos_Ingreso{
    fn informacion_correcta(&self, info:&DatosPersona)->bool{
        return &self.datos_usuario == info;
    }
}

#[derive(PartialEq,Debug,Clone)]
pub struct Datos_Retiro{
    datos_genericos : Datos_Ingreso,
    medio_pago : MediosPago
}

impl Datos_Retiro{
    fn new(d:&DatosPersona,fe:&Fecha,monto:f64,medio:&MediosPago)->Datos_Retiro{
        return Datos_Retiro { datos_genericos: Datos_Ingreso::new(d, fe , monto), medio_pago: medio.clone() }
    }

    fn get_medio_pago(&self)->MediosPago{
        return self.medio_pago.clone();
    }
}

//Tipos de transacciones implementados
#[derive(PartialEq,Debug,Clone)]
pub struct Datos_Operacion_Criptomoneda{
    datos_genericos : Datos_Ingreso,
    criptomoneda : Criptomoneda,
    cotizacion : f64
}

impl Datos_Operacion_Criptomoneda{
    fn new(user:&DatosPersona,f:&Fecha,m:f64,c:&Criptomoneda,cotiz:f64)->Datos_Operacion_Criptomoneda{
        return Datos_Operacion_Criptomoneda { datos_genericos: Datos_Ingreso::new(user, f, m ),
         criptomoneda: c.clone(), 
         cotizacion: cotiz };
    }
    fn get_criptomoneda(&self)->Criptomoneda{
        return self.criptomoneda.clone();
    }
    fn get_cotizacion(&self)->f64{
        return self.cotizacion;
    }
}

#[derive(PartialEq,Debug,Clone)]
pub struct Datos_Retiro_Blockchain{
    datos_criptomoneda : Datos_Operacion_Criptomoneda,
    blockchain : Blockchain,
    hash : String
}

impl Datos_Retiro_Blockchain{
    fn new(user:&DatosPersona,f:&Fecha,m:f64,c:&Criptomoneda,cotiz:f64,b:&Blockchain)->Datos_Retiro_Blockchain{
        return Datos_Retiro_Blockchain { datos_criptomoneda: Datos_Operacion_Criptomoneda::new(user, f, m, c, cotiz),
             blockchain: b.clone(),
            hash: "Random".to_string() }
    }
    fn get_blockchain(&self)->Blockchain{
        return self.blockchain.clone();
    }
    fn get_hash(&self)->String{
        return self.hash.clone();
    }
}

#[derive(PartialEq,Debug,Clone)]
pub struct Datos_Extraccion_Blockchain{
    datos_criptomoneda : Datos_Operacion_Criptomoneda,
    blockchain : Blockchain
}

impl Datos_Extraccion_Blockchain{
    fn new(user:&DatosPersona,f:&Fecha,m:f64,c:&Criptomoneda,cotiz:f64,b:&Blockchain)->Datos_Extraccion_Blockchain{
        return Datos_Extraccion_Blockchain { datos_criptomoneda: Datos_Operacion_Criptomoneda::new(user, f, m, c, cotiz),
             blockchain: b.clone()
        }
    }
    fn get_blockchain(&self)->Blockchain{
        return self.blockchain.clone();
    }
}

#[derive(PartialEq,Debug,Clone)]
pub enum TiposTransacciones{
    IngresoFiat(Datos_Ingreso),
    CompraCriptomoneda(Datos_Operacion_Criptomoneda),
    VentaCriptomoneda(Datos_Operacion_Criptomoneda),
    RetiroCriptomoneda(Datos_Retiro_Blockchain),
    RecepcionCriptomoneda(Datos_Extraccion_Blockchain),
    RetiroFiat(Datos_Retiro),
}

#[derive(PartialEq,Debug,Clone)]
pub enum MediosPago{
    MercadoPago,
    TransferenciaBancaria
}

#[derive(PartialEq,Debug,Clone)]
pub struct Criptomoneda_disponible(Criptomoneda,f64);
impl Criptomoneda_disponible{
    fn get_cotiza(&self)->f64{
        return self.1;
    }
}

#[derive(PartialEq,Debug,Clone)]
pub struct Plataforma{
    usuarios : Vec<Usuario>,
    criptomonedas_dispone : Vec<Criptomoneda_disponible>, //Datos de la criptomoneda y cotiza
    registro_transacciones : Vec<TiposTransacciones>
}

impl Plataforma{
    fn registrar_transaccion(&mut self,t : &TiposTransacciones){
        self.registro_transacciones.push(t.clone());
    }
    fn ingresar_monto_usuario(&mut self,u1:&Usuario,f:&Fecha,m:f64)->bool{
        let mut completo = false;
        if let Some(u) = self.usuarios.iter_mut().find(|user| user.informacion_correcta(&u1.datos)){
            u.ingresar_monto_fiat(m);
            let datos = Datos_Ingreso::new(&u.datos, f, m);
            self.registrar_transaccion(&TiposTransacciones::IngresoFiat(datos));
            completo = true;
        }
        return completo;
    }
    fn retirar_monto_usuario(&mut self,u1:&Usuario,f:&Fecha,m:f64,med:&MediosPago)->bool{
        let mut completo = false;
        if let Some(u) = self.usuarios.iter_mut().find(|user| user.informacion_correcta(&u1.datos)){
            u.retirar_monto_fiat(m);
            let datos = Datos_Retiro::new(&u.datos, f, m,med);
            self.registrar_transaccion(&TiposTransacciones::RetiroFiat(datos));
            completo = true;
        }
        return completo;
    }
}
