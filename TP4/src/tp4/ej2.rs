use std::collections::LinkedList;

//Estructura Persona 
#[derive(Debug,Clone)]
pub struct Persona<'a>{
	nombre:&'a str,
	apellido:&'a str,
	direccion:&'a str,
	ciudad:&'a str,
	salario:f64,
	edad:u8,
}

//Trait
pub trait DatosPersona<'a>{
	fn new(nom:&'a str,ape:&'a str,dir:&'a str,ci:&'a str,s:f64,e:u8)->Persona<'a>{
		return Persona{
			nombre : nom,
			apellido : ape,
			direccion : dir,
			ciudad : ci,
			salario : s,
			edad : e
		}
	}
	fn obtener_nombre(&self)->String;
	fn obtener_apellido(&self)->String;
	fn obtener_direccion(&self)->String;
	fn obtener_ciudad(&self)->String;
	fn obtener_salario(&self)->f64;
	fn obtener_edad(&self)->u8;
	fn es_igual_a(&self, p: Persona<'a>)->bool;
	//primarios
	fn salario_mayor_a(&self,s:f64)->bool;
}

//Persona
impl<'a> DatosPersona<'a> for Persona<'a> {
	//Metodos secundarios
	fn obtener_nombre(&self)->String{
		return self.nombre.to_string().clone();
	}
	fn obtener_apellido(&self)->String{
		return self.apellido.to_string().clone();
	}
	fn obtener_direccion(&self)->String{
		return self.direccion.to_string().clone();
	}
	fn obtener_ciudad(&self)->String{
		return self.ciudad.to_string().clone();
	}
	fn obtener_salario(&self)->f64{
		return self.salario;
	}
	fn obtener_edad(&self)->u8{
		return self.edad;
	}
	fn es_igual_a(&self, p: Persona<'a>)->bool{
		return (self.nombre == p.obtener_nombre())&&(self.apellido == p.obtener_apellido())&&(self.direccion == p.obtener_direccion())&&
		(self.ciudad == p.obtener_ciudad())&&(self.salario == p.obtener_salario())&&(self.edad == p.obtener_edad());
	}

	//Metodos primarios
	fn salario_mayor_a(&self,salario_in:f64)->bool{
		return self.salario > salario_in;
	}

}

//Trait primario

pub trait PersonaIteratorExt<'a>: Iterator<Item = &'a Persona<'a>> + Sized {
    fn modulo_a(self, monto: f64) -> LinkedList<Persona<'a>>
	where
        Persona<'a>: Clone,
    {
        return self.filter(|p| p.obtener_salario() > monto).cloned().collect()
    }
	fn modulo_b(self, edad:u8 , nom_ciu : String)-> LinkedList<Persona<'a>>
	where
        Persona<'a>: Clone,
	{
		return self.filter(|p| (p.obtener_edad() > edad)&&(p.obtener_ciudad() == nom_ciu)).cloned().collect()
	}
	fn modulo_c(self, nom_ciu : String)-> bool
	where
        Persona<'a>: Clone,
	{	
		for p in self.cloned() {
			if p.obtener_ciudad() != nom_ciu {
				return false;
			}
		}
		return true;
	}
	fn modulo_d(self, nom_ciu : String)-> bool
	where
        Persona<'a>: Clone,
	{	
		let mut it = self.cloned();
		return it.any(|p| p.obtener_ciudad() == nom_ciu);
	}
	fn modulo_e(self, p2 : Persona<'a>)-> bool
	where
        Persona<'a>: Clone,
	{	
		let mut it = self.cloned();
		return it.any(|p| p.es_igual_a(p2.clone() ) );
	}
	fn modulo_f<const N:usize>(self)-> [u8;N]
	where
    	Self: Sized, 
	{
    	let mut result = [0u8; N]; 

	    for (i, edad) in self.map(|p| p.obtener_edad()).enumerate().take(N) {
	        result[i] = edad;
	    }
    
    	return result;
	}
	//Revisar que las personas se reciban como option (corregir)
	fn modulo_g<const N:usize>(self, pMax:&mut Persona<'a> , pMin:&mut Persona<'a>)
	where
    	Self: Sized,
    	Persona<'a>: Clone,
	{
		let mut max : f64 = -1.0;
		let mut min : f64 = f64::MAX;
    	
    	for persona in self {
    		let salario = persona.obtener_salario();
    		let edad = persona.obtener_edad();
    		//Procesa pMax
    		if (salario > max) || (salario == max && edad > pMax.obtener_edad()){
    			max = salario;
    			*pMax = persona.clone();
    		}
    		//Procesa pMin
    		if (salario < min) || (salario == max && edad < pMin.obtener_edad()) {
    			min = salario;
    			*pMin = persona.clone();
    		}
    	}
	}

}

