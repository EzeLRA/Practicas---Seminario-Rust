//Funciones Publicas
pub fn sumar_pares(arr : &[i32]) -> i32{
	let mut res = 0;
	for num in arr.iter(){
		if num % 2 == 0 {res += num;}
	}
	return res; 
}