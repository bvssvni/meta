use range::Range;

use {
    ret_err,
    update,
    ParseError,
    MetaReader,
    Rule,
};

/// Stores information about optional.
pub struct Optional {
    /// The optional rules.
    pub args: Vec<Rule>,
}

impl Optional {
    /// Parse optional.
    /// Returns the old state if any sub rule fails.
    pub fn parse<M>(
        &self,
        meta_reader: &mut M,
        state: &M::State,
        mut chars: &[char],
        mut offset: usize
    ) -> (Range, M::State, Option<(Range, ParseError)>)
        where M: MetaReader
    {
        let start_offset = offset;
        let mut success_state = state.clone();
        let mut opt_error = None;
        for sub_rule in &self.args {
            success_state = match sub_rule.parse(meta_reader, &success_state,
                                         chars, offset) {
                Ok((range, state, err)) => {
                    update(range, err, &mut chars, &mut offset, &mut opt_error);
                    state
                }
                Err(err) => {
                    return (Range::new(start_offset, 0), state.clone(),
                        Some(ret_err(err, opt_error)))
                }
            }
        }
        (Range::new(start_offset, offset - start_offset), success_state,
            opt_error)
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use range::Range;
    use std::rc::Rc;

    #[test]
    fn fail_but_continue() {
        let text = "2";
        let chars: Vec<char> = text.chars().collect();
        let mut tokenizer = Tokenizer::new();
        let s = TokenizerState::new();
        let num: Rc<String> = Rc::new("num".into());
        // Will fail because text is expected first.
        let optional = Optional {
            args: vec![
                Rule::Text(Text {
                    allow_empty: true,
                    property: None
                }),
                Rule::Number(Number {
                    property: Some(num.clone())
                })
            ]
        };
        let res = optional.parse(&mut tokenizer, &s, &chars, 0);
        assert_eq!(res, (Range::new(0, 0), TokenizerState(0),
            Some((Range::new(0, 0), ParseError::ExpectedText))));
        assert_eq!(tokenizer.tokens.len(), 0);
    }
}
