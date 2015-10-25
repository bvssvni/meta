use range::Range;
use read_token::ReadToken;

use super::{
    ret_err,
    err_update,
    ParseResult,
};
use {
    DebugId,
    MetaData,
    ParseError,
    Rule,
};
use tokenizer::TokenizerState;

/// Stores information about lines.
#[derive(Clone, Debug, PartialEq)]
pub struct Lines {
    /// The rule to read lines.
    /// This can be a multi-line rule.
    pub rule: Rule,
    /// A debug id to track down the rule generating an error.
    pub debug_id: DebugId,
}

impl Lines {
    /// Parses rule separated by one or more lines.
    /// Ignores lines that only contain whitespace characters.
    pub fn parse(
        &self,
        tokenizer: &mut Vec<Range<MetaData>>,
        state: &TokenizerState,
        read_token: &ReadToken,
        refs: &[Rule]
    ) -> ParseResult<TokenizerState> {
        let mut state = state.clone();
        let mut opt_error = None;
        match read_token.lines(|read_token| {
            match self.rule.parse(tokenizer, &state, read_token, refs) {
                Err(err) => {
                    err_update(Some(err), &mut opt_error);
                    None
                }
                Ok((range, new_state, err)) => {
                    err_update(err, &mut opt_error);
                    state = new_state;
                    Some(range)
                }
            }
        }) {
            Err(range) => {
                let err = range.wrap(
                    ParseError::ExpectedNewLine(self.debug_id));
                Err(ret_err(err, opt_error))
            }
            Ok(range) => {
                Ok((range, state, opt_error))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use all::*;
    use all::tokenizer::*;
    use meta_rules::{ Lines, Number, Sequence, Text, Whitespace };
    use range::Range;
    use read_token::ReadToken;
    use std::sync::Arc;

    #[test]
    fn fail() {
        let text = "
1
2

3


\"error\"
4
        ";
        let mut tokenizer = vec![];
        let s = TokenizerState::new();
        let lines = Lines {
            debug_id: 0,
            rule: Rule::Number(Number {
                debug_id: 1,
                property: None,
                allow_underscore: false,
            }),
        };
        let res = lines.parse(&mut tokenizer, &s,
            &ReadToken::new(&text, 0), &[]);
        assert_eq!(res, Ok((Range::new(0, 10), s,
            Some(Range::new(10, 0).wrap(ParseError::ExpectedNumber(1))))));
    }

    #[test]
    fn fails_same_line() {
        let text = "
1
2

3 4

5
 ";
        let mut tokenizer = vec![];
        let s = TokenizerState::new();
        let val: Arc<String> = Arc::new("val".into());
        let lines = Lines {
            debug_id: 0,
            rule: Rule::Sequence(Sequence {
                debug_id: 1,
                args: vec![
                    Rule::Number(Number {
                        debug_id: 1,
                        property: Some(val.clone()),
                        allow_underscore: false,
                    }),
                    Rule::Whitespace(Whitespace {
                        debug_id: 2,
                        optional: true,
                    })
                ]
            }),
        };
        let res = lines.parse(&mut tokenizer, &s,
            &ReadToken::new(&text, 0), &[]);
        assert_eq!(res, Err(Range::new(8, 0).wrap(
            ParseError::ExpectedNewLine(0))));
    }

    #[test]
    fn success() {
        let text = "
1
2

3


4
 ";
        let mut tokenizer = vec![];
        let s = TokenizerState::new();
        let val: Arc<String> = Arc::new("val".into());
        let lines = Lines {
            debug_id: 0,
            rule: Rule::Number(Number {
                debug_id: 1,
                property: Some(val.clone()),
                allow_underscore: false,
            }),
        };
        let res = lines.parse(&mut tokenizer, &s,
            &ReadToken::new(&text, 0), &[]);
        assert_eq!(res, Ok((Range::new(0, 13), TokenizerState(4), None)));
    }

    #[test]
    fn sequence() {
        let text = "
1
2
3
\"one\"
\"two\"
\"three\"
        ";
        let num: Arc<String> = Arc::new("num".into());
        let tex: Arc<String> = Arc::new("tex".into());
        let rule = Rule::Sequence(Sequence {
            debug_id: 0,
            args: vec![
                Rule::Lines(Box::new(Lines {
                    debug_id: 1,
                    rule: Rule::Number(Number {
                        debug_id: 2,
                        allow_underscore: true,
                        property: Some(num.clone()),
                    })
                })),
                Rule::Lines(Box::new(Lines {
                    debug_id: 3,
                    rule: Rule::Text(Text {
                        debug_id: 4,
                        allow_empty: false,
                        property: Some(tex.clone()),
                    })
                }))
            ]
        });

        let mut syntax = Syntax::new();
        syntax.push(Arc::new("".into()), rule);
        let mut res = vec![];
        assert_eq!(parse(&syntax, text, &mut res), Ok(()));
        assert_eq!(res, vec![
            Range::new(1, 1).wrap(MetaData::F64(num.clone(), 1.0)),
            Range::new(3, 1).wrap(MetaData::F64(num.clone(), 2.0)),
            Range::new(5, 1).wrap(MetaData::F64(num.clone(), 3.0)),
            Range::new(7, 5).wrap(
                MetaData::String(tex.clone(), Arc::new("one".into()))),
            Range::new(13, 5).wrap(
                MetaData::String(tex.clone(), Arc::new("two".into()))),
            Range::new(19, 7).wrap(
                MetaData::String(tex.clone(), Arc::new("three".into())))
        ]);
    }
}
