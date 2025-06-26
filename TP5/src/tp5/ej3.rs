/***
 * 
 *      EXTRACCION DE LA ESTRUCTURA FECHA DE TP3
 * 
***/
//Atributos
#[derive(Debug,Clone ,Serialize, Deserialize)]
pub struct Fecha{
    pub dia : u8,
    pub mes : u8,
    pub anio : u16
}

/*
    Metodos
*/

impl Fecha{

    //Metodos Secundarios
    pub fn get_dia(&self)->u8{
        return self.dia;
    }
    pub fn get_mes(&self)->u8{
        return self.mes;
    }
    pub fn get_anio(&self)->u16{
        return self.anio;
    }
    pub fn es_igual_a(&self,f:&Fecha)->bool{
        return if(self.get_dia() == f.get_dia())&&(self.get_mes() == f.get_mes())&&(self.get_anio() == f.get_anio()){true}else{false}
    }
    /*
        Metodos Primarios    
     */
    pub fn new(d:u8,m:u8,a:u16)->Fecha{
        return Fecha { dia: d , mes: m , anio: a }
    }
    pub fn es_fecha_valida(&self)->bool{
        
        if (self.mes > 0) && (self.mes <= 12) && (self.anio > 0) && (self.dia > 0) {
        
            match self.mes{
                2 => if self.es_bisiesto() { return self.dia <= 29 }else{ return self.dia <= 28},
                9|4|6|11 => return self.dia <= 30,
                _ => return self.dia <= 31
            }
            
        }

        return false;
    }

    pub fn es_bisiesto(&self)->bool{
        return (self.anio % 4)==0;
    }

    //Auxiliar para determinar el ultimo dia de un mes
    fn ultimo_dia(&self)->u8{
        match self.mes{
            2 => if self.es_bisiesto() {29}else{28},
            9|4|6|11 => 30,
            _ => 31
        }
    
    }

    //Auxiliar para avanzar de mes y anio
    fn avanzar_mes(&mut self) {
        if self.mes == 12 {
            self.mes = 1;
            self.anio += 1;
        } else {
            self.mes += 1;
        }
        self.dia = 1;
    }

    //Se considera que la fecha es valida
    pub fn sumar_dias(&mut self,mut dias_sumar:u32){
        //Bucle principal para el calculo
        while dias_sumar > 0 {
            //Obtiene el ultimo dia del mes (Cantidad total de dias que le corresponde)
            let dias_mes = self.ultimo_dia();
            //Calcula el resto de dias que debera actualizar en "dias_sumar" para avanzar en mes y anio hasta llegar al mes con la cantidad minima a sumar de dias correspondiente
            let dias_restantes = dias_mes - self.dia + 1;
            
            //Avanza en los meses y anios(si fuera necesario) hasta llegar al mes y sumar la cantidad minima de dias
            if dias_sumar >= dias_restantes as u32 {
                dias_sumar -= dias_restantes as u32;
                self.avanzar_mes();
            } else {
                //Suma la cantidad correspondiente al mes
                self.dia += dias_sumar as u8;
                //Fin de ejecucion
                dias_sumar = 0;
            }
        }

    }

    //Auxiliar para retroceder de mes y anio
    fn retroceder_mes(&mut self){
        if self.mes == 1{
            self.mes = 12;
            self.anio -= 1;
        } else {
            self.mes -= 1;
        }
        self.dia = self.ultimo_dia();
    }

    //Se considera que la fecha es valida
    //Y que no se llegara a una fecha negativa(anio negativo)
    pub fn restar_dias(&mut self, mut dias_restar:u32){
        //Bucle principal para el calculo
        while dias_restar > 0 {
            
            //Retrocede en los meses y anios(si fuera necesario) hasta llegar al mes y restar la cantidad minima de dias
            if dias_restar >= self.dia as u32 {
                dias_restar -= self.dia as u32;
                self.retroceder_mes();
            } else {
                //Resta la cantidad correspondiente al mes
                self.dia -= dias_restar as u8;
                //Fin de ejecucion
                dias_restar = 0;
            }
        }
    }

    pub fn es_mayor(&self , f:&Fecha)->bool{
        return if self.anio > f.anio {true}else 
        if (self.anio == f.anio) && (self.mes > f.mes) {true}else 
        if (self.mes == f.mes) && (self.dia > f.dia) {true}else{false};
    }

}

#[cfg(test)]
mod testing_fecha{
    use super::Fecha;

    #[test]
    fn creacion_fecha(){
        let f = Fecha::new(1, 1, 2025);
        assert_eq!(f.es_igual_a(&Fecha::new(1, 1, 2025)),true);
    }

