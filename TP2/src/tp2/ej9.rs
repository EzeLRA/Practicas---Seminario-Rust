pub fn cantidad_en_rango<const N:usize>(arr : [i32;N], inf:i32 , sup:i32)-> u32{
    let mut cant:u32 = 0;
    for num in arr{
        if(num >= inf)&&(num <= sup){
            cant += 1;
        }
    }
    return cant;
}