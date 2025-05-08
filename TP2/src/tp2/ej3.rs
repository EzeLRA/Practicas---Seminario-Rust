//Funciones Publicas
pub fn sumar_pares<const N:usize>(arr : [i32;N]) -> i32{
	let mut res = 0;
	for num in arr.iter(){
		if num % 2 == 0 {res += num;}
	}
	return res; 
}

#[cfg(test)]
mod testing_ejercicio3{
    use crate::tp2::ej3;
    #[test]
    fn test_sumatoria_vector(){
        let arr : [i32;5] = [2,23,4,5,6];
        let res1 = ej3::sumar_pares(arr);
        let mut res2=0;
        for num in arr.iter(){
            if num%2 == 0 {res2 += num}
        }
        assert_eq!((res1==res2),true);
    }
}