use crate::{core::ast::Name, syntax::parse::*};

pub fn parse_name_ident(i: ParseInput) -> ParseResult<Name> {
    map(parse_identifier, Name::Ident)(i)
}

pub fn parse_name_placeholder(i: ParseInput) -> ParseResult<Name> {
    map(tag(TokenKind::KeywordPlaceholderName), |_| {
        Name::Placeholder
    })(i)
}

pub fn parse_name(i: ParseInput) -> ParseResult<Name> {
    alt((parse_name_ident, parse_name_placeholder))(i)
}
