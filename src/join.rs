use ::std::{
    cell::RefCell,
    fmt,
};

/// `separator.join(iterable)` method on anything [`Display`]able.
///
/// Note that the separator goes in first argument position (before the dot),
/// following Python's style.
///
/// The difference with stdlib's [`.join()`] is that it can take a **lazy**
/// sequence of [`Display`]ables, such as [`Itertools::join()`],
/// while also returning a [lazy struct] that, when [`Display`]ed, shall write
/// each item separated by `self`.
///
/// # Example
///
/// ```rust,edition2018
/// use ::join_lazy_fmt::*;
///
/// let sequence = format!("[{}]", ", ".join(0 .. 5));
/// assert_eq!(sequence, "[0, 1, 2, 3, 4]");
///
/// // Since `.join()` is lazy, this does not compute an infinite string.
/// let _ = ", ".join(0 ..);
///
/// const N: usize = 6;
/// let line = format!("+-{}-+", "-+-".join((1 .. N).map(|_| "---")));
/// // And the following allocates only one `String`:
/// let matrix = format!(
///     "{line}\n{body}\n{line}\n",
///     line=line,
///     body="\n".join(
///         (1 .. N).map(|i| lazy_format!(
///             "| {row} |",
///             row=" | ".join(
///                 (1 .. N).map(|j| lazy_format!(
///                     "a{i}{j}",
///                     i=i,
///                     j=j,
///                 ))
///             ),
///         ))
///     ),
/// );
/// assert_eq!(matrix, "\
/// +-----+-----+-----+-----+-----+
/// | a11 | a12 | a13 | a14 | a15 |
/// | a21 | a22 | a23 | a24 | a25 |
/// | a31 | a32 | a33 | a34 | a35 |
/// | a41 | a42 | a43 | a44 | a45 |
/// | a51 | a52 | a53 | a54 | a55 |
/// +-----+-----+-----+-----+-----+
/// ");
/// ```
///
/// [`Display`]: fmt::Display
/// [`.join()`]: https://doc.rust-lang.org/std/slice/trait.SliceConcatExt.html#tymethod.join
/// [`Itertools::join()`]: https://docs.rs/itertools/0.8.0/itertools/trait.Itertools.html#method.format
/// [lazy struct]: DisplayableJoin
pub
trait Join : fmt::Display
{
    #[inline]
    fn join <Iterable> (
        self: &'_ Self,
        iterable: Iterable,
    ) -> DisplayableJoin<'_, Self, Iterable::IntoIter>
    where
        Iterable       : IntoIterator,
        Iterable::Item : fmt::Display,
    {
        DisplayableJoin {
            separator: self,
            iterator: RefCell::new(iterable.into_iter()),
        }
    }
}

impl<Separator : fmt::Display + ?Sized> Join for Separator {}

/// The return value of [`Join::join`],
/// which is lazily [`Display`]able.
///
/// This means that it can be easily instanciated into a [`String`]
/// by calling [`.to_string()`][`::std::string::ToString`] on it,
/// while remaining lazy by default for a performant combination with nested
/// [`Join::join`].
///
/// # Can only be `Display`ed once!
///
/// This is to avoid having to memoize the iterator values from [`Join::join`].
///
/// So, if you need to [`Display`] it multiple times, you have no other choice
/// but to instanciate it into a [`String`] and [`Display`] that instead
/// (_i.e._, no need to implement a non-zero-cost memoizing logic when
/// memoization remains possible through
/// [conversion to a `String`][::std::string::ToString::to_string]).
///
/// [`Display`]: fmt::Display
pub
struct DisplayableJoin<'sep, Sep, Iter>
where
    Sep        : 'sep + fmt::Display + ?Sized,
    Iter       : Iterator,
    Iter::Item : fmt::Display,
{
    separator: &'sep Sep,
    iterator: RefCell<Iter>,
}

impl<'sep, Sep, Iter> fmt::Display for DisplayableJoin<'sep, Sep, Iter>
where
    Sep        : 'sep + fmt::Display + ?Sized,
    Iter       : Iterator,
    Iter::Item : fmt::Display,
{
    /// Note: This should only be called once.
    ///
    /// If you need to call it multiple times,
    /// you should [convert it into a String][::std::string::ToString]
    fn fmt (
        self: &'_ Self,
        stream: &'_ mut fmt::Formatter<'_>,
    ) -> fmt::Result
    {
        let iterator = &mut *self.iterator.borrow_mut();
        if let Some(first_item) = iterator.next() {
            write!(&mut *stream,
                "{item}", item=first_item,
            )?;
            while let Some(next_item) = iterator.next() {
                write!(&mut *stream,
                    "{sep}{item}", sep=self.separator, item=next_item,
                )?;
            }
        }
        Ok(())
    }
}
