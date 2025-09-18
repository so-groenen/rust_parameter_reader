# Basic Parameter Reader for rust
Very basic file parser to easily pass down parameters to Rust.<br>
For use for example for physics simulation in Rust. 
This works nicely in conjection with [Python Simulation Manager](https://github.com/so-groenen/python_simulation_manager) to control,
from a Python notebook, high-performance calculations in Rust.<br>
Example usage: [2D Ising Simulation using Metropolis algorithm in Rust](https://github.com/so-groenen/2d_ising_in_rust) as well as
  [Swendsen-Wang Cluster algorithm in Rust](https://github.com/so-groenen/swendsen_wang_ising_rust).


# Basic Usage:
Let us assume we want to send the following parameters in a text file `basic_parameters.txt`:
```
my_int: 42
my_float: 3.14
my_array: 1.0, 2.5, 4.0, 5.5, 7.0, 8.5, 10.0
my_bool: true
my_string: "Hello_world!"
```
to Rust.<br> 
First declare our parameters as a **constant** array of *&str*, and build the parameter_reader from the commandline argument:
```rust
const PARAMETERS: [&str; 5] = [ 
    "my_int",
    "my_float",
    "my_array",
    "my_bool",
    "my_string"];
```

The reader can easily
```rust
fn main()
{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2
    {
        println!("Error: Usage: cargo run --example {} examples/basic_parameters.txt", &args[1]);
        std::process::exit(1);
    }

    let reader = ParameterReader::build(&args[1], &PARAMETERS).unwrap_or_else(|err|
    {
        println!("Error: Could not create reader: {err}");
        std::process::exit(1);
    });
    //
    // continue vvv
}
```

We can then read, selecting the desired delimiter and create our map:
```rust
fn main()
{
    //
    // -- snip --
    // 
    let parameters = reader.parse_parameters(":").expect("Should be able to create parameter map!");

    let my_int: i32        = parameters["my_int"].parse().expect("Should be able to parse int!");
    let my_float: f32      = parameters["my_float"].parse().expect("Should be able to parse my_float!");
    let my_bool: bool      = parameters["my_bool"].parse().expect("Should be able to parse my_bool!");
    let my_string          = parameters["my_string"].to_owned();
    let my_array: Vec<f32> = parameters["my_array"].split(",").map(|v| v.trim().parse().expect("Should be able to parse array")).collect();
    //
    // continue vvv
}
```
We can then use the parameters as we wish:
```rust
fn main()
{
    // --- snip ----
    //
    println!("We got: my_int={my_int}, my_float={my_float}, my_bool={my_bool}, my_string={my_string}, and");
    for (n, v) in my_array.iter().enumerate()
    {
        println!("my_array[{n}]={v}");
    }
}
```

# Running the example:

The example can be run using cargo & passing the ` examples/basic_parameters.txt` as command line arguments:
```
cargo run --example basic_example examples/basic_parameters.txt
```
# Adding to project:

In your project:
```
cargo add --git https://github.com/so-groenen/rust_parameter_reader.git
```
or manually in the *Cargo.toml*:
```
[dependencies]
parameter_reader = { git = "https://github.com/so-groenen/rust_parameter_reader.git" }
```
# Tests & error handling:

Use 
```
cargo test -- --nocapture
```
to run the tests with dbg output. <br> The tests 'tests' the following cases:
* "bad delimiter",
* "file not found",
* "missing parameters"
and of course 
* "it_works".<br> 

Errors are handled by the `ParameterError` enum. If Rust needs parameters that are not present in the file, an error `ParameterError::MissingParam(missing)` will be returned, containing a vector of the missing files.<br>
Similarly, if the file contains a different delimiter (say *"="* instead of *":"*), an error `ParameterError::BadDelimiter(line)` will be returned, where *line* is the line responsible for the error.<br>
Other standard I/O errors is returned by `ParameterError::ReadContentError(runtime_error)` where `runtime_error` is the String display of the IO error.