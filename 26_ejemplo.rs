struct Ave {
    especie:String
}

impl Ave {
    fn new(especie:String)-> Self{
        Self{especie: format!("{}",especie)}
    }
    
    fn imprime(a:&Ave){
        println!("La especie del ave es {}", a.especie);
    }
}

fn main(){
   let a = Ave{especie: "Gaviota".to_string()};
   
}