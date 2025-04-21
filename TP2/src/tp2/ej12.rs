pub fn reemplazar_pares<const N:usize>(arr : &mut [i32;N]){
    for i in 0..arr.len(){
        if arr[i] % 2 == 0 {
            arr[i] = -1;
        }
    }
}