/*
    Estructuras: Estudiantes y Examenes
*/

//Atributos
#[derive(PartialEq, Debug)]
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

    //Se maneja Option<f32> para el caso de que el alumno no haya rendido un examen
    pub fn obtener_promedio(&self)->Option<f32>{
        if !self.examenes.is_empty() {
            let mut prom : f32 = 0.0;
            for exam in &self.examenes{
                prom += exam.obtener_nota();
            }
            prom = prom/self.examenes.len() as f32 ;
            return Some(prom);
        }else{
            return None;
        }
    }

    pub fn obtener_calificacion_mas_alta(&self)->Option<f32>{
        if !self.examenes.is_empty() {
            let mut nota : f32 = -1.0;
            for exam in &self.examenes{
                if exam.obtener_nota()>nota {
                    nota = exam.obtener_nota();
                }
            }
            return Some(nota);
        }else{
            return None;
        }
    }

    pub fn obtener_calificacion_mas_baja(&self)->Option<f32>{
        if !self.examenes.is_empty() {
            let mut nota : f32 = 11.0;
            for exam in &self.examenes{
                if exam.obtener_nota()<nota {
                    nota = exam.obtener_nota();
                }
            }
            return Some(nota);
        }else{
            return None;
        }
    }

}


#[cfg(test)]
mod testing_estudiante{
    use super::Estudiante;
    use super::Examen;

    #[test]
    fn creacion_estudiante(){
        let est = Estudiante::new("Carlos".to_string(), 1);
        assert_eq!(est,Estudiante::new("Carlos".to_string(), 1));
    }

    #[test]
    fn calculo_promedios(){
        let mut est = Estudiante::new("Damian".to_string(), 621);
        assert_eq!(est.obtener_promedio(),None);
        est.agregar_examen(Examen::new(String::from("Mat"), 8.0));
        assert_eq!(est.obtener_promedio(),Some(8.0));
        est.agregar_examen(Examen::new(String::from("Cadp"), 4.5));
        est.agregar_examen(Examen::new(String::from("Oc"), 7.2));
        est.agregar_examen(Examen::new(String::from("Mat2"), 10.0));
        assert_eq!(est.obtener_promedio(),Some(7.425));
    }

    #[test]
    fn obtener_nota_max(){
        let mut est = Estudiante::new("Julio".to_string(), 231);
        assert_eq!(est.obtener_calificacion_mas_alta(),None);
        est.agregar_examen(Examen::new(String::from("Mat"), 8.0));
        est.agregar_examen(Examen::new(String::from("Cadp"), 4.5));
        est.agregar_examen(Examen::new(String::from("Mat3"), 7.2));
        est.agregar_examen(Examen::new(String::from("Mat2"), 10.0));
        assert_eq!(est.obtener_calificacion_mas_alta(),Some(10.0));
    }

    #[test]
    fn obtener_nota_min(){
        let mut est = Estudiante::new("Tobias".to_string(), 321);
        assert_eq!(est.obtener_calificacion_mas_baja(),None);
        est.agregar_examen(Examen::new(String::from("Mat"), 2.0));
        est.agregar_examen(Examen::new(String::from("Mat3"), 7.2));
        est.agregar_examen(Examen::new(String::from("Mat2"), 8.0));
        assert_eq!(est.obtener_calificacion_mas_baja(),Some(2.0));
    }

}