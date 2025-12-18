fn main()->Result<(),CalcError>{
    // let inputs = "3 +"; // (3+10)/2
    let inputs = "3 10 + 2 /"; // (3+10)/2    

    let mut stack:Vec<f64> = Vec::new();

    for token in inputs.split_whitespace(){
        // try to parse it 
        if let Ok(num) = token.parse::<f64>() {
            stack.push(num);
        }else{
            // means we got an operator
            let b = stack.pop().ok_or(CalcError::NotEnoughOperands)?;  // CRASH
            let a = stack.pop().ok_or(CalcError::NotEnoughOperands)?;

            let result = apply_op(token, a, b)?;

            stack.push(result);
        }
    }

    if stack.len() != 1 {
        return Err(CalcError::NotEnoughOperands)
    }

    println!("The result is : {}", stack.pop().unwrap());

    Ok(())
}


#[derive(Debug)]
enum CalcError{
    NotEnoughOperands,
    UnknownOperator(String),
    DivisonByZero,
}

fn apply_op(op:&str,a:f64,b:f64)->Result<f64,CalcError>{
    let res = match op {
        "+"=> a+b,
        "-"=> a-b,
        "/"=> {
            if b == 0.0{
                return Err(CalcError::DivisonByZero)
            }
            a/b
        },
        "*"=> a*b,
        _=> return Err(CalcError::UnknownOperator(op.to_string())),
    };

    Ok(res)
}
