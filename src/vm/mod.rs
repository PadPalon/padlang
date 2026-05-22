use crate::{Assignment, Binary, Expr, Literal, Operator, PrettyPrint, Program, State, StateValue};

pub fn run(program: Program) {
    program.expressions.iter().fold(
        State {
            strings: Default::default(),
            numbers: Default::default(),
            booleans: Default::default(),
        },
        |mut state, expression| {
            let value = evaluate_expression(expression.clone(), &mut state);
            println!("{}", value);
            state
        },
    );
    return;
}

fn evaluate_expression(expression: Expr, state: &mut State) -> StateValue {
    match expression {
        Expr::Literal(literal) => evaluate_literal(state, literal),
        Expr::Binary(binary) => evaluate_binary(&binary, state),
        Expr::Assignment(assignment) => evaluate_assignment(state, assignment),
        _ => {
            println!("Unexpected expression {}", expression.pretty_print());
            todo!()
        }
    }
}

fn evaluate_literal(state: &mut State, literal: Literal) -> StateValue {
    match literal {
        Literal::String(value) => StateValue::String(value),
        Literal::Number(value) => StateValue::Number(value),
        Literal::True() => StateValue::Boolean(true),
        Literal::False() => StateValue::Boolean(false),
        Literal::Identifier(identifier) => {
            if state.strings.contains_key(&identifier) {
                StateValue::String(state.strings.get(&identifier).unwrap().clone())
            } else if state.numbers.contains_key(&identifier) {
                StateValue::Number(state.numbers.get(&identifier).unwrap().clone())
            } else if state.booleans.contains_key(&identifier) {
                StateValue::Boolean(state.booleans.get(&identifier).unwrap().clone())
            } else {
                panic!("Undefined identifier {}", identifier.to_string());
            }
        }
    }
}

fn evaluate_binary(binary: &Box<Binary>, state: &mut State) -> StateValue {
    let left = evaluate_expression(*binary.left.clone(), state);
    let right = evaluate_expression(*binary.right.clone(), state);
    let result = match binary.operator {
        Operator::And => {
            let left_boolean = match left {
                StateValue::Boolean(value) => value,
                _ => false,
            };
            let right_boolean = match right {
                StateValue::Boolean(value) => value,
                _ => false,
            };
            left_boolean && right_boolean
        }
        Operator::Or => {
            let left_boolean = match left {
                StateValue::Boolean(value) => value,
                _ => false,
            };
            let right_boolean = match right {
                StateValue::Boolean(value) => value,
                _ => false,
            };
            left_boolean || right_boolean
        }
        Operator::Xor => {
            let left_boolean = match left {
                StateValue::Boolean(value) => value,
                _ => false,
            };
            let right_boolean = match right {
                StateValue::Boolean(value) => value,
                _ => false,
            };
            left_boolean ^ right_boolean
        }
        Operator::Is => left == right,
    };
    StateValue::Boolean(result)
}

fn evaluate_assignment(state: &mut State, assignment: Box<Assignment>) -> StateValue {
    let state_value = evaluate_expression(*assignment.expression.clone(), state);
    match state_value {
        StateValue::String(ref value) => {
            state
                .strings
                .insert(assignment.identifier.clone(), value.clone());
        }
        StateValue::Number(ref value) => {
            state.numbers.insert(assignment.identifier.clone(), *value);
        }
        StateValue::Boolean(ref value) => {
            state.booleans.insert(assignment.identifier.clone(), *value);
        }
    }
    state_value
}
