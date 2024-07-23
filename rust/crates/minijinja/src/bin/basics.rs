use chrono::{DateTime, Utc};
use minijinja::value::ValueKind;
use minijinja::{context, Environment, Value};
use serde::Serialize;

#[derive(Serialize)]
struct Data {
    name: String,
    value: u16,
    tags: Vec<String>,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

const TEMPLATE: &str = r#"
data list:
{% for d in data_list -%}
name: {{ d.name }}, value: {{d.value | dt}}
tags: {{ d.tags }}
created_at: {{ d.created_at | dt}}
updated_at: {{ d.updated_at | dt }}
-----------------------------------
{% endfor %}
"#;

fn main() {
    let d1 = Data {
        name: "n1".to_string(),
        value: 10,
        tags: vec!["a".to_string(), "b".to_string()],
        created_at: Utc::now(),
        updated_at: None,
    };
    let d2 = Data {
        name: "n2".to_string(),
        value: 20,
        tags: vec!["aa".to_string(), "bb".to_string()],
        created_at: Utc::now(),
        updated_at: Some(Utc::now()),
    };
    let data_list = vec![d1, d2];

    let mut env = Environment::new();
    env.add_filter("dt", dt_filter);
    env.add_template("template1", TEMPLATE).unwrap();
    let tmpl = env.get_template("template1").unwrap();
    println!("{}", tmpl.render(context! {data_list => data_list}).unwrap());
}

fn dt_filter(value: Value) -> String {
    if value.kind() == ValueKind::String {
        if let Ok(value) = DateTime::parse_from_rfc3339(value.as_str().unwrap()) {
            return value.format("%Y-%m-%d %H:%M:%S").to_string();
        }
    }
    "".to_string()
}
