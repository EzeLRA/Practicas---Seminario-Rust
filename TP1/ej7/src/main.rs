const 	X: i32 = 2;
const	DIM_F:usize = 6;
fn main() {
    let mut arreglo : [i32;DIM_F] = [4,5,6,7,2,1] ;
    for i in 0..DIM_F{
        arreglo[i] = arreglo[i] * X;
    }
    for nums in arreglo.iter(){
        println!("{}",nums);
    }
}
