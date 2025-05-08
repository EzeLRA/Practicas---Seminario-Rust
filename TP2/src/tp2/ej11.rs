pub fn multiplicar_valores<const N:usize>(arr : &mut [i32;N] , factor : i32){
    for i in 0..arr.len(){
        arr[i] = arr[i] * factor;
    }
}

#[cfg(test)]
mod testing_ejercicio11{
    use crate::tp2::ej11;
    #[test]
    fn test_multiplicar_valores(){
        let mut arr : [i32;5] = [5,4,2,8,9];
        ej11::multiplicar_valores(&mut arr, 5);
        assert_eq!(arr[0],25);
    }
}