    #[test]
    fn validacion_de_fecha(){
        let mut f = Fecha::new(1, 1, 2025);
        assert_eq!(f.es_fecha_valida(),true);
        f = Fecha::new(31, 2, 2004);
        assert_eq!(f.es_fecha_valida(),false);
    }

    #[test]
    fn validar_bisiesto(){
        let mut f = Fecha::new(1, 1, 2028);
        assert_eq!(f.es_bisiesto(),true);
        f = Fecha::new(1, 1, 2025);
        assert_eq!(f.es_bisiesto(),false);
    }

    #[test]
    fn adicion_fecha(){
        let mut f = Fecha::new(1, 1, 2028);
        f.sumar_dias(30);
        assert_eq!(f.es_igual_a(&Fecha::new(31, 1, 2028)),true);
        f.sumar_dias(1);
        assert_eq!(f.es_igual_a(&Fecha::new(1, 2, 2028)),true);
        f.sumar_dias(29);
        assert_eq!(f.es_igual_a(&Fecha::new(1,3,2028)),true);
    }

    #[test]
    fn sustraccion_fecha(){
        let mut f = Fecha::new(10, 4, 2028);
        f.restar_dias(9);
        assert_eq!(f.es_igual_a(&Fecha::new(1, 4, 2028)),true);
        f.restar_dias(31);
        assert_eq!(f.es_igual_a(&Fecha::new(1,3,2028)),true);
        f.restar_dias(1);
        assert_eq!(f.es_igual_a(&Fecha::new(29, 2, 2028)),true);
    }

    #[test]
    fn comparacion_fechas(){
        let f1 = Fecha::new(25, 5, 2000);
        let f2 = Fecha::new(25, 2, 2004);
        assert_eq!(f1.es_mayor(&f2),false);
        assert_eq!(f2.es_mayor(&f1),true);
    }

}

/***
 * 
 * 
***/


use std::fmt::{write, Display};
use std::io;
use std::{fs::{File,OpenOptions}, io::{Error,Read,Write}};
use std::path::Path;
//Se debe importar serde para su uso "cargo add serde"
use serde::{Serialize, Deserialize};
use serde_json;
/**
        EXTRACCION DEL EJERCICIO 9 - TP3
**/

/*
    Estructuras (Se entiende que para la consigna esta restringido el uso del trait PartialEq para el TP3)
*/

