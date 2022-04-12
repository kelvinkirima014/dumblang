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
