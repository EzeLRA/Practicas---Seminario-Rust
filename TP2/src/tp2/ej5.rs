//Funciones publicas
pub fn duplicar_valores_arr<const N:usize>(arr : [f32;N])-> [f32;N] {
    let mut res:[f32;N] = [0.0;N];
    for i in 0..N{
        res[i] = arr[i] * 2.0;
    }
    return res;
}

#[cfg(test)]
mod testing_ejercicio5{
    use crate::tp2::ej5;
    #[test]
    fn test_duplicar_elementos_array(){
        let arr1 : [f32;3] = [2.0,4.0,8.0];
        let mut arr2 = ej5::duplicar_valores_arr(arr1);
        for i in 0..arr2.len(){
            arr2[i] = arr2[i] / 2.0;
        }
        assert_eq!((arr1 == arr2),true);
    }
}