#[derive(Debug, Clone ,Serialize, Deserialize)]
pub enum Animales{
    Perro,
    Gato,
    Caballo,
    Otro,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Duenio {
    nombre: String,
    direccion: String,
    telefono: u32
}
#[derive(Debug, Clone , Serialize, Deserialize)]
pub struct Mascota {
    nombre: String,
    edad: u32,
    tipo: Animales,
    duenio: Duenio
}
#[derive(Debug, Clone , Serialize, Deserialize)]
pub struct Atencion {
    mascota: Mascota,
    diagnostico: String,
    tratamiento: String,
    proxima_visita: Option<Fecha>
}
#[derive(Debug, Clone , Serialize, Deserialize)]
pub struct Veterinaria {
    nombre: String,
    direccion: String,
    id: u32,
    cola_atencion: Vec<Mascota>,
    atenciones_realizadas: Vec<Atencion>
}


/*
    Metodos asociados
*/
impl Animales{
    pub fn es_igual_a(&self,a:&Animales)->bool{
        match (self, a) {
            (Animales::Perro, Animales::Perro) => true,
            (Animales::Gato, Animales::Gato) => true,
            (Animales::Caballo, Animales::Caballo) => true,
            (Animales::Otro, Animales::Otro) => true,
            _ => false
        }
    }
}
impl Duenio {
    //Metodos Secundarios
    pub fn get_nombre(&self)->String{
        return self.nombre.clone();
    }
    pub fn get_direccion(&self)->String{
        return self.direccion.clone();
    }
    pub fn es_igual_a(&self,d:&Duenio)->bool{
        return (self.nombre == d.get_nombre())&&(self.direccion == d.get_direccion())&&(self.telefono == d.telefono);
    }
    pub fn get_tel(&self)->u32{
        return self.telefono;
    }
    //Metodos Primarios
    pub fn new(nombre_in: String,direccion_in: String,telefono_in: u32) -> Duenio {
        return Duenio{
            nombre : nombre_in,
            direccion : direccion_in,
            telefono : telefono_in
        }
    }
    
}

impl Mascota {
    //Metodos secundarios
    pub fn get_nombre(&self)->String{
        return self.nombre.clone();
    }
    pub fn es_igual_a(&self,m:&Mascota)->bool{
        return (self.nombre == m.get_nombre())&&(self.edad == m.edad)&&(self.tipo.es_igual_a(&m.tipo))&&(self.duenio.es_igual_a(&m.duenio));
    }
    //Metodos primarios
    pub fn new(nombre_in: String,edad_in: u32,tipo_in: Animales,duenio_in: &Duenio) -> Mascota {
        return Mascota{
            nombre : nombre_in,
            edad : edad_in,
            tipo : tipo_in,
            duenio : duenio_in.clone()
        }
    }
}

impl Atencion {
    //Metodos secundarios
    pub fn es_igual_a(&self,ate:&Atencion)->bool{
        let mut cumple = false;
        if let Some(tiene_fecha) = &self.proxima_visita{
            if let Some(hay_fecha) = &ate.proxima_visita{
                cumple = tiene_fecha.es_igual_a(&hay_fecha)&&(self.mascota.es_igual_a(&ate.mascota.clone()))&&(self.diagnostico == ate.diagnostico)&&(self.tratamiento == ate.tratamiento);
            }
        }else{
            if ate.proxima_visita.is_none(){
                cumple = true;
            }
        }
        return (self.mascota.es_igual_a(&ate.mascota))&&(self.diagnostico == ate.diagnostico.clone())&&(self.tratamiento == ate.tratamiento.clone())&&(cumple);
    }
    pub fn cambiar_diagnostico(&mut self,diag:&String){
        self.diagnostico = diag.clone();
    }
    pub fn cambiar_fecha(&mut self,f:&Option<Fecha>){
        self.proxima_visita = f.clone();
    }
    //Metodos primarios
    pub fn new(mascota_in: &Mascota,diagnostico_in: String,tratamiento_in: String,proxima_visita_in: Option<Fecha>) -> Atencion {
        Atencion{
            mascota : mascota_in.clone(),
            diagnostico : diagnostico_in,
            tratamiento : tratamiento_in,
            proxima_visita : proxima_visita_in
        }
    }
}

impl Veterinaria{
    //Metodos secundarios
    pub fn es_igual_a(&self,v:&Veterinaria)->bool{
        return (self.nombre == v.nombre)&&(self.direccion == v.direccion)&&(self.id == v.id);
    }
    //Metodos primarios
    pub fn new(nom_in:String,dir_in:String,id_in:u32)->Veterinaria{
        return Veterinaria{
            nombre : nom_in,
            direccion : dir_in,
            id : id_in,
            cola_atencion : Vec::new(),
            atenciones_realizadas : Vec::new()
        }
    }

    //Mascotas
    pub fn agregar_mascota(&mut self,m:&Mascota){
        self.cola_atencion.push(m.clone());
    }   
    pub fn priorizar_mascota(&mut self,m:&Mascota){
        self.cola_atencion.insert(0,m.clone());
    }
    pub fn atender_mascota(&mut self)->Option<Mascota>{
        if self.cola_atencion.is_empty() {
            return None;
        }else{
            return Some(self.cola_atencion.remove(0));
        }
    }
    pub fn eliminar_mascota(&mut self, m:&Mascota){
        if !self.cola_atencion.is_empty(){
            for i in 0..self.cola_atencion.len(){
                if self.cola_atencion[i].es_igual_a(&m) {
                    self.cola_atencion.remove(i);
                    break;
                }
            }
        }
    }

    //Atenciones
    pub fn registrar_atencion(&mut self,a:&Atencion){
        self.atenciones_realizadas.push(a.clone());
    }
    pub fn buscar_atencion(&self,nom_mascota:String,nom_duenio:String,tel:u32)->Option<Atencion>{
        let mut res : Option<Atencion> = None;
        if !self.atenciones_realizadas.is_empty(){
            for ate in self.atenciones_realizadas.clone(){
                if(ate.mascota.get_nombre() == nom_mascota.clone())&&(ate.mascota.duenio.get_nombre() == nom_duenio.clone())&&(ate.mascota.duenio.telefono == tel){
                    res = Some(ate);
                    break;
                }
            }
        }
        return res;
    }
    pub fn modificar_diagnostico(&mut self,ate:&Atencion,diag:&String){
        if !self.atenciones_realizadas.is_empty(){
            for i in 0..self.atenciones_realizadas.len(){
                if self.atenciones_realizadas[i].es_igual_a(&ate){
                    self.atenciones_realizadas[i].cambiar_diagnostico(diag);
                    break;
                }
            }
        }
    }
    pub fn modificar_fecha(&mut self,ate:&Atencion,fecha: Option<Fecha>) {
        if !self.atenciones_realizadas.is_empty(){
            for i in 0..self.atenciones_realizadas.len() {
                if self.atenciones_realizadas[i].es_igual_a(&ate) {
                    self.atenciones_realizadas[i].cambiar_fecha(&fecha);
                    break;
                }
            }
        }
    }
    pub fn eliminar_atencion(&mut self,ate:&Atencion){
        if !self.atenciones_realizadas.is_empty(){
            for i in 0..self.atenciones_realizadas.len(){
                if self.atenciones_realizadas[i].es_igual_a(&ate){
                    self.atenciones_realizadas.remove(i);
                    break;
                }
            }
        }
    }
}
#[cfg(test)]
mod testing_veterinaria{
    use super::*;

