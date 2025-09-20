use parameter_reader::ParameterReader;
use std::env;


const PARAMETERS: [&str; 5] = 
[ 
    "my_int",
    "my_float",
    "my_array",
    "my_bool",
    "my_string"];

//  cargo run --example basic_example examples/basic_parameters.txt
fn main()
{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2
    {
        println!("Error: Usage: cargo run --example {} examples/basic_parameters.txt", &args[1]);
        std::process::exit(1);
    }

    let reader = ParameterReader::build(&args[1]).unwrap_or_else(|err|
    {
        println!("Error: Could not create reader: {err}");
        std::process::exit(1);
    });

    let parameters = reader.parse_parameters(&PARAMETERS, ":").expect("Should be able to create parameter map!");

    let my_int: i32        = parameters["my_int"].parse().expect("Should be able to parse int!");
    let my_float: f32      = parameters["my_float"].parse().expect("Should be able to parse my_float!");
    let my_bool: bool      = parameters["my_bool"].parse().expect("Should be able to parse my_bool!");
    let my_string          = parameters["my_string"].to_owned();
    let my_array: Vec<f32> = parameters["my_array"].split(",").map(|v| v.trim().parse().expect("Should be able to parse array")).collect();

    println!("We got: my_int={my_int}, my_float={my_float}, my_bool={my_bool}, my_string={my_string}, and");
    for (n, v) in my_array.iter().enumerate()
    {
        println!("my_array[{n}]={v}");
    }

}