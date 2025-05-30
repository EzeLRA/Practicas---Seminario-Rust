pub enum Medios_de_pago{
    Tarjeta_credito, 
    Tarjeta_débito, 
    Transferencia_bancaria,
    Efectivo
}

//Implementar para que cada categoria tenga su propio porcentaje de descuento integrado
pub enum Categorias{
	Limpieza,
	Embutidos,
	Perfumeria
}

pub struct Producto {
    nombre : String,
    categoria : Categorias,
    precio_base : f64,   
}

pub struct Cliente {
    nombre : String,
    apellido : String,
    direccion : String,
    dni : u64,
    correo_newsletter : Option<String>
}

pub struct Vendedor {
    nombre : String,
    apellido : String,
    direccion : String,
    dni : u64,
    nro_legajo : u64,
    antiguedad : u8,
    salario : f64
}