// pest. The Elegant Parser
// Copyright (C) 2017  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[doc(hidden)]
#[macro_export]
macro_rules! consumes_to {
    ( $_rules:ident, $tokens:expr, [] ) => ();
    ( $rules:ident, $tokens:expr, [ $name:ident ( $start:expr, $end:expr ) ] ) => {
        let expected = format!("expected Start {{ rule: {:?}, pos: Position {{ pos: {} }} }}",
                               $rules::$name, $start);
        match $tokens.next().expect(&format!("{} but found nothing", expected)) {
            $crate::Token::Start { rule, pos } => {
                let message = format!("{} but found Start {{ rule: {:?}, pos: Position {{ {} }} }}",
                                      expected, rule, pos.pos());

                if rule != $rules::$name || pos.pos() != $start {
                    panic!("{}", message);
                }
            },
            token => panic!("{}", format!("{} but found {:?}", expected, token))
        };

        let expected = format!("expected End {{ rule: {:?}, pos: Position {{ pos: {} }} }}",
                               $rules::$name, $end);
        match $tokens.next().expect(&format!("{} but found nothing", expected)) {
            $crate::Token::End { rule, pos } => {
                let message = format!("{} but found End {{ rule: {:?}, pos: Position {{ {} }} }}",
                                      expected, rule, pos.pos());

                if rule != $rules::$name || pos.pos() != $end {
                    panic!("{}", message);
                }
            },
            token => panic!("{}", format!("{} but found {:?}", expected, token))
        };
    };
    ( $rules:ident, $tokens:expr, [ $name:ident ( $start:expr, $end:expr ),
                                    $( $names:ident $calls:tt ),* ] ) => {

        let expected = format!("expected Start {{ rule: {:?}, pos: Position {{ pos: {} }} }}",
                               $rules::$name, $start);
        match $tokens.next().expect(&format!("{} but found nothing", expected)) {
            $crate::Token::Start { rule, pos } => {
                let message = format!("{} but found Start {{ rule: {:?}, pos: Position {{ {} }} }}",
                                      expected, rule, pos.pos());

                if rule != $rules::$name || pos.pos() != $start {
                    panic!("{}", message);
                }
            },
            token => panic!("{}", format!("{} but found {:?}", expected, token))
        };

        let expected = format!("expected End {{ rule: {:?}, pos: Position {{ pos: {} }} }}",
                               $rules::$name, $end);
        match $tokens.next().expect(&format!("{} but found nothing", expected)) {
            $crate::Token::End { rule, pos } => {
                let message = format!("{} but found End {{ rule: {:?}, pos: Position {{ {} }} }}",
                                      expected, rule, pos.pos());

                if rule != $rules::$name || pos.pos() != $end {
                    panic!("{}", message);
                }
            },
            token => panic!("{}", format!("{} but found {:?}", expected, token))
        };

        consumes_to!($rules, $tokens, [ $( $names $calls ),* ]);
    };
    ( $rules:ident, $tokens:expr, [ $name:ident ( $start:expr, $end:expr,
                                                  [ $( $names:ident $calls:tt ),* ] ) ] ) => {
        let expected = format!("expected Start {{ rule: {:?}, pos: Position {{ pos: {} }} }}",
                               $rules::$name, $start);
        match $tokens.next().expect(&format!("{} but found nothing", expected)) {
            $crate::Token::Start { rule, pos } => {
                let message = format!("{} but found Start {{ rule: {:?}, pos: Position {{ {} }} }}",
                                      expected, rule, pos.pos());

                if rule != $rules::$name || pos.pos() != $start {
                    panic!("{}", message);
                }
            },
            token => panic!("{}", format!("{} but found {:?}", expected, token))
        };

        consumes_to!($rules, $tokens, [ $( $names $calls ),* ]);

        let expected = format!("expected End {{ rule: {:?}, pos: Position {{ pos: {} }} }}",
                               $rules::$name, $end);
        match $tokens.next().expect(&format!("{} but found nothing", expected)) {
            $crate::Token::End { rule, pos } => {
                let message = format!("{} but found End {{ rule: {:?}, pos: Position {{ {} }} }}",
                                      expected, rule, pos.pos());

                if rule != $rules::$name || pos.pos() != $end {
                    panic!("{}", message);
                }
            },
            token => panic!("{}", format!("{} but found {:?}", expected, token))
        };
    };
    ( $rules:ident, $tokens:expr, [ $name:ident ( $start:expr, $end:expr,
                                                  [ $( $nested_names:ident $nested_calls:tt ),* ] ),
                                    $( $names:ident $calls:tt ),* ] ) => {

        let expected = format!("expected Start {{ rule: {:?}, pos: Position {{ pos: {} }} }}",
                               $rules::$name, $start);
        match $tokens.next().expect(&format!("{} but found nothing", expected)) {
            $crate::Token::Start { rule, pos } => {
                let message = format!("{} but found Start {{ rule: {:?}, pos: Position {{ {} }} }}",
                                      expected, rule, pos.pos());

                if rule != $rules::$name || pos.pos() != $start {
                    panic!("{}", message);
                }
            },
            token => panic!("{}", format!("{} but found {:?}", expected, token))
        };

        consumes_to!($rules, $tokens, [ $( $nested_names $nested_calls ),* ]);

        let expected = format!("expected End {{ rule: {:?}, pos: Position {{ pos: {} }} }}",
                               $rules::$name, $end);
        match $tokens.next().expect(&format!("{} but found nothing", expected)) {
            $crate::Token::End { rule, pos } => {
                let message = format!("{} but found End {{ rule: {:?}, pos: Position {{ {} }} }}",
                                      expected, rule, pos.pos());

                if rule != $rules::$name || pos.pos() != $end {
                    panic!("{}", message);
                }
            },
            token => panic!("{}", format!("{} but found {:?}", expected, token))
        };

        consumes_to!($rules, $tokens, [ $( $names $calls ),* ]);
    };
}

