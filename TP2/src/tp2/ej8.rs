pub fn sumar_arreglos<const N:usize>(arr1:[f32;N] , arr2:[f32;N])-> [f32;N]{
    let mut res : [f32;N] = [0.0;N];
    for i in 0..N{
        res[i] = arr1[i] + arr2[i];
    }
    return res;
}

#[cfg(test)]
mod testing_ejercicio8{
    use crate::tp2::ej8;
    #[test]
    fn test_suma_arrays(){
        let arr = ej8::sumar_arreglos([10.0,43.5,23.5], [2.0,54.5,5.2]);
        assert_eq!(arr[0],12.0);
    }
}
