pub fn incrementar(num : &mut f32){
    *num += 1.0;
}

#[cfg(test)]
mod testing_ejercicio14{
    use crate::tp2::ej14;
    #[test]
    fn test_incrementar(){
        let mut num : f32 = 9.0;
        ej14::incrementar(&mut num);
        assert_eq!(num,10.0);
    }
}