/// A `macro` which facilitates grammar testing and debugging by comparing produced tokens.
///
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate pest;
/// # use std::rc::Rc;
/// # use pest::{Error, Parser};
/// # use pest::inputs::Input;
/// # use pest::iterators::Pairs;
/// # fn main() {
/// # #[allow(non_camel_case_types)]
/// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// # enum Rule {
/// #     a,
/// #     b,
/// #     c
/// # }
/// #
/// # struct AbcParser;
/// #
/// # impl Parser<Rule> for AbcParser {
/// #     fn parse<I: Input>(_: Rule, input: Rc<I>) -> Result<Pairs<Rule, I>, Error<Rule, I>> {
/// #         pest::state(input, |state, pos| {
/// #             state.rule(Rule::a, pos, |state, pos| {
/// #                 state.rule(Rule::b, pos.skip(1).unwrap(), |_, pos| {
/// #                     pos.skip(1)
/// #                 }).unwrap().skip(1)
/// #             }).and_then(|p| {
/// #                 state.rule(Rule::c, p.skip(1).unwrap(), |_, pos| {
/// #                     pos.skip(1)
/// #                 })
/// #             })
/// #         })
/// #     }
/// # }
/// parses_to! {
///     parser: AbcParser,
///     input:  "abcde",
///     rule:   Rule::a,
///     tokens: [
///         a(0, 3, [
///             b(1, 2)
///         ]),
///         c(4, 5)
///     ]
/// };
/// # }
/// ```
#[macro_export]
macro_rules! parses_to {
    ( parser: $parser:ident, input: $string:expr, rule: $rules:tt :: $rule:tt,
      tokens: [ $( $names:ident $calls:tt ),* ] ) => {

        #[allow(unused_mut)]
        {
            use $crate::Parser;

            let mut tokens = $parser::parse_str($rules::$rule, $string).unwrap().tokens();

            consumes_to!($rules, &mut tokens, [ $( $names $calls ),* ]);

            let rest: Vec<_> = tokens.collect();

            assert!(rest.is_empty(), format!("expected end of stream, but found {:?}", rest));
        }
    };
}

