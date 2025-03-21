pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// helper for generating From implementations for enums
/// to convert between two enums with the same variants (eg. api_types and gRPC types)
macro_rules! convert_enum{
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
pub(crate) use convert_enum;

/// Macro for generating From for src->dst and dst->src by specifying the member field names
macro_rules! convert_simple_struct {
    ($src: ty, $dst: ty, $($field: ident,)*) => {
        impl From<$src> for $dst {
            fn from(src: $src) -> Self {
                Self {
                    $($field: src.$field ,)*
                }
            }
        }
        impl From<$dst> for $src {
            fn from(src: $dst) -> Self {
                Self {
                    $($field: src.$field ,)*
                }
            }
        }
    }
}

pub(crate) use convert_simple_struct;
