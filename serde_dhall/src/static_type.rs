use crate::SimpleType;

/// A Rust type that can be represented as a Dhall type.
///
/// A typical example is `Option<bool>`, represented by the Dhall expression `Optional Bool`.
///
/// This trait can be automatically derived, and this is the recommended way of implementing it.
///
/// Some Rust types cannot implement this trait, because there isn't a single Dhall type that
/// corresponds to them. For example, `HashMap<String, u64>` could correspond to multiple different
/// Dhall types, e.g. `{ foo: Natural, bar: Natural }` and `{ baz: Natural }`.
///
/// # Example
///
/// ```rust
/// # fn main() -> serde_dhall::Result<()> {
/// use serde_dhall::{SimpleType, StaticType};
///
/// #[derive(StaticType)]
/// struct Foo {
///     x: bool,
///     y: Vec<u64>,
/// }
///
/// let ty: SimpleType =
///     serde_dhall::from_str("{ x: Bool, y: List Natural }")?;
///
/// assert_eq!(Foo::static_type(), ty);
/// # Ok(())
/// # }
/// ```
pub trait StaticType {
    fn static_type() -> SimpleType;
}

macro_rules! derive_builtin {
    ($rust_ty:ty, $dhall_ty:ident) => {
        impl StaticType for $rust_ty {
            fn static_type() -> SimpleType {
                SimpleType::$dhall_ty
            }
        }
    };
}

derive_builtin!(bool, Bool);
derive_builtin!(usize, Natural);
derive_builtin!(u64, Natural);
derive_builtin!(u32, Natural);
derive_builtin!(isize, Integer);
derive_builtin!(i64, Integer);
derive_builtin!(i32, Integer);
derive_builtin!(f64, Double);
derive_builtin!(f32, Double);
derive_builtin!(String, Text);

impl<A, B> StaticType for (A, B)
where
    A: StaticType,
    B: StaticType,
{
    fn static_type() -> SimpleType {
        SimpleType::Record(
            vec![
                ("_1".to_owned(), A::static_type()),
                ("_2".to_owned(), B::static_type()),
            ]
            .into_iter()
            .collect(),
        )
        .into()
    }
}

impl<T, E> StaticType for std::result::Result<T, E>
where
    T: StaticType,
    E: StaticType,
{
    fn static_type() -> SimpleType {
        SimpleType::Union(
            vec![
                ("Ok".to_owned(), Some(T::static_type())),
                ("Err".to_owned(), Some(E::static_type())),
            ]
            .into_iter()
            .collect(),
        )
        .into()
    }
}

impl<T> StaticType for Option<T>
where
    T: StaticType,
{
    fn static_type() -> SimpleType {
        SimpleType::Optional(Box::new(T::static_type())).into()
    }
}

impl<T> StaticType for Vec<T>
where
    T: StaticType,
{
    fn static_type() -> SimpleType {
        SimpleType::List(Box::new(T::static_type())).into()
    }
}

impl<'a, T> StaticType for &'a T
where
    T: StaticType,
{
    fn static_type() -> SimpleType {
        T::static_type()
    }
}
