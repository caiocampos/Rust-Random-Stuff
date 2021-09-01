pub fn is_armstrong_number(num: u32) -> bool {
    let numbers: Vec<u32> = num.to_string().chars()
        .map(|c| c.to_digit(10).unwrap()).collect();

    let len = numbers.len() as u32;

    let res = numbers.iter().map(|val| val.pow(len)).sum::<u32>();
    res == num
}
