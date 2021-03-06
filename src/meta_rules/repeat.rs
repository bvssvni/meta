use range::Range;
use read_token::ReadToken;

use super::{
    ret_err,
    err_update,
    update,
    IndentSettings,
    ParseResult,
};
use {
    DebugId,
    MetaData,
    Rule,
};
use tokenizer::TokenizerState;

/// Stores inforamtion about separated by.
#[derive(Clone, Debug, PartialEq)]
pub struct Repeat {
    /// The rule to separate.
    pub rule: Rule,
    /// Whether the rule must occur at least once.
    pub optional: bool,
    /// A debug id to track down the rule generating an error.
    pub debug_id: DebugId,
}

impl Repeat {
    /// Parses rule repeatedly.
    pub fn parse(
        &self,
        tokens: &mut Vec<Range<MetaData>>,
        state: &TokenizerState,
        read_token: &ReadToken,
        refs: &[Rule],
        indent_settings: &mut IndentSettings,
    ) -> ParseResult<TokenizerState> {
        let start = read_token;
        let mut read_token = *start;
        let mut state = state.clone();
        let mut opt_error = None;
        let mut first = true;
        loop {
            state = match self.rule.parse(tokens, &state, &read_token, refs, indent_settings) {
                Err(err) => {
                    if first && !self.optional {
                        return Err(ret_err(err, opt_error));
                    } else {
                        err_update(Some(err), &mut opt_error);
                        break;
                    }
                }
                Ok((range, state, err)) => {
                    update(range, err, &mut read_token, &mut opt_error);
                    state
                }
            };
            first = false;
        }
        Ok((read_token.subtract(start), state, opt_error))
    }
}

#[cfg(test)]
mod tests {
    use all::*;
    use all::tokenizer::*;
    use meta_rules::{ IndentSettings, Repeat, Tag };
    use std::sync::Arc;
    use range::Range;
    use read_token::ReadToken;

    #[test]
    fn fail() {
        let ref mut indent_settings = IndentSettings::default();
        let text = "[a][a][a]";
        let mut tokens = vec![];
        let s = TokenizerState::new();
        let token: Arc<String> = Arc::new("(a)".into());
        let rule = Repeat {
            debug_id: 0,
            optional: false,
            rule: Rule::Tag(Tag {
                debug_id: 1,
                text: token.clone(),
                not: false,
                inverted: false,
                property: None,
            })
        };
        let res = rule.parse(&mut tokens, &s, &ReadToken::new(&text, 0), &[], indent_settings);
        assert_eq!(res, Err(Range::new(0, 0).wrap(
            ParseError::ExpectedTag(token.clone(), 1))))
    }

    #[test]
    fn success() {
        let ref mut indent_settings = IndentSettings::default();
        let text = "(a)(a)(a)";
        let mut tokens = vec![];
        let s = TokenizerState::new();
        let token: Arc<String> = Arc::new("(a)".into());
        let rule = Repeat {
            debug_id: 0,
            optional: false,
            rule: Rule::Tag(Tag {
                debug_id: 1,
                text: token.clone(),
                not: false,
                inverted: false,
                property: None,
            })
        };
        let res = rule.parse(&mut tokens, &s, &ReadToken::new(&text, 0), &[], indent_settings);
        assert_eq!(res, Ok((Range::new(0, 9), TokenizerState(0),
            Some(Range::new(9, 0).wrap(
                ParseError::ExpectedTag(token.clone(), 1))))))
    }
}
