mod tp3;
use tp3::ej1::Persona;
use tp3::ej2::Rectangulo;
use tp3::ej3::Fecha;
use crate::tp3::ej4::Triangulo;
use tp3::ej5::Producto;
use tp3::ej6::*;

fn main() {

    //Implementar

    let mut est = Estudiante::new(String::from("Jose"), 1234);
    est.agregar_examen(Examen::new(String::from("Mat"), 10.0));
    //est.agregar_examen(Examen::new(String::from("Mat"), 8.0));
    //est.agregar_examen(Examen::new(String::from("Mat"), 7.5));
    
}

/*
    Test de Persona     -      Ejercicio 1
*/

#[cfg(test)]
mod testing_persona{
    use super::Persona;

    #[test]
    fn creacion_persona(){
        let mut persona = Persona::new(String::from("Mario"),23,None );
        //Persona con direccion nula
        assert_eq!( persona , Persona::new(String::from("Mario"),23,None ) );
        
        //Persona sin direccion nula
        persona = Persona::new(String::from("Mario"),23,Some(String::from("Av.Entre Rios")));
        assert_eq!( persona , Persona::new(String::from("Mario"),23,Some(String::from("Av.Entre Rios"))) );
    }

    #[test]
    fn representacion_string(){
        let mut persona = Persona{
            nombre : String::from("Mario"),
            edad : 23 ,
            direccion : None
        };
        //Persona con direccion nula
        assert_eq!( persona.to_string() , String::from("Mario;23;no identificado") );
        
        //Persona sin direccion nula        
        persona.direccion = Some(String::from("Av.Entre Rios"));

        assert_eq!( persona.to_string() , String::from("Mario;23;Av.Entre Rios") );
    }

    #[test]
    fn retorno_edad(){
        let persona = Persona{
            nombre : String::from("Mario"),
            edad : 23 ,
            direccion : None
        };
        assert_eq!( persona.obtener_edad() , 23 );
    }

    #[test]
    fn modificacion_direccion(){
        let mut persona = Persona{
            nombre : String::from("Mario"),
            edad : 24 ,
            direccion : None
        };
        //Persona con direccion nula
        assert_eq!( persona.direccion , None );
        
        //Persona sin direccion nula
        persona.actualizar_direccion( Some(String::from("Av.Corrientes")) );
        assert_eq!( persona.direccion , Some(String::from("Av.Corrientes")) );
    }
}

/*
    Test de Rectangulo  -   Ejercicio 2
*/

#[cfg(test)]
mod testing_rectangulo{
    use super::Rectangulo;

    #[test]
    fn creacion_rectangulo(){
        let r = Rectangulo::new(25.0,2.0);
        assert_eq!(r, Rectangulo::new(25.0,2.0) );
    }

    #[test]
    fn retorno_no_negativo(){
        let r = Rectangulo{
            longitud: 25.0,
            ancho: 2.0
        };
        assert_eq!( (r.calcular_area() > 0.0) , true );
        assert_eq!( (r.calcular_perimetro() > 0.0) , true );
    }

    #[test]
    fn calculo_correcto(){
        let r = Rectangulo{
            longitud: 25.0,
            ancho: 2.0
        };
        assert_eq!( (r.calcular_area() == 50.0) , true );
        assert_eq!( (r.calcular_perimetro() == 54.0) , true );
    }


    #[test]
    fn figura_cuadrada(){
        let r = Rectangulo{
            longitud: 2.0,
            ancho: 2.0
        };
        assert_eq!(r.es_cuadrado(), true);
    }

}

/*
    Test de Fecha   -   Ejercicio 3
*/

#[cfg(test)]
mod testing_fecha{
    use super::Fecha;

    #[test]
    fn creacion_fecha(){
        let f = Fecha::new(1, 1, 2025);
        assert_eq!(f,Fecha::new(1, 1, 2025));
    }

    #[test]
    fn validacion_de_fecha(){
        let mut f = Fecha::new(1, 1, 2025);
        assert_eq!(f.es_fecha_valida(),true);
        f = Fecha::new(31, 2, 2004);
        assert_eq!(f.es_fecha_valida(),false);
    }

