mod mod_rs;
mod types_rs;

pub struct Template {
    pub name: &'static str,
    pub source: &'static str,
}

pub fn load_templates() -> Vec<Template> {
    vec!(
        mod_rs::load(),
        types_rs::load(),
    )
}