/*
    Estructuras: Estudiantes y Examenes
*/

//Atributos
#[derive(PartialEq, Debug,Clone)]
pub struct Examen{
    nombre : String,
    nota : f32
}

#[derive(PartialEq, Debug)]
pub struct Estudiante{
    nombre : String,
    num_id : i32,
    examenes : Vec<Examen>
}

//Metodos
impl Examen{
    pub fn new(nom:String,n:f32)->Examen{
        return Examen{nombre:nom,nota:n};
    }
    //Getter
    pub fn obtener_nota(&self)->f32{
        return self.nota;
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
    //Retorna el ultimo examen y lo retorna
    pub fn conseguir_examen(&mut self) -> Option<Examen> {
        return self.examenes.pop();
    }

    pub fn obtener_promedio(&self)->f32{
        let mut prom : f32 = 0.0;
        if(!self.examenes.is_empty()){
            for exam in &self.examenes{
                prom += exam.obtener_nota();
            }
            prom = prom/self.examenes.len() as f32 ;
            
        }
        return prom
    }

    pub fn obtener_calificacion_mas_alta(&self)->f32{
        let mut nota : f32 = -1.0;
        if(self.examenes.len()>0){
            for exam in &self.examenes{
                if(exam.obtener_nota()>nota){
                    nota = exam.obtener_nota();
                }
            }
        }
        return nota;
    }

    pub fn obtener_calificacion_mas_baja(&self)->f32{
        let mut nota : f32 = 11.0;
        if(self.examenes.len()>0){
            for exam in &self.examenes{
                if(exam.obtener_nota()<nota){
                    nota = exam.obtener_nota();
                }
            }
        }
        return nota;
    }

}