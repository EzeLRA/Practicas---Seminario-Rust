//Funciones publicas
pub fn duplicar_valores_arr<const N:usize>(arr : [f32;N])-> [f32;N] {
    let mut res:[f32;N] = [0.0;N];
    for i in 0..N{
        res[i] = arr[i] * 2.0;
    }
    return res;
}