    #[test]
    fn creacion_veterinaria(){
        let v = Veterinaria::new("mordidas".to_string(),"av1".to_string(),1);
        let v2 = Veterinaria::new("mordidas".to_string(),"av1".to_string(),1);
        assert_eq!(v.es_igual_a(&v2),true);
    }

    #[test]
    fn operatoria_mascotas(){
        let mut v = Veterinaria::new("mordidas".to_string(),"av1".to_string(),1);
        let d1 = Duenio::new("Marcos".to_string(),"av2".to_string(),1234);
        let animal1 = Mascota::new(String::from("Luchito"), 2, Animales::Perro, &d1);
        v.agregar_mascota(&animal1);
        v.agregar_mascota(&animal1);

        let animal2 = Mascota::new(String::from("Piecitos"), 1, Animales::Gato, &d1);
        v.priorizar_mascota(&animal2);

        //Atendiende un gato
        if let Some(ani) = v.atender_mascota(){
            assert_eq!(ani.es_igual_a(&animal2),true);
        }else{
            panic!("No se encontro el animal");
        }

        //Atendiende un perro
        if let Some(ani) = v.atender_mascota(){
            assert_eq!(ani.es_igual_a(&animal1),true);
        }else{
            panic!("No se encontro el animal");
        }

        //Borra el perro repetido(del anterior)
        v.eliminar_mascota(&animal1);
        assert_eq!(v.atender_mascota().is_none(),true);
    }

    #[test]
    fn operar_atenciones(){
        let mut v = Veterinaria::new("mordidas".to_string(),"av1".to_string(),1);
        let d1 = Duenio::new("Marcos".to_string(),"av2".to_string(),1234);
        let animal1 = Mascota::new(String::from("Luchito"), 2, Animales::Perro, &d1);
        v.agregar_mascota(&animal1);
        v.agregar_mascota(&animal1);

        let mut ate1 : Atencion;
        let mut ate2 : Atencion;
        //Primera recepcion
        if let Some(ani) = v.atender_mascota(){
            ate1 = Atencion::new(&ani,"Pulgas".to_string(),"Pipeta".to_string(),None);
            v.registrar_atencion(&ate1);
        }else{
            panic!("No se atendio a ningun animal");
        }

        //Segunda recepcion
        if let Some(ani) = v.atender_mascota(){
            ate2 = Atencion::new(&ani,"Garrapatas".to_string(),"Pipeta".to_string(),Some(Fecha::new(5,5,2025)));
            v.registrar_atencion(&ate2);
        }else{
            panic!("No se atendio a ningun animal");
        }
        
        //Busqueda y eliminacion de la primera atencion
        if let Some(ate_actual) = v.buscar_atencion("Luchito".to_string(),"Marcos".to_string(),1234){
            assert_eq!(ate_actual.es_igual_a(&ate1),true);
            v.eliminar_atencion(&ate1);
        }else{
            panic!("No se encontro tal recepcion");
        }
    
        //Busqueda de atencion
        if let Some(ate_actual) = v.buscar_atencion("Luchito".to_string(),"Marcos".to_string(),1234){
            assert_eq!(ate_actual.es_igual_a(&ate1),false);
        }else{
            panic!("No se encontro tal recepcion");
        }

        //Modificar la atencion actual(segunda recepcion)
        v.modificar_diagnostico(&ate2,&"Vomitos".to_string());

        //Busqueda de atencion y modificacion de fecha
        if let Some(ate_actual) = v.buscar_atencion("Luchito".to_string(),"Marcos".to_string(),1234){
            let ate3 = Atencion::new(&animal1,"Vomitos".to_string(),"Pipeta".to_string(),Some(Fecha::new(5,5,2025)));
            assert_eq!(ate_actual.es_igual_a(&ate3),true);
            v.modificar_fecha(&ate_actual,None);
        }else{
            panic!("No se encontro tal recepcion");
        }

        //Busqueda de atencion modificada 
        
        if let Some(ate_actual) = v.buscar_atencion("Luchito".to_string(),"Marcos".to_string(),1234){
            let ate3 = Atencion::new(&animal1,"Vomitos".to_string(),"Pipeta".to_string(),None );
            assert_eq!(ate_actual.es_igual_a(&ate3),true);
        }else{
            panic!("No se encontro tal recepcion");
        }
        
    }
}

/*
    IMPLEMENTACION DE EJERCICIO 3 - TP5 (Si sale un error de serializacion,volver a ejecutar)
*/

/*
    Tipos de errores
*/
#[derive(Debug)]
pub enum error_operatoria{
    Inexistente(String),
    EstructuraVacia(String)
}

impl Display for error_operatoria{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            error_operatoria::Inexistente(val) => write!(f, "No se encontro el elemento en la estructura {} ",val),
            error_operatoria::EstructuraVacia(val) => write!(f, "La estrucutra {} no dispone de elementos ",val)
        }
    }
}

