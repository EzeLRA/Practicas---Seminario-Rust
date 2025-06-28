use std::fmt::{write, Display};
use std::fs::{File,OpenOptions};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::path::Path;
use serde_json;
/**
        EXTRACCION DEL EJERCICIO 8 DE TP3
**/
/*
    Estructuras : Cancion , Generos y PlayList
*/
#[derive(Debug,Clone,Serialize, Deserialize )]
pub enum Generos{
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros
}
#[derive(Debug,Clone,Serialize, Deserialize )]
pub struct Cancion{
    titulo : String,
    artista : String,
    genero : Generos
}
#[derive(Debug,Clone,Serialize, Deserialize )]
pub struct PlayList{
    nombre: String,
    canciones : Vec<Cancion>
}

/*
    Metodos asociados
*/

//Metodos para Cancion
impl Generos{
    pub fn es_igual_a(&self,g:&Generos)->bool{
        match (self, g) {
            (Generos::Rock, Generos::Rock) => true,
            (Generos::Pop, Generos::Pop) => true,
            (Generos::Rap, Generos::Rap) => true,
            (Generos::Jazz, Generos::Jazz) => true,
            (Generos::Otros, Generos::Otros) => true,
            _ => false
        }
    }
}
impl Cancion{
    //Metodos secundarios
    pub fn get_titulo(&self)->String{
        return self.titulo.clone();
    }
    pub fn get_artista(&self)->String{
        return self.artista.clone();
    }
    pub fn es_igual_a(&self,c:&Cancion)->bool{
        return (self.titulo == c.get_titulo())&&(self.artista == c.get_artista())&&
        (self.genero.es_igual_a(&c.genero));
    }
    //Metodos primarios
    pub fn new(nom1:String,nom2:String,gen_in:Generos)->Cancion{
        return Cancion{
            titulo : nom1,
            artista : nom2,
            genero : gen_in
        }
    }    
}

impl PlayList{
    //Metodos secundarios
    pub fn get_nombre(&self)->String{
        return self.nombre.clone(); 
    }
    //Metodos primarios
    pub fn new(nom:&String)->PlayList{
        return PlayList { nombre: nom.to_string(), canciones: Vec::new() }
    }
    pub fn agregar_cancion(&mut self,c:&Cancion){
        self.canciones.push(c.clone());
    }
    pub fn eliminar_cancion(&mut self,c:&Cancion)->bool{
        let mut pude = false;
        if !self.canciones.is_empty() {
            for i in 0..self.canciones.len(){
                if let Some(cancion) = self.canciones.get(i){
                    if cancion.es_igual_a(&c) {
                        self.canciones.remove(i);
                        pude = true;
                        break;
                    }
                }
            }
        }
        return pude;
    }
    pub fn mover_cancion(&mut self,c:&Cancion,pos:usize)->bool{
        let mut pude = false;
            if !self.canciones.is_empty()&&(pos<=self.canciones.len()){
                for i in 0..self.canciones.len(){
                    if self.canciones[i].es_igual_a(&c) {
                        let cancion = self.canciones[i].clone();
                        self.canciones.remove(i);
                        self.canciones.insert(pos, cancion);
                        pude = true;
                        break;
                    }
                }
            }
        return pude;
    }
    pub fn buscar_cancion(&self,nom:String)->Option<Cancion>{
		let mut res : Option<Cancion> = None;
		if !self.canciones.is_empty() {
			for cancion in self.canciones.clone(){
				if cancion.get_titulo() == nom {
					res = Some(cancion);
                    break;
				}
			}
		}
		return res;
	}
    pub fn canciones_genero(&self,gen_in:&Generos)->Vec<Cancion>{
        let mut res : Vec<Cancion> = Vec::new();
        if !self.canciones.is_empty() {
            for cancion in self.canciones.clone(){
                if cancion.genero.es_igual_a(gen_in) {
                    res.push(cancion);
                    break;
                }
            }
        }
        return res;
    }
    pub fn canciones_artista(&self,nom:String)->Vec<Cancion>{
        let mut res : Vec<Cancion> = Vec::new();
        if !self.canciones.is_empty() {
            for cancion in self.canciones.clone(){
                if cancion.get_artista() == nom {
                    res.push(cancion);
                    break;
                }
            }
        }
        return res;
    }
    pub fn modificar_titulo(&mut self,nom_nuevo:&String){
        self.nombre = nom_nuevo.clone();
    }
    pub fn eliminar_canciones(&mut self)->bool{
        let mut pude = false;
        if !self.canciones.is_empty(){
            self.canciones.clear();
            pude = true;
        }
        return pude;
    }
}

