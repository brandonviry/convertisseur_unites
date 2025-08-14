use clap::ArgMatches;

// Structure pour stocker les résultats de conversion de masse
#[derive(Debug, PartialEq)]
pub struct ConversionsMasse {
    pub grammes: f64,
    pub kilogrammes: f64,
    pub tonnes: f64,
}

// Fonction pure pour convertir vers les grammes
pub fn vers_grammes(valeur: f64, unite: &str) -> Result<f64, String> {
    match unite {
        "g" => Ok(valeur),
        "kg" => Ok(valeur * 1000.0),
        "t" => Ok(valeur * 1_000_000.0),
        _ => Err(format!("Unité de masse non reconnue: {}", unite)),
    }
}

// Fonction pure pour convertir depuis les grammes vers toutes les unités
pub fn depuis_grammes(grammes: f64) -> ConversionsMasse {
    ConversionsMasse {
        grammes,
        kilogrammes: grammes / 1000.0,
        tonnes: grammes / 1_000_000.0,
    }
}

// Fonction principale pour convertir les masses
pub fn convertir_masse_complete(valeur: f64, unite_origine: &str) -> Result<ConversionsMasse, String> {
    let grammes = vers_grammes(valeur, unite_origine)?;
    Ok(depuis_grammes(grammes))
}

// Fonction d'affichage pour l'interface CLI
pub fn afficher_conversions_masse(valeur: f64, unite_origine: &str, conversions: &ConversionsMasse) {
    match unite_origine {
        "g" => println!("{} g = {} kg, {} t", 
            valeur, conversions.kilogrammes, conversions.tonnes),
        "kg" => println!("{} kg = {} g, {} t", 
            valeur, conversions.grammes, conversions.tonnes),
        "t" => println!("{} t = {} g, {} kg", 
            valeur, conversions.grammes, conversions.kilogrammes),
        _ => println!("Unité de masse non reconnue."),
    }
}

// Fonction CLI originale refactorisée
pub fn convertir_masse(matches: &ArgMatches) {
    let from = matches.get_one::<String>("from").expect("Argument 'from' manquant");
    let value = matches.get_one::<String>("value")
        .expect("Argument 'value' manquant")
        .parse::<f64>()
        .expect("Valeur invalide");

    match convertir_masse_complete(value, from) {
        Ok(conversions) => afficher_conversions_masse(value, from, &conversions),
        Err(erreur) => println!("Erreur: {}", erreur),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vers_grammes_grammes() {
        assert_eq!(vers_grammes(500.0, "g").unwrap(), 500.0);
    }

    #[test]
    fn test_vers_grammes_kilogrammes() {
        assert_eq!(vers_grammes(2.0, "kg").unwrap(), 2000.0);
    }

    #[test]
    fn test_vers_grammes_tonnes() {
        assert_eq!(vers_grammes(1.0, "t").unwrap(), 1_000_000.0);
    }

    #[test]
    fn test_vers_grammes_unite_invalide() {
        assert!(vers_grammes(1.0, "lb").is_err());
    }

    #[test]
    fn test_depuis_grammes() {
        let conversions = depuis_grammes(2000.0);
        assert_eq!(conversions.grammes, 2000.0);
        assert_eq!(conversions.kilogrammes, 2.0);
        assert_eq!(conversions.tonnes, 0.002);
    }

    #[test]
    fn test_convertir_masse_complete_grammes() {
        let resultat = convertir_masse_complete(1500.0, "g").unwrap();
        assert_eq!(resultat.grammes, 1500.0);
        assert_eq!(resultat.kilogrammes, 1.5);
        assert_eq!(resultat.tonnes, 0.0015);
    }

    #[test]
    fn test_convertir_masse_complete_kilogrammes() {
        let resultat = convertir_masse_complete(3.5, "kg").unwrap();
        assert_eq!(resultat.grammes, 3500.0);
        assert_eq!(resultat.kilogrammes, 3.5);
        assert_eq!(resultat.tonnes, 0.0035);
    }

    #[test]
    fn test_convertir_masse_complete_tonnes() {
        let resultat = convertir_masse_complete(0.5, "t").unwrap();
        assert_eq!(resultat.grammes, 500_000.0);
        assert_eq!(resultat.kilogrammes, 500.0);
        assert_eq!(resultat.tonnes, 0.5);
    }

    #[test]
    fn test_convertir_masse_complete_unite_invalide() {
        assert!(convertir_masse_complete(100.0, "invalid").is_err());
    }

    #[test]
    fn test_conversion_aller_retour() {
        // Test que convertir 1 kg vers grammes puis retour vers kg donne 1 kg
        let kg_vers_grammes = vers_grammes(1.0, "kg").unwrap();
        let grammes_vers_kg = kg_vers_grammes / 1000.0;
        assert!((grammes_vers_kg - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_valeurs_negatives() {
        let resultat = convertir_masse_complete(-2.0, "kg").unwrap();
        assert_eq!(resultat.grammes, -2000.0);
        assert_eq!(resultat.kilogrammes, -2.0);
        assert_eq!(resultat.tonnes, -0.002);
    }

    #[test]
    fn test_valeur_zero() {
        let resultat = convertir_masse_complete(0.0, "t").unwrap();
        assert_eq!(resultat.grammes, 0.0);
        assert_eq!(resultat.kilogrammes, 0.0);
        assert_eq!(resultat.tonnes, 0.0);
    }

    #[test]
    fn test_grandes_valeurs() {
        let resultat = convertir_masse_complete(1000.0, "t").unwrap();
        assert_eq!(resultat.grammes, 1_000_000_000.0);
        assert_eq!(resultat.kilogrammes, 1_000_000.0);
        assert_eq!(resultat.tonnes, 1000.0);
    }

    #[test]
    fn test_petites_valeurs() {
        let resultat = convertir_masse_complete(0.001, "g").unwrap();
        assert_eq!(resultat.grammes, 0.001);
        assert_eq!(resultat.kilogrammes, 0.000001);
        assert_eq!(resultat.tonnes, 0.000000001);
    }
}
