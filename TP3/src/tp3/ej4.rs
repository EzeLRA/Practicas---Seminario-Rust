/* 
    Estructura Triangulo
*/

//Atributos
#[derive(PartialEq, Debug)]
pub struct Triangulo{
    long1 : f32,
    long2 : f32,
    long3 : f32
}

//Enum
#[derive(Debug, PartialEq)]
pub enum TipoTriangulo {
    Equilatero,
    Isoceles,
    Escaleno,
}

//Metodos
impl Triangulo{

    pub fn new(l1:f32,l2:f32,l3:f32)->Triangulo{
        return Triangulo{ long1 : l1 , long2 : l2 , long3 : l3}
    }

    pub fn determinar_tipo(&self) -> TipoTriangulo {
        //Se evalua los tres valores de las condiciones en el match
        match (self.long1 == self.long2, self.long2 == self.long3, self.long1 == self.long3) {
            (true, true, true) => TipoTriangulo::Equilatero,
            (false, false, false) => TipoTriangulo::Escaleno,
            _ => TipoTriangulo::Isoceles,
        }
    }

    pub fn calcular_area(&self)-> f32{
        //Formula de Heron (Calculo de area con el perimetro)
        /*
            Semiperimetro :  s = perimetro / 2

            Area : ( √(s*(s – a)*(s – b)*(s – c)) )
         */

        let s = self.calcular_perimetro() / 2.0;
        return f32::sqrt(s*(s-self.long1)*(s-self.long2)*(s-self.long3))
    }

    pub fn calcular_perimetro(&self)-> f32{
        return self.long1 + self.long2 + self.long3;
    }

}


#[cfg(test)]
mod testing_triangulo{
    use crate::tp3::ej4::{TipoTriangulo, Triangulo};
    
    #[test]
    fn creacion_triangulo(){
        let t = Triangulo::new(5.2, 5.2, 5.2);
        assert_eq!(t,Triangulo::new(5.2, 5.2, 5.2));
    }

    #[test]
    fn clasificar_tipos(){
        let mut t = Triangulo::new(5.2, 5.2, 5.2);
        assert_eq!(t.determinar_tipo(), TipoTriangulo::Equilatero);
        t = Triangulo::new(5.2, 5.2, 8.0);
        assert_eq!(t.determinar_tipo(), TipoTriangulo::Isoceles);
        t = Triangulo::new(5.2, 3.2, 8.0);
        assert_eq!(t.determinar_tipo(), TipoTriangulo::Escaleno);
    }

    #[test]
    fn resultado_calculo(){
        let t = Triangulo::new(5.0, 6.0, 5.0);
        assert_eq!(t.calcular_perimetro(),16.0);
        assert_eq!(t.calcular_area(),12.0);
    }

}