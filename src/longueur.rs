use clap::ArgMatches;

// Structure pour stocker les résultats de conversion de longueur
#[derive(Debug, PartialEq)]
pub struct ConversionsLongueur {
    pub metres: f64,
    pub centimetres: f64,
    pub millimetres: f64,
    pub kilometres: f64,
    pub pouces: f64,
    pub pieds: f64,
    pub yards: f64,
    pub miles: f64,
}

// Fonction pure pour convertir vers les mètres
pub fn vers_metres(valeur: f64, unite: &str) -> Result<f64, String> {
    match unite {
        "m" => Ok(valeur),
        "cm" => Ok(valeur / 100.0),
        "mm" => Ok(valeur / 1000.0),
        "km" => Ok(valeur * 1000.0),
        "in" => Ok(valeur * 0.0254),
        "ft" => Ok(valeur * 0.3048),
        "yd" => Ok(valeur * 0.9144),
        "mi" => Ok(valeur * 1609.344),
        _ => Err(format!("Unité de longueur non reconnue: {}", unite)),
    }
}

// Fonction pure pour convertir depuis les mètres vers toutes les unités
pub fn depuis_metres(metres: f64) -> ConversionsLongueur {
    ConversionsLongueur {
        metres,
        centimetres: metres * 100.0,
        millimetres: metres * 1000.0,
        kilometres: metres / 1000.0,
        pouces: metres / 0.0254,
        pieds: metres / 0.3048,
        yards: metres / 0.9144,
        miles: metres / 1609.344,
    }
}

// Fonction principale pour convertir les longueurs
pub fn convertir_longueur_complete(valeur: f64, unite_origine: &str) -> Result<ConversionsLongueur, String> {
    let metres = vers_metres(valeur, unite_origine)?;
    Ok(depuis_metres(metres))
}

// Fonction d'affichage pour l'interface CLI
pub fn afficher_conversions_longueur(valeur: f64, unite_origine: &str, conversions: &ConversionsLongueur) {
    match unite_origine {
        "m" => println!("{} m = {} cm, {} mm, {} km, {} in, {} ft, {} yd, {} mi", 
            valeur, conversions.centimetres, conversions.millimetres, conversions.kilometres,
            conversions.pouces, conversions.pieds, conversions.yards, conversions.miles),
        "cm" => println!("{} cm = {} m, {} mm, {} km, {} in, {} ft, {} yd, {} mi", 
            valeur, conversions.metres, conversions.millimetres, conversions.kilometres,
            conversions.pouces, conversions.pieds, conversions.yards, conversions.miles),
        "mm" => println!("{} mm = {} m, {} cm, {} km, {} in, {} ft, {} yd, {} mi", 
            valeur, conversions.metres, conversions.centimetres, conversions.kilometres,
            conversions.pouces, conversions.pieds, conversions.yards, conversions.miles),
        "km" => println!("{} km = {} m, {} cm, {} mm, {} in, {} ft, {} yd, {} mi", 
            valeur, conversions.metres, conversions.centimetres, conversions.millimetres,
            conversions.pouces, conversions.pieds, conversions.yards, conversions.miles),
        "in" => println!("{} in = {} m, {} cm, {} mm, {} km, {} ft, {} yd, {} mi", 
            valeur, conversions.metres, conversions.centimetres, conversions.millimetres,
            conversions.kilometres, conversions.pieds, conversions.yards, conversions.miles),
        "ft" => println!("{} ft = {} m, {} cm, {} mm, {} km, {} in, {} yd, {} mi", 
            valeur, conversions.metres, conversions.centimetres, conversions.millimetres,
            conversions.kilometres, conversions.pouces, conversions.yards, conversions.miles),
        "yd" => println!("{} yd = {} m, {} cm, {} mm, {} km, {} in, {} ft, {} mi", 
            valeur, conversions.metres, conversions.centimetres, conversions.millimetres,
            conversions.kilometres, conversions.pouces, conversions.pieds, conversions.miles),
        "mi" => println!("{} mi = {} m, {} cm, {} mm, {} km, {} in, {} ft, {} yd", 
            valeur, conversions.metres, conversions.centimetres, conversions.millimetres,
            conversions.kilometres, conversions.pouces, conversions.pieds, conversions.yards),
        _ => println!("Unité de longueur non reconnue."),
    }
}

