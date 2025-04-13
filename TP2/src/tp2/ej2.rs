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