use crate::tp5::ej3Fecha::Fecha;
use std::fmt::{write, Display};
use std::io;
use std::{fs::{File,OpenOptions}, io::{Error,Read,Write}};
use std::path::Path;
//Se debe importar serde para su uso "cargo add serde"
use serde::{Serialize, Deserialize};
use serde_json;
/**
        EXTRACCION DEL EJERCICIO 10 - TP3 (Se comprende que esta restringido el uso del trait PartialEq)
**/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Genero {
    Novela,
    Infantil,
    Tecnico,
    Otro,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Estado {
    EnPrestamo,
    Devuelto
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Libro { 
    isbn : u32,
    titulo: String,
    autor: String,
    paginas: u32,
    genero: Genero
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibrosDispone {
    libro: Libro,
    cantidad: u32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cliente { 
    nombre: String,
    telefono: u32,
    correo: String
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prestamo {
    libro: Libro,
    cliente: Cliente,
    vencimiento: Fecha,
    estado: Estado,
    devolucion: Option<Fecha>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Biblioteca {
    nombre: String,
    direccion: String,
    disponibles: Vec<LibrosDispone>,
    prestamos: Vec<Prestamo>
}
impl Estado{
    pub fn es_igual_a(&self,e:&Estado)->bool{
        match (self, e) {
            (Estado::EnPrestamo, Estado::EnPrestamo) => true,
            (Estado::Devuelto, Estado::Devuelto) => true,
            _ => false
        }
    }
}

impl Genero{
    pub fn es_igual_a(&self,gen_in:&Genero)->bool{
        match (self, gen_in) {
            (Genero::Novela, Genero::Novela) => true,
            (Genero::Infantil, Genero::Infantil) => true,
            (Genero::Tecnico, Genero::Tecnico) => true,
            (Genero::Otro, Genero::Otro) => true,
            _ => false
        }
    }
}

impl Libro {
    pub fn get_titulo(&self)->String{
        return self.titulo.clone();
    }
    pub fn get_autor(&self)->String{
        return self.autor.clone();
    }
    pub fn es_igual_a(&self,l:&Libro)->bool{
        return (self.isbn == l.isbn)&&(self.titulo == l.get_titulo())&&(self.autor == l.get_autor())&&(self.paginas == l.paginas)&&(self.genero.es_igual_a(&l.genero));
    }
    pub fn new(num : u32 ,ti: String,au: String,pag: u32,gen_in: Genero) -> Libro {
        return Libro{
            isbn : num,
            titulo : ti,
            autor : au,
            paginas : pag,
            genero : gen_in
        }
    }
}

impl Cliente {
    pub fn get_nombre(&self)->String{
        return self.nombre.clone();
    }
    pub fn get_correo(&self)->String{
        return self.correo.clone();
    }
    pub fn es_igual_a(&self,c:&Cliente)->bool{
        return (self.nombre == c.get_nombre())&&(self.telefono == c.telefono)&&(self.correo == c.get_correo());
    }
    pub fn new(nom: String,tel: u32,cor: String) -> Cliente {
        return Cliente{
            nombre : nom,
            telefono : tel,
            correo : cor,
        }
    }
}

impl Prestamo {
    pub fn new(libro: Libro,cliente: Cliente,vencimiento: Fecha) -> Prestamo {
        return Prestamo{
            libro,
            cliente,
            vencimiento,
            estado:Estado::EnPrestamo,
            devolucion:None
        }
    }
}

impl Biblioteca {
    pub fn get_direccion(&self)->String{
        return self.direccion.clone();
    }
    pub fn get_nombre(&self)->String{
        return self.nombre.clone();
    }
    pub fn es_igual_a(&self,b:&Biblioteca)->bool{
        return (self.nombre == b.get_nombre())&&(self.direccion == b.get_direccion());
    }
    pub fn new(nombre: String,direccion: String) -> Biblioteca {
        let disponibles = Vec::new();
        let prestamos =  Vec::new();
        return Biblioteca{
            nombre,
            direccion,
            disponibles,
            prestamos,
        }
    }

    //Agregar libros en biblioteca
    pub fn agregar_libro(&mut self,libro:Libro,cantidad:u32) {
        self.disponibles.push(LibrosDispone{libro,cantidad});
    }

    pub fn copias(&self,libro:&Libro) -> u32 {
        let mut copias = 0;
        for libro_biblo in &self.disponibles {
            if libro_biblo.libro.es_igual_a(&libro) {
                copias = libro_biblo.cantidad;
            }
        }
        return copias
    }

    pub fn decrementar(&mut self,libro:&Libro) {
        for i in 0..self.disponibles.len(){
            if self.disponibles[i].libro.es_igual_a(&libro)&&(self.disponibles[i].cantidad > 0) {
                self.disponibles[i].cantidad -=1;
            }
        }
    }
    pub fn incrementar(&mut self,libro:&Libro) {
        for i in 0..self.disponibles.len(){
            if self.disponibles[i].libro.es_igual_a(&libro) {
                self.disponibles[i].cantidad +=1;
            }
        }
    }
    pub fn prestamos(&self,cliente:&Cliente) -> u32 {
        let mut cantidad = 0;
        for prestamo in &self.prestamos {
            if (prestamo.cliente.es_igual_a(&cliente)) && (prestamo.estado.es_igual_a(&Estado::EnPrestamo)) {
                cantidad = cantidad + 1;
            }
        }
        return cantidad;
    }

    pub fn prestar(&mut self,cliente:Cliente,libro:&Libro,vencimiento:Fecha) -> bool {
        if (self.copias(&libro)>0) && (self.prestamos(&cliente)<=5) {
            self.prestamos.push(Prestamo::new(libro.clone(), cliente, vencimiento));
            self.decrementar(&libro.clone());
            return true
        } else {
            return false
        }
    }
    
    //Parametro auxiliar de fecha para el calculo de proximidad
    pub fn vencimientos_proximos(&self,f:&Fecha,dias:u32) -> Vec<Prestamo> {
        let mut fecha = f.clone();
        fecha.sumar_dias(dias);
        let mut prestamos: Vec<Prestamo> = Vec::new();
        for prestamo in &self.prestamos {
            if (fecha.es_mayor(&prestamo.vencimiento)) && (prestamo.estado.es_igual_a(&Estado::EnPrestamo)) {
                prestamos.push(prestamo.clone());
            }
        }

        return prestamos
    }

    //Reutiliza el metodo de arriba
    pub fn prestamos_vencidos(&self,f:&Fecha) -> Vec<Prestamo> {
        return self.vencimientos_proximos(&f,0);
    }

    fn buscar(&self,libro:&Libro,cliente:&Cliente) -> Option<Prestamo> {
        let mut res = None;
        for prestamo in &self.prestamos {
            if (prestamo.cliente.es_igual_a(&cliente)) && (prestamo.libro.es_igual_a(&libro)) {
                res = Some(prestamo.clone());
                break;
            }
        }
        return res
    }

    fn devolver(&mut self,f:&Fecha,libro:&Libro,cliente:&Cliente) {
        let mut pude = false;
        for prestamo in &mut self.prestamos {
            if (prestamo.cliente.es_igual_a(&cliente)) && (prestamo.libro.es_igual_a(&libro)) {
                prestamo.estado = Estado::Devuelto;
                //Se utiliza una fecha definida para la prueba
                prestamo.devolucion = Some(f.clone());
                pude = true;
            }
        }
        if pude {self.incrementar(&libro.clone());}
    }
}

#[cfg(test)]
mod biblioteca_tests {
    use super::*;

    #[test]
    fn libros_biblioteca(){
        let nombre = String::from("Silencio");
        let direccion = String::from("1 e 2 y 3");
       
        let mut biblioteca = Biblioteca::new(nombre,direccion);
        assert_eq!(biblioteca.es_igual_a(&Biblioteca::new("Silencio".to_string(),"1 e 2 y 3".to_string())),true);

        let libro1 = Libro::new(10, "Autor1".to_string(), "Libro1".to_string(), 50 , Genero::Infantil);
        let libro2 = Libro::new(20, "Autor2".to_string(), "Libro2".to_string(), 50 , Genero::Novela);
        let libro3 = Libro::new(30, "Autor3".to_string(), "Libro3".to_string(), 50 , Genero::Tecnico);
        let libro4 = Libro::new(40, "Autor4".to_string(), "Libro4".to_string(), 50 , Genero::Otro);

        biblioteca.agregar_libro(libro1.clone(),0);
        biblioteca.incrementar(&libro1);
        biblioteca.agregar_libro(libro2.clone(),3);
        biblioteca.decrementar(&libro2);
        biblioteca.agregar_libro(libro3.clone(),3);
        biblioteca.agregar_libro(libro4.clone(),4);

        assert_eq!(biblioteca.copias(&libro1),1);
        assert_eq!(biblioteca.copias(&libro2),2);
    }

    #[test]
    fn operatoria_prestamos() {
        let nombre = String::from("Silencio");
        let direccion = String::from("1 e 2 y 3");
       
        let mut biblioteca = Biblioteca::new(nombre,direccion);
        
        //Clientela
        let cliente1 = Cliente::new("Persona1".to_string(),1,"Carlos.com".to_string());
        let cliente2 = Cliente::new("Persona2".to_string(),2,"Mateo.com".to_string());
        let cliente3 = Cliente::new("Persona3".to_string(),3,"Juan.com".to_string());
        let cliente4 = Cliente::new("Persona4".to_string(),4,"Pedro.com".to_string());

        //Libros
        let libro1 = Libro::new(10, "Autor1".to_string(), "Libro1".to_string(), 50 , Genero::Infantil);
        let libro2 = Libro::new(20, "Autor2".to_string(), "Libro2".to_string(), 50 , Genero::Novela);
        let libro3 = Libro::new(30, "Autor3".to_string(), "Libro3".to_string(), 50 , Genero::Tecnico);
        let libro4 = Libro::new(40, "Autor4".to_string(), "Libro4".to_string(), 50 , Genero::Otro);

        biblioteca.agregar_libro(libro1.clone(),5);
        biblioteca.agregar_libro(libro2.clone(),5);
        biblioteca.agregar_libro(libro3.clone(),3);
        biblioteca.agregar_libro(libro4.clone(),4);

        //Operacion de los prestamos
        let mut ayer = Fecha::new(14, 5, 2025);
        let mut quince_dias = ayer.clone();
        quince_dias.sumar_dias(15);
        ayer.restar_dias(1);
        let mut nunca = ayer.clone();
        nunca.sumar_dias(99999);    //Fecha maxima de ejemplo

        biblioteca.prestar(cliente1.clone(), &libro1, quince_dias.clone());
        biblioteca.prestar(cliente2.clone(), &libro2, quince_dias.clone());
        biblioteca.prestar(cliente3.clone(), &libro3, nunca.clone());
        biblioteca.prestar(cliente4.clone(), &libro1, nunca.clone());
        biblioteca.prestar(cliente4.clone(), &libro2, nunca.clone());
        biblioteca.prestar(cliente4.clone(), &libro3, ayer.clone());
        biblioteca.prestar(cliente4.clone(), &libro4, ayer.clone());
        assert_eq!(biblioteca.prestamos(&cliente1),1);
        assert_eq!(biblioteca.prestamos(&cliente4),4);

        //Copias prestadas en total de "libro1"
        assert_eq!(biblioteca.copias(&libro1),3); 

        //Fecha a operar
        let act = Fecha::new(15, 5, 2025);
        
        if let Some(pres) = biblioteca.buscar(&libro1, &cliente1){
            assert_eq!(pres.estado.es_igual_a(&Estado::EnPrestamo), true);
        }else{
            panic!("No se registro el prestamo");
        }

        //Contabiliza los prestamos de ayer y de 15 dias(definido arriba)
        assert_eq!(biblioteca.vencimientos_proximos(&act,20).len(),4);
        
        //Contabiliza los prestamos de ayer
        assert_eq!(biblioteca.prestamos_vencidos(&act).len(),2);

        //Contabiliza las copias de "libro1"
        biblioteca.devolver(&act,&libro1, &cliente1);
        assert_eq!(biblioteca.disponibles[0].cantidad,4);
        
        let prestamo = biblioteca.prestamos[0].clone();
        assert_eq!(prestamo.estado.es_igual_a(&Estado::Devuelto), true);
        
        if let Some(f) = prestamo.devolucion{
            assert_eq!(f.es_igual_a(&act),true);
        }else{
            panic!("No se registro la devolucion");
        }
    }
}

/*
    Implementacion EJ4-TP5
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

//Implementacion extra para la biblioteca
impl Biblioteca{
    fn get_libros_displonibles(&self)->Vec<LibrosDispone>{
        return self.disponibles.clone();
    }
}
//Archivo de almacenamiento (Solo respalda el repositorio de libros y el listado de prestamos)
#[derive(Debug)]
pub struct Archivo<T>{
    informacion : Vec<T>,
    path : String,
    autoguardado : bool 
}

//Implementacion generica
impl<T:Clone + Serialize> Archivo<T>{
    fn new(dato:&Vec<T>,dir:String,estado:bool)->Archivo<T>{
        return Archivo { informacion: dato.clone(), path: dir , autoguardado : estado};
    }
    fn existe_archivo(&self)->bool{
        return Path::new(&self.path.clone()).exists();
    }
    fn set_informacion(&mut self,datos:&Vec<T>)-> Result<(), Errores>{
        self.informacion = datos.clone();
        
        if self.autoguardado{
			self.respaldar_informacion()?;
		}

		Ok(())
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
}

//Implemetacion para el repositorio de libros
//impl Archivo<LibrosDispone>{

//}


//La opcion de autoguardado se mantiene como activa a lo largo de los testing

#[cfg(test)]
mod testing_implementacion_ejercicio4{
    use super::*;

    #[test]
    fn operatoria_archivo_repositorio_libros(){
        //Creacion de biblioteca
        let nombre = String::from("Sabiondo");
        let direccion = String::from("1 e 2 y 3");
       
        let mut biblioteca = Biblioteca::new(nombre,direccion);

        let libro1 = Libro::new(10, "Autor1".to_string(), "Libro1".to_string(), 50 , Genero::Infantil);
        let libro2 = Libro::new(20, "Autor2".to_string(), "Libro2".to_string(), 50 , Genero::Novela);
        let libro3 = Libro::new(30, "Autor3".to_string(), "Libro3".to_string(), 50 , Genero::Tecnico);
        let libro4 = Libro::new(40, "Autor4".to_string(), "Libro4".to_string(), 50 , Genero::Otro);

        biblioteca.agregar_libro(libro1.clone(),10);
        biblioteca.agregar_libro(libro2.clone(),10);
        biblioteca.agregar_libro(libro3.clone(),10);
        biblioteca.agregar_libro(libro4.clone(),10);

        //Creacion del archivo repositorio
        let mut archivo1 = Archivo::new(&biblioteca.get_libros_displonibles(), "src/tp5/repositorio_libros.json".to_string(),true);
        let r = archivo1.respaldar_informacion();
        match r {
            Ok(_) => assert!(true),
            Err(e) => {println!("error: {}",e); assert!(false)}
        }
    }

}
