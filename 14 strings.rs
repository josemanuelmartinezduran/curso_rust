//Todo valor tiene un dueño
//Un valor no puede tener mas de un dueño
//Cuando el valor se sale del 'scope' se destruye

fn main(){
    //String al menos 6 tipos diferentes de strings
    //No usar nunca!!
    let a = String::from("Hola mundo");
    println!("A contiene {}", a);
    let b = a.clone();
    println!("B contiene {}", b);
    println!("A contiene {}", a);
    //Todas las variables que no son primitivas, 
}