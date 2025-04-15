//Importacion de ejercicios
mod tp2;


//Programa principal
fn main() {
    //Inserte codigo
    println!("Programa principal");
    let arr = ["asd","fsdsdds","sadfg"];
    tp2::ej6::longitud_de_cadenas(arr);
}


/*
    Ejercicio 1
*/

#[test]
fn test_numeros_pares(){
    let mut res = true;
    let numeros_pares = [0 ,2, 4, 6, 8, 10];
    for num in numeros_pares {
        res = tp2::ej1::es_par(num);
        if !res {
            break;
        }
    }
    assert_eq!(res,true);
}

#[test]
fn test_numeros_impares(){
    let mut res = true;
    let numeros_impares = [1 ,3, 5, 7, 9, 11];
    for num in numeros_impares {
        res = !(tp2::ej1::es_par(num));
        if !res {
            break;
        }
    }
    assert_eq!(res,true);
}

/*
    Ejercicio 2
*/

#[test]
fn test_numeros_primos(){
    let mut res = true;
    let numeros_primos = [2, 3, 5, 7, 11];
    for num in numeros_primos {
        res = tp2::ej2::es_primo(num);
        if !res {
            break;
        }
    }
    assert_eq!(res,true);
}

/*
    Ejercicio 3
*/ 
#[test]
fn test_sumatoria_vector(){
    let arr : [i32;5] = [2,23,4,5,6];
    let res1 = tp2::ej3::sumar_pares(&arr);
    let mut res2=0;
    for num in arr.iter(){
        if num%2 == 0 {res2 += num}
    }
    assert_eq!((res1==res2),true);
}

/*
    Ejercicio 4
*/
#[test]
fn test_cantidad_impares(){
    let arr : [i32;5] = [3,5,7,11,13];
    let res : u32 = tp2::ej4::cantidad_impares(&arr);

    assert_eq!((res == arr.len() as u32 ),true);
}

/*
    EJercicio 5
*/
#[test]
fn test_duplicar_elementos_array(){
    let arr1 : [f32;3] = [2.0,4.0,8.0];
    let mut arr2 = tp2::ej5::duplicar_valores_arr(arr1);
    for i in 0..arr2.len(){
        arr2[i] = arr2[i] / 2.0;
    }
    assert_eq!((arr1 == arr2),true);
}

/*
    Ejercicio 6
*/
#[test]
fn test_longitud_cadenas(){
    let arr = tp2::ej6::longitud_de_cadenas(["asdf"]);
    assert_eq!(arr[0],4);
}
