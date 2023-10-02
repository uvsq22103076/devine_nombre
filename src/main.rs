use std::io;
use rand::Rng;
use std::cmp::Ordering;
use console::Style; // Importez la bibliothèque console.


fn main() {
    // Affiche un message d'accueil en bleu.
    let style = Style::new().blue();
    println!("{}", style.apply_to("Devine le nombre!"));

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    // Affiche le nombre secret uniquement en mode de débogage.
    #[cfg(debug_assertions)]
    {
        println!("Le nombre secret est : {secret_number}");
    }

    loop {
        println!("Choisissez un nombre entre 1 et 100.");

        let mut choix = String::new();

        io::stdin()
            .read_line(&mut choix)
            .expect("Erreur de lecture");

        // Tente de convertir l'entrée utilisateur en un nombre u32.
        let nombre: Result<u32, _> = choix.trim().parse();
        
        match nombre {
            Ok(nombre) => {
                println!("You guessed: {}", nombre);

                match nombre.cmp(&secret_number) {
                    Ordering::Less => println!("Trop petit!"),
                    Ordering::Greater => println!("Trop grand!"),
                    Ordering::Equal => {
                        println!("Tu as trouvé!");
                        break;
                    }
                }
            }
            Err(_) => {
                println!("Veuillez entrer un nombre valide!");
            }
        }
    }
}
