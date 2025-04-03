const DIM_F:usize = 5;
fn main() {
    let mut total:i32 = 0;
    let arreglo : [i32;DIM_F] = [2,8,10,23,4];
    for num in arreglo.iter(){
        total = total + num;
    }
    println!("{}", total);
}