#[derive(Debug)]
pub enum Errores{
    ErrorOperatoria(error_operatoria),
    ErrorIO(io::Error),
    ErrorSerde(serde_json::Error)
}

impl Display for Errores{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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

//Implementacion extra a la veterinaria
impl Veterinaria{
    fn to_string(&self)->String{
        return self.nombre.clone();
    }
    //Hace el proceso de atender y retornar la atencion con los datos ingresados(Sabiendo que se atiende a la 1º mascota de la cola)
    fn realizar_atencion(&mut self,diag:&String,tratam:&String,f:&Fecha)->Option<Atencion>{
        let mut res : Option<Atencion> = None;
        if !self.cola_atencion.is_empty() {
            res = if let Some(m) = self.atender_mascota(){
                Some(Atencion::new(&m,diag.clone(),tratam.clone(),Some(f.clone())))
            }else{ None }
        }
        return res;
    }

}


//Archivo de almacenamiento (Solo respalda la lista de atenciones realizadas por la veterinaria)
#[derive(Debug)]
pub struct Archivo{
    informacion : Vec<Atencion>,
    path : String,
    autoguardado : bool 
}

impl Archivo{
    fn new(dato:&Vec<Atencion>,dir:String,estado:bool)->Archivo{
        return Archivo { informacion: dato.clone(), path: dir , autoguardado : estado};
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
    
    fn registrar_atencion(&mut self,a:&Atencion)-> Result<(), Errores>{
        self.informacion.push(a.clone());

		if self.autoguardado{
			self.respaldar_informacion()?;
		}

		Ok(())
    }

    fn eliminar_atencion(&mut self,a:&Atencion)-> Result<(), Errores>{

        if !self.informacion.is_empty(){
            if let Some(pos) = self.informacion.iter().position(|ate| ate.es_igual_a(&a) ){
                self.informacion.remove(pos);
            }else{
                return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente("Historial de atenciones".to_string() )) );
            }
        }else{
            return Err(Errores::ErrorOperatoria(error_operatoria::EstructuraVacia("Historial de atenciones".to_string() )) );
        }

        if self.autoguardado{
			self.respaldar_informacion()?;
		}

        Ok(())
    }

