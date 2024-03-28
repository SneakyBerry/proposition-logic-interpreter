use proposition_logic_interpreter::prop_logic::LexPropParser;
use std::collections::HashMap;
use proposition_logic_interpreter::eval;

#[test]
fn demorgan1() {
    const EXPR_A: &str = "¬(p ∨ q)";
    const EXPR_B: &str = "!p & !q";

    let parser = LexPropParser::new();
    let prop_a = parser.parse(EXPR_A).unwrap();
    let prop_b = parser.parse(EXPR_B).unwrap();

    for p in [true, false] {
        for q in [true, false] {
            let mut ctx = HashMap::new();
            ctx.insert("p", p);
            ctx.insert("q", q);

            let result_a = eval(&ctx, &prop_a).unwrap();
            let result_b = eval(&ctx, &prop_b).unwrap();
            assert_eq!(result_a, result_b)
        }
    }
}

#[test]
fn demorgan2() {
    const EXPR_A: &str = "¬(p ∧ q)";
    const EXPR_B: &str = "!p | !q";

    let parser = LexPropParser::new();
    let prop_a = parser.parse(EXPR_A).unwrap();
    let prop_b = parser.parse(EXPR_B).unwrap();

    for p in [true, false] {
        for q in [true, false] {
            let mut ctx = HashMap::new();
            ctx.insert("p", p);
            ctx.insert("q", q);

            let result_a = eval(&ctx, &prop_a).unwrap();
            let result_b = eval(&ctx, &prop_b).unwrap();
            assert_eq!(result_a, result_b)
        }
    }
}

#[test]
fn pogloshenie1() {
    const EXPR_A: &str = "p ∨ p ∧ q";
    const EXPR_B: &str = "p";

    let parser = LexPropParser::new();
    let prop_a = parser.parse(EXPR_A).unwrap();
    let prop_b = parser.parse(EXPR_B).unwrap();

    for p in [true, false] {
        for q in [true, false] {
            let mut ctx = HashMap::new();
            ctx.insert("p", p);
            ctx.insert("q", q);

            let result_a = eval(&ctx, &prop_a).unwrap();
            let result_b = eval(&ctx, &prop_b).unwrap();
            assert_eq!(result_a, result_b)
        }
    }
}

#[test]
fn pogloshenie2() {
    const EXPR_A: &str = "p ∧ (p ∨ q)";
    const EXPR_B: &str = "p";

    let parser = LexPropParser::new();
    let prop_a = parser.parse(EXPR_A).unwrap();
    let prop_b = parser.parse(EXPR_B).unwrap();

    for p in [true, false] {
        for q in [true, false] {
            let mut ctx = HashMap::new();
            ctx.insert("p", p);
            ctx.insert("q", q);

            let result_a = eval(&ctx, &prop_a).unwrap();
            let result_b = eval(&ctx, &prop_b).unwrap();
            assert_eq!(result_b, result_a)
        }
    }
}

#[test]
fn distrib1() {
    const EXPR_A: &str = "p ∧ (q ∨ r)";
    const EXPR_B: &str = "p ∧ q ∨ p ∧ r";

    let parser = LexPropParser::new();
    let prop_a = parser.parse(EXPR_A).unwrap();
    let prop_b = parser.parse(EXPR_B).unwrap();

    for p in [true, false] {
        for q in [true, false] {
            for r in [true, false] {
                let mut ctx = HashMap::new();
                ctx.insert("p", p);
                ctx.insert("q", q);
                ctx.insert("r", r);

                let result_a = eval(&ctx, &prop_a).unwrap();
                let result_b = eval(&ctx, &prop_b).unwrap();
                assert_eq!(result_a, result_b)
            }
        }
    }
}

#[test]
fn distrib2() {
    const EXPR_A: &str = "p ∨ q ∧ r";
    const EXPR_B: &str = "(p ∨ q) ∧ (p ∨ r)";

    let parser = LexPropParser::new();
    let prop_a = parser.parse(EXPR_A).unwrap();
    let prop_b = parser.parse(EXPR_B).unwrap();

    for p in [true, false] {
        for q in [true, false] {
            for r in [true, false] {
                let mut ctx = HashMap::new();
                ctx.insert("p", p);
                ctx.insert("q", q);
                ctx.insert("r", r);

                let result_a = eval(&ctx, &prop_a).unwrap();
                let result_b = eval(&ctx, &prop_b).unwrap();
                assert_eq!(result_a, result_b)
            }
        }
    }
}
