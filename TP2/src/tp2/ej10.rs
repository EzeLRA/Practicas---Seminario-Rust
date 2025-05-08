pub fn cantidad_de_cadenas_mayor_a<const N:usize>(arr : [String;N] , lim : u32)-> u32{
    let mut cant : u32 = 0;
    for cadena in arr{
        if cadena.len() as u32 > lim {
            cant += 1;
        }
    }
    return cant;
}

#[cfg(test)]
mod testing_ejercicio10{
    use crate::tp2::ej10;
    #[test]
    fn test_cadenas_mayores_a(){
        let cant = ej10::cantidad_de_cadenas_mayor_a([String::from("Alexander"),String::from("Martinez"),String::from("Fernandez")], 5);
        assert_eq!(cant,3);
    }
}
