use std::fmt::Debug;

use minijinja::{context, Environment};
use serde::Serialize;

#[derive(Serialize, Debug)]
struct Data {
    name: String,
    value: u16,
}

impl Data {
    pub fn f1(&self) -> String {
        self.name.clone()
    }
}

const TEMPLATE: &str = r#"
name: {{ data.name }}
value: {{ data.value }}
f1: {{ data.f1() }}
"#;

fn main() {
    let data = Data { name: "n1".to_string(), value: 123 };

    let mut env = Environment::new();
    env.add_template("template1", TEMPLATE).unwrap();
    let tmpl = env.get_template("template1").unwrap();
    println!("{}", tmpl.render(context! {data => data}).unwrap());
}
