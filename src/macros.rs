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

        $(
            fn to_base(&self, v: f64) -> f64 {
                v * Self::SCALE + $o
            }

            fn to_base(&self, v: f64) -> f64 {
                (v - $o) / Self::SCALE
            }
        )?

        impl Unit for $nym {
            const SCALE: f64 = $f;
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
