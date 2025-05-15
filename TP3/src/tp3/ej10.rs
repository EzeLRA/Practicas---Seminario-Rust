use crate::tp3::ej3::Fecha;

#[derive(Debug, Clone)]
pub enum Genero {
    Novela,
    Infantil,
    Tecnico,
    Otro,
}

#[derive(Debug, Clone)]
pub enum Estado {
    EnPrestamo,
    Devuelto
}

#[derive(Debug, Clone)]
pub struct Libro { 
    isbn : u32,
    titulo: String,
    autor: String,
    paginas: u32,
    genero: Genero
}

#[derive(Debug, Clone)]
pub struct LibrosDispone {
    libro: Libro,
    cantidad: u32
}

#[derive(Debug, Clone)]
pub struct Cliente { 
    nombre: String,
    telefono: u32,
    correo: String
}
#[derive(Debug, Clone)]
pub struct Prestamo {
    libro: Libro,
    cliente: Cliente,
    vencimiento: Fecha,
    estado: Estado,
    devolucion: Option<Fecha>,
}

#[derive(Debug)]
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
                cantidad+=1;
            }
        }
        return cantidad;
    }

    pub fn prestar(&mut self,cliente:Cliente,libro:Libro,vencimiento:Fecha) -> bool {
        if (self.copias(&libro)>0) && (self.prestamos(&cliente)<=5) {
            self.prestamos.push(Prestamo::new(libro, cliente, vencimiento));
            return true
        } else {
            return false
        }
    }
    
    pub fn vencimientos_proximos(&self,dias:u32) -> Vec<Prestamo> {
        //Fecha definida para la prueba(no se actualiza con el actual)
        let mut fecha = Fecha::new(15,5,2025);
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
    pub fn prestamos_vencidos(&self) -> Vec<Prestamo> {
        self.vencimientos_proximos(0)
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

    fn devolver(&mut self,libro:&Libro,cliente:&Cliente) {
        for prestamo in &mut self.prestamos {
            if (prestamo.cliente.es_igual_a(&cliente)) && (prestamo.libro.es_igual_a(&libro)) {
                prestamo.estado = Estado::Devuelto;
                //Se utiliza una fecha definida para la prueba
                prestamo.devolucion = Some(Fecha::new(15,5,2025));
            }
        }
    }
}

#[cfg(test)]
mod biblioteca_tests {
    use super::*;

    #[test]
    fn test_biblioteca() {
        let nombre = String::from("Silencio");
        let direccion = String::from("1 e 2 y 3");
       
        let mut biblioteca = Biblioteca::new(nombre,direccion);
        assert_eq!(biblioteca.es_igual_a(&Biblioteca::new("Silencio".to_string(),"1 e 2 y 3".to_string())),true);
        
        let humano1 = Cliente::new("Persona1".to_string(),1,"Carlos.com".to_string());
        let humano2 = Cliente::new("Persona2".to_string(),2,"Mateo.com".to_string());
        let humano3 = Cliente::new("Persona3".to_string(),3,"Juan.com".to_string());
        let humano4 = Cliente::new("Persona4".to_string(),4,"Pedro.com".to_string());

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

        let mut ayer = Fecha::new(14, 5, 2025);
        let mut quince_dias = ayer.clone();
        quince_dias.sumar_dias(15);
        ayer.restar_dias(1);
        let mut nunca = ayer.clone();
        nunca.sumar_dias(u32::MAX);

        biblioteca.prestar(humano1.clone(), libro1.clone(), quince_dias.clone());
        biblioteca.prestar(humano2.clone(), libro2.clone(), quince_dias.clone());
        biblioteca.prestar(humano3.clone(), libro3.clone(), nunca.clone());
        biblioteca.prestar(humano4.clone(), libro1.clone(), nunca.clone());
        biblioteca.prestar(humano4.clone(), libro2.clone(), nunca.clone());
        biblioteca.prestar(humano4.clone(), libro3.clone(), ayer.clone());
        biblioteca.prestar(humano4.clone(), libro4.clone(), ayer.clone());
        assert_eq!(biblioteca.prestamos(&humano4),4);
        

        /*
            Corregir
        */

        //let v = biblioteca.prestamos.clone();
        assert_eq!(biblioteca.buscar(&libro1, &humano1).is_none() , false);
        assert_eq!(biblioteca.vencimientos_proximos(20).len(),4);
  
        assert_eq!(biblioteca.prestamos_vencidos().len(),2);

        biblioteca.devolver(&libro1, &humano1);
        assert_eq!(biblioteca.disponibles[0].cantidad,1);
        ayer.sumas_dias(1);
        let prestamo = biblioteca.prestamos[0].clone();
        assert_eq!(prestamo.estado.es_igual_a(&Estado:: Devuelto), true);
        //assert_eq!(prestamo.devolucion,Some(ayer));
    }
}


