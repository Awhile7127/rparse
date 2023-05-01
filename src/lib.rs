pub mod builder {

    // Declare struct used to create simple command-line flags

    #[derive(Debug, PartialEq)]
    pub struct Argument {
        pub name: String,
        pub long: String,
        pub short: String,
    }

    impl Argument {

        // Create argument as struct
        pub fn new(name: &str, long: &str, short: &str) -> Argument {
            let arg = Argument {
                name: name.to_string(),
                long: long.to_string(),
                short: short.to_string(),
            };
            return arg;
        }
    }
}


pub mod parser {
    use crate::builder::Argument;
    use std::env::Args;
    use std::collections::HashMap;

    // Declare struct for parsed command-line arguments

    #[derive(Debug)]
    pub struct ParsedArguments {

        // ParsedArguments contains object "args"
        // Comprising a HashMap (similar to a dictionary)
        pub args: HashMap<String, String>,
    }


    impl ParsedArguments {

        // Used to declare a new struct
        pub fn new() -> ParsedArguments {
            return ParsedArguments {
                args: HashMap::new(),
            };
        }
    }
    

    pub fn parse(args: Vec<Argument>, passed: Args) -> ParsedArguments {

        // Implement peeking to the next command-line argument,
        // Instead of std::env::Args.nth()
        let mut args_iter = passed.peekable();

        // Create a new ParsedArguments struct
        let mut parsed_args = ParsedArguments::new();

        // While there is still an item left to iterate,
        // and there are no None elements, loop
        while let Some(current_arg) = args_iter.next() {

            // Match the currently iterated command-line argument
            // against the args built and passed to this function earlier
            // in the mod builder
            let found = args
                .iter()
                .find(|arg| arg.long == current_arg || arg.short == current_arg);

            // If a command-line argument provided matches one found in the vector
            // array passed to the function, fetch its value,
            // populate a struct with its information and push it to the earlier
            // declared vector array
            if let Some(arg) = found {

                // If no argument is provided after the flag,
                // default to an empty string
                let next_arg = args_iter
                    .peek()
                    .unwrap_or(&String::new())
                    .to_string();

                // Insert values into the HashMap
                parsed_args.args.insert(arg.name.clone(), next_arg);
            };
        };
        return parsed_args;
    }
}
