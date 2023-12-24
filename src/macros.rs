#[macro_export]
macro_rules! units {
    () => {};
    ([$l:expr, $t:expr, $m:expr, $c:expr, $k:expr, $a:expr, $b:expr] { $($f:expr $(, $o:expr)? => $nym:ident)* } $($rest:tt)*) => {
        $(
            units! { @inner [$l, $t, $m, $c, $k, $a, $b] $f $(, $o)? => $nym }
        )*
        units! { $( $rest )* }
    };
    (@inner [$l:expr, $t:expr, $m:expr, $c:expr, $k:expr, $a:expr, $b:expr] $f:expr $(, $o:expr)? => $nym:ident) => {
        #[derive(Copy, Clone)]
        pub struct $nym;

        impl Unit for $nym {
            fn to_base(&self, v: f64) -> f64 {
                v * $f $(+ $o)?
            }

            fn from_base(&self, v: f64) -> f64 {
                (v $(- $o)?) / $f
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
