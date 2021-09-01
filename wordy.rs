pub struct WordProblem;

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
enum OP {
    SUM = 1,
    SUB = 2,
    MUL = 3,
    DIV = 4,
    POW = 5,
}

pub fn answer(command: &str) -> Option<i32> {
    if !command.starts_with("What is ") {
        return None;
    }
    let command = command.trim_start_matches("What is ").trim_end_matches('?');
    let elements: Vec<&str> = command.split_ascii_whitespace().collect();
    let (mut result, mut elements) = match elements.split_first() {
        Some((first, rem)) => match first.parse::<i32>() {
            Ok(n) => (n, rem),
            _ => return None,
        },
        _ => return None,
    };
    loop {
        if elements.is_empty() {
            break Some(result);
        }
        let (op, n) = match (
            elements.get(0),
            elements.get(1),
            elements.get(2),
            elements.get(3),
            elements.get(4),
        ) {
            (Some(&"plus"), Some(&n), _, _, _) => (OP::SUM, n),
            (Some(&"minus"), Some(&n), _, _, _) => (OP::SUB, n),
            (Some(&"multiplied"), Some(&"by"), Some(&n), _, _) => (OP::MUL, n),
            (Some(&"divided"), Some(&"by"), Some(&n), _, _) => (OP::DIV, n),
            (Some(&"raised"), Some(&"to"), Some(&"the"), Some(&n), Some(&"power")) => (OP::POW, n),
            _ => return None,
        };
        let n = match parse_pow(n, &op) {
            Ok(num) => num,
            _ => return None,
        };
        result = match op {
            OP::SUM => (result + n),
            OP::SUB => result - n,
            OP::MUL => result * n,
            OP::DIV => result / n,
            OP::POW => result.pow(n as u32),
        };
        elements = match op {
            OP::SUM | OP::SUB => &elements[2..],
            OP::MUL | OP::DIV => &elements[3..],
            OP::POW => &elements[5..],
        };
    }
}

fn parse_pow(num: &str, op: &OP) -> Result<i32, std::num::ParseIntError> {
    match op {
        OP::POW => match num.contains('-') {
            true => String::new(),
            _ => num.replace(is_not_natural, ""),
        },
        _ => num.into(),
    }
    .parse::<i32>()
}

fn is_not_natural(c: char) -> bool {
    match c {
        '0'..='9' => false,
        _ => true,
    }
}
