use ::std::fmt;

/// Lazy formatting, see [LazyFormat](
/// https://docs.rs/join-lazy-fmt/0.9.2/join_lazy_fmt/struct.LazyFormat.html).
#[macro_export]
macro_rules! lazy_format {(
    $fmt:expr $(, $($args:tt)*)?
) => (
    $crate::LazyFormat(move |stream|
        write!(stream, $fmt $(, $($args)*)? )
    )
)}

/// Value returned by [`lazy_format!`]: struct implementing [`fmt::Display`],
/// thanks to the inner closure expliciting the formatting logic.
///
/// Useful, for instance, for a performant combination with [`Join::join`][
/// crate::Join::join].
///
/// Although you may use it directly (it's just a wrapper around a closure),
/// it may be easier to use in conjunction with the [`lazy_format!`] macro.
#[derive(Debug, Clone, Copy)]
pub
struct LazyFormat<Closure> /* = */ (pub Closure)
where
    Closure : Fn(&mut fmt::Formatter<'_>) -> fmt::Result,
;

impl<Closure> fmt::Display for LazyFormat<Closure>
where
    Closure : Fn(&mut fmt::Formatter<'_>) -> fmt::Result,
{
    #[inline]
    fn fmt (
        self: &Self,
        other: &mut fmt::Formatter<'_>,
    ) -> fmt::Result
    {
        self.0(other)
    }
}
