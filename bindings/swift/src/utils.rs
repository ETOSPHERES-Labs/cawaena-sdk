//! helper macros for generating From implementations for enums and structs

/// Macro to convert between two enums with the same variants (e.g. sdk and FFI types)
#[macro_export]
macro_rules! convert_enum {
    ($src: ty, $dst: ty, $($variant: ident,)*)=> {
        impl From<$src> for $dst {
            fn from(src: $src) -> Self {
                type SourceT = $src;
                match src {
                    $(SourceT::$variant => Self::$variant,)*
                }
            }
        }
        impl From<$dst> for $src {
            fn from(src: $dst) -> Self {
                type SourceT = $dst;
                match src {
                    $(SourceT::$variant => Self::$variant,)*
                }
            }

        }
    }
}

/// Macro for generating From for src->dst and dst->src by specifying the member field names
#[macro_export]
macro_rules! convert_simple_struct{($src: ty, $dst: ty, $($field: ident,)*)=> {
    impl From<$src> for $dst {
        fn from(src: $src) -> Self {
            Self {
                $($field: src.$field ,)*
            }
        }
    }
}}
