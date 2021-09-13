#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}
fn add(x: i32, y:i32) -> i32 { x + y}
fn sub(x: i32, y:i32) -> i32 { x - y}
fn mul(x: i32, y:i32) -> i32 { x * y}
fn div(x: i32, y:i32) -> i32 { x / y}

fn apply(acc: &mut Vec<i32>, f: fn(i32, i32)->i32) {
    if acc.is_empty() { return; }
    let second = acc.pop().unwrap();
    if acc.is_empty() { return; }
    let first = acc.pop().unwrap();
    let ans = f(first, second);
    acc.push(ans);
}
pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut acc = Vec::new();
    for input in inputs {
        match input {
            CalculatorInput::Add      => apply(&mut acc, add),
            CalculatorInput::Subtract => apply(&mut acc, sub),
            CalculatorInput::Multiply => apply(&mut acc, mul),
            CalculatorInput::Divide   => apply(&mut acc, div),
            &CalculatorInput::Value(x) => acc.push(x)
        }
    }
    if acc.len() != 1 { return None; }
    acc.pop()
}
