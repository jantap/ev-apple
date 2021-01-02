// macro to easier implement Message trait which
// defines where do messages go

#[macro_export]
macro_rules! messages_internal {
    ($i:ty) => {
        _
    };
}

#[macro_export]
macro_rules! messages {
    ($enu:ident, $( $x:ident $( ( $( $s:ty),* ) )? => $y:expr ),* $( , )?) => {
        use ev_apple::messages_internal;

        #[derive(Clone, PartialEq, Debug)]
        pub enum $enu {
            $( $x $( ( $( $s ),* ) )? ),*
        }

        impl Message for $enu {
            fn id(&self) -> u32 {
                match self {
                    $( $enu::$x $( ( $( messages_internal!($s) ),* ) )? => $y ),*
                }
            }
        }
    }
}
