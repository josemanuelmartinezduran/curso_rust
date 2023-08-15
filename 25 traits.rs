struct Ave{
    nombre: String
}

trait Sonido {
    fn haz_sonido(&self) -> String;
}

impl Ave{
    fn new()->Self{Self{nombre: "Pajaro".to_string()}}
    fn vuela(&self)->(){
        println!("Estoy volando {}", self.nombre)
    }
}

impl Sonido for Ave{
    fn haz_sonido(&self) -> String {
        "Cuak??".to_string();
    }
}

fn imprime_sonido<T: Sonido>(item: t) -> (){
    println!("{}", item.haz_sonido);
}

fn main(){
   let a = Ave::new();
   a.vuela();
}