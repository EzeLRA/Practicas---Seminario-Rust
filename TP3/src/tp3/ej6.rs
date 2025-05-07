/*
    Estructuras: Estudiantes y Examenes
*/

//Atributos
#[derive(PartialEq, Debug,Clone)]
pub struct Examen{
    nombre : String,
    nota : u8
}

#[derive(PartialEq, Debug)]
pub struct Estudiante{
    nombre : String,
    num_id : i32,
    examenes : Vec<Examen>
}

//Metodos
impl Examen{
    pub fn new(nom:String,n:u8)->Examen{
        return Examen{nombre:nom,nota:n};
    }
}

//Revisar si se agregan elementos de un unico tipo

impl Estudiante{
    //Se crea un estudiante sin examenes registrados
    pub fn new(nom:String,id:i32)->Estudiante{
        return Estudiante{nombre:nom,num_id:id,examenes:Vec::new()};
    }
    //Funcion para agregar examenes
    pub fn agregar_examen(&mut self,e:Examen){
        self.examenes.push(e);
    }
    //Revisar y tomar precauciones
    /* 
    pub fn primer_examen(&self) -> Option<Examen> {
        return self.examenes.first().cloned();
    }*/
}