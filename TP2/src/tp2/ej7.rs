pub fn cantidad_de_mayores<const N:usize>(arr : [i32;N] , limite : i32)-> u32{
    let mut cant:u32 = 0;
    for num in arr{
        if limite < num {
            cant += 1;
        }
    }
    return cant;
}