pub fn cantidad_en_rango<const N:usize>(arr : [i32;N], inf:i32 , sup:i32)-> u32{
    let mut cant:u32 = 0;
    for num in arr{
        if(num >= inf)&&(num <= sup){
            cant += 1;
        }
    }
    return cant;
}

#[cfg(test)]
mod testing_ejercicio9{
    use crate::tp2::ej9;
    #[test]
    fn test_rangos(){
        let cant = ej9::cantidad_en_rango([-2,3,10,25], 0, 10);
        assert_eq!(cant,2);
    }
}