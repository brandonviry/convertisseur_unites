use clap::ArgMatches;

pub fn convertir_masse(matches: &ArgMatches) {
    let from = matches.get_one::<String>("from").expect("Argument 'from' manquant");
    let value = matches.get_one::<String>("value")
        .expect("Argument 'value' manquant")
        .parse::<f64>()
        .expect("Valeur invalide");

    match from.as_str() {
        "g" => {
            // Convertir grammes (g) vers autres unités
            println!("{} g = {} kg, {} t", 
                value, 
                value / 1000.0, 
                value / 1_000_000.0);
            return;
        },
        "kg" => {
            // Convertir kilogrammes (kg) vers autres unités
            println!("{} kg = {} g, {} t", 
                value, 
                value * 1000.0, 
                value / 1000.0);
            return;
        },
        "t" => {
            // Convertir tonnes (t) vers autres unités
            println!("{} t = {} g, {} kg", 
                value, 
                value * 1_000_000.0, 
                value * 1000.0);
            return;
        },
        _ => {
            println!("Unité de masse non reconnue.");
            return;
        }
    };
}
