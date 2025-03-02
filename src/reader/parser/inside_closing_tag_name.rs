use crate::{namespace, common::is_whitespace_char};

use crate::reader::lexer::Token;

use super::{ClosingTagSubstate, PullParser, QualifiedNameTarget, Result, State};

impl PullParser {
    pub fn inside_closing_tag_name(&mut self, t: Token, s: ClosingTagSubstate) -> Option<Result> {
        match s {
            ClosingTagSubstate::CTInsideName => self.read_qualified_name(t, QualifiedNameTarget::ClosingTagNameTarget, |this, token, name| {
                match name.prefix_ref() {
                    Some(prefix) if prefix == namespace::NS_XML_PREFIX ||
                                    prefix == namespace::NS_XMLNS_PREFIX =>
                        // TODO: {:?} is bad, need something better
                        Some(self_error!(this; "'{:?}' cannot be an element name prefix", name.prefix)),
                    _ => {
                        this.data.element_name = Some(name.clone());
                        match token {
                            Token::TagEnd => this.emit_end_element(),
                            Token::Character(c) if is_whitespace_char(c) => this.into_state_continue(State::InsideClosingTag(ClosingTagSubstate::CTAfterName)),
                            _ => Some(self_error!(this; "Unexpected token inside closing tag: {}", token))
                        }
                    }
                }
            }),
            ClosingTagSubstate::CTAfterName => match t {
                Token::TagEnd => self.emit_end_element(),
                Token::Character(c) if is_whitespace_char(c) => None,  //  Skip whitespace
                _ => Some(self_error!(self; "Unexpected token inside closing tag: {}", t))
            }
        }
    }
}
