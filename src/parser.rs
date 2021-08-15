use std::{collections::HashMap, env::Args};

use evalexpr::{ContextWithMutableFunctions, Function, Value, context_map, eval_with_context};

#[derive(Debug, Clone)]
struct Circuit {
    name: String,
    inputs: Vec<(String, u64)>,
    outputs: Vec<(String, u64)>,
    command: String,
    meaning: (Vec<String>, Vec<u64>),
}

pub fn parse(inputs: HashMap<String, u64>) {
    let mut statement = "not(A)".to_owned();

    let not_gate = Circuit {
        name: "not".to_owned(),
        inputs: vec![("A".to_owned(), 1)],
        outputs: vec![("Out".to_owned(), 1)],
        command: "nand(A,A)".to_owned(),
        meaning: (vec![], vec![]),
    };
    let mut gates: Vec<Circuit> = vec![];
    gates.push(not_gate);

    let mut context = context_map! {
        "nand" => Function::new(|argument| {
            let arguments = argument.as_tuple()?;
            if let (Value::Int(a), Value::Int(b)) = (&arguments[0], &arguments[1]) {
                Ok(Value::Int(nand(a, b)))
            } else {
                panic!("This is not suppose to happen");
            }
        }),
    }.unwrap();
    
    
    for x in gates {
        let mut index = 0;
        context.set_function(x.name, Function::new(| argument| {
            let arguments = argument.as_tuple()?;
            let mut args: Vec<i64> = Vec::new();
            for v in arguments.iter() {
                if let Value::Int(i) = v {
                    args.push(*i);
                }
            }
            let function = Value::String(format!("{}", x.name));
            Ok(function)
        })).unwrap();
        index += 1;
    }

    for (k, x) in inputs.iter() {
        statement = statement.replace(k, x.to_string().as_str());
    }

    println!("{}", statement);
    println!("{:?}", eval_with_context(statement.as_str(), &context).unwrap());
}

fn nand(a: &i64, b: &i64) -> i64 {
    if (a&b)&(!1) == 0 { 
        return !(a&b)&1
    } else { 
        panic!("This is not supposed to happen");
    }
}