pub fn multiplicar_valores<const N:usize>(arr : &mut [i32;N] , factor : i32){
    for i in 0..arr.len(){
        arr[i] = arr[i] * factor;
    }
}