#[cfg(test)]
mod testing_playlist{
    use super::*;

    #[test]
    fn manipulacion_playlist(){
        let mut nom_pri = "reproductor1".to_string();
        let mut p = PlayList::new(&nom_pri);
        assert_eq!(p.get_nombre(),nom_pri);
        nom_pri = "repro1".to_string();
        p.modificar_titulo(&nom_pri);
        assert_eq!(p.get_nombre(),nom_pri);
    }
    #[test]
    fn operatoria_canciones(){
        let mut p = PlayList::new(&"asd".to_string());
        let c = Cancion::new(String::from("pepe"), String::from("pepito"), Generos::Rap);
        p.agregar_cancion(&c);
        p.agregar_cancion(&c);
        
        //Operacion de busqueda

        if let Some(aux) = p.buscar_cancion("pepe".to_string()){
            assert_eq!(aux.es_igual_a(&c),true);
        }else{
            panic!("No existe esa cancion");
        }

        //Operacion de desplazamiento

        let c2 = Cancion::new(String::from("pepo"), String::from("pepe"), Generos::Rap);
        p.agregar_cancion(&c2);
        assert!(p.mover_cancion(&c2,0));
        if let Some(aux) = p.canciones.get(0){
            assert_eq!(aux.es_igual_a(&c2),true);
        }else{
            panic!("No existe esa cancion");
        }

        //Operacion de baja

        assert!(p.eliminar_canciones());
        assert_eq!(p.canciones.is_empty(),true);
    }
    #[test]
    fn listado_canciones(){
        let mut p = PlayList::new(&"asd".to_string());
        let c1 = Cancion::new(String::from("pepe"), String::from("pepito"), Generos::Rap);
        let c2 = Cancion::new(String::from("donPepe"), String::from("donPepito"), Generos::Rap);
        let c3 = Cancion::new(String::from("qwe"), String::from("Qwe"), Generos::Rock);
        p.agregar_cancion(&c1);
        p.agregar_cancion(&c2);
        p.agregar_cancion(&c1);
        p.agregar_cancion(&c3);
        p.agregar_cancion(&c3);

        //Listados de un unico uso para el test
        let lista1 = p.canciones_genero(&Generos::Rap);
        
        if !lista1.is_empty(){
            for cancion in lista1{
                assert_eq!(cancion.genero.es_igual_a(&Generos::Rap),true);
            }
        }else{
            panic!("Lista 1 no generada");
        }

        let lista2 = p.canciones_artista("pepito".to_string());

        if !lista2.is_empty(){
            for cancion in lista2{
                assert_eq!(cancion.get_artista() == "pepito".to_string(),true);
            }
        }else{
            panic!("Lista 2 no generada");
        }
    }
}




/*
    IMPLEMENTACION TP5 - EJ2
*/

/*
	Tipos de errores
*/
#[derive(Debug)]
pub enum error_operatoria{
    SinDesplazamiento(String),
	Inexistente(String),
	EstructuraVacia(String)
}

impl Display for error_operatoria{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self{
            error_operatoria::SinDesplazamiento(val) => write!(f, "La posicion recibida no es valida para la operacion en la estructura {} ",val),
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

//Implementacion extra
impl PlayList{
    fn to_string(&self)->String{
        return self.get_nombre();
    }
    fn is_Vacio(&self)->bool{
        return self.canciones.is_empty();
    }
    fn get_canciones_cant(&self)->usize{
        return self.canciones.len();
    }
}

//Archivo de almacenamiento
#[derive(Debug)]
pub struct Archivo{
    informacion : PlayList,
	path : String,
	autoguardado : bool 
}

impl Archivo{
	fn new(dato:&PlayList,dir:String,estado:bool)->Archivo{
		return Archivo { informacion: dato.clone(), path: dir , autoguardado : estado};
	}
    fn set_informacion(&mut self,dato:&PlayList){   //"Setea la informacion logica" para hacer una modificacion total
        self.informacion = dato.clone();
    }
    fn existe_archivo(&self)->bool{
		return Path::new(&self.path.clone()).exists();
	}
    fn respaldar_informacion(&self) -> Result<(), Errores> {
        // Apertura/Creación del archivo (Se utiliza OpenOptions para la apertura y edicion de un archivo existente)
        let mut file = if self.existe_archivo() {
        	// Abrir en modo lectura/escritura si existe
        	OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)?
    	} else {
        	// Crear nuevo archivo si no existe
        	File::create(&self.path)?
    	};

