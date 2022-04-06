use std::collections::HashMap;

enum Lang {
    SetVar(String, Value),
    GetVar(String)
}

#[derive(Clone, PartialEq, Debug)]
enum Value {
    Int(i64),
    Nothing,
}

#[derive(Debug)]
enum EvalError{
    MissingVariable(String),
}
struct Evaluator {
    variables: HashMap<String, Value>,
}

impl Evaluator {

    fn new() -> Evaluator {
        Self {
            variables: HashMap::new(),
        }
    }

    fn evaluate(&mut self, commands: &[Lang]) -> Result<Value, EvalError> {
        let mut output = Ok(Value::Nothing);
        for command in commands{
            match command {
                Lang::SetVar(name, value) => {
                    self.variables.insert(name.into(), value.clone());
                },
                Lang::GetVar(name) => {
                    match self.variables.get(name){
                        Some(value) => output = Ok(value.clone()),
                        None => return Err(EvalError::MissingVariable(name.into())),
                    }
                }
            }

        }
        output
    }
}


#[test]

fn test1() -> Result<(), EvalError> {
    let commands = vec![
        Lang::SetVar("x".into(), Value::Int(1)),
        Lang::GetVar("x".into()),
    ];

    let mut evaluator = Evaluator::new();

    let result = evaluator.evaluate(&commands)?;

    assert_eq!(result, Value::Int(1));

    Ok(())
}

fn main() {
    println!("Hello, world!");
}
