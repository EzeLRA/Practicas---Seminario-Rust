//Funciones publicas
pub fn longitud_de_cadenas<const N:usize>(arr:[&str;N]) -> [u32 ; N]{
    let mut arr_nuevo : [u32;N] = [0;N];
    for i in 0..N{
        arr_nuevo[i] = arr[i].chars().count() as u32;
    }
    return arr_nuevo;
}