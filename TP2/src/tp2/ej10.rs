pub fn cantidad_de_cadenas_mayor_a<const N:usize>(arr : [String;N] , lim : u32)-> u32{
    let mut cant : u32 = 0;
    for cadena in arr{
        if(cadena.len() as u32 > lim){
            cant += 1;
        }
    }
    return cant;
}