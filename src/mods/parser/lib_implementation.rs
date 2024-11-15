use crate::mods::ast::lib_implementation::{LibImplState, LibraryImplementation};
use crate::mods::constants::constants::FILE_PATH;
use crate::mods::errors::error::{CompilerError, SyntaxError};
use crate::mods::lexer::{lexer::TTokenTrait, tokens::Token};
use crate::mods::utils::functions::global::{get_env_vars, validate_identifier};
use crate::mods::utils::types::line_descriptors::LineDescriptions;

pub fn parse_lib_implementations(
    lexems: Vec<Vec<LineDescriptions<Vec<Token>>>>,
) -> Vec<LibraryImplementation> {
    let mut lib_implementations = Vec::new();

    for lexem in lexems {
        /* SANITY CHECKS */
        {
            if lexem.is_empty() {
                continue;
            }

            if lexem.first().unwrap().data.is_empty() {
                continue;
            }
            let first_element = lexem.first().unwrap().data.first().unwrap();

            if *first_element != Token::Using {
                CompilerError::InternalError(&format!(
                    "Expecting using but found {}",
                    first_element.to_string()
                ))
                .throw_with_file_info(
                    &get_env_vars(FILE_PATH).unwrap(),
                    lexem.first().unwrap().line,
                )
            }
        }

        let mut state = LibImplState::None;
        let mut lib_identifier = String::new();
        let mut data_type = String::new();
        let mut is_array = false;
        let mut array_size: Option<String> = None;
        let mut pad = 0;
        for (parent_index, lex) in lexem.iter().enumerate() {
            for (index, token) in lex.data.iter().enumerate() {
                if pad > index {
                    continue;
                }
                match token {
                    Token::Using => {
                        if let LibImplState::None = state {
                            state = LibImplState::Declaration;
                        } else {
                            CompilerError::SyntaxError(SyntaxError::UnexpectedToken(
                                &token.to_string(),
                            ))
                            .throw_with_file_info(&get_env_vars(FILE_PATH).unwrap(), lex.line)
                        }
                    }

                    Token::Space => {}

                    Token::For => {
                        if let LibImplState::LibDefinition = state {
                            state = LibImplState::For;
                        } else {
                            CompilerError::SyntaxError(SyntaxError::UnexpectedToken(
                                &token.to_string(),
                            ))
                            .throw_with_file_info(&get_env_vars(FILE_PATH).unwrap(), lex.line)
                        }
                    }

                    Token::Uint(_)
                    | Token::Int(_)
                    | Token::Bool
                    | Token::Bytes(_)
                    | Token::Address
                    | Token::String => {
                        if let LibImplState::For = state {
                            data_type.push_str(&token.to_string());
                            state = LibImplState::DataType;
                        } else {
                            CompilerError::SyntaxError(SyntaxError::UnexpectedToken(
                                &token.to_string(),
                            ))
                            .throw_with_file_info(&get_env_vars(FILE_PATH).unwrap(), lex.line);
                        }
                    }

                    Token::Identifier(_identifier) => {
                        if let LibImplState::Declaration = state {
                            validate_identifier(&_identifier).unwrap_or_else(|err| {
                                CompilerError::SyntaxError(SyntaxError::SyntaxError(&err))
                                    .throw_with_file_info(
                                        &get_env_vars(FILE_PATH).unwrap(),
                                        lex.line,
                                    )
                            });
                            lib_identifier.push_str(_identifier);
                            state = LibImplState::LibDefinition;
                        } else if let LibImplState::For = state {
                            validate_identifier(&_identifier).unwrap_or_else(|err| {
                                CompilerError::SyntaxError(SyntaxError::SyntaxError(&err))
                                    .throw_with_file_info(
                                        &get_env_vars(FILE_PATH).unwrap(),
                                        lex.line,
                                    )
                            });
                            data_type.push_str(_identifier);
                            state = LibImplState::DataType;
                        } else {
                            CompilerError::SyntaxError(SyntaxError::UnexpectedToken(
                                &token.to_string(),
                            ))
                            .throw_with_file_info(&get_env_vars(FILE_PATH).unwrap(), lex.line);
                        }
                    }

                    Token::OpenSquareBracket => {
                        if let LibImplState::DataType = state {
                            is_array = true;
                            let close_index = &lex.data[index + 1..]
                                .iter()
                                .position(|pred| *pred == Token::CloseSquareBracket);
                            if let Some(_close_index) = close_index {
                                let slice = &lex.data[index + 1..][..*_close_index];

                                if !slice.is_empty() {
                                    let mut stringified_array_size = String::new();
                                    pad = index + 1 + _close_index + 1;
                                    for slc in slice {
                                        stringified_array_size.push_str(&slc.to_string());
                                    }
                                    array_size = Some(stringified_array_size);
                                } else {
                                    pad = index + 2;
                                }
                            } else {
                                CompilerError::SyntaxError(SyntaxError::MissingToken("]"))
                                    .throw_with_file_info(
                                        &get_env_vars(FILE_PATH).unwrap(),
                                        lex.line,
                                    );
                            }
                        } else {
                            CompilerError::SyntaxError(SyntaxError::UnexpectedToken(
                                &token.to_string(),
                            ))
                            .throw_with_file_info(&get_env_vars(FILE_PATH).unwrap(), lex.line)
                        }
                    }

                    Token::SemiColon => {
                        if parent_index != lexem.len() - 1 || index != lex.data.len() - 1 {
                            CompilerError::SyntaxError(SyntaxError::UnexpectedToken(
                                &token.to_string(),
                            ))
                            .throw_with_file_info(&get_env_vars(FILE_PATH).unwrap(), lex.line)
                        }
                    }

                    _other => {
                        CompilerError::SyntaxError(SyntaxError::UnexpectedToken(
                            &_other.to_string(),
                        ))
                        .throw_with_file_info(&get_env_vars(FILE_PATH).unwrap(), lex.line);
                    }
                }
            }
        }

        let impl_construct = LibraryImplementation {
            array_size,
            line: lexem[0].line.to_string(),
            data_type,
            is_array,
            library_identifier: lib_identifier,
        };
        lib_implementations.push(impl_construct);
    }

    lib_implementations
}