// Fonction CLI originale refactorisée
pub fn convertir_longueur(matches: &ArgMatches) {
    let from = matches.get_one::<String>("from").expect("Argument 'from' manquant");
    let value = matches.get_one::<String>("value")
        .expect("Argument 'value' manquant")
        .parse::<f64>()
        .expect("Valeur invalide");

    match convertir_longueur_complete(value, from) {
        Ok(conversions) => afficher_conversions_longueur(value, from, &conversions),
        Err(erreur) => println!("Erreur: {}", erreur),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vers_metres_metres() {
        assert_eq!(vers_metres(5.0, "m").unwrap(), 5.0);
    }

    #[test]
    fn test_vers_metres_centimetres() {
        assert_eq!(vers_metres(100.0, "cm").unwrap(), 1.0);
    }

    #[test]
    fn test_vers_metres_millimetres() {
        assert_eq!(vers_metres(1000.0, "mm").unwrap(), 1.0);
    }

    #[test]
    fn test_vers_metres_kilometres() {
        assert_eq!(vers_metres(1.0, "km").unwrap(), 1000.0);
    }

    #[test]
    fn test_vers_metres_pouces() {
        let resultat = vers_metres(1.0, "in").unwrap();
        assert!((resultat - 0.0254).abs() < 1e-10);
    }

    #[test]
    fn test_vers_metres_pieds() {
        let resultat = vers_metres(1.0, "ft").unwrap();
        assert!((resultat - 0.3048).abs() < 1e-10);
    }

    #[test]
    fn test_vers_metres_yards() {
        let resultat = vers_metres(1.0, "yd").unwrap();
        assert!((resultat - 0.9144).abs() < 1e-10);
    }

    #[test]
    fn test_vers_metres_miles() {
        let resultat = vers_metres(1.0, "mi").unwrap();
        assert!((resultat - 1609.344).abs() < 1e-10);
    }

    #[test]
    fn test_vers_metres_unite_invalide() {
        assert!(vers_metres(1.0, "xyz").is_err());
    }

    #[test]
    fn test_depuis_metres() {
        let conversions = depuis_metres(1.0);
        assert_eq!(conversions.metres, 1.0);
        assert_eq!(conversions.centimetres, 100.0);
        assert_eq!(conversions.millimetres, 1000.0);
        assert_eq!(conversions.kilometres, 0.001);
        assert!((conversions.pouces - 39.3700787).abs() < 1e-6);
        assert!((conversions.pieds - 3.2808399).abs() < 1e-6);
        assert!((conversions.yards - 1.0936133).abs() < 1e-6);
        assert!((conversions.miles - 0.00062137).abs() < 1e-7);
    }

    #[test]
    fn test_convertir_longueur_complete_metres() {
        let resultat = convertir_longueur_complete(100.0, "m").unwrap();
        assert_eq!(resultat.metres, 100.0);
        assert_eq!(resultat.centimetres, 10000.0);
        assert_eq!(resultat.millimetres, 100000.0);
        assert_eq!(resultat.kilometres, 0.1);
    }

    #[test]
    fn test_convertir_longueur_complete_centimetres() {
        let resultat = convertir_longueur_complete(200.0, "cm").unwrap();
        assert_eq!(resultat.metres, 2.0);
        assert_eq!(resultat.centimetres, 200.0);
        assert_eq!(resultat.millimetres, 2000.0);
        assert_eq!(resultat.kilometres, 0.002);
    }

    #[test]
    fn test_convertir_longueur_complete_unite_invalide() {
        assert!(convertir_longueur_complete(100.0, "invalid").is_err());
    }

    #[test]
    fn test_conversion_aller_retour() {
        // Test que convertir 1 mètre vers pouces puis retour vers mètres donne 1 mètre
        let metres_vers_pouces = vers_metres(1.0, "m").unwrap() / 0.0254;
        let pouces_vers_metres = vers_metres(metres_vers_pouces, "in").unwrap();
        assert!((pouces_vers_metres - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_valeurs_negatives() {
        let resultat = convertir_longueur_complete(-5.0, "m").unwrap();
        assert_eq!(resultat.metres, -5.0);
        assert_eq!(resultat.centimetres, -500.0);
    }

    #[test]
    fn test_valeur_zero() {
        let resultat = convertir_longueur_complete(0.0, "km").unwrap();
        assert_eq!(resultat.metres, 0.0);
        assert_eq!(resultat.centimetres, 0.0);
        assert_eq!(resultat.millimetres, 0.0);
    }
}
