use range::Range;

use {
    Whitespace,
    Token,
    UntilAnyOrWhitespace,
    Text,
    Number,
    Parameter,
    MetaReader,
    ParseError,
    Select,
    Optional,
};

/// A rule describes how some section of a document should be parsed.
pub enum Rule {
    /// Read whitespace.
    Whitespace(Whitespace),
    /// Match against a token.
    Token(Token),
    /// Read until any or whitespace.
    UntilAnyOrWhitespace(UntilAnyOrWhitespace),
    /// Read text.
    Text(Text),
    /// Read number.
    Number(Number),
    /// Select one of the sub rules.
    /// If the first one does not succeed, try another and so on.
    /// If all sub rules fail, then the rule fails.
    Select(Select),
    /// Read parameter.
    Parameter(Parameter),
    /// Read optional.
    Optional(Optional),
}

impl Rule {
    /// Parses rule.
    pub fn parse<M>(
        &self,
        meta_reader: &mut M,
        state: &M::State,
        chars: &[char],
        offset: usize
    ) -> Result<(Range, M::State), (Range, ParseError)>
        where M: MetaReader
    {
        match self {
            &Rule::Whitespace(ref w) => {
                w.parse(chars, offset).map(|r| (r, state.clone()))
            }
            &Rule::Token(ref t) => {
                t.parse(meta_reader, state, chars, offset)
            }
            &Rule::UntilAnyOrWhitespace(ref u) => {
                u.parse(meta_reader, state, chars, offset)
            }
            &Rule::Text(ref t) => {
                t.parse(meta_reader, state, chars, offset)
            }
            &Rule::Number(ref n) => {
                n.parse(meta_reader, state, chars, offset)
            }
            &Rule::Select(ref s) => {
                s.parse(meta_reader, state, chars, offset)
            }
            &Rule::Parameter(ref p) => {
                p.parse(meta_reader, state, chars, offset)
            }
            &Rule::Optional(ref o) => {
                Ok(o.parse(meta_reader, state, chars, offset))
            }
        }
    }
}