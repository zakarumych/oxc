use std::{rc::Rc, sync::Arc};

use miette::NamedSource;
use oxc_allocator::Allocator;
use oxc_ast::SourceType;
use oxc_diagnostics::Diagnostics;
use oxc_linter::Linter;
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use wasm_bindgen::JsValue;

pub struct Driver {
    allocator: Allocator,
}

impl Driver {
    pub fn new() -> Self {
        Self { allocator: Allocator::default() }
    }

    #[allow(deprecated)]
    pub fn run(
        &self,
        path: &str,
        source_text: &str,
        source_type: SourceType,
        eslintrc: &str,
    ) -> JsValue {
        let ret = Parser::new(&self.allocator, source_text, source_type)
            .allow_return_outside_function(true)
            .parse();

        let program = self.allocator.alloc(ret.program);

        let diagnostics = Diagnostics::default();
        let semantic = SemanticBuilder::new(source_type).build(program, Rc::new(ret.trivias));
        let mut diagnostics = diagnostics.into_inner();

        let source = Arc::new(NamedSource::new(path, source_text.to_string()));

        diagnostics.extend(
            Linter::from_json_str(eslintrc)
                .run(&Rc::new(semantic), source_text, false)
                .into_iter()
                .map(|m| m.error.with_source_code(source.clone()))
                .chain(ret.errors),
        );

        if diagnostics.is_empty() {
            if let Ok(ast) = JsValue::from_serde(program) {
                return ast;
            }
        }

        let result = diagnostics
            .into_iter()
            .map(|error| format!("{error:?}"))
            .collect::<Vec<String>>()
            .join("\n");
        JsValue::from_str(&result)
    }
}
