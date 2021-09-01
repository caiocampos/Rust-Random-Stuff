use std::cmp::Ordering;

fn main() {
    println!("{:?}", to_target_length(100, 100, 100));
    println!("{:?}", to_target_length(90, 100, 100));
    println!("{:?}", to_target_length(100, 90, 100));
    println!("{:?}", to_target_length(90, 80, 100));
    println!("{:?}", to_target_length(80, 90, 100));
    println!("{:?}", to_target_length(80, 80, 100));
    println!("{:?}", to_target_length(100, 90, 90));
    println!("{:?}", to_target_length(90, 100, 90));
    println!("{:?}", to_target_length(100, 100, 90));
    println!("{:?}", to_target_length(90, 100, 80));
    println!("{:?}", to_target_length(100, 90, 80));
    println!("{:?}", to_target_length(100, 100, 80));
    println!("{:?}", to_target_length(90, 70, 80));
    println!("{:?}", to_target_length(70, 90, 80));
    println!("{:?}", to_target_length(100, 100, 80));
    println!("{:?}", to_target_dimension(90, 70, 80, 70));
    println!("{:?}", to_target_dimension(70, 90, 80, 70));
    println!("{:?}", to_target_dimension(100, 100, 70, 80));
    println!("{:?}", to_target_dimension(90, 70, 70, 80));
    println!("{:?}", to_target_dimension(70, 90, 70, 80));
    println!("{:?}", to_target_dimension(100, 100, 70, 80));
}

fn to_target_length(width: u32, height: u32, target: u32) -> (u32, u32) {
    match width.cmp(&height) {
        Ordering::Greater => (target, (height * target) / width),
        Ordering::Less => ((width * target) / height, target),
        Ordering::Equal => (target, target),
    }
}

fn to_target_dimension(width: u32, height: u32, target_w: u32, target_h: u32) -> (u32, u32) {
    to_target_length(width, height, target_w.min(target_h))
}