    #[test]
    fn validar_bisiesto(){
        let mut f = Fecha::new(1, 1, 2028);
        assert_eq!(f.es_bisiesto(),true);
        f = Fecha::new(1, 1, 2025);
        assert_eq!(f.es_bisiesto(),false);
    }

    #[test]
    fn adicion_fecha(){
        let mut f = Fecha::new(1, 1, 2028);
        f.sumar_dias(30);
        assert_eq!(f,Fecha::new(31, 1, 2028));
        f.sumar_dias(1);
        assert_eq!(f,Fecha::new(1, 2, 2028));
        f.sumar_dias(29);
        assert_eq!(f,Fecha::new(1,3,2028));
    }

    #[test]
    fn sustraccion_fecha(){
        let mut f = Fecha::new(10, 4, 2028);
        f.restar_dias(9);
        assert_eq!(f,Fecha::new(1, 4, 2028));
        f.restar_dias(31);
        assert_eq!(f,Fecha::new(1,3,2028));
        f.restar_dias(1);
        assert_eq!(f,Fecha::new(29, 2, 2028));
    }

    #[test]
    fn comparacion_fechas(){
        let f1 = Fecha::new(25, 5, 2000);
        let f2 = Fecha::new(25, 2, 2004);
        assert_eq!(f1.es_mayor(&f2),false);
        assert_eq!(f2.es_mayor(&f1),true);
    }

}

/*
    Test de Triangulo   -   Ejercicio 4
*/

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

/*
    Test de Producto    -   Ejercicio 5
*/

#[cfg(test)]
mod testing_producto{
    use super::Producto;

    #[test]
    fn crear_producto(){
        let p = Producto::new(String::from("Serenito"), 8500.0, 12452);
        assert_eq!(p,Producto::new(String::from("Serenito"), 8500.0, 12452));
    }

    #[test]
    fn calcular_precios(){
        let p = Producto::new(String::from("Baggio"), 5000.0, 5432);
        assert_eq!(p.calcular_impuestos(10.0),5500.0);
        assert_eq!(p.calcular_impuestos(0.0),5000.0);
        assert_eq!(p.calcular_descuento(10.0),4500.0);
        assert_eq!(p.calcular_descuento(0.0),5000.0);
    }

    #[test]
    fn estimar_precio(){
        let p = Producto::new(String::from("Milka"), 1000.0, 8932);
        assert_eq!(p.calcular_precio_total(10.0, 10.0),1000.0);
        assert_eq!(p.calcular_precio_total(0.0, 0.0),1000.0);
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
        assert_eq!(est.obtener_promedio(),0.0);
        est.agregar_examen(Examen::new(String::from("Mat"), 8.0));
        assert_eq!(est.obtener_promedio(),8.0);
        est.agregar_examen(Examen::new(String::from("Cadp"), 4.5));
        est.agregar_examen(Examen::new(String::from("Oc"), 7.2));
        est.agregar_examen(Examen::new(String::from("Mat2"), 10.0));
        assert_eq!(est.obtener_promedio(),7.425);
    }

    #[test]
    fn obtener_nota_max(){
        let mut est = Estudiante::new("Julio".to_string(), 231);
        assert_eq!(est.obtener_calificacion_mas_alta(),-1.0);
        est.agregar_examen(Examen::new(String::from("Mat"), 8.0));
        est.agregar_examen(Examen::new(String::from("Cadp"), 4.5));
        est.agregar_examen(Examen::new(String::from("Mat3"), 7.2));
        est.agregar_examen(Examen::new(String::from("Mat2"), 10.0));
        assert_eq!(est.obtener_calificacion_mas_alta(),10.0);
    }

    #[test]
    fn obtener_nota_min(){
        let mut est = Estudiante::new("Tobias".to_string(), 321);
        assert_eq!(est.obtener_calificacion_mas_baja(),11.0);
        est.agregar_examen(Examen::new(String::from("Mat"), 2.0));
        est.agregar_examen(Examen::new(String::from("Mat3"), 7.2));
        est.agregar_examen(Examen::new(String::from("Mat2"), 8.0));
        assert_eq!(est.obtener_calificacion_mas_baja(),2.0);
    }

}
