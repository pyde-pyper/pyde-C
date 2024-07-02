use super::{
    compiler_errors::{CompilerError, SyntaxError},
    line_descriptors::{LineDescriptions, StringDescriptor},
    token::Token,
};

#[derive(PartialEq, Debug)]
pub enum VariantContext {
    Import,
    Interface,
    Library,
    Header,
    Contract,
    Error,
    None,
}

#[derive(PartialEq, Debug)]
pub enum TerminationTypeContext {
    None,
    Struct,
    Enum,
    Variable,
    Implementation,
    Function,
    Error,
}

pub trait ContextFn {
    fn validate_clash(&self, tokens: &Vec<Token>, lexems: &Option<&LineDescriptions<String>>);
}

impl ContextFn for VariantContext {
    fn validate_clash(&self, tokens: &Vec<Token>, lexems: &Option<&LineDescriptions<String>>) {
        if let Some(_lexems) = lexems {
            if *self != Self::None && !tokens.is_empty() {
                CompilerError::SyntaxError(SyntaxError::MissingToken(match self {
                    Self::Contract | Self::Interface | Self::Library => "}",
                    _ => ";",
                }))
                .throw_with_file_info("Contract.sol", _lexems.lex().line);
            }
        } else {
            CompilerError::InternalError("Unprocessible entity").throw();
        }
    }
}

impl ContextFn for TerminationTypeContext {
    fn validate_clash(&self, tokens: &Vec<Token>, lexems: &Option<&LineDescriptions<String>>) {
        if let Some(_lexems) = lexems {
            if *self != Self::None && !tokens.is_empty() {
                CompilerError::SyntaxError(SyntaxError::MissingToken(match self {
                    Self::Struct | Self::Enum | Self::Function => "}",
                    _ => ";",
                }))
                .throw_with_file_info("Contract.sol", _lexems.lex().line);
            }
        } else {
            CompilerError::InternalError("Unprocessible entity").throw();
        }
    }
}
