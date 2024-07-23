use minijinja::value::ViaDeserialize;
use minijinja::{context, Environment};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    name: String,
}

#[allow(dead_code)]
impl Data {
    // it's impossible to use it in a template like this: {{ data.my_method() }}
    // use https://docs.rs/minijinja/latest/minijinja/functions/index.html#arguments-in-custom-functions
    // or Objects trait
    fn my_method(&self) -> String {
        format!("my-{}", self.name)
    }
}

const TEMPLATE: &str = r#"name: {{ data.name }}, my_method: {{ data | my_method  }}"#;

fn my_method(data: ViaDeserialize<Data>) -> String {
    format!("my-{}=>{}", data.name, data.my_method())
}

fn main() {
    let mut env = Environment::new();
    env.add_template("t1", TEMPLATE).unwrap();
    env.add_filter("my_method", my_method);

    let data = Data { name: "t1".to_string() };
    let tmpl = env.get_template("t1").unwrap();
    println!("{}", tmpl.render(context! {data => data}).unwrap());
    // name: t1, my_method: my-t1=>my-t1
}
