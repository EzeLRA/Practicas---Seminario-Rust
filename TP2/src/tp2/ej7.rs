pub fn cantidad_de_mayores<const N:usize>(arr : [i32;N] , limite : i32)-> u32{
    let mut cant:u32 = 0;
    for num in arr{
        if limite < num {
            cant += 1;
        }
    }
    return cant;
}

#[cfg(test)]
mod testing_ejercicio7{
    use crate::tp2::ej7;
    #[test]
    fn test_numeros_mayores(){
        let cant = ej7::cantidad_de_mayores([12,43,54,76], 20);
        assert_eq!(cant,3);
    }
}
