pub fn verse(n: i32) -> String {
    let bottle = |x: i32, upper: bool| {
        let mut res = String::new();
        let lit = x.to_string();
        res.push_str(match x {
            0 => if upper {
                    "No more"
                } else {
                    "no more"
                },
            _ => lit.as_str()
        });
        res.push_str(match x {
            1 => " bottle of beer",
            _ => " bottles of beer"
        });
        res
    };
    let next: i32 = if n == 0 {
        99
    } else {
        n - 1
    };
    let mut res = String::new();

    res.push_str(&bottle(n, true));
    res.push_str(" on the wall, ");
    res.push_str(&bottle(n, false));
    res.push_str(".\n");
    res.push_str(match n {
        0 => "Go to the store and buy some more",
        1 => "Take it down and pass it around",
        _ => "Take one down and pass it around"
    });
    res.push_str(", ");
    res.push_str(&bottle(next, false));
    res.push_str(" on the wall.\n");
    res
}

pub fn sing(start: i32, end: i32) -> String {
    (end..=start).map(|n| verse(n)).rev().collect::<Vec<String>>().join("\n")
}
