pub fn ordenar_nombres<const N: usize>(arr: &mut [String; N]) {
    arr.sort();
}

#[cfg(test)]
mod testing_ejercicio13{
    use crate::tp2::ej13;
    #[test]
    fn test_ordenar_nombres(){
        let mut noms : [String;3] = [String::from("Marcos"),String::from("Beto"),String::from("Tolosa")];
        ej13::ordenar_nombres(&mut noms);
        assert_eq!(noms[0],String::from("Beto"));
    }
}