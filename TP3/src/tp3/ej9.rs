use crate::tp3::ej3::Fecha;

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
        return (self.nombre == m.get_nombre())&&(self.edad == m.edad)&&(self.tipo.es_igual_a(m.tipo))&&(self.duenio.es_igual_a(m.duenio));
    }
    //Metodos primarios
    pub fn new(nombre_in: String,edad_in: u32,tipo_in: Animales,duenio_in: Duenio) -> Mascota {
        return Mascota{
            nombre : nombre_in,
            edad : edad_in,
            tipo : tipo_in,
            duenio : duenio_in
        }
    }
}

impl Atencion {
    pub fn new(mascota_in: Mascota,diagnostico_in: String,tratamiento_in: String,proxima_visita_in: Option<Fecha>) -> Atencion {
        Atencion{
            mascota : mascota_in,
            diagnostico : diagnostico_in,
            tratamiento : tratamiento_in,
            proxima_visita : proxima_visita_in
        }
    }
}

impl Veterinaria{
    pub fn new(nom_in:String,dir_in:String,id_in:u32)->Veterinaria{
        return Veterinaria{
            nombre : nom_in,
            direccion : dir_in,
            id : id_in,
            cola_atencion : Vec::new(),
            atenciones_realizadas : Vec::new()
        }
    }
    pub fn agregar_mascota(&mut self,m:Mascota){
        self.cola_atencion.push(m.clone());
    }   
    pub fn priorizar_mascota(&mut self,m:Mascota){
        self.cola_atencion.insert(0,m.clone());
    }
    pub fn atender_mascota(&mut self)->Option<Mascota>{
        if self.cola_atencion.is_empty() {
            return None;
        }else{
            return Some(self.cola_atencion.remove(0));
        }
    }
}