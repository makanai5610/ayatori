use ayatori;
use clap;
use serde_json;
use std::io;

struct Arg {
    environment: String,
    base_file_path: String,
    topic_file_name: String,
    subscription_file_name: String,
    output_format: String,
    is_concurrent: bool,
}

fn parse_arg() -> Arg {
    let matches = clap::App::new("ayatori")
        .version("0.1.0")
        .author("Takumi Karibe <takumi.k.5610@gmail.com>")
        .about("Analysis of dependency between services in microservices")
        .arg(
            clap::Arg::with_name("environment")
                .help("Environment")
                .takes_value(true)
                .short("e")
                .long("environment")
                .possible_values(&["develop", "staging", "production"])
                .required(true),
        )
        .arg(
            clap::Arg::with_name("base_file_path")
                .help("Base file path")
                .takes_value(true)
                .short("b")
                .long("base_file_path")
                .required(true),
        )
        .arg(
            clap::Arg::with_name("topic_file_name")
                .help("Topic file name")
                .takes_value(true)
                .short("t")
                .long("topic")
                .required(true),
        )
        .arg(
            clap::Arg::with_name("subscription_file_name")
                .help("Subscription file name")
                .takes_value(true)
                .short("s")
                .long("subscription")
                .required(true),
        )
        .arg(
            clap::Arg::with_name("output_format")
                .help("Output format")
                .takes_value(true)
                .short("f")
                .long("format")
                .possible_values(&["json", "dot"]),
        )
        .arg(
            clap::Arg::with_name("run_concurrent")
                .help("run concurrent")
                .short("c")
                .long("concurrent"),
        )
        .get_matches();

    let environment = matches
        .value_of("environment")
        .unwrap_or_else(|| panic!("environment is required"));
    let base_file_path = matches
        .value_of("base_file_path")
        .unwrap_or_else(|| panic!("base path file path is required"));
    let topic_file_name = matches
        .value_of("topic_file_name")
        .unwrap_or_else(|| panic!("topic file name is required"));
    let subscription_file_name = matches
        .value_of("subscription_file_name")
        .unwrap_or_else(|| panic!("subscription file name is required"));
    let output_format = matches.value_of("output_format").unwrap_or("");
    let is_concurrent = matches.value_of("run_concurrent").is_some();

    Arg {
        environment: environment.into(),
        base_file_path: base_file_path.into(),
        topic_file_name: topic_file_name.into(),
        subscription_file_name: subscription_file_name.into(),
        output_format: output_format.into(),
        is_concurrent,
    }
}

fn main() -> Result<(), io::Error> {
    let arg = parse_arg();

    let graph = ayatori::run(
        arg.environment,
        arg.base_file_path,
        arg.topic_file_name,
        arg.subscription_file_name,
        arg.is_concurrent,
    )?;

    match arg.output_format.as_str() {
        "json" => {
            let json = serde_json::to_string(&graph)?;
            println!("{}", json);
        }
        "dot" => {
            let dot =
                petgraph::dot::Dot::with_config(&graph, &[petgraph::dot::Config::EdgeNoLabel]);
            println!("{}", dot);
        }
        _ => {
            if cfg!(debug_assertions) {
                dbg!(graph);
            } else {
                println!("{:#?}", graph);
            }
        }
    }

    Ok(())
}
