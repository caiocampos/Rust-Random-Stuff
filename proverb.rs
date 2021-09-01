pub fn build_proverb(list: &[&str]) -> String {
    build_proverb_vb(list)
}

pub fn build_proverb_va(list: &[&str]) -> String {
    let mut pos = 1;
    list.iter()
        .map(|s| {
            let mut res = String::new();
            if pos == list.len() {
                res.push_str("And all for the want of a ");
                res.push_str(list[0]);
                res.push_str(".");
            } else {
                res.push_str("For want of a ");
                res.push_str(s);
                res.push_str(" the ");
                res.push_str(list[pos]);
                res.push_str(" was lost.");
                pos += 1;
            }
            res
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn build_proverb_vb(list: &[&str]) -> String {
    let and_all = |s: &str| {
        let mut res = String::new();
        res.push_str("And all for the want of a ");
        res.push_str(s);
        res.push_str(".");
        res
    };
    let for_want = |window: &[&str]| {
        let mut res = String::new();
        res.push_str("For want of a ");
        res.push_str(window[0]);
        res.push_str(" the ");
        res.push_str(window[1]);
        res.push_str(" was lost.");
        res
    };
    match list.len() {
        0 => String::new(),
        1 => and_all(list[0]),
        _ => list
            .windows(2)
            .map(|w| for_want(w))
            .chain(std::iter::once(and_all(list[0])))
            .collect::<Vec<_>>()
            .join("\n"),
    }
}
