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
    let min = min_factor(num);
    if min == num {
        return Some(Classification::Deficient);
    }
    match 1 + (min..=(num / min)).filter(|n| num % n == 0).sum::<u64>() {
        sum if sum > num => Some(Classification::Abundant),
        sum if sum < num => Some(Classification::Deficient),
        _ => Some(Classification::Perfect),
    }
}

fn min_factor(num: u64) -> u64 {
    match num {
        1 => 1,
        _ if num % 2 == 0 => 2,
        _ if num % 3 == 0 => 3,
        _ => {
            let mut i = 5;
            while i * i <= num {
                match true {
                    _ if num % i == 0 => return i,
                    _ if num % (i + 2) == 0 => return i + 2,
                    _ => i += 6,
                }
            }
            i
        }
    }
}
