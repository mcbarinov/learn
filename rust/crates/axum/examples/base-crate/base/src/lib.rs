use minijinja::Environment;

pub fn get_base_jinja_env() -> Environment<'static> {
    let mut env = Environment::new();
    minijinja_embed::load_templates!(&mut env);
    env
}
