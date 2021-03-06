use std::collections::HashMap;

enum Commands {
    SetVar(String, Value),
    GetVar(String)
}

#[derive(Clone, PartialEq, Debug)]
enum Value {
    Int(i64),
    Nothing,
}

#[derive(Debug)]
enum ParseError{
    MismatchNumParams,
    MismatchType
}

#[derive(Debug)]
enum EvaluationError {
    MissingVariable(String),
}
struct Evaluator {
    variables: HashMap<String, Value>,
}

impl Evaluator {

    fn new() -> Evaluator {
        Evaluator {
            variables: HashMap::new(),
        }
    }
    fn evaluate(&mut self, commands: &[Commands]) -> Result<Value, EvaluationError> {
        let mut output = Ok(Value::Nothing);
        for command in commands{
            match command {
                Commands::SetVar(name, value) => {
                    self.variables.insert(name.into(), value.clone());
                },
                Commands::GetVar(name) => {
                    match self.variables.get(name){
                        Some(value) => output = Ok(value.clone()),
                        None => return Err(EvaluationError::MissingVariable(name.into())),
                    }
                }
            }

        }
        output
    }
}

fn parse_var_name(var_name: &str) -> Result<String, ParseError>{
    Ok(var_name.into())
}

fn parse_value(val: &str) -> Result<String, ParseError>{
    let result = val.parse::<i64>();

    match result {
        Ok(x) => Ok(Value::Int(x)),
        _=> Err(ParseError::MismatchType)
    }
}

fn parse_set(input: &[str]) -> Result<Commands, ParseError>{
    if input.len() !=3 {
        return Err(ParseError::MismatchNumParams);
    }

    let var_name = parse_var_name(input[1])?;
    let value = parse_value(input[2])?;

    Ok(Commands::SetVar(var_name, value))
}

fn parse_get(input: &[str]) -> Result<Commands, ParseError>{
    todo!()
}



fn parse(input: &str) -> Result<Vec<Commands>, ParseError>{



    for line in input.lines() {
        let command = line.split_ascii_whitespace().collect();

        match command.get(0) {
            Some(x) if *x == "set" => {
                parse_set(&command);
            }
            Some(x) if *x == "get" => {
                parse_get(&command);
            }
        }
    }
}

#[test]

fn test1() -> Result<(), EvaluationError> {
    let commands = vec![
        Commands::SetVar("x".into(), Value::Int(1)),
        Commands::GetVar("x".into()),
    ];

    let mut evaluator = Evaluator::new();

    let result = evaluator.evaluate(&commands)?;

    assert_eq!(result, Value::Int(1));

    Ok(())
}

fn main() {
    println!("Hello, world!");
}
