use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Advinhe o número! (números de 1 à 1000)");

    let secret_number = rand::thread_rng().gen_range(1, 1001);

    loop {
        println!("Por favor escreva seu palpite:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Não foi possível ler o seu palpite");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Houve uma falha ao tentar compreender o seu palpite!");
                continue
            },
        };

        println!("Seu palpite foi: {}", guess);

        match (secret_number as i64 - guess as i64).abs() {
            0 => {
                println!("Você venceu!");
                break;
            },
            1 => println!("Do lado!"),
            2..=10 => println!("Muito perto!"),
            11..=50 => println!("Perto!"),
            51..=100 => println!("Um pouco longe!"),
            101..=500 => println!("Longe!"),
            _ => println!("Muito longe!")
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Tente um número maior!"),
            Ordering::Greater => println!("Tente um número menor!"),
            Ordering::Equal => ()
            /*{
                println!("Você venceu!");
                break;
            }*/
        }
    }
}