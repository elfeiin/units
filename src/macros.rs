#[macro_export]
macro_rules! units {
    () => {};
    ($m:expr; { $($f:expr $(, $o:expr)? => $nym:ident)* } $($rest:tt)*) => {
        $(
            units! { @inner $m; $f $(, $o)? => $nym }
        )*
        units! { $( $rest )* }
    };
    (@inner $m:expr; $f:expr $(, $o:expr)? => $nym:ident) => {
        #[derive(Copy, Clone)]
        pub struct $nym;

        impl Unit for $nym {
            fn to_base(&self, v: f64) -> f64 {
                v * $f $(+ $o)?
            }

            fn from_base(&self, v: f64) -> f64 {
                (v $(- $o)?) / $f
            }
            fn matrix(&self) -> Units {
                Units::from($m)
            }
        }
    };
    (@inner $m:expr; $f:expr $(, ($o:expr))? => $nym:ident) => {
        #[derive(Copy, Clone)]
        pub struct $nym;

        impl Unit for $nym {
            fn to_base(&self, v: f64) -> f64 {
                (v $(+ $o)?) * $f
            }

            fn from_base(&self, v: f64) -> f64 {
                (v / $f) $(- $o)?
            }
            fn matrix(&self) -> UnitMatrix {
                Units::from($m)
            }
        }
    };
}
