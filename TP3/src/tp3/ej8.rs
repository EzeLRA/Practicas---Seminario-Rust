/*
    Estructuras : Cancion , Generos y PlayList
*/
pub enum Generos{
    Rock,Pop,Rap,Jazz,Otros
}

pub struct Cancion{
    titulo : String,
    artista : String,
    genero : Generos
}

pub struct PlayList{
    nombre: String,
    canciones : Vec<Cancion>
}