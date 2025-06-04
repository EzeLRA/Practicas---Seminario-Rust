/*
    Estructuras base para el sistema
*/

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
}

/*
    Estructura pertenciente al sistema
*/

#[derive(PartialEq,Debug,Clone)]
pub struct Fecha(u8,u8,u64);

#[derive(PartialEq,Debug,Clone)]
pub struct Datos_comprobante{
    datos_usuario : DatosPersona,
    fecha: Fecha,
    tipo : TiposTransacciones,
    monto : f64
}

impl InformacionPersonal for Datos_comprobante{
    fn informacion_correcta(&self, info:&DatosPersona)->bool{
        return &self.datos_usuario == info;
    }
}

/*
    Agregar propiedades a cada enum 
*/

#[derive(PartialEq,Debug,Clone)]
pub enum TiposTransacciones{
    IngresoFiat,
    CompraCriptomoneda,
    VentaCriptomoneda,
    RetiroCriptomoneda,
    RecepcionCriptomoneda,
    RetiroFiat,
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
    criptomonedas_dispone : Vec<Criptomoneda_disponible> //Datos de la criptomoneda y cotiza
}

