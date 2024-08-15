use clap::{Arg, Command, ArgMatches};

// Importation des fonctions de conversion des différents modules
mod longueur;
mod masse;
mod temperature;


// Fonction principale
fn main() {
    let matches = Command::new("Convertisseur d'unités")
        .version("1.0")
        .author("VIRY Brandon")
        .about("Convertit les unités de longueur, de masse et de température")
        .subcommand(
            Command::new("longueur")
                .about("Convertit les unités de longueur")
                .arg(
                    Arg::new("from")
                        .short('f')
                        .long("from")

                        .required(true)
                        .help("Unité d'origine (m, cm, mm, km, in, ft, yd, mi)"),
                )
                .arg(
                    Arg::new("value")
                        .short('v')
                        .long("value")

                        .required(true)
                        .help("Valeur à convertir"),
                ),
        )
        .subcommand(
            Command::new("masse")
                .about("Convertit les unités de masse")
                .arg(
                    Arg::new("from")
                        .short('f')
                        .long("from")
  
                        .required(true)
                        .help("Unité d'origine (g, kg, t)"),
                )
                .arg(
                    Arg::new("value")
                        .short('v')
                        .long("value")
                  
                        .required(true)
                        .help("Valeur à convertir"),
                ),
        )
        .subcommand(
            Command::new("temperature")
                .about("Convertit les unités de température")
                .arg(
                    Arg::new("from")
                        .short('f')
                        .long("from")
              
                        .required(true)
                        .help("Unité d'origine (C, F, K)"),
                )
                .arg(
                    Arg::new("value")
                        .short('v')
                        .long("value")
                   
                        .required(true)
                        .help("Valeur à convertir"),
                ),
        )
        .get_matches();

    // Appelle les fonctions appropriées selon la sous-commande
    match matches.subcommand() {
        Some(("longueur", sub_m)) => longueur::convertir_longueur(sub_m),
        Some(("masse", sub_m)) => masse::convertir_masse(sub_m),
        Some(("temperature", sub_m)) => temperature::convertir_temperature(sub_m),

        _ => println!("Aucune commande valide spécifiée"),
    }
}
