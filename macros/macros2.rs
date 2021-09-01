#[macro_export]
macro_rules! hashmap {
    (@el $el:expr) => (());
    (@len $($el:expr),*) => (
		<[()]>::len(&[$(hashmap!(@el $el)),*])
    );
    () => (
        std::collections::HashMap::new()
    );
    ($($k:expr => $v:expr),* $(,)?) => {
        {
            let mut hm = {
				let len = hashmap!(@len $($k),*);
				std::collections::HashMap::with_capacity(len)
			};
            $(hm.insert($k, $v);)*
            hm
        }
    };
}