        // Serialización de la informacion
        let serializado = serde_json::to_string(&self.informacion)?;

        // Escritura en el archivo
        file.write_all(serializado.as_bytes())?;

        Ok(())
    }
    fn validar_alta(&mut self,c:&Cancion)->Result<(), Errores>{
		self.informacion.agregar_cancion(&c);

		if self.autoguardado{
			self.respaldar_informacion()?;
		}

		Ok(())
	}
    fn validar_baja(&mut self, c:&Cancion)->Result<(),Errores>{
		if !self.informacion.is_Vacio() {
			if !self.informacion.eliminar_cancion(&c) {
				return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente(self.informacion.to_string())) );
			}
		}else{
			return Err(Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(self.informacion.to_string())) );
		}

		if self.autoguardado{
			self.respaldar_informacion()?;
		}
		Ok(())
	}
    fn validar_desplazamiento(&mut self, c:&Cancion,pos:usize)->Result<(),Errores>{
        if !self.informacion.is_Vacio(){
            if self.informacion.get_canciones_cant() >= pos{
                if !self.informacion.mover_cancion(c, pos){
                    return Err(Errores::ErrorOperatoria(error_operatoria::Inexistente(self.informacion.to_string())) );
                }
            }else{
                return Err(Errores::ErrorOperatoria(error_operatoria::SinDesplazamiento(self.informacion.to_string())) );
            }
        }else{
            return Err(Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(self.informacion.to_string())) );
        }

        if self.autoguardado{
			self.respaldar_informacion()?;
		}

        Ok(())
    }
    fn validar_baja_total(&mut self)->Result<(),Errores>{
        if !self.informacion.eliminar_canciones(){
            return Err(Errores::ErrorOperatoria(error_operatoria::EstructuraVacia(self.informacion.to_string())) );
        }

        if self.autoguardado{
			self.respaldar_informacion()?;
		}

        Ok(())
    }
    fn cambiar_nombre_playlist(&mut self,nom:&String)->Result<(),Errores>{
        self.informacion.modificar_titulo(&nom);

        if self.autoguardado{
			self.respaldar_informacion()?;
		}

        Ok(())
    }
}

#[cfg(test)]
mod testing_implementacion_ejercicio2{
    use super::*;

    //Test para maximar el coverage
    #[test]
    fn test_error_guardado(){
        let mut p = PlayList::new(&"asd".to_string());
        //Direccion nula
		let mut archivo1 = Archivo_respaldable::new(&p, "".to_string(),true);

        //Intento de guardardo forzoso
        match archivo1.respaldar_informacion() {
			Ok(_) => assert!(false),
			Err(e) => assert!(format!("{}",e).contains("Error de E/S al guardar")) 
		}

    }
	
    #[test]
    fn operatoria_informacion(){
        let mut p = PlayList::new(&"asd".to_string());
        let c1 = Cancion::new(String::from("pepe"), String::from("pepito"), Generos::Rap);
        let c2 = Cancion::new(String::from("plumon"), String::from("plumazo"), Generos::Rap);
        let c3 = Cancion::new(String::from("Bartone"), String::from("Bartolome"), Generos::Jazz);

        p.agregar_cancion(&c1);
        p.agregar_cancion(&c2);
        p.agregar_cancion(&c3);
        
        //Se agrega una informacion "logica" por parte de "p" (PlayLists)

        let mut archivo1 = Archivo::new(&p, "".to_string(),false);

        assert!(archivo1.validar_alta(&c1).is_ok());

        assert!(archivo1.validar_baja(&c2).is_ok());

        //Retorna error de inexistencia de un elemento
		match archivo1.validar_baja(&c2) {
			Ok(_) => assert!(false,"Aqui tendria que haber fallado"),
			Err(e) => assert!(format!("{}",e).contains("No se encontro el elemento en la estructura"))
		}
        
        assert!(archivo1.cambiar_nombre_playlist(&"PlayL1".to_string()).is_ok());
		
        assert!(archivo1.validar_desplazamiento(&c3, 0).is_ok());
		
        //Retorna error de posicion invalida
		match archivo1.validar_desplazamiento(&c3, 10) {
			Ok(_) => assert!(false,"Aqui tendria que haber fallado"),
			Err(e) => assert!(format!("{}",e).contains("La posicion recibida no es valida para la operacion en la estructura"))
		}
        
        assert!(archivo1.validar_baja_total().is_ok());
        
        //Retorna error de estructura vacia 
		match archivo1.validar_baja_total() {
			Ok(_) => assert!(false,"Aqui tendria que haber fallado"),
			Err(e) => assert!(format!("{}",e).contains("no dispone de elementos"))
		}
        
    }

