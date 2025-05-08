//Funciones publicas
pub fn cantidad_impares(arr : &[i32]) -> u32{
	let mut res = 0;
	for num in arr.iter(){
		if num % 2 == 1 {res += 1;}
	}
	return res; 
}

#[cfg(test)]
mod testing_ejercicio4{
	use crate::tp2::ej4;
	#[test]
	fn test_cantidad_impares(){
		let arr : [i32;5] = [3,5,7,11,13];
		let res : u32 = ej4::cantidad_impares(&arr);

		assert_eq!((res == arr.len() as u32 ),true);
	}
}