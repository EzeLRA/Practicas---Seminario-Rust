/* 
    Estructura Producto
*/

//Atributos
#[derive(PartialEq, Debug)]
pub struct Producto{
    nombre : String,
    precio_bruto : f32,
    num_identificacion : i32
}

//Metodos
impl Producto{
    pub fn new(nom:String,precio:f32,num:i32)->Producto{
        Producto { nombre: nom, precio_bruto: precio, num_identificacion: num }
    }
    pub fn calcular_impuestos(&self,porcentaje:f32)->f32{
        return if(porcentaje > 0.0){ (self.precio_bruto*(100.0+porcentaje))/100.0 }else{self.precio_bruto};
    }
    pub fn calcular_descuento(&self,porcentaje:f32)->f32{
        return if(porcentaje > 0.0){ (self.precio_bruto*(100.0-porcentaje))/100.0 }else{self.precio_bruto};
    }
    pub fn calcular_precio_total(&self,porcentaje_impuesto:f32,porcentaje_descuento:f32)->f32{
        return self.precio_bruto + (self.calcular_impuestos(porcentaje_impuesto) - self.precio_bruto) - (self.precio_bruto - self.calcular_descuento(porcentaje_descuento));
    }
}


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