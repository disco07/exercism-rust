#[macro_export]
macro_rules! hashmap {
    () => (
        ::std::collections::HashMap::new()
    );
    ( $( $x:expr => $y:expr ),+ $(,)? ) => {
        {
            let mut temp_vec = ::std::collections::HashMap::new();
            $(
                temp_vec.insert($x, $y);
            )*
            temp_vec
        }
    };
}