use rand::Rng;
use std::io;
use std::cmp::Ordering;


// ojo con esto en este ejemplo tenemos que 2 variavles con le mismo nombre
// esto en Rust se conoce como shadonws o sombreado. lo que nos ayuda poder
// reutilizar la variable en lugar de crear otra variable 
fn main() {
  println!("Adivina el nùmero ");


  let secre_number = rand::thread_rng()
  .gen_range(1..101);
  
  
  
    loop {
        println!("El secreto númeoro es: {}", secre_number);

        println!("please input your guees ");
      

 
  let mut guess = String::new();

    
    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
    println!("You guessed: {}", guess );
    let guess : u32 = guess.trim().parse()
    .expect("Please type a number");
    
        println!("Please input your guess.");

        match guess.cmp(&secre_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>  {
                println!("You win!");
                break;
            },
       
       }
   
    }



}
