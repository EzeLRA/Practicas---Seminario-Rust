pub struct Blockchain{
    nombre : String,
    prefijo : String
}

pub struct Criptomoneda{
    nombre : String,
    prefijo : String,
    blockchains : Vec<Blockchain>
}

pub struct Usuario{
    nombre : String,
    apellido : String,
    email : String,
    dni : u64,
    validado : bool,
    balance : Vec<Criptomoneda>
}