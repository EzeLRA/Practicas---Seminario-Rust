//Funciones publicas
pub fn cantidad_impares(arr : &[i32]) -> u32{
	let mut res = 0;
	for num in arr.iter(){
		if num % 2 == 1 {res += 1;}
	}
	return res; 
}