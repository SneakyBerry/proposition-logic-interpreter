use proposition_logic_interpreter::prop_logic::LexPropParser;
use proposition_logic_interpreter::{eval, Prop};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::Write;
use text_io::read;

#[must_use]
fn read_variables(variable_names: HashSet<&str>) -> HashMap<&str, bool> {
    let mut variables = HashMap::new();
    for name in variable_names {
        print!("{} = ", &name);
        std::io::stdout().flush().unwrap();
        let v: bool = read!("{}\r\n");
        variables.insert(name, v);
    }

    variables
}

fn get_variables_set<'l>(prop: &'l Prop, hs: &mut HashSet<&'l str>) {
    match prop {
        Prop::Var(name) => {
            hs.insert(*name);
        }
        Prop::And(a, b) => {
            get_variables_set(a, hs);
            get_variables_set(b, hs)
        }
        Prop::Or(a, b) => {
            get_variables_set(a, hs);
            get_variables_set(b, hs)
        }

        Prop::Not(a) => get_variables_set(a, hs),
        _ => (),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let expr: String = read!("{}\r\n");
    let parser = LexPropParser::new();
    let prop = parser.parse(&expr).unwrap();

    let mut variable_names = HashSet::new();
    get_variables_set(&prop, &mut variable_names);

    let ctx = read_variables(variable_names);

    let result = eval(&ctx, &prop).unwrap();

    println!("Result: {}", &result);
    Ok(())
}
