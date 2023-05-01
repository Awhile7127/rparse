# rparse


## Description

Simple human-readable command-line argument parser.


## Usage

`rparse` allows for users to build their own command-line arguments
in the form of `Argument` structs in a vector array.

These are then passed alongside `std::env::args()` to a parsing module
to check and fetch command-line arguments.


### Example

```
use rparse::{parser, builder::Argument};

// Build arguments into a vector array
fn build_arguments() -> Vec<Argument> {
    let args: Vec<Argument> = vec![
        Argument::new("feeds", "--feed", "-f"),
        Argument::new("file", "--file", "-i"),
    ];
    return args;
}

fn main() {

    // Build arguments
    let args: Vec<Argument> = build_arguments();

    // Fetch arguments passed to the program
    let passed = std::env::args();

    // Parse command-line arguments
    let returned = parser::parse(args, passed).args;
    println!("{:?}", returned["feeds"]);
}
```


### Notes

`parser::parse(args, passed)` returns a `ParsedArguments` struct.
This contains a HashMap of arguments found on the command line,
referenced as `args`.
To then access this, call the key from the HashMap:
`parser::parse(args, passed).args["key"]`.
