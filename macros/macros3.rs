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
                let len = [$($k),*].len();
                std::collections::HashMap::with_capacity(len)
            };
            $(map.insert($k, $v);)*
            map
        }
    }
}
