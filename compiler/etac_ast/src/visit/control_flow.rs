use std::{convert::Infallible, ops::ControlFlow};

/// The result of a traversal: `()`, or [`ControlFlow<B>`] for a traversal that can stop early.
///
/// This is what lets the same traversal code serve both visitors that always visit the whole tree and visitors that
/// stop at the first `Break`.
///
/// Pick one of the two implementations as your visitor's
/// [`Result`](crate::visit::Visitor::Result); there should be no need to implement this trait for anything else.
///
/// The methods are used behind [`try_visit!`](crate::try_visit!), which is very similar to `?`.
pub trait VisitorResult {
    /// What an early exit carries: `B` for `ControlFlow<B>`, or [`Infallible`] for `()`, which can't exit early.
    type Residual;
    /// The result of a traversal that ran to completion: `()` or `ControlFlow::Continue(())`.
    fn output() -> Self;
    /// The result of a traversal that stopped early: `ControlFlow::Break(r)`.
    fn from_residual(r: Self::Residual) -> Self;
    /// Whether to keep going (`Continue(())`) or stop (`Break` with the residual).
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

/// Evaluates a [`VisitorResult`], returning it from the enclosing function if it is a `Break`.
///
/// Basically `?` for visitor control flow however to be generic over the visitor we need a seperate macro since the
/// result type might be `()` which the [`Try`](std::ops::Try) doesn't support.
///
/// ```
/// use etac_ast::try_visit;
/// use etac_ast::visit::{Visitable, Visitor};
///
/// /// Visits `a` and then `b`, unless visiting `a` stopped the traversal.
/// fn visit_both<'ast, V, A, B>(v: &mut V, a: &'ast A, b: &'ast B) -> V::Result
/// where
///     V: Visitor<'ast>,
///     A: Visitable<'ast, V>,
///     B: Visitable<'ast, V>,
/// {
///     try_visit!(a.visit(v));
///     b.visit(v)
/// }
/// ```
///
/// To visit several nodes in a row like this, [`visit!`](crate::visit!) is shorter.
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

/// Visits each node in turn with a [`Visitor`](crate::visit::Visitor), returning from the enclosing function at the
/// first `Break`.
///
/// `visit!(v, a, b, c)` is shorthand for `try_visit!(a.visit(v)); try_visit!(b.visit(v)); try_visit!(c.visit(v));`
/// (see [`try_visit!`](crate::try_visit!)). Each node is a reference to anything
/// [`Visitable`](crate::visit::Visitable), which includes a couple containers for nodes (see
/// [`VisitResult`](mod@super::visitable)).
///
/// The most common use is to visit some of a node's children by hand:
///
/// ```
/// use etac_ast::ast::WhileStmt;
/// use etac_ast::visit::Visitor;
///
/// /// Visits the bodies of `while` loops, but not their conditions.
/// struct SkipLoopConditions;
///
/// impl<'ast> Visitor<'ast> for SkipLoopConditions {
///     type Result = ();
///
///     fn visit_while_stmt(&mut self, node: &'ast WhileStmt) -> Self::Result {
///         let WhileStmt { span: _, cond: _, body } = node;
///         etac_ast::visit!(self, body);
///     }
/// }
/// ```
///
/// The macro itself evaluates to `()`, so in a hook that returns a `ControlFlow`, follow it with
/// `ControlFlow::Continue(())`.
///
/// [`visit_mut!`](crate::visit_mut!) and [`visit_owned!`](crate::visit_owned!) do the same for the other two visitor
/// flavors.
#[macro_export]
macro_rules! visit {
    ($v:expr, $($node:expr),* $(,)?) => {{
        $( $crate::try_visit!($crate::visit::Visitable::visit($node, $v)); )*
    }};
}

/// Visits each node in turn with a [`VisitorMut`](crate::visit::VisitorMut), returning from the enclosing function
/// at the first `Break`.
///
/// The `&mut` version of [`visit!`](crate::visit!): each node is a `&mut` reference to anything
/// [`VisitableMut`](crate::visit::VisitableMut).
#[macro_export]
macro_rules! visit_mut {
    ($v:expr, $($node:expr),* $(,)?) => {{
        $( $crate::try_visit!($crate::visit::VisitableMut::visit_mut($node, $v)); )*
    }};
}

/// Visits each node in turn with a [`VisitorOwned`](crate::visit::VisitorOwned), returning from the enclosing
/// function at the first `Break`.
///
/// The by-value version of [`visit!`](crate::visit!): each node is moved into the visitor, and can be anything
/// [`VisitableOwned`](crate::visit::VisitableOwned).
#[macro_export]
macro_rules! visit_owned {
    ($v:expr, $($node:expr),* $(,)?) => {{
        $( $crate::try_visit!($crate::visit::VisitableOwned::visit_owned($node, $v)); )*
    }};
}
