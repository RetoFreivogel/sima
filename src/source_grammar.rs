// auto-generated: "lalrpop 0.14.0"
use source_ast::*;
use std::cell::Cell;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Module {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use source_ast::*;
    use std::cell::Cell;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__intern_token::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2c_22(&'input str),
        Term_22_2d_3e_22(&'input str),
        Term_22_3a_3a_22(&'input str),
        Term_22_3b_22(&'input str),
        Term_22_3d_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Term_22drop_22(&'input str),
        Term_22dup_22(&'input str),
        Term_22id_22(&'input str),
        Term_22module_22(&'input str),
        Term_22opaque_22(&'input str),
        Term_22swap_22(&'input str),
        TermCOMMENT(&'input str),
        TermIDENTIFIER(&'input str),
        TermNUMBER(&'input str),
        TermSIMPLETYPE(&'input str),
        TermSTRINGLITERAL(&'input str),
        NtBasicType(SimaType),
        NtConcat(Expression),
        NtFunctionType(SimaType),
        NtModule(Module),
        NtPointerType(SimaType),
        NtPointerType_2a(::std::vec::Vec<SimaType>),
        NtPointerType_2b(::std::vec::Vec<SimaType>),
        NtSidecat(Expression),
        NtSymbol(Expression),
        Nt____Module(Module),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 6, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 15, 0,
        // State 6
        19, 0, 0, 0, 0, 0, 0, 0, 20, 0, 21, 22, 23, 0, 0, 24, 0, 25, 26, 0, 27,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0,
        // State 8
        0, 0, 28, 0, -14, 0, -14, 0, 0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0, -14, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, -17, 0, -9, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, 0, 0, -17, 0,
        // State 11
        0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 15, 0,
        // State 12
        0, 0, 0, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 15, 0,
        // State 13
        0, 0, -2, 0, -2, 0, -2, 0, 0, 0, 0, 0, 0, 0, -2, 0, 0, 0, 0, -2, 0,
        // State 14
        0, 0, -1, 0, -1, 0, -1, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, -1, 0,
        // State 15
        19, 0, 0, 0, 0, 0, 35, 0, 20, 0, 21, 22, 23, 0, 0, 24, 0, 25, 26, 0, 27,
        // State 16
        -4, -4, 0, 36, 0, 0, -4, 0, -4, -4, -4, -4, -4, 0, 0, -4, 0, -4, -4, 0, -4,
        // State 17
        -20, -20, 0, -20, 0, 0, -20, 0, -20, -20, -20, -20, -20, 0, 0, -20, 0, -20, -20, 0, -20,
        // State 18
        19, 0, 0, 0, 0, 0, 0, 0, 20, 0, 21, 22, 23, 0, 0, 24, 0, 25, 26, 0, 27,
        // State 19
        19, 0, 0, 0, 0, 0, 0, 0, 20, 0, 21, 22, 23, 0, 0, 24, 0, 25, 26, 0, 27,
        // State 20
        -24, -24, 0, -24, 0, 0, -24, 0, -24, -24, -24, -24, -24, 0, 0, -24, 0, -24, -24, 0, -24,
        // State 21
        -23, -23, 0, -23, 0, 0, -23, 0, -23, -23, -23, -23, -23, 0, 0, -23, 0, -23, -23, 0, -23,
        // State 22
        -26, -26, 0, -26, 0, 0, -26, 0, -26, -26, -26, -26, -26, 0, 0, -26, 0, -26, -26, 0, -26,
        // State 23
        -25, -25, 0, -25, 0, 0, -25, 0, -25, -25, -25, -25, -25, 0, 0, -25, 0, -25, -25, 0, -25,
        // State 24
        -29, -29, 0, -29, 0, 0, -29, 0, -29, -29, -29, -29, -29, 0, 0, -29, 0, -29, -29, 0, -29,
        // State 25
        -28, -28, 0, -28, 0, 0, -28, 0, -28, -28, -28, -28, -28, 0, 0, -28, 0, -28, -28, 0, -28,
        // State 26
        -27, -27, 0, -27, 0, 0, -27, 0, -27, -27, -27, -27, -27, 0, 0, -27, 0, -27, -27, 0, -27,
        // State 27
        0, 0, 0, 0, -13, 0, -13, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, 0, -13, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -11, 0, 0, 0,
        // State 29
        0, 0, 0, 0, -18, 0, -18, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0, -18, 0,
        // State 30
        0, 0, 0, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 15, 0,
        // State 31
        0, 0, 0, 0, 0, 0, -17, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, 0, 0, -17, 0,
        // State 32
        0, 0, 0, 0, 0, 0, -6, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 15, 0,
        // State 33
        -3, -3, 0, 36, 0, 0, -3, 0, -3, -3, -3, -3, -3, 0, 0, -3, 0, -3, -3, 0, -3,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0,
        // State 35
        19, 0, 0, 0, 0, 0, 0, 0, 20, 0, 21, 22, 23, 0, 0, 24, 0, 25, 26, 0, 27,
        // State 36
        19, 41, 0, 0, 0, 0, 0, 0, 20, 0, 21, 22, 23, 0, 0, 24, 0, 25, 26, 0, 27,
        // State 37
        19, 0, 0, 0, 0, 0, 0, 0, 20, 42, 21, 22, 23, 0, 0, 24, 0, 25, 26, 0, 27,
        // State 38
        0, 0, 0, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 15, 0,
        // State 39
        -19, -19, 0, -19, 0, 0, -19, 0, -19, -19, -19, -19, -19, 0, 0, -19, 0, -19, -19, 0, -19,
        // State 40
        -21, -21, 0, -21, 0, 0, -21, 0, -21, -21, -21, -21, -21, 0, 0, -21, 0, -21, -21, 0, -21,
        // State 41
        -22, -22, 0, -22, 0, 0, -22, 0, -22, -22, -22, -22, -22, 0, 0, -22, 0, -22, -22, 0, -22,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        // State 0
        0,
        // State 1
        -30,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        -12,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        -11,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        -10,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 2, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        9, 0, 10, 0, 11, 0, 12, 0, 0, 0,
        // State 6
        0, 16, 0, 0, 0, 0, 0, 17, 18, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        9, 0, 0, 0, 30, 0, 0, 0, 0, 0,
        // State 12
        9, 0, 0, 0, 32, 0, 33, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 34, 18, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 37, 0, 0, 0, 0, 0, 17, 18, 0,
        // State 19
        0, 38, 0, 0, 0, 0, 0, 17, 18, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        9, 0, 0, 0, 32, 0, 39, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        9, 0, 0, 0, 30, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 40, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 34, 18, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 34, 18, 0,
        // State 38
        9, 0, 0, 0, 30, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###"",""###,
            r###""->""###,
            r###""::""###,
            r###"";""###,
            r###""=""###,
            r###""[""###,
            r###""]""###,
            r###""drop""###,
            r###""dup""###,
            r###""id""###,
            r###""module""###,
            r###""opaque""###,
            r###""swap""###,
            r###"COMMENT"###,
            r###"IDENTIFIER"###,
            r###"NUMBER"###,
            r###"SIMPLETYPE"###,
            r###"STRINGLITERAL"###,
        ];
        __ACTION[(__state * 21)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    #[allow(dead_code)]
    pub fn parse_Module<
        'input,
    >(
        input: &'input str,
    ) -> Result<Module, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let __last_location = &mut Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            *__last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                Token(4, _) if true => 0,
                Token(5, _) if true => 1,
                Token(6, _) if true => 2,
                Token(7, _) if true => 3,
                Token(8, _) if true => 4,
                Token(9, _) if true => 5,
                Token(10, _) if true => 6,
                Token(11, _) if true => 7,
                Token(12, _) if true => 8,
                Token(13, _) if true => 9,
                Token(14, _) if true => 10,
                Token(15, _) if true => 11,
                Token(16, _) if true => 12,
                Token(17, _) if true => 13,
                Token(18, _) if true => 14,
                Token(19, _) if true => 15,
                Token(3, _) if true => 16,
                Token(2, _) if true => 17,
                Token(1, _) if true => 18,
                Token(20, _) if true => 19,
                Token(0, _) if true => 20,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 21 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            Token(4, __tok0) => __Symbol::Term_22_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            Token(5, __tok0) => __Symbol::Term_22_29_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            Token(6, __tok0) => __Symbol::Term_22_2a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            Token(7, __tok0) => __Symbol::Term_22_2c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            Token(8, __tok0) => __Symbol::Term_22_2d_3e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            Token(9, __tok0) => __Symbol::Term_22_3a_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            Token(10, __tok0) => __Symbol::Term_22_3b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            Token(11, __tok0) => __Symbol::Term_22_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            Token(12, __tok0) => __Symbol::Term_22_5b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            Token(13, __tok0) => __Symbol::Term_22_5d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            Token(14, __tok0) => __Symbol::Term_22drop_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            Token(15, __tok0) => __Symbol::Term_22dup_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            Token(16, __tok0) => __Symbol::Term_22id_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            Token(17, __tok0) => __Symbol::Term_22module_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            Token(18, __tok0) => __Symbol::Term_22opaque_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            Token(19, __tok0) => __Symbol::Term_22swap_22((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            Token(3, __tok0) => __Symbol::TermCOMMENT((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            Token(2, __tok0) => __Symbol::TermIDENTIFIER((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            Token(1, __tok0) => __Symbol::TermNUMBER((__tok0)),
                            _ => unreachable!(),
                        },
                        19 => match __lookahead.1 {
                            Token(20, __tok0) => __Symbol::TermSIMPLETYPE((__tok0)),
                            _ => unreachable!(),
                        },
                        20 => match __lookahead.1 {
                            Token(0, __tok0) => __Symbol::TermSTRINGLITERAL((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        if r.is_err() {
                            return r;
                        }
                        return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                    }
                } else {
                    let mut __err_lookahead = Some(__lookahead);
                    let mut __err_integer: Option<usize> = Some(__integer);
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let mut __err_lookahead = None;
                let mut __err_integer: Option<usize> = None;
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: __err_lookahead,
                    expected: __expected_tokens(__state),
                };
                return Err(__error)
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Module,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // BasicType = SIMPLETYPE => ActionFn(8);
                let __sym0 = __pop_TermSIMPLETYPE(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBasicType(__nt), __end));
                0
            }
            2 => {
                // BasicType = "opaque" => ActionFn(9);
                let __sym0 = __pop_Term_22opaque_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBasicType(__nt), __end));
                0
            }
            3 => {
                // Concat = Concat, Sidecat => ActionFn(10);
                let __sym1 = __pop_NtSidecat(__symbols);
                let __sym0 = __pop_NtConcat(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action10::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtConcat(__nt), __end));
                1
            }
            4 => {
                // Concat = Sidecat => ActionFn(11);
                let __sym0 = __pop_NtSidecat(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtConcat(__nt), __end));
                1
            }
            5 => {
                // FunctionType = "->" => ActionFn(27);
                let __sym0 = __pop_Term_22_2d_3e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFunctionType(__nt), __end));
                2
            }
            6 => {
                // FunctionType = "->", PointerType+ => ActionFn(28);
                let __sym1 = __pop_NtPointerType_2b(__symbols);
                let __sym0 = __pop_Term_22_2d_3e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action28::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtFunctionType(__nt), __end));
                2
            }
            7 => {
                // FunctionType = PointerType+, "->" => ActionFn(29);
                let __sym1 = __pop_Term_22_2d_3e_22(__symbols);
                let __sym0 = __pop_NtPointerType_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action29::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtFunctionType(__nt), __end));
                2
            }
            8 => {
                // FunctionType = PointerType+, "->", PointerType+ => ActionFn(30);
                let __sym2 = __pop_NtPointerType_2b(__symbols);
                let __sym1 = __pop_Term_22_2d_3e_22(__symbols);
                let __sym0 = __pop_NtPointerType_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action30::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFunctionType(__nt), __end));
                2
            }
            9 => {
                // FunctionType = PointerType => ActionFn(5);
                let __sym0 = __pop_NtPointerType(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFunctionType(__nt), __end));
                2
            }
            10 => {
                // Module = Module, IDENTIFIER, "=", Concat, ";" => ActionFn(1);
                let __sym4 = __pop_Term_22_3b_22(__symbols);
                let __sym3 = __pop_NtConcat(__symbols);
                let __sym2 = __pop_Term_22_3d_22(__symbols);
                let __sym1 = __pop_TermIDENTIFIER(__symbols);
                let __sym0 = __pop_NtModule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action1::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtModule(__nt), __end));
                3
            }
            11 => {
                // Module = Module, IDENTIFIER, "::", FunctionType, ";" => ActionFn(2);
                let __sym4 = __pop_Term_22_3b_22(__symbols);
                let __sym3 = __pop_NtFunctionType(__symbols);
                let __sym2 = __pop_Term_22_3a_3a_22(__symbols);
                let __sym1 = __pop_TermIDENTIFIER(__symbols);
                let __sym0 = __pop_NtModule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action2::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtModule(__nt), __end));
                3
            }
            12 => {
                // Module = "module", IDENTIFIER, ";" => ActionFn(3);
                let __sym2 = __pop_Term_22_3b_22(__symbols);
                let __sym1 = __pop_TermIDENTIFIER(__symbols);
                let __sym0 = __pop_Term_22module_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action3::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtModule(__nt), __end));
                3
            }
            13 => {
                // PointerType = BasicType, "*" => ActionFn(6);
                let __sym1 = __pop_Term_22_2a_22(__symbols);
                let __sym0 = __pop_NtBasicType(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action6::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtPointerType(__nt), __end));
                4
            }
            14 => {
                // PointerType = BasicType => ActionFn(7);
                let __sym0 = __pop_NtBasicType(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtPointerType(__nt), __end));
                4
            }
            15 => {
                // PointerType* =  => ActionFn(23);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action23::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtPointerType_2a(__nt), __end));
                5
            }
            16 => {
                // PointerType* = PointerType+ => ActionFn(24);
                let __sym0 = __pop_NtPointerType_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtPointerType_2a(__nt), __end));
                5
            }
            17 => {
                // PointerType+ = PointerType => ActionFn(25);
                let __sym0 = __pop_NtPointerType(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtPointerType_2b(__nt), __end));
                6
            }
            18 => {
                // PointerType+ = PointerType+, PointerType => ActionFn(26);
                let __sym1 = __pop_NtPointerType(__symbols);
                let __sym0 = __pop_NtPointerType_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action26::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtPointerType_2b(__nt), __end));
                6
            }
            19 => {
                // Sidecat = Sidecat, ",", Symbol => ActionFn(12);
                let __sym2 = __pop_NtSymbol(__symbols);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtSidecat(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action12::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtSidecat(__nt), __end));
                7
            }
            20 => {
                // Sidecat = Symbol => ActionFn(13);
                let __sym0 = __pop_NtSymbol(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSidecat(__nt), __end));
                7
            }
            21 => {
                // Symbol = "(", Concat, ")" => ActionFn(14);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtConcat(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action14::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtSymbol(__nt), __end));
                8
            }
            22 => {
                // Symbol = "[", Concat, "]" => ActionFn(15);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtConcat(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action15::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtSymbol(__nt), __end));
                8
            }
            23 => {
                // Symbol = "dup" => ActionFn(16);
                let __sym0 = __pop_Term_22dup_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSymbol(__nt), __end));
                8
            }
            24 => {
                // Symbol = "drop" => ActionFn(17);
                let __sym0 = __pop_Term_22drop_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSymbol(__nt), __end));
                8
            }
            25 => {
                // Symbol = "swap" => ActionFn(18);
                let __sym0 = __pop_Term_22swap_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSymbol(__nt), __end));
                8
            }
            26 => {
                // Symbol = "id" => ActionFn(19);
                let __sym0 = __pop_Term_22id_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSymbol(__nt), __end));
                8
            }
            27 => {
                // Symbol = STRINGLITERAL => ActionFn(20);
                let __sym0 = __pop_TermSTRINGLITERAL(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSymbol(__nt), __end));
                8
            }
            28 => {
                // Symbol = NUMBER => ActionFn(21);
                let __sym0 = __pop_TermNUMBER(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSymbol(__nt), __end));
                8
            }
            29 => {
                // Symbol = IDENTIFIER => ActionFn(22);
                let __sym0 = __pop_TermIDENTIFIER(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtSymbol(__nt), __end));
                8
            }
            30 => {
                // __Module = Module => ActionFn(0);
                let __sym0 = __pop_NtModule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 10 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22drop_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22drop_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22dup_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22dup_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22id_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22id_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22module_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22module_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22opaque_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22opaque_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22swap_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22swap_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermCOMMENT<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermCOMMENT(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermIDENTIFIER<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermIDENTIFIER(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermNUMBER<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermNUMBER(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermSIMPLETYPE<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermSIMPLETYPE(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermSTRINGLITERAL<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermSTRINGLITERAL(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBasicType<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, SimaType, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBasicType(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtConcat<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expression, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtConcat(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFunctionType<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, SimaType, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFunctionType(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtModule<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Module, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtModule(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPointerType<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, SimaType, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPointerType(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPointerType_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<SimaType>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPointerType_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPointerType_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<SimaType>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPointerType_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSidecat<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expression, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSidecat(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtSymbol<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expression, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtSymbol(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Module<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Module, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Module(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Module::parse_Module;
mod __intern_token {
    #![allow(unused_imports)]
    use source_ast::*;
    use std::cell::Cell;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    extern crate regex as __regex;
    use std::fmt as __fmt;

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Token<'input>(pub usize, pub &'input str);
    impl<'a> __fmt::Display for Token<'a> {
        fn fmt(&self, formatter: &mut __fmt::Formatter) -> Result<(), __fmt::Error> {
            __fmt::Display::fmt(self.1, formatter)
        }
    }

    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            let __strs: &[&str] = &[
                "^(?u:\")(?u:[\u{0}-!\\#-\u{10ffff}])*(?u:\")",
                "^(?u:\\-)?(?u:[0-9])(?u:[0-9])*",
                "^(?u:[A-Za-z])(?u:[0-9A-Za-z])*",
                "^(?u:\\\\\\\\)(?u:.)*(?u:\n)",
                "^(?u:\\()",
                "^(?u:\\))",
                "^(?u:\\*)",
                "^(?u:,)",
                "^(?u:\\->)",
                "^(?u:::)",
                "^(?u:;)",
                "^(?u:=)",
                "^(?u:\\[)",
                "^(?u:\\])",
                "^(?u:drop)",
                "^(?u:dup)",
                "^(?u:id)",
                "^(?u:module)",
                "^(?u:opaque)",
                "^(?u:swap)",
                "^(?u:[f-fi-iu-u])(?u:[1-9])(?u:[0-9])*",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(?u:\")(?u:[\u{0}-!\\#-\u{10ffff}])*(?u:\")").unwrap(),
                __regex::Regex::new("^(?u:\\-)?(?u:[0-9])(?u:[0-9])*").unwrap(),
                __regex::Regex::new("^(?u:[A-Za-z])(?u:[0-9A-Za-z])*").unwrap(),
                __regex::Regex::new("^(?u:\\\\\\\\)(?u:.)*(?u:\n)").unwrap(),
                __regex::Regex::new("^(?u:\\()").unwrap(),
                __regex::Regex::new("^(?u:\\))").unwrap(),
                __regex::Regex::new("^(?u:\\*)").unwrap(),
                __regex::Regex::new("^(?u:,)").unwrap(),
                __regex::Regex::new("^(?u:\\->)").unwrap(),
                __regex::Regex::new("^(?u:::)").unwrap(),
                __regex::Regex::new("^(?u:;)").unwrap(),
                __regex::Regex::new("^(?u:=)").unwrap(),
                __regex::Regex::new("^(?u:\\[)").unwrap(),
                __regex::Regex::new("^(?u:\\])").unwrap(),
                __regex::Regex::new("^(?u:drop)").unwrap(),
                __regex::Regex::new("^(?u:dup)").unwrap(),
                __regex::Regex::new("^(?u:id)").unwrap(),
                __regex::Regex::new("^(?u:module)").unwrap(),
                __regex::Regex::new("^(?u:opaque)").unwrap(),
                __regex::Regex::new("^(?u:swap)").unwrap(),
                __regex::Regex::new("^(?u:[f-fi-iu-u])(?u:[1-9])(?u:[0-9])*").unwrap(),
            ];
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: __regex_set,
                regex_vec: __regex_vec,
            }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, Token<'input>, usize), __lalrpop_util::ParseError<usize,Token<'input>,&'static str>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                let __matches = self.regex_set.matches(__text);
                if !__matches.matched_any() {
                    Some(Err(__lalrpop_util::ParseError::InvalidToken {
                        location: __start_offset,
                    }))
                } else {
                    let mut __longest_match = 0;
                    let mut __index = 0;
                    for __i in 0 .. 21 {
                        if __matches.matched(__i) {
                            let __match = self.regex_vec[__i].find(__text).unwrap();
                            let __len = __match.end();
                            if __len >= __longest_match {
                                __longest_match = __len;
                                __index = __i;
                            }
                        }
                    }
                    let __result = &__text[..__longest_match];
                    let __remaining = &__text[__longest_match..];
                    let __end_offset = __start_offset + __longest_match;
                    self.text = __remaining;
                    self.consumed = __end_offset;
                    Some(Ok((__start_offset, Token(__index, __result), __end_offset)))
                }
            }
        }
    }
}
pub use self::__intern_token::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Module, usize),
) -> Module
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, m, _): (usize, Module, usize),
    (_, id, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, expr, _): (usize, Expression, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Module
{
    {
        let mut m = m;
        m.add_definition(id, expr);
        m
    }
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, m, _): (usize, Module, usize),
    (_, id, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, sima_type, _): (usize, SimaType, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Module
{
    {
        let mut m = m; 
        m.add_declaration(id, sima_type);
        m
    }
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, id, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Module
{
    Module::new(id)
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, args, _): (usize, ::std::vec::Vec<SimaType>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, ret, _): (usize, ::std::vec::Vec<SimaType>, usize),
) -> SimaType
{
    SimaType::Function{args, ret}
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, SimaType, usize),
) -> SimaType
{
    (__0)
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, target, _): (usize, SimaType, usize),
    (_, _, _): (usize, &'input str, usize),
) -> SimaType
{
    SimaType::Pointer{target: Box::new(target)}
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, SimaType, usize),
) -> SimaType
{
    (__0)
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, id, _): (usize, &'input str, usize),
) -> SimaType
{
    SimaType::Basic{id: id.to_string()}
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> SimaType
{
    SimaType::Opaque
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, left, _): (usize, Expression, usize),
    (_, right, _): (usize, Expression, usize),
) -> Expression
{
    Expression::Concat{left: Box::new(left), right: Box::new(right)}
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    (__0)
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, left, _): (usize, Expression, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, right, _): (usize, Expression, usize),
) -> Expression
{
    Expression::Sidecat{left: Box::new(left), right: Box::new(right)}
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    (__0)
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Expression, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expression
{
    (__0)
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, inner, _): (usize, Expression, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::Block{inner: Box::new(inner)}
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::Duplicate
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::Discard
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::Exchange
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::Keep
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    input: &'input str,
    (_, lit, _): (usize, &'input str, usize),
) -> Expression
{
    {
        Expression::StringLiteral(lit[1..lit.len()-1].to_string())
    }
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::Number(__0.to_string())
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    input: &'input str,
    (_, id, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::Identifier{
        id: id.to_string(),
        in_ariety: Cell::new(0),
        out_ariety: Cell::new(0),
    }
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<SimaType>
{
    vec![]
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<SimaType>, usize),
) -> ::std::vec::Vec<SimaType>
{
    v
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, SimaType, usize),
) -> ::std::vec::Vec<SimaType>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<SimaType>, usize),
    (_, e, _): (usize, SimaType, usize),
) -> ::std::vec::Vec<SimaType>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> SimaType
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __start1 = __0.2.clone();
    let __end1 = __0.2.clone();
    let __temp0 = __action23(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action23(
        input,
        &__start1,
        &__end1,
    );
    let __temp1 = (__start1, __temp1, __end1);
    __action4(
        input,
        __temp0,
        __0,
        __temp1,
    )
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, ::std::vec::Vec<SimaType>, usize),
) -> SimaType
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __start1 = __1.0.clone();
    let __end1 = __1.2.clone();
    let __temp0 = __action23(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action24(
        input,
        __1,
    );
    let __temp1 = (__start1, __temp1, __end1);
    __action4(
        input,
        __temp0,
        __0,
        __temp1,
    )
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<SimaType>, usize),
    __1: (usize, &'input str, usize),
) -> SimaType
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __start1 = __1.2.clone();
    let __end1 = __1.2.clone();
    let __temp0 = __action24(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action23(
        input,
        &__start1,
        &__end1,
    );
    let __temp1 = (__start1, __temp1, __end1);
    __action4(
        input,
        __temp0,
        __1,
        __temp1,
    )
}

#[allow(unused_variables)]
fn __action30<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<SimaType>, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, ::std::vec::Vec<SimaType>, usize),
) -> SimaType
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __start1 = __2.0.clone();
    let __end1 = __2.2.clone();
    let __temp0 = __action24(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action24(
        input,
        __2,
    );
    let __temp1 = (__start1, __temp1, __end1);
    __action4(
        input,
        __temp0,
        __1,
        __temp1,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, Token<'input>, usize) {
    type Error = &'static str;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),&'static str> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, Token<'input>, usize),&'static str> {
    type Error = &'static str;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),&'static str> {
        value
    }
}
