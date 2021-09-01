#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num <= 0 {
        return None;
    }
    let sum: u64 = (1..num).filter(|n| num % n == 0).sum();
    match true {
        _ if sum > num => Some(Classification::Abundant),
        _ if sum < num => Some(Classification::Deficient),
        _ => Some(Classification::Perfect),
    }
}
