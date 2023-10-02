use std::cmp::Ordering;

// Fonction get_ordering factice pour les tests
fn get_ordering(a: i32, b: i32) -> Ordering {
    if a < b {
        Ordering::Less
    } else if a > b {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

// Sous-module de tests
#[cfg(test)]
mod tests {
    // Importez les symboles du module englobant.
    use super::*;
    
    #[test]
    fn twelve_equals_twelve() {
        // Testez si get_ordering retourne Ordering::Equal lorsque les deux paramètres sont égaux à 12.
        assert_eq!(get_ordering(12, 12), Ordering::Equal);
    }

    #[test]
    fn twelve_greater_than_five() {
        // Testez si get_ordering retourne Ordering::Greater lorsque le premier paramètre est 12 et le deuxième est 5.
        assert_eq!(get_ordering(12, 5), Ordering::Greater);
    }

    #[test]
    fn twelve_lesser_than_twenty() {
        // Testez si get_ordering retourne Ordering::Less lorsque le premier paramètre est 12 et le deuxième est 20.
        assert_eq!(get_ordering(12, 20), Ordering::Less);
    }
}
