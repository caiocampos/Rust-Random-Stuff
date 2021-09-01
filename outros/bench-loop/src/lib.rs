pub fn hard_work(input: u64) -> f64 {
    let mut aux = input as u128;
    if aux % 2 == 1 {
        aux *= 3;
    }
    aux += 9283;
    aux <<= 3;
    aux -= 1;
    aux >>= 1;
    let mut aux = (aux / 31479) as f64;
    aux *= 1.7654324567865432456;
    aux *= aux.sqrt();
    aux *= std::f64::consts::PI;
    aux /= aux.sin() * (input as f64).ln();
    aux
}
