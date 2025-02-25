mod builder;
mod node;
mod scope;

use std::rc::Rc;

pub use builder::SemanticBuilder;
pub use node::{AstNode, AstNodes};
use oxc_ast::Trivias;
pub use scope::{Scope, ScopeTree};

pub struct Semantic<'a> {
    nodes: AstNodes<'a>,

    trivias: Rc<Trivias>,
}

impl<'a> Semantic<'a> {
    #[must_use]
    pub fn nodes(&self) -> &AstNodes<'a> {
        &self.nodes
    }

    #[must_use]
    pub fn trivias(&self) -> &Trivias {
        &self.trivias
    }
}
