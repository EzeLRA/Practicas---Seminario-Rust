const X: i32 = 2;
fn main() {
    let mut arreglo : [i32;6] = [4,5,6,7,2,1] ;
    for i in 0..6{
        arreglo[i] = arreglo[i] * X;
    }
    for nums in arreglo.iter(){
        println!("{}",nums);
    }
}
