//! # Convertisseur d'Unités
//! 
//! Une bibliothèque Rust pour convertir entre différentes unités de mesure.
//! 
//! ## Modules
//! 
//! - `longueur`: Conversions d'unités de longueur (mètres, centimètres, pouces, etc.)
//! - `masse`: Conversions d'unités de masse (grammes, kilogrammes, tonnes)
//! - `temperature`: Conversions d'unités de température (Celsius, Fahrenheit, Kelvin)
//! 
//! ## Exemple d'utilisation
//! 
//! ```rust
//! use convertisseur_unites::longueur::convertir_longueur_complete;
//! 
//! let resultat = convertir_longueur_complete(100.0, "m").unwrap();
//! assert_eq!(resultat.centimetres, 10000.0);
//! ```

pub mod longueur;
pub mod masse;
pub mod temperature;
