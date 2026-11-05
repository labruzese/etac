use std::{convert::Infallible, ops::ControlFlow};

pub trait VisitorResult {
    type Residual;
    fn output() -> Self;
    fn from_residual(r: Self::Residual) -> Self;
    fn branch(self) -> ControlFlow<Self::Residual>;
}

impl VisitorResult for () {
    type Residual = Infallible;
    fn output() -> Self {}
    fn from_residual(r: Infallible) -> Self {
        match r {}
    }
    fn branch(self) -> ControlFlow<Infallible> {
        ControlFlow::Continue(())
    }
}

impl<T> VisitorResult for ControlFlow<T> {
    type Residual = T;
    fn output() -> Self {
        ControlFlow::Continue(())
    }
    fn from_residual(r: T) -> Self {
        ControlFlow::Break(r)
    }
    fn branch(self) -> ControlFlow<T> {
        self
    }
}

#[macro_export]
macro_rules! try_visit {
    ($e:expr) => {
        match $crate::visit::VisitorResult::branch($e) {
            ::core::ops::ControlFlow::Continue(()) => {}
            #[allow(unreachable_code)]
            ::core::ops::ControlFlow::Break(r) => {
                return $crate::visit::VisitorResult::from_residual(r);
            }
        }
    };
}

#[macro_export]
macro_rules! visit {
    ($v:expr, $($node:expr),* $(,)?) => {{
        $( $crate::try_visit!($crate::visit::Visitable::visit($node, $v)); )*
    }};
}
 
#[macro_export]
macro_rules! visit_mut {
    ($v:expr, $($node:expr),* $(,)?) => {{
        $( $crate::try_visit!($crate::visit::VisitableMut::visit_mut($node, $v)); )*
    }};
}
 
#[macro_export]
macro_rules! visit_owned {
    ($v:expr, $($node:expr),* $(,)?) => {{
        $( $crate::try_visit!($crate::visit::VisitableOwned::visit_owned($node, $v)); )*
    }};
}

