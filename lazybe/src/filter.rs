use std::marker::PhantomData;

use sea_query::{Cond, Condition, ConditionExpression, Expr, IntoColumnRef, IntoLikeExpr, SimpleExpr};

#[derive(Debug, Clone)]
pub struct Filter<Entity> {
    cond: Condition,
    entity: PhantomData<Entity>,
}

impl<Entity> From<Filter<Entity>> for ConditionExpression {
    fn from(value: Filter<Entity>) -> Self {
        Self::Condition(value.cond)
    }
}

impl<Entity> From<FilterExpr<Entity>> for Filter<Entity> {
    fn from(value: FilterExpr<Entity>) -> Self {
        let cond = Cond::all().add(value.expr);
        Self {
            cond,
            entity: PhantomData,
        }
    }
}

impl<Entity> Filter<Entity> {
    pub fn empty() -> Self {
        Self {
            cond: Cond::all(),
            entity: PhantomData,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn not(self) -> Self {
        Self {
            cond: self.cond.not(),
            entity: PhantomData,
        }
    }

    pub fn all<E>(exprs: impl IntoIterator<Item = E>) -> Self
    where
        E: Into<Filter<Entity>>,
    {
        let cond = Cond::all();
        Self::add_exprs_to_cond(cond, exprs)
    }

    pub fn any<E>(exprs: impl IntoIterator<Item = E>) -> Self
    where
        E: Into<Filter<Entity>>,
    {
        let cond = Cond::any();
        Self::add_exprs_to_cond(cond, exprs)
    }

    fn add_exprs_to_cond<E>(mut cond: Condition, exprs: impl IntoIterator<Item = E>) -> Self
    where
        E: Into<Filter<Entity>>,
    {
        for expr in exprs {
            cond = cond.add(expr.into());
        }
        Self {
            cond,
            entity: PhantomData,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FilterExpr<Entity> {
    expr: SimpleExpr,
    entity: PhantomData<Entity>,
}

impl<Entity> FilterExpr<Entity> {
    pub fn into_expr(self) -> SimpleExpr {
        self.expr
    }
}

#[derive(Debug, Clone)]
pub struct FilterCol<Entity, Col> {
    col_expr: Expr,
    entity: PhantomData<Entity>,
    col_ty: PhantomData<Col>,
}

impl<Entity, Col> FilterCol<Entity, Col> {
    pub fn new<C: IntoColumnRef>(col: C) -> Self {
        Self {
            col_expr: Expr::col(col),
            entity: PhantomData,
            col_ty: PhantomData,
        }
    }
}

impl<Entity, Col> FilterCol<Entity, Col>
where
    Col: Into<SimpleExpr>,
{
    pub fn eq(self, value: Col) -> FilterExpr<Entity> {
        let expr = self.col_expr.eq(value);
        FilterExpr {
            expr,
            entity: PhantomData,
        }
    }

    pub fn neq(self, value: Col) -> FilterExpr<Entity> {
        let expr = self.col_expr.ne(value);
        FilterExpr {
            expr,
            entity: PhantomData,
        }
    }

    pub fn gt(self, value: Col) -> FilterExpr<Entity> {
        let expr = self.col_expr.gt(value);
        FilterExpr {
            expr,
            entity: PhantomData,
        }
    }

    pub fn gte(self, value: Col) -> FilterExpr<Entity> {
        let expr = self.col_expr.gte(value);
        FilterExpr {
            expr,
            entity: PhantomData,
        }
    }

    pub fn lt(self, value: Col) -> FilterExpr<Entity> {
        let expr = self.col_expr.lt(value);
        FilterExpr {
            expr,
            entity: PhantomData,
        }
    }

    pub fn lte(self, value: Col) -> FilterExpr<Entity> {
        let expr = self.col_expr.lte(value);
        FilterExpr {
            expr,
            entity: PhantomData,
        }
    }

    pub fn is_in<I>(self, values: I) -> FilterExpr<Entity>
    where
        I: IntoIterator<Item = Col>,
    {
        let expr = self.col_expr.is_in(values);
        FilterExpr {
            expr,
            entity: PhantomData,
        }
    }

    pub fn is_null(self) -> FilterExpr<Entity>
    where
        Col: IsNullFilterable,
    {
        let expr = self.col_expr.is_null();
        FilterExpr {
            expr,
            entity: PhantomData,
        }
    }

    pub fn like<S>(self, like_stm: S) -> FilterExpr<Entity>
    where
        Col: LikeFilterable,
        S: IntoLikeExpr,
    {
        let expr = self.col_expr.like(like_stm);
        FilterExpr {
            expr,
            entity: PhantomData,
        }
    }

    pub fn not_like<S>(self, like_stm: S) -> FilterExpr<Entity>
    where
        Col: LikeFilterable,
        S: IntoLikeExpr,
    {
        let expr = self.col_expr.not_like(like_stm);
        FilterExpr {
            expr,
            entity: PhantomData,
        }
    }
}

pub trait LikeFilterable {}
impl LikeFilterable for String {}
impl LikeFilterable for &str {}
impl LikeFilterable for Option<String> {}
impl LikeFilterable for Option<&str> {}

pub trait IsNullFilterable {}
impl<T> IsNullFilterable for Option<T> {}