/// A `macro` which facilitates grammar testing and debugging by comparing produced errors.
///
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate pest;
/// # use std::rc::Rc;
/// # use pest::{Error, Parser};
/// # use pest::inputs::Input;
/// # use pest::iterators::Pairs;
/// # fn main() {
/// # #[allow(non_camel_case_types)]
/// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// # enum Rule {
/// #     a,
/// #     b,
/// #     c
/// # }
/// #
/// # struct AbcParser;
/// #
/// # impl Parser<Rule> for AbcParser {
/// #     fn parse<I: Input>(_: Rule, input: Rc<I>) -> Result<Pairs<Rule, I>, Error<Rule, I>> {
/// #         pest::state(input, |state, pos| {
/// #             state.rule(Rule::a, pos, |state, pos| {
/// #                 state.rule(Rule::b, pos.skip(1).unwrap(), |_, pos| {
/// #                     pos.skip(1)
/// #                 }).unwrap().skip(1)
/// #             }).and_then(|p| {
/// #                 state.rule(Rule::c, p.skip(1).unwrap(), |_, pos| {
/// #                     pos.match_string("e")
/// #                 })
/// #             })
/// #         })
/// #     }
/// # }
/// fails_with! {
///     parser: AbcParser,
///     input: "abcdf",
///     rule: Rule::a,
///     positives: vec![Rule::c],
///     negatives: vec![],
///     pos: 4
/// };
/// # }
/// ```
#[macro_export]
macro_rules! fails_with {
    ( parser: $parser:ident, input: $string:expr, rule: $rules:tt :: $rule:tt,
      positives: $positives:expr, negatives: $negatives:expr, pos: $pos:expr ) => {

        #[allow(unused_mut)]
        {
            use $crate::Parser;

            let error = $parser::parse_str($rules::$rule, $string).unwrap_err();

            match error {
                $crate::Error::ParsingError { positives, negatives, pos } => {
                    assert_eq!(positives, $positives);
                    assert_eq!(negatives, $negatives);
                    assert_eq!(pos.pos(), $pos);
                }
                _ => unreachable!()
            };
        }
    };
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::super::error::Error;
    use super::super::inputs::Input;
    use super::super::{Parser, state};
    use super::super::iterators::Pairs;

    #[allow(non_camel_case_types)]
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    enum Rule {
        a,
        b,
        c
    }

    struct AbcParser;

    impl Parser<Rule> for AbcParser {
        fn parse<I: Input>(_: Rule, input: Rc<I>) -> Result<Pairs<Rule, I>, Error<Rule, I>> {
            state(input, |state, pos| {
                state.rule(Rule::a, pos, |state, pos| {
                    state.rule(Rule::b, pos.skip(1).unwrap(), |_, pos| {
                        pos.skip(1)
                    }).unwrap().skip(1)
                }).and_then(|p| {
                    state.rule(Rule::c, p.skip(1).unwrap(), |_, pos| {
                        pos.match_string("e")
                    })
                })
            })
        }
    }

    #[test]
    fn parses_to() {
        parses_to! {
            parser: AbcParser,
            input: "abcde",
            rule: Rule::a,
            tokens: [
                a(0, 3, [
                    b(1, 2)
                ]),
                c(4, 5)
            ]
        };
    }

    #[test]
    #[should_panic]
    fn missing_end() {
        parses_to! {
            parser: AbcParser,
            input: "abcde",
            rule: Rule::a,
            tokens: [
                a(0, 3, [
                    b(1, 2)
                ])
            ]
        };
    }

    #[test]
    #[should_panic]
    fn empty() {
        parses_to! {
            parser: AbcParser,
            input: "abcde",
            rule: Rule::a,
            tokens: []
        };
    }

    #[test]
    fn fails_with() {
        fails_with! {
            parser: AbcParser,
            input: "abcdf",
            rule: Rule::a,
            positives: vec![Rule::c],
            negatives: vec![],
            pos: 4
        };
    }

    #[test]
    #[should_panic]
    fn wrong_positives() {
        fails_with! {
            parser: AbcParser,
            input: "abcdf",
            rule: Rule::a,
            positives: vec![Rule::a],
            negatives: vec![],
            pos: 4
        };
    }

    #[test]
    #[should_panic]
    fn wrong_negatives() {
        fails_with! {
            parser: AbcParser,
            input: "abcdf",
            rule: Rule::a,
            positives: vec![Rule::c],
            negatives: vec![Rule::c],
            pos: 4
        };
    }

    #[test]
    #[should_panic]
    fn wrong_pos() {
        fails_with! {
            parser: AbcParser,
            input: "abcdf",
            rule: Rule::a,
            positives: vec![Rule::c],
            negatives: vec![],
            pos: 3
        };
    }
}
