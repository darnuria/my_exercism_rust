#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

// TODO: Not so smart may use function pointers to avoid duplication in the future.
pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    if inputs.is_empty() {
        return None;
    }
    let mut stack = Vec::new();
    for e in inputs {
        match e {
            CalculatorInput::Add => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(b + a);
            }
            CalculatorInput::Subtract => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(b - a);
            }
            CalculatorInput::Multiply => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(b * a);
            }
            CalculatorInput::Divide => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(b / a);
            }
            CalculatorInput::Value(v) => stack.push(*v),
        }
    }
    if stack.len() > 1 {
        return None;
    } else {
        Some(stack[0])
    }
}
