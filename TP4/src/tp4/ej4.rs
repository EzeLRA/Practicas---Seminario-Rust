/*
    Producto y venta
*/
#[derive(PartialEq,Debug,Clone)]
pub enum MediosDePago{
    TarjetaCredito, 
    TarjetaDébito, 
    TransferenciaBancaria,
    Efectivo
}

//Implementa su propio porcentaje de descuento integrado(100% a 0%)
#[derive(PartialEq,Debug,Clone)]
pub enum Categorias{
	Alimento(f64),
    Bazar(f64),
    Limpieza(f64),
    Otro(f64)
}

impl Default for Categorias {
    fn default() -> Self {
        return Categorias::Otro(0.0)
    }
}

impl Categorias {
    // Método que devuelve el valor numérico sin importar la variante
    pub fn porcentaje(&self) -> f64 {
        let mut res : f64 = 0.0;
        match self {
            Categorias::Alimento(val) => res = *val,
            Categorias::Bazar(val) => res = *val,
            Categorias::Limpieza(val) => res = *val,
            Categorias::Otro(val) => res = *val,
        }
        return res;
    }
    
    pub fn igual_a(&self,categ:&Categorias)->bool{
        return categ == self;
    }
}

#[derive(PartialEq,Debug,Clone)]
pub struct Producto{
    nombre : String,
    categoria : Categorias,
    precio_base : f64,   
}

impl Producto{
    fn new(nom : &String,cate : &Categorias,precio : f64)->Producto{
        return Producto{
            nombre : nom.clone(),
            categoria : cate.clone(),
            precio_base : precio
        }
    }
    fn obtener_precio_sin_descuento(&self)->f64{
        return self.precio_base;
    }
    fn obtener_precio_con_descuento(&self)->f64{
        return (self.precio_base * (100.0-self.categoria.porcentaje()) )/100.0;
    }
    fn categoria_igual_a(&self,categ : &Categorias)->bool{
        return self.categoria.igual_a(&categ);
    }
}
#[derive(PartialEq,Debug,Clone)]
pub struct ProductoVendido(Producto,u64);

impl ProductoVendido{
    fn new(p : &Producto,cant : u64)->ProductoVendido{
        return ProductoVendido(p.clone(),cant);
    }
}

#[derive(PartialEq,Debug,Clone)]
pub struct Fecha(u8,u8,u64);

impl Fecha{
    fn new(dia:u8,mes:u8,anio:u64)->Fecha{
        return Fecha(dia,mes,anio);
    }
}

#[derive(PartialEq,Debug,Clone)]
pub struct Venta{
    fecha : Fecha,
    cliente : Cliente,
    vendedor : Vendedor,
    medio_pago : MediosDePago,
    productos : Vec<ProductoVendido>
}

impl Venta{
    fn new(f:&Fecha,c:&Cliente,v:&Vendedor,medio:&MediosDePago)->Venta{
        return Venta{
            fecha : f.clone(),
            cliente : c.clone(),
            vendedor : v.clone(),
            medio_pago : medio.clone(),
            productos : Vec::new()
        }
    }
    fn agregar_producto(&mut self,p:&ProductoVendido){
        self.productos.push(p.clone());
    }
    
    fn monto_total(&self)->f64{
        let mut total = 0.0;
        if !self.productos.is_empty(){
            let cumple = self.cliente.tiene_newsletter();
            for producto in &self.productos{
                if cumple {
                    total += (producto.0.obtener_precio_con_descuento())*(producto.1 as f64);
                }else {
                    total += (producto.0.obtener_precio_sin_descuento())*(producto.1 as f64);
                }
            }
        }
        return total;
    }

    fn retornar_venta_por_categoria(&self,categ:&Categorias)->Option<Venta>{
        let mut res_fin : Option<Venta> = None;
        
        if !self.productos.is_empty(){
            let mut res : Vec<ProductoVendido> = Vec::new();
            for p in &self.productos{
                if p.0.categoria_igual_a(categ){
                    res.push(p.clone());
                }
            }
            res_fin = Some( Venta{
                fecha : self.fecha.clone(),
                cliente : self.cliente.clone(),
                vendedor : self.vendedor.clone(),
                medio_pago : self.medio_pago.clone(),
                productos : res
                });
        }

        return res_fin;
    }

    fn get_vendedor(&self)->Vendedor{
        return self.vendedor.clone();
    }

}

/*
    Personas
*/
#[derive(PartialEq,Debug,Clone)]
pub struct Datos_Persona {
    nombre : String,
    apellido : String,
    direccion : String,
    dni : u64
}

