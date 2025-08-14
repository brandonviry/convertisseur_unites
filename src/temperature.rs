use clap::ArgMatches;

// Structure pour stocker les résultats de conversion de température
#[derive(Debug, PartialEq)]
pub struct ConversionsTemperature {
    pub celsius: f64,
    pub fahrenheit: f64,
    pub kelvin: f64,
}

// Fonction pure pour convertir vers Celsius
pub fn vers_celsius(valeur: f64, unite: &str) -> Result<f64, String> {
    match unite {
        "C" => Ok(valeur),
        "F" => Ok((valeur - 32.0) * 5.0 / 9.0),
        "K" => Ok(valeur - 273.15),
        _ => Err(format!("Unité de température non reconnue: {}", unite)),
    }
}

// Fonction pure pour convertir depuis Celsius vers toutes les unités
pub fn depuis_celsius(celsius: f64) -> ConversionsTemperature {
    ConversionsTemperature {
        celsius,
        fahrenheit: celsius * 9.0 / 5.0 + 32.0,
        kelvin: celsius + 273.15,
    }
}

// Fonction principale pour convertir les températures
pub fn convertir_temperature_complete(valeur: f64, unite_origine: &str) -> Result<ConversionsTemperature, String> {
    let celsius = vers_celsius(valeur, unite_origine)?;
    Ok(depuis_celsius(celsius))
}

// Fonction d'affichage pour l'interface CLI (améliorée pour afficher toutes les conversions)
pub fn afficher_conversions_temperature(valeur: f64, unite_origine: &str, conversions: &ConversionsTemperature) {
    match unite_origine {
        "C" => println!("{} C = {} F, {} K", 
            valeur, conversions.fahrenheit, conversions.kelvin),
        "F" => println!("{} F = {} C, {} K", 
            valeur, conversions.celsius, conversions.kelvin),
        "K" => println!("{} K = {} C, {} F", 
            valeur, conversions.celsius, conversions.fahrenheit),
        _ => println!("Unité de température non reconnue."),
    }
}

// Fonction CLI originale refactorisée (maintenant affiche toutes les conversions)
pub fn convertir_temperature(matches: &ArgMatches) {
    let from = matches.get_one::<String>("from").expect("Argument 'from' manquant");
    let value = matches.get_one::<String>("value")
        .expect("Argument 'value' manquant")
        .parse::<f64>()
        .expect("Valeur invalide");

    match convertir_temperature_complete(value, from) {
        Ok(conversions) => afficher_conversions_temperature(value, from, &conversions),
        Err(erreur) => println!("Erreur: {}", erreur),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vers_celsius_celsius() {
        assert_eq!(vers_celsius(25.0, "C").unwrap(), 25.0);
    }

    #[test]
    fn test_vers_celsius_fahrenheit() {
        let resultat = vers_celsius(32.0, "F").unwrap();
        assert!((resultat - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_vers_celsius_fahrenheit_100() {
        let resultat = vers_celsius(212.0, "F").unwrap();
        assert!((resultat - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_vers_celsius_kelvin() {
        let resultat = vers_celsius(273.15, "K").unwrap();
        assert!((resultat - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_vers_celsius_kelvin_100() {
        let resultat = vers_celsius(373.15, "K").unwrap();
        assert!((resultat - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_vers_celsius_unite_invalide() {
        assert!(vers_celsius(25.0, "R").is_err());
    }

    #[test]
    fn test_depuis_celsius() {
        let conversions = depuis_celsius(0.0);
        assert_eq!(conversions.celsius, 0.0);
        assert_eq!(conversions.fahrenheit, 32.0);
        assert_eq!(conversions.kelvin, 273.15);
    }

    #[test]
    fn test_depuis_celsius_100() {
        let conversions = depuis_celsius(100.0);
        assert_eq!(conversions.celsius, 100.0);
        assert_eq!(conversions.fahrenheit, 212.0);
        assert_eq!(conversions.kelvin, 373.15);
    }

    #[test]
    fn test_convertir_temperature_complete_celsius() {
        let resultat = convertir_temperature_complete(25.0, "C").unwrap();
        assert_eq!(resultat.celsius, 25.0);
        assert_eq!(resultat.fahrenheit, 77.0);
        assert_eq!(resultat.kelvin, 298.15);
    }

    #[test]
    fn test_convertir_temperature_complete_fahrenheit() {
        let resultat = convertir_temperature_complete(68.0, "F").unwrap();
        assert_eq!(resultat.fahrenheit, 68.0);
        assert_eq!(resultat.celsius, 20.0);
        assert_eq!(resultat.kelvin, 293.15);
    }

    #[test]
    fn test_convertir_temperature_complete_kelvin() {
        let resultat = convertir_temperature_complete(300.0, "K").unwrap();
        assert_eq!(resultat.kelvin, 300.0);
        assert!((resultat.celsius - 26.85).abs() < 1e-10);
        assert!((resultat.fahrenheit - 80.33).abs() < 1e-10);
    }

    #[test]
    fn test_convertir_temperature_complete_unite_invalide() {
        assert!(convertir_temperature_complete(25.0, "invalid").is_err());
    }

    #[test]
    fn test_conversion_aller_retour_celsius_fahrenheit() {
        let celsius_original = 25.0_f64;
        let fahrenheit = celsius_original * 9.0 / 5.0 + 32.0;
        let celsius_retour = (fahrenheit - 32.0) * 5.0 / 9.0;
        assert!((celsius_retour - celsius_original).abs() < 1e-10);
    }

    #[test]
    fn test_conversion_aller_retour_celsius_kelvin() {
        let celsius_original = 25.0_f64;
        let kelvin = celsius_original + 273.15;
        let celsius_retour = kelvin - 273.15;
        assert!((celsius_retour - celsius_original).abs() < 1e-10);
    }

    #[test]
    fn test_temperatures_negatives() {
        let resultat = convertir_temperature_complete(-40.0, "C").unwrap();
        assert_eq!(resultat.celsius, -40.0);
        assert_eq!(resultat.fahrenheit, -40.0); // -40°C = -40°F
        assert!((resultat.kelvin - 233.15).abs() < 1e-10);
    }

    #[test]
    fn test_zero_absolu() {
        let resultat = convertir_temperature_complete(0.0, "K").unwrap();
        assert_eq!(resultat.kelvin, 0.0);
        assert_eq!(resultat.celsius, -273.15);
        assert!((resultat.fahrenheit - (-459.67)).abs() < 1e-10);
    }

    #[test]
    fn test_point_ebullition_eau() {
        let resultat = convertir_temperature_complete(100.0, "C").unwrap();
        assert_eq!(resultat.celsius, 100.0);
        assert_eq!(resultat.fahrenheit, 212.0);
        assert_eq!(resultat.kelvin, 373.15);
    }

    #[test]
    fn test_point_congelation_eau() {
        let resultat = convertir_temperature_complete(0.0, "C").unwrap();
        assert_eq!(resultat.celsius, 0.0);
        assert_eq!(resultat.fahrenheit, 32.0);
        assert_eq!(resultat.kelvin, 273.15);
    }

    #[test]
    fn test_temperature_corporelle() {
        let resultat = convertir_temperature_complete(98.6, "F").unwrap();
        assert!((resultat.fahrenheit - 98.6).abs() < 1e-10);
        assert!((resultat.celsius - 37.0).abs() < 1e-10);
        assert!((resultat.kelvin - 310.15).abs() < 1e-10);
    }
}
