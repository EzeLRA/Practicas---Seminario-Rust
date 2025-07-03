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
    fn get_cant(&self)->u64{
        return self.1 ;
    }
    fn get_producto(&self)->Producto{
        return self.0.clone();
    }
    fn obtener_monto_descuento(&self)->f64{
        return self.0.obtener_precio_con_descuento() * (self.1 as f64);
    }
    fn obtener_monto_generico(&self)->f64{
        return self.0.obtener_precio_sin_descuento() * (self.1 as f64);
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
                    total += producto.obtener_monto_descuento();
                }else{
                    total += producto.obtener_monto_generico();
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
            //Si la venta no tiene productos de determinada categoria entonces la venta se retorna como inexistente
            if !res.is_empty(){
                res_fin = Some( Venta{
                    fecha : self.fecha.clone(),
                    cliente : self.cliente.clone(),
                    vendedor : self.vendedor.clone(),
                    medio_pago : self.medio_pago.clone(),
                    productos : res
                });
            }
        }

        return res_fin;
    }

    fn get_vendedor(&self)->Vendedor{
        return self.vendedor.clone();
    }

    fn es_igual_a(&self,v:&Venta)->bool{
        return self == v;
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

impl Datos_Persona{
    fn validar_datos(&self,datos : &Datos_Persona)->bool{
        return (self.nombre == datos.nombre) &&
            (self.apellido == datos.apellido) &&
            (self.direccion == datos.direccion) &&
            (self.dni == datos.dni);
    } 
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
    fn definir_categoria(&self,categ:&Categorias)->Categorias{
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
    fn monto_final_venta(&self,v:&Venta)->Option<f64>{
        let mut res : Option<f64> = None;
        if let Some(v2) = self.ventas.iter().find(|venta| venta.es_igual_a(&v)){
            res = Some(v2.monto_total());
        }
        return res;
    }
    fn otorgar_newsletter(&self,cli:&mut Cliente){
        cli.asignar_newsletter(&self.newsletter);
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

    #[test]
    fn validar_informacion(){
        let cli1 = Cliente::new(&"Marcos".to_string(), &"Lupe".to_string(), &"Av1".to_string(), 124341);
        let vendedor1 = Vendedor::new(&"Julieta".to_string(), &"Murias".to_string(), &"Cantilo".to_string(), 645634, 1234,1, 10000.0);
        assert!(cli1.datos_cliente.validar_datos(&cli1.datos_cliente));
        assert!(vendedor1.datos_vendedor.validar_datos(&vendedor1.datos_vendedor));
        assert!(!cli1.datos_cliente.validar_datos(&vendedor1.datos_vendedor));
        assert!(!vendedor1.datos_vendedor.validar_datos(&cli1.datos_cliente));
    }

    #[test]
    fn operar_venta(){
        //Personas
        let cli1 = Cliente::new(&"Lucas".to_string(), &"Daniel".to_string(), &"AvBelgrano".to_string(), 871265);
        let ven1 = Vendedor::new(&"Tobias".to_string(), &"Serio".to_string(), &"AvBelgrano".to_string(), 237863, 9876, 2, 12000.0);

        //Productos registrados
        let p1 = Producto::new(&"CocaCola".to_string(), &Categorias::Alimento(25.0), 3500.0);
        let p2 = Producto::new(&"Escoba".to_string(), &Categorias::Limpieza(30.0), 1000.0);
        let p3 = Producto::new(&"ElEjemplo".to_string(), &Categorias::Bazar(0.0), 1500.0);

        //Generar ventas
        let mut v1 = Venta::new(&Fecha::new(05, 02, 2025), &cli1, &ven1, &MediosDePago::Efectivo);
        v1.agregar_producto(&ProductoVendido::new(&p1, 2));
        v1.agregar_producto(&ProductoVendido::new(&p2, 1));
        v1.agregar_producto(&ProductoVendido::new(&p3, 5));

        //Retorno de monto total sin descuento (ya que cli1 no tiene newsletter)
        assert_eq!(v1.monto_total(),15500.0);
    }

    #[test]
    fn operar_sistema(){
        //Sistema
        let mut sis = Sistema::new(&CategPorcentajes(0.0, 60.0, 40.0, 0.0), &"correo@example.com".to_string());

        //Personas
        let cli1 = Cliente::new(&"Lucas".to_string(), &"Daniel".to_string(), &"AvBelgrano".to_string(), 871265);
        let mut cli2 = Cliente::new(&"Mariana".to_string(), &"Santos".to_string(), &"Centenario".to_string(), 2987865);
        //Otorga cli2 el newsletter por parte del sistema
        sis.otorgar_newsletter(&mut cli2);
        let ven1 = Vendedor::new(&"Tobias".to_string(), &"Serio".to_string(), &"AvBelgrano".to_string(), 237863, 9876, 2, 12000.0);

        //Productos registrados
        let p1 = Producto::new(&"CocaCola".to_string(), &sis.definir_categoria(&Categorias::Alimento(0.0)), 3500.0);
        let p2 = Producto::new(&"Escoba".to_string(), &sis.definir_categoria(&Categorias::Limpieza(0.0)), 1000.0);
        let p3 = Producto::new(&"ElEjemplo".to_string(), &Categorias::Bazar(0.0), 1500.0);

        //Generar ventas
        let mut v1 = Venta::new(&Fecha::new(05, 02, 2025), &cli1, &ven1, &MediosDePago::Efectivo);
        v1.agregar_producto(&ProductoVendido::new(&p1, 2));
        v1.agregar_producto(&ProductoVendido::new(&p2, 1));
        v1.agregar_producto(&ProductoVendido::new(&p3, 5));

        let mut v2 = Venta::new(&Fecha::new(15, 6, 2025), &cli2, &ven1, &MediosDePago::TarjetaDébito);
        v2.agregar_producto(&ProductoVendido::new(&p1, 1));
        v2.agregar_producto(&ProductoVendido::new(&p2, 2));
        v2.agregar_producto(&ProductoVendido::new(&p3, 3));
        
        let v3 = Venta::new(&Fecha::new(25, 8, 2025), &cli1, &ven1, &MediosDePago::TarjetaDébito);  //Sin productos y sin registrar en el sistema

        //Operar en el sistema
        sis.agregar_venta(&v1);
        sis.agregar_venta(&v2);

        //Retorno de ventas por categorias
        let res = sis.retornar_ventas_por_categoria(&sis.definir_categoria(&Categorias::Otro(0.0)));
        assert!(res.is_empty());

        let res = sis.retornar_ventas_por_categoria(&sis.definir_categoria(&Categorias::Alimento(0.0)));
        assert!(!res.is_empty());

        //Retorno de ventas por vendedor
        let res = sis.retornar_ventas_por_vendedor(&ven1);
        assert!(!res.is_empty());

        let res = sis.retornar_ventas_por_vendedor(&Vendedor::new(&"Matias".to_string(), &"Serio".to_string(), &"AvBelgrano".to_string(), 237863, 9876, 2, 12000.0));
        assert!(res.is_empty());

        //Retorno monto final (sin descuento)
        if let Some(monto) = sis.monto_final_venta(&v1){
            assert_eq!(monto,15500.0);
        }

        //Retorno monto final (con descuento)
        if let Some(monto) = sis.monto_final_venta(&v2){
            assert_eq!(monto,9200.0);
        }

        //Retorno nulo de una venta no registrada
        assert!(sis.monto_final_venta(&v3).is_none());
    }
}

/*
    IMPLEMENTACION PRINCIPAL ENTREGABLE 2
*/

//Extrae los datos necesarios para la resolucion (solo lo pedido)
#[derive(Debug)]
pub struct Compra{
    fecha : Fecha,
    medio_pago : MediosDePago,
    productos : Vec<ProductoVendido>,
    monto_final : f64
}

impl Compra{
    fn new(f:&Fecha,medio:&MediosDePago,prods:&Vec<ProductoVendido>,monto:f64)->Compra{
        return Compra{
            fecha : f.clone(),
            medio_pago : medio.clone(),
            productos : prods.clone(),
            monto_final : monto
        }
    }
    fn get_fecha(&self)->Fecha{
        return self.fecha.clone();
    }
    fn get_monto_final(&self)->f64{
        return self.monto_final;
    }
}

#[derive(Debug)]
pub struct Informe{
    compras : Vec<Compra>
}

impl Informe{
    //Funciones secundarias
    fn new()->Informe{
        return Informe{compras:Vec::new()}
    }
    fn is_vacio(&self)->bool{
        return self.compras.is_empty();
    }
    //Funciones primarias (las propuestas inicialmente)

    //Recibe un venta para obtener la informacion necesaria para el informe
    fn procesar_venta(&mut self,v:&Venta,monto:f64){
        let compra = Compra::new(&v.fecha,&v.medio_pago,&v.productos,monto);
        self.compras.push(compra);
    }
    //Ordena de menor a mayor para cada espacio dia/mes/anio
    fn ordenar_cronologia(&mut self){
        self.compras.sort_by_key(|c| c.get_fecha().0);
        self.compras.sort_by_key(|c| c.get_fecha().1);
        self.compras.sort_by_key(|c| c.get_fecha().2);
    }
}

impl Sistema {
    //Se utiliza el dni del cliente como criterio principal para el algoritmo
    fn get_historial_compras(&self,id:u64,monto_minimo:f64)->Option<Informe>{
        let mut res : Option<Informe> = None;

        //Se procesa las ventas que se disponen
        if !self.ventas.is_empty(){
            let mut info = Informe::new();
            //Filtra las ventas de un cliente por dni y si tienen un monto final mayor a "monto_minimo"
            self.ventas.iter().for_each(|venta|{  
                let monto = venta.monto_total();
                if(venta.cliente.get_dni(&venta.cliente.datos_cliente) == id)&&(monto>monto_minimo){
                    info.procesar_venta(&venta,monto);
                }
            });
            if !info.is_vacio(){
                info.ordenar_cronologia();
                res = Some(info);
            }
        }

        return res;
    }
}

#[cfg(test)]
mod test_entregable2{    
    use super::*;

    //1º Test
    //Se observa el correcto funcionamiento del metodo implementado (funcionamiento minimo)
    #[test]
    fn creacion_informe(){
        //Sistema
        let sis = Sistema::new(&CategPorcentajes(0.0, 60.0, 40.0, 0.0), &"correo@example.com".to_string());

        //No retornara un informe (el sistema no dispone de ninguna venta hecha,inclusive de datos como clientes,vendedores o productos)
        assert!(sis.get_historial_compras(123,1000.0).is_none(),"Se esperaba un retorno de None");
    }

    //2º Test
    //Se evaluan los diferentes casos de retorno del nuevo metodo implementado
    #[test]
    fn operar_sistema_informes(){
        //Sistema
        let mut sis = Sistema::new(&CategPorcentajes(0.0, 60.0, 40.0, 0.0), &"correo@example.com".to_string());

        //Personas
        let cli1 = Cliente::new(&"Lucas".to_string(), &"Daniel".to_string(), &"AvBelgrano".to_string(), 871265);
        let mut cli2 = Cliente::new(&"Mariana".to_string(), &"Santos".to_string(), &"Centenario".to_string(), 2987865);
        //Otorga cli2 el newsletter por parte del sistema
        sis.otorgar_newsletter(&mut cli2);
        let ven1 = Vendedor::new(&"Tobias".to_string(), &"Serio".to_string(), &"AvBelgrano".to_string(), 237863, 9876, 2, 12000.0);

        //Productos registrados
        let p1 = Producto::new(&"CocaCola".to_string(), &sis.definir_categoria(&Categorias::Alimento(0.0)), 3500.0);
        let p2 = Producto::new(&"Escoba".to_string(), &sis.definir_categoria(&Categorias::Limpieza(0.0)), 1000.0);
        let p3 = Producto::new(&"ElEjemplo".to_string(), &sis.definir_categoria(&Categorias::Bazar(0.0)), 1500.0);

        //Generar ventas con cli1
        let mut v1 = Venta::new(&Fecha::new(05, 02, 2025), &cli1, &ven1, &MediosDePago::Efectivo);
        v1.agregar_producto(&ProductoVendido::new(&p1, 2));
        v1.agregar_producto(&ProductoVendido::new(&p2, 1));
        v1.agregar_producto(&ProductoVendido::new(&p3, 5));

        let mut v2 = Venta::new(&Fecha::new(15, 6, 2025), &cli1, &ven1, &MediosDePago::TarjetaDébito);
        v2.agregar_producto(&ProductoVendido::new(&p1, 1));
        v2.agregar_producto(&ProductoVendido::new(&p2, 2));
        v2.agregar_producto(&ProductoVendido::new(&p3, 3));
        
        //Agregar ventas al sistema
        sis.agregar_venta(&v1);
        sis.agregar_venta(&v2);

        //Caso 1º
        //No retornara un informe (el sistema no encuentra el dni en el registro de ventas)
        assert!(sis.get_historial_compras(2987865,1000.0).is_none(),"Se esperaba un retorno de None");

        //Caso 2º
        //No retornara un informe (el sistema encuentra un dni pero el mismo no cumple la condicion de monto minimo)
        assert!(sis.get_historial_compras(871265,900000.0).is_none(),"Se esperaba un retorno de None");

        //Caso 3º
        //Retorna un informe (el sistema encuentra un dni y cumple la condicion de monto minimo)
        assert!(sis.get_historial_compras(871265,1000.0).is_some(),"Se esperaba un retorno de un informe");
    }

    //3º Test extra
    //Valida si el informe cumple con las condiciones pedidas:
        //Las compras estan ordenadas por orden cronologico
        //Cumplen la condicion de monto minimo
    #[test]
    fn validar_informacion_informe(){
        //Sistema
        let mut sis = Sistema::new(&CategPorcentajes(0.0, 60.0, 40.0, 0.0), &"correo@example.com".to_string());

        //Personas (Solo se trabaja con un unico cliente)
        let mut cli1 = Cliente::new(&"Lucas".to_string(), &"Daniel".to_string(), &"AvBelgrano".to_string(), 871265);
        
        let ven1 = Vendedor::new(&"Tobias".to_string(), &"Serio".to_string(), &"AvBelgrano".to_string(), 237863, 9876, 2, 12000.0);

        //Productos registrados
        let p1 = Producto::new(&"CocaCola".to_string(), &sis.definir_categoria(&Categorias::Alimento(0.0)), 3500.0);
        let p2 = Producto::new(&"Escoba".to_string(), &sis.definir_categoria(&Categorias::Limpieza(0.0)), 1000.0);
        let p3 = Producto::new(&"ElEjemplo".to_string(), &sis.definir_categoria(&Categorias::Bazar(0.0)), 1500.0);
        let p4 = Producto::new(&"Shampoo".to_string(),&sis.definir_categoria(&Categorias::Limpieza(0.0)),3000.0);

        //Generar ventas con cli1
        let mut v1 = Venta::new(&Fecha::new(05, 02, 2024), &cli1, &ven1, &MediosDePago::Efectivo);
        v1.agregar_producto(&ProductoVendido::new(&p1, 3));
        v1.agregar_producto(&ProductoVendido::new(&p2, 1));
        v1.agregar_producto(&ProductoVendido::new(&p3, 5));

        let mut v2 = Venta::new(&Fecha::new(15, 6, 2025), &cli1, &ven1, &MediosDePago::Efectivo);
        v2.agregar_producto(&ProductoVendido::new(&p1, 5));
        v2.agregar_producto(&ProductoVendido::new(&p2, 2));
        v2.agregar_producto(&ProductoVendido::new(&p3, 3));

        //Agregar ventas al sistema
        sis.agregar_venta(&v1);
        sis.agregar_venta(&v2);

        //Verificacion sin modificaciones en el sistema
        //Se verifica si retorna el informe , ya que existe el cliente con el dni ingresado en el sistema
        if let Some(info) = sis.get_historial_compras(871265,10000.0){
            //Verifico si el informe incluye las dos compras hechas por el cliente (ya que deben cumplir con las condiciones de monto minimo y pertenecientes a un dni ingresado)
            assert_eq!(info.compras.len(),2);
            //Se demuestra que cumplen con la condicion de monto minimo cada compra
            for compra in &info.compras{
                assert!(compra.get_monto_final() > 10000.0 ,"Una de las compras no cumple la condicion");
                assert_eq!(compra.medio_pago,MediosDePago::Efectivo); //Validacion extra del medio de pago
            }
            //Verifico que se cumple la condicion de orden cronologico
            assert_eq!(info.compras[0].get_fecha(),Fecha(5, 2, 2024)); //Primer fecha
            assert_eq!(info.compras[info.compras.len()-1].get_fecha(),Fecha(15, 6, 2025)); //Ultima fecha
        }else{
            panic!("No se genero el informe correctamente");
        }


        //Se agregan nuevas ventas al sistema

        let mut v3 = Venta::new(&Fecha::new(25, 1, 2023), &cli1, &ven1, &MediosDePago::Efectivo);
        v3.agregar_producto(&ProductoVendido::new(&p1, 4));
        v3.agregar_producto(&ProductoVendido::new(&p2, 1));
        v3.agregar_producto(&ProductoVendido::new(&p3, 5));
        v3.agregar_producto(&ProductoVendido::new(&p4, 2));

        let mut v4 = Venta::new(&Fecha::new(24, 12, 2025), &cli1, &ven1, &MediosDePago::Efectivo);
        v4.agregar_producto(&ProductoVendido::new(&p1, 4));
        v4.agregar_producto(&ProductoVendido::new(&p3, 4));

        sis.agregar_venta(&v3);
        sis.agregar_venta(&v4);

        //Otorga cli1 el newsletter por parte del sistema
        sis.otorgar_newsletter(&mut cli1);

        //Verificacion con modificaciones en el sistema
        //Se vuelve a verificar si retorna el informe correctamente para el mismo cliente existente
        if let Some(info) = sis.get_historial_compras(871265,10000.0){
            //Verifico si el informe incluye las cuatro compras hechas por el cliente (ya que deben cumplir con las condiciones de monto minimo y pertenecientes a un dni ingresado)
            assert_eq!(info.compras.len(),4);
            //Se demuestra que cumplen con la condicion de monto minimo cada compra
            for compra in &info.compras{
                assert!(compra.get_monto_final() > 10000.0 ,"Una de las compras no cumple la condicion");
                assert_eq!(compra.medio_pago,MediosDePago::Efectivo); //Validacion extra del medio de pago
            }
            //Verifico que se cumple la condicion de orden cronologico
            assert_eq!(info.compras[0].get_fecha(),Fecha(25, 1, 2023)); //Primer fecha
            assert_eq!(info.compras[info.compras.len()-1].get_fecha(),Fecha(24, 12, 2025)); //Ultima fecha
        }else{
            panic!("No se genero el informe correctamente");
        }
    }

}

