//! Control flow analysis for Silk programs
//!
//! Performs control flow analysis to detect:
//! - Unreachable code after returns/breaks/continues
//! - Uninitialized variable usage
//! - Missing return statements in functions
//! - Infinite loops
//! - Dead code

use crate::SemanticError;
use silk_ast::Program;

/// Control flow analyzer for detecting control flow errors
pub struct ControlFlowAnalyzer {
    /// Errors collected during analysis
    errors: Vec<SemanticError>,
    /// Whether the current function has a return on all paths
    current_function_returns: bool,
    /// Whether we're currently inside a loop
    in_loop: bool,
}

impl ControlFlowAnalyzer {
    /// Create a new control flow analyzer
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            current_function_returns: false,
            in_loop: false,
        }
    }

    /// Analyze a program and return errors if any
    pub fn analyze(&mut self, _program: &Program) -> Result<(), Vec<SemanticError>> {
        // TODO: Implement program analysis in subsequent steps
        
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }

    /// Get a reference to the collected errors (for testing)
    pub fn errors(&self) -> &[SemanticError] {
        &self.errors
    }
}

impl Default for ControlFlowAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_analyzer_has_no_errors() {
        let analyzer = ControlFlowAnalyzer::new();
        assert!(analyzer.errors().is_empty());
        assert!(!analyzer.current_function_returns);
        assert!(!analyzer.in_loop);
    }

    #[test]
    fn test_default_creates_new_analyzer() {
        let analyzer = ControlFlowAnalyzer::default();
        assert!(analyzer.errors().is_empty());
    }

    #[test]
    fn test_analyze_empty_program_succeeds() {
        let mut analyzer = ControlFlowAnalyzer::new();
        let program = Program {
            body: Vec::new(),
        };

        let result = analyzer.analyze(&program);
        assert!(result.is_ok());
        assert!(analyzer.errors().is_empty());
    }

    #[test]
    fn test_analyzer_collects_errors() {
        let mut analyzer = ControlFlowAnalyzer::new();
        
        // Manually add an error to test error collection
        analyzer.errors.push(SemanticError::UnreachableCode {
            statement_type: "return".to_string(),
            line: 1,
            column: 1,
            span: silk_lexer::Span::new(0, 1, 1, 1),
        });

        let result = analyzer.analyze(&Program { body: Vec::new() });
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(matches!(errors[0], SemanticError::UnreachableCode { .. }));
    }
}
