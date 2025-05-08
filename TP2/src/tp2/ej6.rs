//Funciones publicas
pub fn longitud_de_cadenas<const N:usize>(arr:[&str;N]) -> [u32 ; N]{
    let mut arr_nuevo : [u32;N] = [0;N];
    for i in 0..N{
        arr_nuevo[i] = arr[i].chars().count() as u32;
    }
    return arr_nuevo;
}

#[cfg(test)]
mod testing_ejercicio6{
    use crate::tp2::ej6;
    #[test]
    fn test_longitud_cadenas(){
        let arr = ej6::longitud_de_cadenas(["asdf"]);
        assert_eq!(arr[0],4);
    }
}
