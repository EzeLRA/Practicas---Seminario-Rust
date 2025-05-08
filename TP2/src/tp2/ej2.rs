//Funciones publicas
pub fn es_primo(x:u32) -> bool{
	let mut resultado = true;
	let mut divisibles = 2;

	while (divisibles < x)&&(resultado) {
		if x % divisibles == 0 {
			resultado = false;
		}else{
			divisibles +=1 ;
		}
	}

	return resultado;
}


#[cfg(test)]
mod testing_ejercicio2{
	use crate::tp2::ej2;
	#[test]
	fn test_numeros_primos(){
		let mut res = true;
		let numeros_primos = [2, 3, 5, 7, 11];
		for num in numeros_primos {
			res = ej2::es_primo(num);
			if !res {
				break;
			}
		}
		assert_eq!(res,true);
	}
}