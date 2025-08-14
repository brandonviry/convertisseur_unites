use convertisseur_unites::longueur::convertir_longueur_complete;
use convertisseur_unites::masse::convertir_masse_complete;
use convertisseur_unites::temperature::convertir_temperature_complete;

#[test]
fn test_integration_longueur_metres_vers_toutes_unites() {
    let resultat = convertir_longueur_complete(1.0, "m").unwrap();
    
    // Vérifications précises
    assert_eq!(resultat.metres, 1.0);
    assert_eq!(resultat.centimetres, 100.0);
    assert_eq!(resultat.millimetres, 1000.0);
    assert_eq!(resultat.kilometres, 0.001);
    
    // Vérifications avec tolérance pour les conversions impériales
    assert!((resultat.pouces - 39.3700787).abs() < 1e-6);
    assert!((resultat.pieds - 3.2808399).abs() < 1e-6);
    assert!((resultat.yards - 1.0936133).abs() < 1e-6);
    assert!((resultat.miles - 0.00062137).abs() < 1e-7);
}

#[test]
fn test_integration_masse_kilogrammes_vers_toutes_unites() {
    let resultat = convertir_masse_complete(1.0, "kg").unwrap();
    
    assert_eq!(resultat.kilogrammes, 1.0);
    assert_eq!(resultat.grammes, 1000.0);
    assert_eq!(resultat.tonnes, 0.001);
}

#[test]
fn test_integration_temperature_celsius_vers_toutes_unites() {
    let resultat = convertir_temperature_complete(0.0, "C").unwrap();
    
    assert_eq!(resultat.celsius, 0.0);
    assert_eq!(resultat.fahrenheit, 32.0);
    assert_eq!(resultat.kelvin, 273.15);
}

#[test]
fn test_integration_conversions_complexes() {
    // Test avec des valeurs réelles courantes
    
    // Distance d'un marathon en kilomètres vers miles
    let marathon = convertir_longueur_complete(42.195, "km").unwrap();
    assert!((marathon.miles - 26.219).abs() < 0.001);
    
    // Poids d'une personne en livres vers kilogrammes (approximation)
    // Note: ce test nécessiterait l'ajout des livres comme unité
    
    // Température corporelle normale
    let temp_corporelle = convertir_temperature_complete(98.6, "F").unwrap();
    assert!((temp_corporelle.celsius - 37.0).abs() < 0.1);
}

#[test]
fn test_integration_erreurs_unites_invalides() {
    // Test que toutes les fonctions gèrent correctement les unités invalides
    assert!(convertir_longueur_complete(100.0, "invalid").is_err());
    assert!(convertir_masse_complete(100.0, "invalid").is_err());
    assert!(convertir_temperature_complete(100.0, "invalid").is_err());
}

#[test]
fn test_integration_valeurs_extremes() {
    // Test avec des valeurs très grandes
    let grande_distance = convertir_longueur_complete(1000000.0, "m").unwrap();
    assert_eq!(grande_distance.kilometres, 1000.0);
    
    // Test avec des valeurs très petites
    let petite_masse = convertir_masse_complete(0.001, "g").unwrap();
    assert_eq!(petite_masse.kilogrammes, 0.000001);
    
    // Test avec des températures extrêmes
    let temp_extreme = convertir_temperature_complete(-273.15, "C").unwrap();
    assert_eq!(temp_extreme.kelvin, 0.0); // Zéro absolu
}

#[test]
fn test_integration_precision_conversions() {
    // Test de précision pour les conversions aller-retour
    
    // Longueur: mètre -> pouce -> mètre
    let original_metres = 5.0;
    let longueur = convertir_longueur_complete(original_metres, "m").unwrap();
    let retour_longueur = convertir_longueur_complete(longueur.pouces, "in").unwrap();
    assert!((retour_longueur.metres - original_metres).abs() < 1e-10);
    
    // Masse: kg -> g -> kg
    let original_kg = 2.5;
    let masse = convertir_masse_complete(original_kg, "kg").unwrap();
    let retour_masse = convertir_masse_complete(masse.grammes, "g").unwrap();
    assert!((retour_masse.kilogrammes - original_kg).abs() < 1e-10);
    
    // Température: C -> F -> C
    let original_celsius = 25.0;
    let temperature = convertir_temperature_complete(original_celsius, "C").unwrap();
    let retour_temperature = convertir_temperature_complete(temperature.fahrenheit, "F").unwrap();
    assert!((retour_temperature.celsius - original_celsius).abs() < 1e-10);
}

#[test]
fn test_integration_cas_usage_reel() {
    // Scénarios d'usage réel
    
    // Recette de cuisine: convertir 2 cups en millilitres (approximation avec volume)
    // Pour cet exemple, on utilise la longueur comme proxy
    
    // Construction: convertir 10 pieds en mètres
    let construction = convertir_longueur_complete(10.0, "ft").unwrap();
    assert!((construction.metres - 3.048).abs() < 1e-10);
    
    // Voyage: convertir 500 miles en kilomètres
    let voyage = convertir_longueur_complete(500.0, "mi").unwrap();
    assert!((voyage.kilometres - 804.672).abs() < 0.001);
    
    // Météo: convertir 75°F en Celsius
    let meteo = convertir_temperature_complete(75.0, "F").unwrap();
    assert!((meteo.celsius - 23.888889).abs() < 1e-5);
    
    // Expédition: convertir 2.5 tonnes en kilogrammes
    let expedition = convertir_masse_complete(2.5, "t").unwrap();
    assert_eq!(expedition.kilogrammes, 2500.0);
}

#[test]
fn test_integration_coherence_structures() {
    // Test que les structures retournent des valeurs cohérentes
    
    let longueur = convertir_longueur_complete(1.0, "m").unwrap();
    // Vérifier que toutes les valeurs sont positives pour une entrée positive
    assert!(longueur.metres > 0.0);
    assert!(longueur.centimetres > 0.0);
    assert!(longueur.millimetres > 0.0);
    assert!(longueur.kilometres > 0.0);
    assert!(longueur.pouces > 0.0);
    assert!(longueur.pieds > 0.0);
    assert!(longueur.yards > 0.0);
    assert!(longueur.miles > 0.0);
    
    let masse = convertir_masse_complete(1.0, "kg").unwrap();
    assert!(masse.grammes > 0.0);
    assert!(masse.kilogrammes > 0.0);
    assert!(masse.tonnes > 0.0);
    
    // Pour la température, les valeurs peuvent être négatives, donc on teste la cohérence relative
    let temperature = convertir_temperature_complete(0.0, "C").unwrap();
    assert_eq!(temperature.celsius, 0.0);
    assert!(temperature.fahrenheit > temperature.celsius); // 32°F > 0°C
    assert!(temperature.kelvin > temperature.celsius);     // 273.15K > 0°C
}
