use lalrpop_util::lalrpop_mod;
use std::collections::HashMap;

lalrpop_mod!(pub prop_logic);

#[derive(Debug)]
pub enum Prop<'l> {
    Var(&'l str),
    True,
    False,
    And(Box<Prop<'l>>, Box<Prop<'l>>),
    Or(Box<Prop<'l>>, Box<Prop<'l>>),
    Not(Box<Prop<'l>>),
}

pub fn eval(ctx: &HashMap<&str, bool>, prop: &Prop) -> Option<bool> {
    match prop {
        Prop::Var(name) => ctx.get(name).copied(),
        Prop::True => Some(true),
        Prop::False => Some(false),
        Prop::And(a, b) => Some(eval(ctx, a)? && eval(ctx, b)?),
        Prop::Or(a, b) => Some(eval(ctx, a)? || eval(ctx, b)?),
        Prop::Not(a) => Some(!eval(ctx, a)?),
    }
}
