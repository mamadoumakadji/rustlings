// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i32 {
    num * num //pas de point virgule ici c'était ça l'erreur 
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}