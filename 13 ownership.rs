//Todo valor tiene un dueño
//Un valor no puede tener mas de un dueño
//Cuando el valor se sale del 'scope' se destruye
fn main(){
    let a = String::from("Hola");
    println("Imprime a {}", a);
    let b = a;//a.clone();
    println("Imprime a {}", a);
}