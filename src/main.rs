use clap::{Arg, Command};

fn main() {
    let matches = Command::new("Crudlove")
        .version("0.1.0")
        .author("Lucas Onofre")
        .about("Generates a simple CRUD for you")
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .required(true)
                .help("Name of the project"),
        )
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .required(true)
                .help("Path to generate the project"),
        )
        .arg(
            Arg::new("model")
                .short('m')
                .long("model")
                .required(true)
                .help("Base model to generate the CRUD"),
        )
        .get_matches();

    let name = matches.get_one::<String>("name").expect("name is required");
    let path = matches.get_one::<String>("path").expect("path is required");
    let model = matches.get_one::<String>("model").expect("model is required");

    dbg!(name);
    dbg!(path);
    dbg!(model);
}