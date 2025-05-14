/*
    Estructuras : Cancion , Generos y PlayList
*/
#[derive(Debug,Clone)]
pub enum Generos{
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros
}
#[derive(Debug,Clone)]
pub struct Cancion{
    titulo : String,
    artista : String,
    genero : Generos
}
#[derive(Debug)]
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
    pub fn modificar_titulo(&mut self,nom_nuevo:&String){
        self.nombre = nom_nuevo.to_string();
    }
    pub fn buscar_cancion(&self,c1:&Cancion)->Option<Cancion>{
		let mut res : Option<Cancion> = None;
		if !self.canciones.is_empty() {
			for cancion in self.canciones.clone(){
				if(cancion.es_igual_a(c1)){
					res = Some(cancion);
                    break;
				}
			}
		}
		return res;
	}
}


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
    fn act1(){
        let mut p = PlayList::new(&"asd".to_string());
        let c = Cancion::new(String::from("pepe"), String::from("pepito"), Generos::Rap);
        p.agregar_cancion(&c);
        p.agregar_cancion(&c);
        if let Some(aux) = p.buscar_cancion(&c){
            assert_eq!(aux.es_igual_a(&c),true);
        }else{
            panic!("No existe esa cancion");
        }
        
    }

}