pub trait DatosPersona {
    fn get_nombre(&self , datos:&Datos_Persona)->String{
        return datos.nombre.clone();
    }
    fn get_apellido(&self , datos:&Datos_Persona)->String{
        return datos.apellido.clone();
    }
    fn get_direccion(&self , datos:&Datos_Persona)->String{
        return datos.direccion.clone();
    }
    fn get_dni(&self , datos:&Datos_Persona)->u64{
        return datos.dni;
    }
}
#[derive(PartialEq,Debug,Clone)]
pub struct Cliente {
    datos_cliente : Datos_Persona,
    correo_newsletter : Option<String>
}

impl Cliente{
    fn new(nom : &String,ape : &String,dir : &String,dni_in : u64)->Cliente{
        return Cliente{
            datos_cliente : Datos_Persona{nombre: nom.clone(),apellido: ape.clone(),direccion: dir.clone(),dni: dni_in},
            correo_newsletter : None        }        
    }
    fn asignar_newsletter(&mut self,correo:&String){
        self.correo_newsletter = Some(correo.clone());
    }
    fn tiene_newsletter(&self)->bool{
        return self.correo_newsletter.is_some();
    }
}

impl DatosPersona for Cliente{}

#[derive(PartialEq,Debug,Clone)]
pub struct Vendedor {
    datos_vendedor : Datos_Persona,
    nro_legajo : u64,
    antiguedad : u8,
    salario : f64
}

impl Vendedor{
    fn new(nom : &String,ape : &String,dir : &String,Dni : u64,legajo : u64,ant : u8,monto : f64)->Vendedor{
        return Vendedor{
            datos_vendedor : Datos_Persona{nombre: nom.clone(),apellido: ape.clone(),direccion: dir.clone(),dni: Dni},
            nro_legajo : legajo,
            antiguedad : ant,
            salario : monto
        }        
    }
    fn get_legajo(&self)->u64{
        return self.nro_legajo;
    }
    fn get_antiguedad(&self)->u8{
        return self.antiguedad;
    }
    fn get_salario(&self)->f64{
        return self.salario;
    }
}

impl DatosPersona for Vendedor{}

/*
    Sistema
*/
#[derive(PartialEq,Debug,Clone)]
pub struct CategPorcentajes(f64,f64,f64,f64);

pub struct Sistema{
    ventas : Vec<Venta>,
    categorias_porcentajes : CategPorcentajes,
    newsletter : String
}

impl Sistema{
    fn new(porcentajes:&CategPorcentajes,c:&String)->Sistema{
        return Sistema {
            ventas : Vec::new(),
            categorias_porcentajes : porcentajes.clone(),
            newsletter : c.clone()
        }
    }
    fn definir_categoria(&self,categ:&mut Categorias)->Categorias{
        return match categ {
            Categorias::Alimento(_) => Categorias::Alimento(self.categorias_porcentajes.0),
            Categorias::Bazar(_) => Categorias::Bazar(self.categorias_porcentajes.1),
            Categorias::Limpieza(_) => Categorias::Limpieza(self.categorias_porcentajes.2),
            Categorias::Otro(_) => Categorias::Otro(self.categorias_porcentajes.3),
        }

    }
    fn agregar_venta(&mut self,v:&Venta){
        self.ventas.push(v.clone());
    }
    fn retornar_ventas_por_categoria(&self,categ:&Categorias)->Vec<Venta>{
        let mut res : Vec<Venta> = Vec::new();

        if !self.ventas.is_empty(){
            for v in &self.ventas{
                if let Some(v2) = v.retornar_venta_por_categoria(&categ){
                    res.push(v2);
                }
            }
        }

        return res;
    }
    fn retornar_ventas_por_vendedor(&self,ve:&Vendedor)->Vec<Venta>{
        let mut res : Vec<Venta> = Vec::new();

        if !self.ventas.is_empty(){
            for v in &self.ventas{
                if v.get_vendedor() == *ve{
                    res.push(v.clone());
                }
            }
        }

        return res;
    }
}

#[cfg(test)]
mod test_ejercicio4{    
use super::*;
    #[test]
    fn operar_producto(){
        let p = Producto::new(&"Shampoo".to_string(),&Categorias::default(),3500.0);
        assert_eq!(p.obtener_precio_sin_descuento(),3500.0);
        assert_eq!(p.obtener_precio_con_descuento(),3500.0);
        assert!(p.categoria_igual_a(&Categorias::default()));
        let p = Producto::new(&"Shampoo".to_string(),&Categorias::Limpieza(50.0),3500.0);
        assert_eq!(p.obtener_precio_sin_descuento(),3500.0);
        assert_eq!(p.obtener_precio_con_descuento(),1750.0);
        assert!(p.categoria_igual_a(&Categorias::Limpieza(50.0)));
    }
}