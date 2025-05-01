/* 
    Estructura Fecha
*/

//Atributos
#[derive(PartialEq, Debug)]
pub struct Fecha{
    pub dia : u8,
    pub mes : u8,
    pub anio : u16
}

/*
    Metodos
*/

impl Fecha{
    pub fn new(d:u8,m:u8,a:u16)->Fecha{
        return Fecha { dia: d , mes: m , anio: a }
    }
    pub fn es_fecha_valida(&self)->bool{
        
        if((self.mes > 0) && (self.mes <= 12) && (self.anio > 0)){
            if(self.dia > 0){
                if(self.es_bisiesto()){
                    if((self.dia <= 29)&&(self.mes == 2)){
                        return true;
                    }
                }else{
                    if((self.dia <= 28)&&(self.mes == 2)){
                        return true;
                    }
                }
                if((self.mes == 9)||(self.mes == 4)||(self.mes == 6)||(self.mes == 11)){
                    return (self.dia <= 30);
                }else{
                    return (self.dia <= 31);
                }
            }
        }

        return false;
    }

    pub fn es_bisiesto(&self)->bool{
        return (self.anio % 4)==0;
    }

    //Auxiliar para determinar el ultimo dia de un mes
    fn ultimo_dia(&self)->u8{
        
        
                if(self.es_bisiesto()){
                    if((self.dia <= 29)&&(self.mes == 2)){
                        return true;
                    }
                }else{
                    if((self.dia <= 28)&&(self.mes == 2)){
                        return true;
                    }
                }
                if((self.mes == 9)||(self.mes == 4)||(self.mes == 6)||(self.mes == 11)){
                    return (self.dia <= 30);
                }else{
                    return (self.dia <= 31);
                }
            
        

        
    }

    //Se considera que la fecha es valida
    pub fn sumar_dias(&mut self,dias:u8){
        let mut cant = 0;
        while(cant < dias){
            
        }
    }

}