pub trait ResultMonad
where
    Self: Sized,
{
    fn ok_or<E>(self, err: E) -> Result<Self, E>;
    fn ok_or_else<F: FnOnce() -> E, E>(self, err: F) -> Result<Self, E>;
}

// Shamelessly stolen documentation since its the same thing
impl ResultMonad for bool {
    /// Transforms the `bool` into a [`Result<bool, E>`](https://doc.rust-lang.org/stable/core/result/enum.Result.html), mapping [`true`] to
    /// [`Ok(true)`] and [`false`](https://doc.rust-lang.org/stable/core/option/enum.Option.html) to [`Err(err)`].
    ///
    /// Arguments passed to `ok_or` are eagerly evaluated; if you are passing the
    /// result of a function call, it is recommended to use [`ok_or_else`], which is
    /// lazily evaluated.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bool_ok_or::*;
    ///
    /// let x: bool = true;
    /// assert_eq!(x.ok_or(0), Ok(true));
    ///
    /// let x: bool = false;
    /// assert_eq!(x.ok_or(0), Err(0));
    /// ```
    fn ok_or<E>(self, err: E) -> Result<Self, E> {
        match self {
            true => Ok(true),
            false => Err(err),
        }
    }

    /// Transforms the `bool` into a [`Result<bool, E>`](https://doc.rust-lang.org/stable/core/result/enum.Result.html), mapping [`true`] to
    /// [`Ok(true)`] and [`false`](https://doc.rust-lang.org/stable/core/option/enum.Option.html) to [`Err(err())`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bool_ok_or::*;
    ///
    /// let x: bool = true;
    /// assert_eq!(x.ok_or_else(|| 0), Ok(true));
    ///
    /// let x: bool = false;
    /// assert_eq!(x.ok_or_else(|| 0), Err(0));
    /// ```
    fn ok_or_else<F: FnOnce() -> E, E>(self, err: F) -> Result<Self, E> {
        match self {
            true => Ok(true),
            false => Err(err()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn true_to_ok() {
        let x: bool = true;
        assert_eq!(x.ok_or(0), Ok(true));
    }

    #[test]
    fn false_to_err() {
        let x: bool = false;
        assert_eq!(x.ok_or(0), Err(0));
    }

    #[test]
    fn true_to_ok_fn() {
        let x: bool = true;
        assert_eq!(x.ok_or_else(|| 0), Ok(true));
    }

    #[test]
    fn false_to_err_fn() {
        let x: bool = false;
        assert_eq!(x.ok_or_else(|| 0), Err(0));
    }
}
