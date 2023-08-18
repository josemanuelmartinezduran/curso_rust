fn main() {
    let numeros = vec![1, 2, 3, 4, 5];
    let resultado: Vec<i32> = numeros.iter().map(|&x| x * 2).collect();
    println!("{:?}", resultado);
    
    let res: Vec<i32> = numeros.iter().filter(|&&x| x % 2 == 0).cloned().collect();
    println!("{:?}", res);
}