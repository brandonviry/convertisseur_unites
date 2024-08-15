use clap::ArgMatches;

pub fn convertir_longueur(matches: &ArgMatches) {
    let from = matches.get_one::<String>("from").expect("Argument 'from' manquant");
    let value = matches.get_one::<String>("value")
        .expect("Argument 'value' manquant")
        .parse::<f64>()
        .expect("Valeur invalide");

    match from.as_str() {
        "m" => {
            // Convertir mètres (m) vers autres unités
            println!("{} m = {} cm, {} mm, {} km, {} in, {} ft, {} yd, {} mi", 
                value, 
                value * 100.0, 
                value * 1000.0, 
                value / 1000.0, 
                value / 0.0254, 
                value / 0.3048, 
                value / 0.9144, 
                value / 1609.344);
            return;
        },
        "cm" => {
            // Convertir centimètres (cm) vers autres unités
            let meters = value / 100.0;
            println!("{} cm = {} m, {} mm, {} km, {} in, {} ft, {} yd, {} mi", 
                value, 
                meters, 
                value * 10.0, 
                meters / 1000.0, 
                meters / 0.0254, 
                meters / 0.3048, 
                meters / 0.9144, 
                meters / 1609.344);
            return;
        },
        "mm" => {
            // Convertir millimètres (mm) vers autres unités
            let meters = value / 1000.0;
            println!("{} mm = {} m, {} cm, {} km, {} in, {} ft, {} yd, {} mi", 
                value, 
                meters, 
                value / 10.0, 
                meters / 1000.0, 
                meters / 0.0254, 
                meters / 0.3048, 
                meters / 0.9144, 
                meters / 1609.344);
            return;
        },
        "km" => {
            // Convertir kilomètres (km) vers autres unités
            let meters = value * 1000.0;
            println!("{} km = {} m, {} cm, {} mm, {} in, {} ft, {} yd, {} mi", 
                value, 
                meters, 
                meters * 100.0, 
                meters * 1000.0, 
                meters / 0.0254, 
                meters / 0.3048, 
                meters / 0.9144, 
                meters / 1609.344);
            return;
        },
        "in" => {
            // Convertir pouces (in) vers autres unités
            let meters = value * 0.0254;
            println!("{} in = {} m, {} cm, {} mm, {} km, {} ft, {} yd, {} mi", 
                value, 
                meters, 
                meters * 100.0, 
                meters * 25.4, 
                meters / 1000.0, 
                meters / 0.3048, 
                meters / 0.9144, 
                meters / 1609.344);
            return;
        },
        "ft" => {
            // Convertir pieds (ft) vers autres unités
            let meters = value * 0.3048;
            println!("{} ft = {} m, {} cm, {} mm, {} km, {} in, {} yd, {} mi", 
                value, 
                meters, 
                meters * 100.0, 
                meters * 1000.0, 
                meters / 1000.0, 
                meters / 0.0254, 
                meters / 0.3048, 
                meters / 0.9144);
            return;
        },
        "yd" => {
            // Convertir yards (yd) vers autres unités
            let meters = value * 0.9144;
            println!("{} yd = {} m, {} cm, {} mm, {} km, {} in, {} ft, {} mi", 
                value, 
                meters, 
                meters * 100.0, 
                meters * 1000.0, 
                meters / 1000.0, 
                meters / 0.0254, 
                meters / 0.3048, 
                meters / 0.9144);
            return;
        },
        "mi" => {
            // Convertir miles (mi) vers autres unités
            let meters = value * 1609.344;
            println!("{} mi = {} m, {} cm, {} mm, {} km, {} in, {} ft, {} yd", 
                value, 
                meters, 
                meters * 100.0, 
                meters * 1000.0, 
                meters / 1000.0, 
                meters / 0.0254, 
                meters / 0.3048, 
                meters / 0.9144);
            return;
        },
        _ => {
            println!("Unité de longueur non reconnue.");
            return;
        }
    };
}
