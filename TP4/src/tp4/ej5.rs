use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

//Funcion auxiliar para generar un "hash simulado"
pub fn aleatorio(tam: usize) -> String {
    let rng = thread_rng();
    rng.sample_iter(&Alphanumeric)
        .take(tam)
        .map(char::from)
        .collect()
}

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
    fn es_igual_a(&self,nom:&String)->bool{
        return &self.nombre == nom;
    }
    fn generar_hash(tam:usize)->String{
        aleatorio(tam)
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
    fn agregar_blockchain(&mut self,b:&Blockchain)->bool{
        let mut pude = false;
        
        if self.blockchains.iter().find(|&blockchain| blockchain == b ).is_none() {
            self.blockchains.push(b.clone());
            pude = true;
        }

        return pude;
    }
    fn eliminar_blockchain(&mut self,b:&Blockchain)->bool{
        let mut pude = false;
        
        if let Some(pos) = self.blockchains.iter().position(|blockchain| blockchain == b){
            self.blockchains.remove(pos);
            pude = true;
        }

        return pude;
    }
    fn blockchain_encontrado(&self,nom:&String)->bool{
        return self.blockchains.iter().find(|&blockchain| blockchain.es_igual_a(nom)).is_some()
    }
    fn get_nombre(&self)->String{
        return self.nombre.clone();
    }
    fn get_prefijo(&self)->String{
        return self.prefijo.clone();
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
pub struct CriptomonedaDispone(String,f64);

impl CriptomonedaDispone{
    //Sin limite de ingreso maximo
    fn contabilizar(&mut self,monto:f64){
        self.1 += monto;
    }
    //Con limite de extracion(Sin saldo negativo)
    fn descontabilizar(&mut self,monto:f64)->bool{
        let mut pude = false;
        if self.1 >= monto {
            self.1 -= monto;
            pude = true;
        }
        return pude;
    }
    fn es_igual_a(&self,nom:&String)->bool{
        return self.0 == nom.clone();
    }
    fn get_monto(&self)->f64{
        return self.1;
    }
}

#[derive(PartialEq,Debug,Clone)]
pub struct BalancePropio{
    criptomonedas : Vec<CriptomonedaDispone>,
    dinero_fiat : f64
}

impl BalancePropio{
    fn new()->BalancePropio{
        return BalancePropio { criptomonedas: Vec::new() , dinero_fiat: 0.0 }
    }
    fn fijar_fiat(&mut self,monto:f64){
        self.dinero_fiat = monto;
    }
    //Sin limite de ingreso maximo
    fn contabilizar_fiat(&mut self,monto:f64){
        self.dinero_fiat += monto;
    }
    //Con limite de extracion(Sin saldo negativo)
    fn descontabilizar_fiat(&mut self,monto:f64)->bool{
        let mut pude = false;
        if self.dinero_fiat >= monto {
            self.dinero_fiat -= monto;
            pude = true;
        }
        return pude;
    }
    fn agregar_criptomoneda(&mut self,nom:&String,monto:f64){
        self.criptomonedas.push(CriptomonedaDispone(nom.clone(),monto));
    }
    fn contabilizar_criptomoneda(&mut self,nom:&String,monto:f64)->bool{
        let mut pude = false;
        
        if let Some(dato) = self.criptomonedas.iter_mut().find(|cripto| cripto.es_igual_a(&nom)){
            dato.contabilizar(monto);
            pude = true;
        }

        return pude;
    }
    fn descontabilizar_criptomoneda(&mut self,nom:&String,monto:f64)->bool{
        let mut pude = false;
        
        if let Some(pos) = self.criptomonedas.iter().position(|cripto| cripto.es_igual_a(&nom)){
            pude = self.criptomonedas[pos].descontabilizar(monto);
            if self.criptomonedas[pos].get_monto() <= 0.0 {
                self.criptomonedas.remove(pos);
            }
        }

        return pude;
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
    //Fiat moneda
    fn ingresar_monto_fiat(&mut self,monto:f64){
        self.balance.contabilizar_fiat(monto);
    }
    fn retirar_monto_fiat(&mut self,monto:f64)->bool{
        return self.balance.descontabilizar_fiat(monto);
    }
    fn get_balance_fiat(&self)->f64{
        return self.balance.get_cant_fiat();
    }
    //Criptomoneda(Los montos se basan en lo que le proporcione el sistema)
    fn comprar_criptomoneda(&mut self,nom:&String,montoCompra:f64,montoComprado:f64)->bool{
        let mut pude = false;
        if self.validado {
            pude = self.retirar_monto_fiat(montoCompra);
            if pude {
                if !self.balance.contabilizar_criptomoneda(nom,montoComprado){
                    self.balance.agregar_criptomoneda(nom,montoComprado);
                }
            }
        }
        return pude;
    }
    fn vender_criptomoneda(&mut self,nom:&String,montoVenta:f64,gananciaObtenida:f64)->bool{
        let mut pude = false;
        if self.validado {
            pude = self.balance.descontabilizar_criptomoneda(nom,montoVenta);            
            if pude {
                self.balance.contabilizar_fiat(gananciaObtenida);
            }
        }
        return pude;
    }
    //Transacciones a blockchains
    fn criptomoneda_a_blockchain(&mut self,nomCripto:&String,monto:f64,nomBlockchain:&String,cripto : &Criptomoneda)->bool{
        let mut pude = false;
        if cripto.blockchain_encontrado(nomBlockchain){
            pude = self.balance.descontabilizar_criptomoneda(nomCripto,monto);
        }
        return pude;
    }
    fn blockchain_a_criptomoneda(&mut self,nomCripto:&String,monto:f64,nomBlockchain:&String,cripto : &Criptomoneda)->bool{
        let pude = cripto.blockchain_encontrado(nomBlockchain);
        if pude {
            self.balance.contabilizar_criptomoneda(nomCripto,monto);
        }
        return pude;
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
        return Datos_Operacion_Criptomoneda { 
            datos_genericos: Datos_Ingreso::new(user, f, m ),
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
    //Considerar que son calculos especificos
    //Retorna equivalente (dinero fiat a criptomoneda)
    fn calcular_monto_compra(&self,monto:f64)->f64{
        return monto/self.1;
    }
    //Retorna equivalente (criptomoneda a dinero fiat)
    fn calcular_monto_venta(&self,monto:f64)->f64{
        return monto*self.1;
    }
}



/*
    Estructura principal : Sistema
*/


#[derive(PartialEq,Debug,Clone)]
pub struct Plataforma{
    usuarios : Vec<Usuario>,
    criptomonedas_dispone : Vec<Criptomoneda_disponible>, //Datos de la criptomoneda y cotiza
    registro_transacciones : Vec<TiposTransacciones>
}
/*
    incisos:
    Un modulo debe contabilizar las ventas y compras hechas de criptomonedas
    Un modulo debe contabilizar las solicitudes de ventas y compras de criptomonedas
*/

impl Plataforma{
    //Funciones secundarias
    fn eliminar_criptomoneda(&mut self,cripto:&Criptomoneda)->bool{
        let mut pude = false;
        
        if let Some(pos) = self.criptomonedas_dispone.iter().position(|c| c.0.get_nombre() == cripto.get_nombre()){
            pude = true;
            self.criptomonedas_dispone.remove(pos);
        }       

        return pude;
    }
    fn registrar_criptomoneda(&mut self,cripto:&Criptomoneda,cotiza:f64)->bool{
        let mut pude = false;
        
        if self.criptomonedas_dispone.iter().find(|&c| c.0.get_nombre() == cripto.get_nombre() ).is_none(){
            self.criptomonedas_dispone.push(Criptomoneda_disponible(cripto.clone(),cotiza));
            pude = true;
        }        

        return pude;
    }
    fn obtener_cotizacion_criptomoneda(&self,nom:&String)->f64{
        let mut total = 0.0;
        if let Some(cr) = self.criptomonedas_dispone.iter().find(|&c| &c.0.get_nombre() == nom){
            total = cr.get_cotiza();
        }
        return total;
    }
    fn registrar_transaccion(&mut self,t : &TiposTransacciones){
        self.registro_transacciones.push(t.clone());
    }

    //Funciones primarias
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
    fn comprar_criptomoneda_usuario(&mut self,u1:&Usuario,f:&Fecha,montoCompra:f64,nom:&String)->bool{
        let mut completo = false;
        if let Some(u) = self.usuarios.iter_mut().find(|user| user.informacion_correcta(&u1.datos)){
            
            if let Some(cr) = self.criptomonedas_dispone.iter().find(|&c| &c.0.get_nombre() == nom){
                //Compra
                let montoObtenido = cr.calcular_monto_compra(montoCompra);
                completo = u.comprar_criptomoneda(nom,montoCompra,montoObtenido);

                if completo {
                    //Generacion de comprobante
                    let datos = Datos_Operacion_Criptomoneda::new(&u.datos.clone(),&f.clone(),montoObtenido,&cr.0.clone(),cr.get_cotiza());
                    self.registrar_transaccion(&TiposTransacciones::CompraCriptomoneda(datos));
                }
                
            }
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
