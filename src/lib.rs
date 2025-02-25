#![cfg_attr(yolo_rustc_bootstrap, feature(allow_internal_unstable))]

yolo_rustc_bootstrap::do_crimes!();

#[cfg(yolo_rustc_bootstrap)]
#[macro_export]
#[allow_internal_unstable(allow_internal_unstable)]
macro_rules! nightly_crimes {
    (
        #![feature($($a:ident),* $(,)?)]
        #![feature($($b:ident),* $(,)?)]
        $($code:tt)*
    ) => (
        $crate::nightly_crimes! {
            #![feature($($a,)*$($b),*)]
            $($code)*
        }
    );
    (
        #![feature($($feature:ident),* $(,)?)]
        $($code:tt)*
    ) => (
        #[allow_internal_unstable($($feature,)*)]
        macro_rules! horrible_crimes { () => ( $($code)* ); }
        horrible_crimes! {}
    );
}
