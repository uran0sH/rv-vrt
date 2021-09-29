macro_rules! hal_fn_def {
    (
        $(
            $vis:vis mod $mod_name:ident $( : $base:path )? {
                $($fn:tt)*
            }
        )+
    ) => {
        $(
            $vis mod $mod_name {
                #![allow(unused_imports)]
                $( pub use $base::*; )?
                use super::*;
                pub(crate) trait __HalTrait {
                    __hal_fn_unimpl! {
                        mod $mod_name;
                        $($fn)*
                    }
                }
                pub(crate) struct __HalImpl;
                __hal_fn_export! {
                    $($fn)*
                }
            }
        )+
    };
    () => {};
}

macro_rules! hal_fn_impl {
    (
        $(
            impl mod $mod_name:ident$(::$mod_name_more:ident)* {
                $($fn:item)*
            }
        )+
    ) => {
        $(
            __hal_fn_impl_no_export! {
                impl mod $mod_name$(::$mod_name_more)* {
                    $($fn)*
                }
            }
            #[allow(unused_imports)]
            pub use $mod_name$(::$mod_name_more)*::*;
        )+
    };
    () => {};
}

macro_rules! __hal_fn_unimpl {
    (
        mod $mod_name:ident;
        $(#[$inner:ident $($args:tt)*])*
        $vis:vis fn $fn:ident ( $($arg:ident : $type:ty),* ) $( -> $ret:ty )?;
        $($tail:tt)*
    ) => {
        $(#[$inner $($args)*])*
        #[allow(unused_variables)]
        fn $fn ( $($arg : $type),* ) $( -> $ret )? {
            unimplemented!("{}::{}()", stringify!($mod_name), stringify!($fn));
        }
        __hal_fn_unimpl! {
            mod $mod_name;
            $($tail)*
        }
    };
    (
        mod $mod_name:ident;
        $(#[$inner:ident $($args:tt)*])*
        $vis:vis fn $fn:ident ( $($arg:ident : $type:ty),* ) $( -> $ret:ty )? $body:block
        $($tail:tt)*
    ) => {
        $(#[$inner $($args)*])*
        fn $fn ( $($arg : $type),* ) $( -> $ret )? $body
        __hal_fn_unimpl! {
            mod $mod_name;
            $($tail)*
        }
    };
    ( mod $mod_name:ident; ) => {};
}

macro_rules! __hal_fn_export {
    (
        $(#[$inner:ident $($args:tt)*])*
        $vis:vis fn $fn:ident ( $($arg:ident : $type:ty),* ) $( -> $ret:ty )?;
        $($tail:tt)*
    ) => {
        $(#[$inner $($args)*])*
        #[allow(dead_code)]
        $vis fn $fn ( $($arg : $type),* ) $( -> $ret )? {
            __HalImpl::$fn( $($arg),* )
        }
        __hal_fn_export! {
            $($tail)*
        }
    };
    (
        $(#[$inner:ident $($args:tt)*])*
        $vis:vis fn $fn:ident ( $($arg:ident : $type:ty),* ) $( -> $ret:ty )? $body:block
        $($tail:tt)*
    ) => {
        $(#[$inner $($args)*])*
        #[allow(dead_code)]
        $vis fn $fn ( $($arg : $type),* ) $( -> $ret )? {
            __HalImpl::$fn( $($arg),* )
        }
        __hal_fn_export! {
            $($tail)*
        }
    };
    () => {};
}

macro_rules! __hal_fn_impl_no_export {
    (
        $(
            impl mod $mod_name:ident$(::$mod_name_more:ident)* {
                $($fn:item)*
            }
        )+
    ) => {
        $(
            impl $mod_name$(::$mod_name_more)*::__HalTrait for $mod_name$(::$mod_name_more)*::__HalImpl {
                $($fn)*
            }
        )+
    };
    () => {};
}
