#[macro_export]
macro_rules! hashmap {
    () => (
        HashMap::new()
    );
    ( $( $x:expr => $y:expr ),+ $(,)? ) => {
        {
            let mut temp_vec = HashMap::new();
            $(
                temp_vec.insert($x, $y);
            )*
            temp_vec
        }
    };
}