use std::marker::PhantomData;

use sea_query::{ColumnRef, IntoColumnRef, Order};

#[derive(Debug, Clone)]
pub struct Sort<Entity> {
    exprs: Vec<SortExpr<Entity>>,
    entity: PhantomData<Entity>,
}

impl<Entity> Sort<Entity> {
    pub fn new<I>(exprs: I) -> Self
    where
        I: IntoIterator<Item = SortExpr<Entity>>,
    {
        Self {
            exprs: exprs.into_iter().collect(),
            entity: PhantomData,
        }
    }

    pub fn empty() -> Self {
        Self {
            exprs: Vec::new(),
            entity: PhantomData,
        }
    }

    pub fn into_order_exprs(self) -> Vec<(ColumnRef, Order)> {
        self.exprs.into_iter().map(|s| (s.col, s.order)).collect()
    }
}

#[derive(Debug, Clone)]
pub struct SortCol<Entity> {
    col: ColumnRef,
    entity: PhantomData<Entity>,
}

impl<Entity> SortCol<Entity> {
    pub fn new<C: IntoColumnRef>(col: C) -> Self {
        Self {
            col: col.into_column_ref(),
            entity: PhantomData,
        }
    }

    pub fn asc(self) -> SortExpr<Entity> {
        SortExpr {
            col: self.col,
            entity: PhantomData,
            order: Order::Asc,
        }
    }

    pub fn desc(self) -> SortExpr<Entity> {
        SortExpr {
            col: self.col,
            entity: PhantomData,
            order: Order::Desc,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SortExpr<Entity> {
    col: ColumnRef,
    entity: PhantomData<Entity>,
    order: Order,
}
