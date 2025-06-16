use std::{fs::File, io::{Error, Read, Write}};
use serde::{Deserialize, Serialize};
use crate::tp5::ej3Fecha::Fecha;

/**
        EXTRACCION DEL EJERCICIO 9 - TP3
**/

/*
    Estructuras
*/

#[derive(Debug, Clone)]
pub enum Animales{
    Perro,
    Gato,
    Caballo,
    Otro,
}
#[derive(Debug, Clone)]
pub struct Duenio {
    nombre: String,
    direccion: String,
    telefono: u32
}
#[derive(Debug, Clone)]
pub struct Mascota {
    nombre: String,
    edad: u32,
    tipo: Animales,
    duenio: Duenio
}
#[derive(Debug, Clone)]
pub struct Atencion {
    mascota: Mascota,
    diagnostico: String,
    tratamiento: String,
    proxima_visita: Option<Fecha>
}
#[derive(Debug, Clone)]
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
                cumple = tiene_fecha.es_igual_a(&hay_fecha);
            }
        }else{
            if ate.proxima_visita.is_none(){
                cumple = true;
            }
        }
        return (self.mascota.es_igual_a(&ate.mascota))&&(self.diagnostico == ate.diagnostico.clone())&&(self.tratamiento == ate.tratamiento.clone())&&(cumple);
    }
    pub fn cambiar_diagnostico(&mut self,diag:String){
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
    pub fn modificar_diagnostico(&mut self,ate:&Atencion,diag:String){
        if !self.atenciones_realizadas.is_empty(){
            for i in 0..self.atenciones_realizadas.len(){
                if self.atenciones_realizadas[i].es_igual_a(&ate){
                    self.atenciones_realizadas[i].cambiar_diagnostico(diag.clone());
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
mod testing_playlist{
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
        v.modificar_diagnostico(&ate2,"Vomitos".to_string());

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
