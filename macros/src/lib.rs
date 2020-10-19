/// The desired macro. Methods used in order to achieve the implementation are
/// taken from [a recommended book](https://danielkeep.github.io/tlborm/book/).
#[macro_export]
macro_rules! hashmap {
    // "Private" rule computing the length in elements of a token tree. It produces
    // a slice containing unit values only and uses its `len` method.
    (@len: $($tt:tt),*) => {
        [$($crate::hashmap!(@rep: $tt, ())),*].len()
    };

    // "Private" rule accepting a token and an expression. It discards the token
    // and only returns the expression as-is. Useful for expanding a constant list.
    (@rep: $tt:tt, $ex:expr) => {
        $ex
    };

    // Match void separately in order to forbid a single comma in the next rule.
    () => {
        ::std::collections::HashMap::new()
    };

    // The main rule: expands key to value insertions in a pre-allocated hashmap.
    ($($key:expr => $value:expr),+ $(,)?) => {{
        let mut map = ::std::collections::HashMap
            ::with_capacity($crate::hashmap!(@len: $($key),*));
        $(
            map.insert($key, $value);
        )*
        map
    }};
}
