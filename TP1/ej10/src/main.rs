const dimF:usize = 5; 
fn main() {
    let arr1 : [i32;dimF] = [2,8,10,23,6];
    let arr2 : [i32;dimF] = [4,1,2,12,32];
    let mut arr3 : [i32;dimF] = [0,0,0,0,0];
    //let mut arr3 : [i32;dimF]; Averiguar
    
    for i in 0..dimF{
        arr3[i] = arr1[i] + arr2[i];
    }

    for num in arr3.iter(){
        println!("{}", num);
    }
    
    //Para hacer casting ->"variable" as (tipo)
    
}
