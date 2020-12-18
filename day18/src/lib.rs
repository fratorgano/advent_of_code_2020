#[derive(Debug)]
enum Operation{
    Add,
    Mul
}

#[derive(Debug)]
enum Unit{
    Number(usize),
    Operator(Operation),
    OpenParenthesis,
    ClosedParenthesis
}
#[derive(Debug)]
struct Expression {
    units: Vec<Unit>
}

impl Expression{
    fn eval(&self)-> usize{
        let mut stack = vec![];
        for unit in &self.units{
            match unit{
                Unit::Number(n) => stack.push(*n),
                Unit::Operator(op) => {
                    let left = stack.pop().unwrap();
                    let right = stack.pop().unwrap();
                    match *op{
                        Operation::Add => stack.push(left+right),
                        Operation::Mul => stack.push(left*right)
                    }
                }
                _ => unreachable!("Error while evaluating expression, found a unit that was unexpected")
            }
        }
        stack.pop().unwrap()
    }
}

fn parse_expression(input: &str) -> Expression{
    let expression_str = input.replace("(", " ( ").replace(")", " ) ");

    let units: Vec<_> = expression_str.split_whitespace()
        .map(|unit| match unit{
            "*" => Unit::Operator(Operation::Mul),
            "+" => Unit::Operator(Operation::Add),
            "(" => Unit::OpenParenthesis,
            ")" => Unit::ClosedParenthesis,
            _ => Unit::Number(unit.parse::<usize>().unwrap())
        }).collect();
    
    let mut output = Vec::new();
    let mut op_stack = Vec::new();
    
    for unit in units {
        //println!("output: {:?}\nop_stack: {:?}\n",output,op_stack);
        match unit{
            Unit::Number(_) => output.push(unit),
            Unit::OpenParenthesis => op_stack.push(unit),
            Unit::ClosedParenthesis => while let Some(operator_unit) = op_stack.pop(){
                match operator_unit{
                    Unit::OpenParenthesis => break,
                    _ => output.push(operator_unit)
                }
            }
            Unit::Operator(_) => {
                while let Some(operator_unit) = op_stack.pop(){
                    match operator_unit{
                        Unit::OpenParenthesis => {
                            op_stack.push(operator_unit);
                            break
                        },
                        _ => output.push(operator_unit)
                    }
                }
                op_stack.push(unit);
            }  
        }
    }

    while let Some(operator_unit) = op_stack.pop(){
        match operator_unit{
            Unit::OpenParenthesis => (),
            _ => output.push(operator_unit)
        }
    }
    //println!("output: {:?}\nop_stack: {:?}\n",output,op_stack);

    Expression{
        units: output
    }
}

fn parse_expression2(input: &str) -> Expression{
    let expression_str = input.replace("(", " ( ").replace(")", " ) ");

    let units: Vec<_> = expression_str.split_whitespace()
        .map(|unit| match unit{
            "*" => Unit::Operator(Operation::Mul),
            "+" => Unit::Operator(Operation::Add),
            "(" => Unit::OpenParenthesis,
            ")" => Unit::ClosedParenthesis,
            _ => Unit::Number(unit.parse::<usize>().unwrap())
        }).collect();
    
    let mut output = Vec::new();
    let mut op_stack = Vec::new();
    
    for unit in units {
        //println!("output: {:?}\nop_stack: {:?}\n",output,op_stack);
        match unit{
            Unit::Number(_) => output.push(unit),
            Unit::OpenParenthesis => op_stack.push(unit),
            Unit::ClosedParenthesis => while let Some(operator_unit) = op_stack.pop(){
                match operator_unit{
                    Unit::OpenParenthesis => break,
                    _ => output.push(operator_unit)
                }
            },
            Unit::Operator(Operation::Add) => {
                while let Some(operator_unit) = op_stack.pop(){
                    match operator_unit{
                        Unit::OpenParenthesis => {
                            op_stack.push(operator_unit);
                            break
                        },
                        Unit::Operator(Operation::Mul) => {
                            op_stack.push(operator_unit);
                            break
                        },
                        _ => output.push(operator_unit)
                    }
                }
                op_stack.push(unit);
            },
            Unit::Operator(Operation::Mul) => {
                while let Some(operator_unit) = op_stack.pop(){
                    match operator_unit{
                        Unit::OpenParenthesis => {
                            op_stack.push(operator_unit);
                            break
                        },
                        _ => output.push(operator_unit)
                    }
                }
                op_stack.push(unit);
            }    
        }
    }

    while let Some(operator_unit) = op_stack.pop(){
        match operator_unit{
            Unit::OpenParenthesis => (),
            _ => output.push(operator_unit)
        }
    }
    //println!("output: {:?}\nop_stack: {:?}\n",output,op_stack);

    Expression{
        units: output
    }
}

pub fn sum_results(input: &String) -> usize{
    let mut sum = 0;
    for l in input.lines(){
        let expr = parse_expression(l);
        sum+=expr.eval()
    }
    sum
}

pub fn sum_results2(input: &String) -> usize{
    let mut sum = 0;
    for l in input.lines(){
        let expr = parse_expression2(l);
        sum+=expr.eval()
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day18_test() {
        let input ="2 * 3 + (4 * 5)";
        let expr = parse_expression(input);
        //println!("{:?}",expr);
        assert_eq!(26, expr.eval())
    }
    #[test]
    fn day18_test_2() {
        let input = String::from("5 + (8 * 3 + 9 + 3 * 4 * 3)");
        assert_eq!(437, sum_results(&input))
    }

    #[test]
    fn day18_test_3() {
        let input = String::from("2 * 3 + (4 * 5)");
        assert_eq!(46, sum_results2(&input))
    }
    #[test]
    fn day18_test_4() {
        let input = String::from("5 + (8 * 3 + 9 + 3 * 4 * 3)");
        assert_eq!(1445, sum_results2(&input))
    }
}