    fn modificar_fecha_atencion(&mut self,a:&Atencion,f:&Fecha)-> Result<(), Errores>{

        if !self.informacion.is_empty(){
            if let Some(atencion) = self.informacion.iter_mut().find(|ate| ate.es_igual_a(&a) ){
                atencion.cambiar_fecha(&Some(f.clone()));
            }else{
                return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente("Historial de atenciones".to_string() )) );
            }
        }else{
            return Err(Errores::ErrorOperatoria(error_operatoria::EstructuraVacia("Historial de atenciones".to_string() )) );
        }

        if self.autoguardado{
			self.respaldar_informacion()?;
		}

        Ok(())
    }
    fn modificar_diagnostico_atencion(&mut self,a:&Atencion,diag:&String)-> Result<(), Errores>{
        
        if !self.informacion.is_empty(){
            if let Some(atencion) = self.informacion.iter_mut().find(|ate| ate.es_igual_a(&a) ){
                atencion.cambiar_diagnostico(diag);
            }else{
                return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente("Historial de atenciones".to_string() )) );
            }
        }else{
            return Err(Errores::ErrorOperatoria(error_operatoria::EstructuraVacia("Historial de atenciones".to_string() )) );
        }

        if self.autoguardado{
			self.respaldar_informacion()?;
		}

        Ok(())
    }
    //Busqueda en el archivo logico
    fn recuperar_atencion(&mut self,a:&Atencion)-> Result<Atencion, Errores>{
        if !self.informacion.is_empty(){
            if let Some(atencion) = self.informacion.iter().find(|ate| ate.es_igual_a(&a) ){
                Ok(atencion.clone())
            }else{
                return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente("Historial de atenciones".to_string() )) );
            }
        }else{
            return Err(Errores::ErrorOperatoria(error_operatoria::EstructuraVacia("Historial de atenciones".to_string() )) );
        }
    }
    //Busqueda en el archivo fisico
    fn rescatar_informacion_fisica(&self,a:&Atencion)-> Result<Atencion,Errores>{
        //Apertura(Debe existir el archivo fisico)
        let mut file = File::open(self.path.clone())?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let atenciones : Vec<Atencion> = serde_json::from_str(&buf)?;

        //Busqueda
        if !atenciones.is_empty(){
            if let Some(atencion) = self.informacion.iter().find(|ate| ate.es_igual_a(&a) ){
                Ok(atencion.clone())
            }else{
                return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente("Historial de atenciones".to_string() )) );
            }
        }else{
            return Err(Errores::ErrorOperatoria(error_operatoria::EstructuraVacia("Historial de atenciones".to_string() )) );
        } 
    }
}

#[cfg(test)]
mod testing_implementacion_ejercicio3{
    use super::*;

    #[test]
    fn operatoria_informacion(){
        //Veterinaria
        let mut v = Veterinaria::new("mordidas".to_string(),"av1".to_string(),1);
        let d1 = Duenio::new("Marcos".to_string(),"av2".to_string(),1234);
        let animal1 = Mascota::new(String::from("Luchito"), 2, Animales::Perro, &d1);
        let animal2 = Mascota::new(String::from("Lupe"), 1, Animales::Gato, &d1);
        v.agregar_mascota(&animal1);
        v.agregar_mascota(&animal2);

        //Atenciones

        //Se atiende animal1 (desde la veterinaria)
        if let Some(a) = v.realizar_atencion(&"Fiebre".to_string(), &"Inyecciones".to_string(), &Fecha::new(12, 5, 2025) ){
            v.registrar_atencion(&a);
        }else{
            println!("error: {}", Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(String::from("Cola de atencion")))); assert!(false);
            assert!(false);
        }
        
        
        //Se agrega la informacion al archivo (todavia no se guarda en el archivo fisico)
        let mut archivo1 = Archivo::new(&v.atenciones_realizadas, "".to_string(),false);

