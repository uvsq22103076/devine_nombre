use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Devine le nombre!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    println!("Le nombre secret est : {secret_number}");

    loop {
        println!("Choisit un nombre entre 1 et 100.");

        let mut choix = String::new();

        io::stdin()
            .read_line(&mut choix)
            .expect("Erreur de lecture");

        let nombre: u32 = choix.trim().parse().expect("Veuillez entrer un nombre!");
        
            println!("You guessed: {choix}");

        match nombre.cmp(&secret_number) {
            Ordering::Less => println!("Trop petit!"),
            Ordering::Greater => println!("Trop grand!"),
            Ordering::Equal => {
                println!("Tu as trouvé!");
                break;
            }
        }
    }
}