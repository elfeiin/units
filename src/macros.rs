#[macro_export]
macro_rules! units {
    () => {};
    ([$l:expr, $t:expr, $m:expr, $c:expr, $k:expr, $a:expr, $b:expr] { $($f:expr => $nym:ident)* } $($rest:tt)*) => {
        $(
            units! { @inner [$l, $t, $m, $c, $k, $a, $b] $f => $nym }
        )*
        units! { $( $rest )* }
    };
    (@inner [$l:expr, $t:expr, $m:expr, $c:expr, $k:expr, $a:expr, $b:expr] $f:expr => $nym:ident) => {
        #[derive(Copy, Clone)]
        pub struct $nym;

        impl Unit for $nym {
            fn scale(&self) -> f64 {
                $f
            }
            fn matrix(&self) -> UnitMatrix {
                UnitMatrix {
                    length: $l,
                    time: $t,
                    mass: $m,
                    current: $c,
                    thermal: $k,
                    amount: $a,
                    candela: $b,
                }
            }
        }
    };
}
