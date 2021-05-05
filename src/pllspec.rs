use crate::Result;
use anyhow::anyhow;

/*
    Simple grammar:

    image => statements
    statements => statement statements
               =>
    statement => cubie op cubie
    cubie => [1-9]
    op => '<'
       => '>'
       => '<>'
*/

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Program {
    pub statements: Statements,
}

pub fn parse_program(input: &str) -> Result<Program> {
    let (statements, _) = parse_statements(input)?;

    Ok(Program { statements })
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Statements {
    pub statements: Vec<Statement>,
}

fn parse_statements(input: &str) -> Result<(Statements, &str)> {
    let mut statements = vec![];

    let mut tail = input;
    while !tail.is_empty() {
        let (statement, tail_) = parse_statement(tail)?;
        tail = tail_;
        statements.push(statement);
    }

    Ok((Statements { statements }, input))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Statement {
    pub start: Cubie,
    pub end: Cubie,
    pub op: Operator,
}

fn parse_statement(input: &str) -> Result<(Statement, &str)> {
    let input = input.trim();
    let (start, input) = parse_cubie(input)?;
    let input = input.trim();
    let (op, input) = parse_operator(input)?;
    let input = input.trim();
    let (end, input) = parse_cubie(input)?;

    Ok((Statement { start, op, end }, input))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operator {
    StartHead,
    EndHead,
    BothHead,
}

fn parse_operator(input: &str) -> Result<(Operator, &str)> {
    let input = input.trim();
    if input.starts_with("<>") {
        Ok((Operator::BothHead, &input[2..]))
    } else if input.starts_with("<") {
        Ok((Operator::StartHead, &input[1..]))
    } else if input.starts_with(">") {
        Ok((Operator::EndHead, &input[1..]))
    } else {
        Err(anyhow!(
            "Unexpected string, {}, found while parsing Operator",
            &input[..1]
        ))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cubie {
    pub idx: u8, // 0-indexed, 0-8.
}

fn parse_cubie(input: &str) -> Result<(Cubie, &str)> {
    let input = input.trim();
    let cubie_index = input[0..1].parse::<u8>()?;

    if cubie_index < 1 || cubie_index > 9 {
        return Err(anyhow!(
            "Out-of-range value, {}, found while parsing Cubie",
            cubie_index
        ));
    }

    Ok((
        Cubie {
            idx: cubie_index - 1,
        },
        &input[1..],
    ))
}
