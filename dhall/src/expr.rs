use crate::imports::ImportRoot;
use dhall_core::*;
use std::marker::PhantomData;

macro_rules! derive_other_traits {
    ($ty:ident) => {
        impl<'a> std::cmp::PartialEq for $ty<'a> {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl<'a> std::fmt::Display for $ty<'a> {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter,
            ) -> Result<(), std::fmt::Error> {
                self.0.fmt(f)
            }
        }
    };
}

#[derive(Debug, Clone, Eq)]
pub struct Parsed<'a>(
    pub(crate) SubExpr<X, Import>,
    pub(crate) ImportRoot,
    pub(crate) PhantomData<&'a ()>,
);
derive_other_traits!(Parsed);

#[derive(Debug, Clone, Eq)]
pub struct Resolved<'a>(
    pub(crate) SubExpr<X, Normalized<'static>>,
    pub(crate) PhantomData<&'a ()>,
);
derive_other_traits!(Resolved);

#[derive(Debug, Clone, Eq)]
pub struct Typed<'a>(
    pub(crate) SubExpr<X, Normalized<'static>>,
    pub(crate) Option<Type<'static>>,
    pub(crate) PhantomData<&'a ()>,
);
derive_other_traits!(Typed);

#[derive(Debug, Clone, Eq)]
pub struct Normalized<'a>(
    pub(crate) SubExpr<X, X>,
    pub(crate) Option<Type<'static>>,
    pub(crate) PhantomData<&'a ()>,
);
derive_other_traits!(Normalized);

/// An expression of type `Type` (like `Bool` or `Natural -> Text`, but not `Type`)
#[derive(Debug, Clone, Eq)]
pub struct SimpleType<'a>(
    pub(crate) SubExpr<X, X>,
    pub(crate) PhantomData<&'a ()>,
);
derive_other_traits!(SimpleType);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type<'a>(pub(crate) TypeInternal<'a>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TypeInternal<'a> {
    Expr(Box<Normalized<'a>>),
    // The type of `Sort`
    SuperType,
}

// Exposed for the macros
#[doc(hidden)]
impl<'a> From<SimpleType<'a>> for SubExpr<X, X> {
    fn from(x: SimpleType<'a>) -> SubExpr<X, X> {
        x.0
    }
}

// Exposed for the macros
#[doc(hidden)]
impl<'a> From<SubExpr<X, X>> for SimpleType<'a> {
    fn from(x: SubExpr<X, X>) -> SimpleType<'a> {
        SimpleType(x, PhantomData)
    }
}

// Exposed for the macros
#[doc(hidden)]
impl<'a> From<Normalized<'a>> for Typed<'a> {
    fn from(x: Normalized<'a>) -> Typed<'a> {
        Typed(x.0.absurd(), x.1, x.2)
    }
}

impl<'a> Typed<'a> {
    pub(crate) fn as_expr(&self) -> &SubExpr<X, Normalized<'a>> {
        &self.0
    }
    pub(crate) fn into_expr(self) -> SubExpr<X, Normalized<'a>> {
        self.0
    }
}

impl<'a> Normalized<'a> {
    pub(crate) fn as_expr(&self) -> &SubExpr<X, X> {
        &self.0
    }
    pub(crate) fn into_expr<T>(self) -> SubExpr<X, T> {
        self.0.absurd()
    }
    pub(crate) fn into_type(self) -> Type<'a> {
        crate::expr::Type(TypeInternal::Expr(Box::new(self)))
    }
}

impl<'a> SimpleType<'a> {
    pub(crate) fn into_type(self) -> Type<'a> {
        Normalized(self.0, Some(Type::const_type()), PhantomData).into_type()
    }
}
