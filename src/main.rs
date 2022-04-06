enum Lang {
    SetVar(String, Value),
    GetVar(String)
}

enum Value {
    int(i64),
}

fn evaluate(commands: &[Lang]) {}

#[test]

fn test1() {
    let commands = vec![
        Lang::SetVar("x".to_string(), Value::int(1)),
        Lang::GetVar("x".to_string()),
    ];

    let result = evaluate(&commands);
}

fn main() {
    println!("Hello, world!");
}
