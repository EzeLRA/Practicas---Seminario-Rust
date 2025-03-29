fn main() {
    let arr1 : [i32;5] = [2,8,10,23,6];
    let arr2 : [i32;5] = [4,1,2,12,32];
    let mut arr3 : [i32;5] = [0,0,0,0,0];

    for i in 0..5{
        arr3[i] = arr1[i] + arr2[i];
    }

    for num in arr3.iter(){
        println!("{}", num);
    }
}
