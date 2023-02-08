#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut cal: Option<i32> = None;
    let mut value = vec![];
    let mut new_value = vec![];

    if inputs.len() == 1 {
        return match inputs[0] {
            CalculatorInput::Add => None,
            CalculatorInput::Subtract => None,
            CalculatorInput::Multiply => None,
            CalculatorInput::Divide => None,
            CalculatorInput::Value(val) => Some(val)
        }
    }

    if inputs.len() == 2 {
        return match inputs[1] {
            CalculatorInput::Add => None,
            CalculatorInput::Subtract => None,
            CalculatorInput::Multiply => None,
            CalculatorInput::Divide => None,
            CalculatorInput::Value(_) => None
        }
    }
    for (i, input) in inputs.iter().enumerate() {
        if i == 0 {
            match inputs[i] {
                CalculatorInput::Add => return None,
                CalculatorInput::Subtract => return None,
                CalculatorInput::Multiply => return None,
                CalculatorInput::Divide => return None,
                _ => {}
            }
        }
        match input {
            CalculatorInput::Add => {
                cal = Some(value.iter().sum());
                new_value.push(cal);
                value.drain(0..);
            }
            CalculatorInput::Subtract => {
                cal = Some(value.iter().fold(0, |acc, x| {
                    if acc == 0 {
                        *x
                    } else {
                        acc - x
                    }
                }));
                new_value.push(cal);
            },
            CalculatorInput::Multiply => {
                cal = Some(value.iter().fold(0, |acc, x| {
                    if acc == 0 {
                        *x
                    } else {
                        acc * x
                    }
                }));
                new_value.push(cal);
            },
            CalculatorInput::Divide => {
                if new_value.len() == 2 {
                    cal = Some(new_value[0].unwrap()/new_value[1].unwrap())
                } else {
                    cal = Some(value.iter().fold(0, |acc, x| {
                        if acc == 0 {
                            *x
                        } else {
                            acc / x
                        }
                    }));
                    new_value.push(cal);
                }
            },
            CalculatorInput::Value(val) => value.push(*val)
        }
    }

    cal
}