impl<'a, I> PersonaIteratorExt<'a> for I
where
    I: Iterator<Item = &'a Persona<'a>>,
{
    // Usa la implementación por defecto del trait
}

#[cfg(test)]
mod test_ejercicio2{
	use super::*;

	#[test]
	fn procesar_listados(){
		//Personas con salarios mayor a un monto
		let mut vector : Vec<Persona> = Vec::new();
		vector.push(Persona::new("Carlos","Maro","AvSanMartin","Buenos Aires",1500.0,30));
		vector.push(Persona::new("Maria","Mercedes","AvBelgrano","Buenos Aires",2000.0,25));
		vector.push(Persona::new("Julian","Wen","AvLibertad","Buenos Aires",2800.0,28));

		let res = vector.iter().modulo_a(1000.0);
		
		if !res.is_empty() {
			for p in res.iter(){
				assert_eq!(p.salario_mayor_a(1000.0),true);
			}
		}else{
			panic!("La estructura esta vacia , porque 'vector' no se cargo ninguna persona con tal condicion");
		}

		//Personas mayores a una edad y residentes de una ciudad
		let res = vector.iter().modulo_b(10,"Buenos Aires".to_string());
		
		if !res.is_empty() {
			for p in res.iter(){
				assert_eq!((p.obtener_edad() > 10),true);
				assert_eq!(p.obtener_ciudad() == "Buenos Aires".to_string(),true);
			}
		}else{
			panic!("La estructura esta vacia , porque 'vector' no se cargo ninguna persona con tal condicion");
		}

	}

	#[test]
	fn personas_ciudadanos(){
		//Personas de Buenos Aires
		let mut vector : Vec<Persona> = Vec::new();
		vector.push(Persona::new("Carlos","Maro","AvSanMartin","Buenos Aires",1500.0,30));
		vector.push(Persona::new("Maria","Mercedes","AvBelgrano","Buenos Aires",2000.0,25));
		vector.push(Persona::new("Julian","Wen","AvLibertad","Buenos Aires",2800.0,28));
		
		assert_eq!(vector.iter().modulo_d("La Plata".to_string()),false);
		assert!(vector.iter().modulo_c("Buenos Aires".to_string()));

		vector.push(Persona::new("Julio","Mora","Av1yCa2","La Plata",3800.0,28));

		assert_eq!(vector.iter().modulo_c("Buenos Aires".to_string()),false);
		assert_eq!(vector.iter().modulo_d("La Plata".to_string()),true);

		//Verficar que no exista un problema de borrow
		assert!(vector.iter().modulo_e(Persona::new("Carlos","Maro","AvSanMartin","Buenos Aires",1500.0,30)));
	}

	//Consultar si se debe usar arrays o ArrayList
	#[test]
	fn procesar_arrays(){
		const N : usize = 3;
		let personas: [Persona; N] = [
		    Persona::new("Juan","Cruz", "Av1y12","Buenos Aires", 2500.0, 30),
		    Persona::new("Ana","Lucia", "Pichincha","Buenos Aires" ,3000.0, 25),
		    Persona::new("Carlos","Del Monte", "AvVenezuela","Buenos Aires" ,2500.0, 40), // Mismo salario que Juan pero mayor edad
		];
		let res: [u8; N] = personas.iter().modulo_f();
		assert_eq!(res.is_empty(),false);
		//Probar mas cosas

		//Probar modulo_g
	}

}
