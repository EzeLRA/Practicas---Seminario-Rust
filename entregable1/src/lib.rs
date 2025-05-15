/*
    Estructuras: Estudiantes y Examenes - Implementacion principal
*/

//Atributos
#[derive(Debug)]
pub struct Examen{
    nombre : String,
    nota : f32
}

#[derive(Debug)]
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

    //Metodos secundarios
    pub fn es_igual_a(&self,e:&Estudiante)->bool{
        return (self.nombre == e.nombre)&&(self.num_id == e.num_id);
    }

    //Metodos primarios

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

/*
    Implementacion adicional - 15/5/25
*/

//Estructura para generar el informe
#[derive(Debug)]
pub struct Informe{
    nom_alumno : String,
    id_alumno : i32,
    examenes_rendidos_cant : u32,
    promedio_notas : f32,
    examen_max : Examen,
    examen_min : Examen
}

//Funciones de la estructura
impl Informe{

    //Metodo que procesa un alumno para generar el informe
    pub fn generar_informe(e:&Estudiante)->Option<Informe>{
        if !e.examenes.is_empty() {
            //Procesa el alumno
            let info = Informe{
                nom_alumno : e.nombre.clone(),
                id_alumno : e.num_id,
                examenes_rendidos_cant : e.examenes.len(),
                promedio_notas : if let Some(data) = e.obtener_promedio() { data },
                examen_max : if let Some(data) = e.obtener_calificacion_mas_alta() { data },
                examen_min : if let Some(data) = e.obtener_calificacion_mas_baja() { data }
             }
             return Some(info);
        }else{
            return None;
        }
    }

}

//Testing para la nueva implementacion
#[cfg(test)]
mod testing_informe{
    use super::*;
    //1°Test: Se buscara testear la correcta creacion del informe
    #[test]
    fn creacion_estudiante(){
        //Se evaluara un informe vacio
        let est = Estudiante::new("Carlos".to_string(), 1);

        let info = Informe::new(&est);
        assert_eq!(info.is_none(),true);
    }
    //2°Test: Se probara con casos de alumnos que rindieron examenes y viceversa para el correcto retorno de un informe
    //Se implementara un codigo similar al de arriba tomando ambos casos para la creacion y evaluando los valores de retorno del informe(Datos como las notas , promedio y nombre del alumno)

    //Funcion a implementar aqui 

}

#[cfg(test)]
mod testing_estudiante{
    use super::Estudiante;
    use super::Examen;

    #[test]
    fn creacion_estudiante(){
        let est = Estudiante::new("Carlos".to_string(), 1);
        assert_eq!(est.es_igual_a(&Estudiante::new("Carlos".to_string(), 1)),true);
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