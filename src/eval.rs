use data::*;
use parser::{parse_file, parse};

pub fn run_file(file: &str, env: &mut Env) {
    let parsed = parse_file(file);
    run_parsed(file.to_string(), parsed, env);
}

pub fn run_input(input: String, env: &mut Env) {
    let parsed = parse(&input);
    run_parsed(input, parsed, env);
}

fn run_parsed(original: String, parsed: Result<Expr, String>, env: &mut Env) {
    match parsed {
        Ok(rp) => {
            let evaluated = rp.eval(env);
            match evaluated {
                Ok(Some(r)) => println!("{:?}", r),
                Ok(None) => {},
                Err(e) => println!("Eval of input: \r\n\r\n{input}\r\n failed with error: \r\n\r\n {e}", input = original, e = e)
            }
        },
        Err(e) => println!("Parsing of input: \r\n\r\n{input}\r\n failed with error: \r\n\r\n {e}", input = original, e = e)
    }

}

impl Expr {
    pub fn eval(&self, env: &mut Env) -> Result<Option<Object>, String> {
        if let &Expr::Exprs(ref exprs) = self {
            let (head, tail): (&Expr, &[Expr]) = exprs.split_first().unwrap();
            if let Ok(Some(Object::Symbol(ref function_name))) = head.eval(env) {
                let args = tail.to_vec();
                if function_name == &"define".to_string() {
                    
                    Ok(None)
                } else {
                    let mut evaluated: Vec<Object> = Vec::new();
                    for expr in args.iter() {
                        let evalresult = expr.eval(env);
                        match evalresult {
                            Ok(Some(r)) => evaluated.push(r),
                            Ok(None) => {},
                            Err(_) => return evalresult,
                        }
                    }
                    eval_function(function_name, evaluated, env)
                }
            } else {
                Err(format!("Invalid function name {:?}", head))
            }
        } else {
            if let &Expr::Expr(ref object) = self {
                Ok(Some(object.clone()))
            } else {
                Err(format!("Failed to eval {:?}", self))
            }
        }
    }
}

fn eval_function(function_name: &String, args: Vec<Object>, env: &mut Env) -> Result<Option<Object>, String> {
    let function = match_first_function(function_name, env.functions.clone());
    if function.is_ok() {
        let evaluated = (function.ok().unwrap().procedure)(args, env);
        match evaluated {
            Ok(Some(r)) => Ok(Some(r)),
            Ok(None) => Ok(None),
            Err(e) => Err(e)
        }
    } else {
        Err(function.err().unwrap())
    }
}

fn match_first_function<'a>(function_name: &String, functions: Vec<Function<'a>>) -> Result<Function<'a>, String> {
    if functions.is_empty() {
        return Err(format!("No such function {:?}", function_name));
    }
    let (head, tail): (&Function, &[Function]) = functions.split_first().unwrap();
    let current = head;
    if &current.name.to_string() == function_name {
        Ok(current.clone())
    } else {
        match_first_function(function_name, tail.to_vec())
    }
}
