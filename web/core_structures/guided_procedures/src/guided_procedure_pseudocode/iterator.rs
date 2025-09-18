//! Submodule implementing the iterator for the `GuidedProcedureBuilder`.

use core_structures::ProcedureTemplate;

use crate::GuidedProcedurePseudocode;

impl<'graph> Iterator for GuidedProcedurePseudocode<'graph> {
    type Item = &'graph ProcedureTemplate;

    fn next(&mut self) -> Option<Self::Item> {
        match self.visitor.next()? {
            Ok(output) => {
                match output {
                    super::listener::GPPListenerOutput::NoOp => self.next(),
                    super::listener::GPPListenerOutput::Template(template) => Some(template),
                }
            }
            Err(_) => unreachable!("The PTGVisitor with GPPListener is infallible"),
        }
    }
}
