#[macro_export]
macro_rules! hashmap {
    () => (
        std::collections::HashMap::new()
    );
    ($($k:expr => $v:expr),*) => (
        hashmap!($($k => $v,)*)
    );
    ($($k:expr => $v:expr,)*) => {
        {
            let mut map = {
				use macros::count;
                let len = count!($($k),*);
                std::collections::HashMap::with_capacity(len)
            };
            $(map.insert($k, $v);)*
            map
        }
    }
}

#[macro_export]
macro_rules! count {
    () => (0);
    ($el:expr) => (1);
    ($($el:expr,)*) => (
        count!($($el),*)
    );
    ($first:expr, $($rest:expr),*) => (
        1 + count!($($rest),*)
    )
}
