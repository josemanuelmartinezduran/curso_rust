//Vectores rust
// Escribir un programam en rust que imprima los n numeros pares

fn genera_pares(cantidad: u8) -> Vec<u32> {
    let mut v = Vec::new();
    for i in 0..cantidad {
        v.push((2 * (i as u32)) as u32);
    }
    v
}

fn main(){
    let v = genera_pares(50);
    for i in v{
        println!("El numero es {}", i);
    }
}