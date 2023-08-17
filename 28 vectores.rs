//Vectores rust
fn main(){
    let mut a = vec![2,4,6,8,10];
    for i in &a {
        println!("Valor {}", i);
    }
    println!("El tamaño del vector es {}", a.len());
    let ultimo = a.pop();
    for i in &a {
        println!("Valor {}", i);
    }
    println!("El elemento eliminado es {:?}", ultimo);
    println!("El tamaño del vector es {}", a.len());
    a.push(12);
    for i in &a {
        println!("Valor {}", i);
    }
    println!("El tamaño del vector es {}", a.len());
}