    #[test]
    fn operatoria_archivo_sin_autoguardado(){
        //Creacion de playlist
        let mut p = PlayList::new(&"asd".to_string());
        let c1 = Cancion::new(String::from("pepe"), String::from("pepito"), Generos::Rap);
        let c2 = Cancion::new(String::from("plumon"), String::from("plumazo"), Generos::Rap);
        let c3 = Cancion::new(String::from("Bartone"), String::from("Bartolome"), Generos::Jazz);

        //Creacion del archivo
        let mut archivo1 = Archivo::new(&p, "src/tp5/playlist_info.json".to_string(),false);


        //Altas
        assert!(archivo1.validar_alta(&c1).is_ok());
        
        assert!(archivo1.validar_alta(&c2).is_ok());
        assert!(archivo1.validar_alta(&c3).is_ok());

        //Respaldo de informacion del archivo(luego de altas)
        assert!(archivo1.respaldar_informacion().is_ok());

        //Bajas
        assert!(archivo1.validar_baja(&c2).is_ok());
        assert!(archivo1.validar_baja(&c3).is_ok());

        //Respaldo de informacion del archivo(luego de bajas)
        assert!(archivo1.respaldar_informacion().is_ok());
    
        //Modificaciones
        assert!(archivo1.validar_alta(&c2).is_ok());
        assert!(archivo1.validar_desplazamiento(&c2,0).is_ok());
        
        assert!(archivo1.cambiar_nombre_playlist(&"PlayL1".to_string()).is_ok());
    
        //Respaldo de informacion del archivo(luego de modificaciones)
        assert!(archivo1.respaldar_informacion().is_ok());

        //Baja total
        assert!(archivo1.validar_baja_total().is_ok());

        //Respaldo de informacion del archivo(luego de modificaciones)
        assert!(archivo1.respaldar_informacion().is_ok());

        //Insercion directa(estructura modificada aparte) y respaldo

        p = PlayList::new(&"Pl2".to_string());

        p.agregar_cancion(&c1);
        p.agregar_cancion(&c2);
        p.eliminar_canciones();

        archivo1.set_informacion(&p); 

        assert!(archivo1.respaldar_informacion().is_ok());

        /*
            Resultado final de JSON = "Queda vacio"
        */

    }

    #[test]
    fn operatoria_archivo_con_autoguardado(){
        //Creacion de playlist
        let mut p = PlayList::new(&"asd".to_string());
        let c1 = Cancion::new(String::from("pepe"), String::from("pepito"), Generos::Rap);
        let c2 = Cancion::new(String::from("plumon"), String::from("plumazo"), Generos::Rap);
        let c3 = Cancion::new(String::from("Bartone"), String::from("Bartolome"), Generos::Jazz);

        //Creacion del archivo
        let mut archivo1 = Archivo::new(&p, "src/tp5/playlist_info.json".to_string(),true);


        //Altas
        assert!(archivo1.validar_alta(&c1).is_ok());
        assert!(archivo1.validar_alta(&c2).is_ok());
        assert!(archivo1.validar_alta(&c3).is_ok());

        //Bajas
        assert!(archivo1.validar_baja(&c1).is_ok());
        assert!(archivo1.validar_baja(&c2).is_ok());

        //Modificaciones
        assert!(archivo1.validar_alta(&c1).is_ok());
        assert!(archivo1.validar_desplazamiento(&c1,0).is_ok());
        
        assert!(archivo1.cambiar_nombre_playlist(&"PlayL1".to_string()).is_ok());
        
        //Baja total
        assert!(archivo1.validar_baja_total().is_ok());
           
        //Insercion directa(estructura modificada aparte) y respaldo

        p = PlayList::new(&"Pl2".to_string());

        p.agregar_cancion(&c1);
        p.agregar_cancion(&c2);
        p.eliminar_canciones();

        archivo1.set_informacion(&p); 

        assert!(archivo1.respaldar_informacion().is_ok());

        /*
            Resultado final de JSON = "Queda vacio"
        */

    }

}
