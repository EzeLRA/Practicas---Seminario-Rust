//Funciones Publicas
pub fn es_par(x:u32) -> bool {
	return (x % 2) == 0;
}

#[cfg(test)]
mod testing_ejercicio1{
    use crate::tp2::ej1;

    #[test]
    fn test_numeros_pares(){
        let mut res = true;
        let numeros_pares = [0 ,2, 4, 6, 8, 10];
        for num in numeros_pares {
            res = ej1::es_par(num);
            if !res {
                break;
            }
        }
        assert_eq!(res,true);
    }

    #[test]
    fn test_numeros_impares(){
        let mut res = true;
        let numeros_impares = [1 ,3, 5, 7, 9, 11];
        for num in numeros_impares {
            res = !(ej1::es_par(num));
            if !res {
                break;
            }
        }
        assert_eq!(res,true);
    }

}