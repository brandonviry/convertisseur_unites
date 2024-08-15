use clap::ArgMatches;

pub fn convertir_temperature(matches: &ArgMatches) {
    let from = matches.get_one::<String>("from").expect("Argument 'from' manquant");
    let value = matches.get_one::<String>("value")
        .expect("Argument 'value' manquant")
        .parse::<f64>()
        .expect("Valeur invalide");

    let (to_unit, result) = match from.as_str() {
        "C" => ("F", value * 9.0 / 5.0 + 32.0),
        "F" => ("C", (value - 32.0) * 5.0 / 9.0),
        "K" => ("C", value - 273.15),
        _ => {
            println!("Unité de température non reconnue.");
            return;
        }
    };

    println!("{} {} = {} {}", value, from, result, to_unit);
}

