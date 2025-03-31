use swc_common::{SourceMap, SyntaxContext, sync::Lrc};
use swc_ecma_ast::*;
use swc_ecma_codegen::{Emitter, text_writer::JsWriter};
pub use swc_ecma_transforms_optimization::AssertValid;
use swc_ecma_utils::{DropSpan, drop_span};
use swc_ecma_visit::{VisitMut, VisitMutWith, noop_visit_mut_type};

pub(crate) struct Debugger {}

impl VisitMut for Debugger {
	noop_visit_mut_type!();

	fn visit_mut_ident(&mut self, n: &mut Ident) {
		if !cfg!(feature = "debug") {
			return;
		}

		if n.ctxt == SyntaxContext::empty() {
			return;
		}

		n.sym = format!("{}{:?}", n.sym, n.ctxt).into();
		n.ctxt = SyntaxContext::empty();
	}
}

pub(crate) fn dump<N>(node: &N, force: bool) -> String
where
	N: swc_ecma_codegen::Node + Clone + VisitMutWith<DropSpan> + VisitMutWith<Debugger>,
{
	if !force {
		#[cfg(not(feature = "debug"))]
		{
			return String::new();
		}
	}

	let mut node = node.clone();
	node.visit_mut_with(&mut Debugger {});
	node = drop_span(node);
	let mut buf = Vec::new();
	let cm = Lrc::new(SourceMap::default());

	{
		let mut emitter = Emitter {
			cfg: Default::default(),
			cm: cm.clone(),
			comments: None,
			wr: Box::new(JsWriter::new(cm, "\n", &mut buf, None)),
		};

		node.emit_with(&mut emitter).unwrap();
	}

	String::from_utf8(buf).unwrap()
}
