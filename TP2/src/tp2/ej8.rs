pub fn sumar_arreglos<const N:usize>(arr1:[f32;N] , arr2:[f32;N])-> [f32;N]{
    let mut res : [f32;N] = [0.0;N];
    for i in 0..N{
        res[i] = arr1[i] + arr2[i];
    }
    return res;
}