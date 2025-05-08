pub fn reemplazar_pares<const N:usize>(arr : &mut [i32;N]){
    for i in 0..arr.len(){
        if arr[i] % 2 == 0 {
            arr[i] = -1;
        }
    }
}

#[cfg(test)]
mod testing_ejercicio12{
    use crate::tp2::ej12;
    #[test]
    fn test_reemplazar_pares(){
        let mut arr : [i32;3] = [2,4,8];
        ej12::reemplazar_pares(&mut arr);
        assert_eq!(arr[0] , -1);
    }
}