        //Se atiende animal2 (desde la veterinaria y se registra en el archivo)
        if let Some(a) = v.realizar_atencion(&"Pulgas".to_string(), &"Pipeta".to_string(), &Fecha::new(12, 8, 2025) ){
            v.registrar_atencion(&a);
            let r = archivo1.registrar_atencion(&a);
            match r {
                Ok(_) => assert!(true),
                Err(e) => {println!("error: {}", e); assert!(false);}   //Solo puede ocurrir el error si se realiza un guardado en el archivo fisico
            }
        }else{
            println!("error: {}", Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(String::from("Cola de atencion")))); assert!(false);
            assert!(false);
        }


        //Modificacion de atenciones
        if let Some(mut a) = v.buscar_atencion(animal1.get_nombre(),d1.get_nombre(),d1.get_tel()){
            //Cambio de fecha
            let r = archivo1.modificar_fecha_atencion(&a,&Fecha::new(2, 1, 2025));
            match r {
                Ok(_) => assert!(true),
                Err(e) => {println!("error: {}", e); assert!(false);}  
            }
            a.cambiar_fecha(&Some(Fecha::new(2, 1, 2025))); //Modifico la fecha de la veterina para continuar en el archivo logico
            //Cambio de diagnostico
            let r = archivo1.modificar_diagnostico_atencion(&a,&"Tos".to_string());
            match r {
                Ok(_) => assert!(true),
                Err(e) => {println!("error: {}", e); assert!(false);}  
            }
            a.cambiar_diagnostico(&"Tos".to_string()); //Lo mismo de arriba pero con la seccion de diagnostico
        }else{
            println!("error: {}", Errores::ErrorOperatoria(error_operatoria::Inexistente(String::from("Cola de atencion")))); assert!(false);
            assert!(false);
        }


        //Busqueda
        if let Some(a) = v.buscar_atencion(animal2.get_nombre(),d1.get_nombre(),d1.get_tel()){
            //Busqueda en el archivo logico
            let r = archivo1.recuperar_atencion(&a);
            match r {
                Ok(ate) => assert!(ate.es_igual_a(&a)),
                Err(e) => {println!("error: {}", e); assert!(false);}  
            }
        }else{
            println!("error: {}", Errores::ErrorOperatoria(error_operatoria::Inexistente(String::from("Cola de atencion")))); assert!(false);
            assert!(false);
        }

        //Baja
        if let Some(a) = v.buscar_atencion(animal2.get_nombre(),d1.get_nombre(),d1.get_tel()){
            //Busqueda en el archivo logico
            let r = archivo1.eliminar_atencion(&a);
            match r {
                Ok(_) => assert!(true),
                Err(e) => {println!("error: {}", e); assert!(false);}  
            }
        }else{
            println!("error: {}", Errores::ErrorOperatoria(error_operatoria::Inexistente(String::from("Cola de atencion")))); assert!(false);
            assert!(false);
        }

    }

    #[test]
    fn operatoria_archivo_sin_autoguardado(){
        //Veterinaria
        let mut v = Veterinaria::new("mordidas".to_string(),"av1".to_string(),1);
        let d1 = Duenio::new("Marcos".to_string(),"av2".to_string(),1234);
        let animal1 = Mascota::new(String::from("Luchito"), 2, Animales::Perro, &d1);
        let animal2 = Mascota::new(String::from("Lupe"), 1, Animales::Gato, &d1);
        v.agregar_mascota(&animal1);
        v.agregar_mascota(&animal2);

        //Atenciones
        // 1ª atencion
        if let Some(a) = v.realizar_atencion(&"Fiebre".to_string(), &"Inyecciones".to_string(), &Fecha::new(12, 5, 2025) ){
            v.registrar_atencion(&a);
        }else{
            println!("error: {}", Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(String::from("Cola de atencion")))); assert!(false);
            assert!(false);
        }


        //Se agrega la informacion al archivo
        let mut archivo1 = Archivo::new(&v.atenciones_realizadas, "src/tp5/cola_atencion_info.json".to_string(),false);

        //2º atencion
        if let Some(a) = v.realizar_atencion(&"Fiebre".to_string(), &"Inyecciones".to_string(), &Fecha::new(21, 6, 2025) ){
            v.registrar_atencion(&a);
            let r = archivo1.registrar_atencion(&a);
            match r {
                    Ok(_) => assert!(true),
                    Err(e) => {println!("error: {}", e); assert!(false);}  
            }
        }else{
            println!("error: {}", Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(String::from("Cola de atencion")))); assert!(false);
            assert!(false);
        }

        //Se guarda la informacion en el archivo fisico
        let r = archivo1.respaldar_informacion();
        match r {
            Ok(_) => assert!(true),
            Err(e) => {println!("error: {}", e); assert!(false);}  
        }


        //Busqueda 
        if let Some(a) = v.buscar_atencion(animal1.get_nombre(),d1.get_nombre(),d1.get_tel()){
            let r = archivo1.rescatar_informacion_fisica(&a);
            match r {
                Ok(ate) => assert!(ate.es_igual_a(&a)),
                Err(e) => {println!("error: {}", e); assert!(false);}  
            }
        }else{
            println!("error: {}", Errores::ErrorOperatoria(error_operatoria::Inexistente(String::from("Cola de atencion")))); assert!(false);
            assert!(false);
        }

        //Baja (2º elemento)
        if let Some(a) = v.buscar_atencion(animal2.get_nombre(),d1.get_nombre(),d1.get_tel()){
            let r = archivo1.eliminar_atencion(&a);
            match r {
                Ok(_) => assert!(true),
                Err(e) => {println!("error: {}", e); assert!(false);}  
            }
            //Se actualiza el archivo fisico
            let r = archivo1.respaldar_informacion();
            match r {
                Ok(_) => assert!(true),
                Err(e) => {println!("error: {}", e); assert!(false);}  
            }
        }else{
            println!("error: {}", Errores::ErrorOperatoria(error_operatoria::Inexistente(String::from("Cola de atencion")))); assert!(false);
            assert!(false);
        }

        //Modificacion del archivo
        if let Some(mut a) = v.buscar_atencion(animal1.get_nombre(),d1.get_nombre(),d1.get_tel()){
            
            //Cambio de fecha
            let r = archivo1.modificar_fecha_atencion(&a,&Fecha::new(1,1,2025));
            a.cambiar_fecha(&Some(Fecha::new(1,1,2025)) );
            match r {
                Ok(_) => assert!(true),
                Err(e) => {println!("error: {}", e); assert!(false);}  
            }

            //Cambio de diagnostico
            let r = archivo1.modificar_diagnostico_atencion(&a,&"Parasitos".to_string());
            match r {
                Ok(_) => assert!(true),
                Err(e) => {println!("error: {}", e); assert!(false);}  
            }

            //Se actualiza el archivo fisico
            let r = archivo1.respaldar_informacion();
            match r {
                Ok(_) => assert!(true),
                Err(e) => {println!("error: {}", e); assert!(false);}  
            }
        }else{
            println!("error: {}", Errores::ErrorOperatoria(error_operatoria::Inexistente(String::from("Cola de atencion")))); assert!(false);
            assert!(false);
        }

    }

    #[test]
    fn operatoria_archivo_con_autoguardado(){
        //Veterinaria
        let mut v = Veterinaria::new("mordidas".to_string(),"av1".to_string(),1);
        let d1 = Duenio::new("Marcos".to_string(),"av2".to_string(),1234);
        let animal1 = Mascota::new(String::from("Luchito"), 2, Animales::Perro, &d1);
        let animal2 = Mascota::new(String::from("Lupe"), 1, Animales::Gato, &d1);
        v.agregar_mascota(&animal1);
        v.agregar_mascota(&animal2);

        //Atenciones
        // 1ª atencion
        if let Some(a) = v.realizar_atencion(&"Garrapatas".to_string(), &"Pastillas".to_string(), &Fecha::new(22, 5, 2025) ){
            v.registrar_atencion(&a);
        }else{
            println!("error: {}", Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(String::from("Cola de atencion")))); assert!(false);
            assert!(false);
        }

        //Se agrega la informacion al archivo
        let mut archivo1 = Archivo::new(&v.atenciones_realizadas, "src/tp5/cola_atencion_info.json".to_string(),true);

        //2º atencion
        if let Some(a) = v.realizar_atencion(&"Pulgas".to_string(), &"Pipeta".to_string(), &Fecha::new(25, 6, 2025) ){
            v.registrar_atencion(&a);
            let r = archivo1.registrar_atencion(&a);
            match r {
                    Ok(_) => assert!(true),
                    Err(e) => {println!("error: {}", e); assert!(false);}  
            }
        }else{
            println!("error: {}", Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(String::from("Cola de atencion")))); assert!(false);
            assert!(false);
        }


        //Busqueda 
        if let Some(a) = v.buscar_atencion(animal1.get_nombre(),d1.get_nombre(),d1.get_tel()){
            let r = archivo1.rescatar_informacion_fisica(&a);
            match r {
                Ok(ate) => assert!(ate.es_igual_a(&a)),
                Err(e) => {println!("error: {}", e); assert!(false);}  
            }
        }else{
            println!("error: {}", Errores::ErrorOperatoria(error_operatoria::Inexistente(String::from("Cola de atencion")))); assert!(false);
            assert!(false);
        }
        

        //Baja (1º elemento)
        if let Some(a) = v.buscar_atencion(animal1.get_nombre(),d1.get_nombre(),d1.get_tel()){
            let r = archivo1.eliminar_atencion(&a);
            match r {
                Ok(_) => assert!(true),
                Err(e) => {println!("error: {}", e); assert!(false);}  
            }
        }else{
            println!("error: {}", Errores::ErrorOperatoria(error_operatoria::Inexistente(String::from("Cola de atencion")))); assert!(false);
            assert!(false);
        }      


        //Modificacion del archivo
        if let Some(mut a) = v.buscar_atencion(animal2.get_nombre(),d1.get_nombre(),d1.get_tel()){
            
            //Cambio de fecha
            let r = archivo1.modificar_fecha_atencion(&a,&Fecha::new(18,9,2025));
            a.cambiar_fecha(&Some(Fecha::new(18,9,2025)) );
            match r {
                Ok(_) => assert!(true),
                Err(e) => {println!("error: {}", e); assert!(false);}  
            }

            //Cambio de diagnostico
            let r = archivo1.modificar_diagnostico_atencion(&a,&"Parasitos".to_string());
            match r {
                Ok(_) => assert!(true),
                Err(e) => {println!("error: {}", e); assert!(false);}  
            }
            
        }else{
            println!("error: {}", Errores::ErrorOperatoria(error_operatoria::Inexistente(String::from("Cola de atencion")))); assert!(false);
            assert!(false);
        }  
    }

}

