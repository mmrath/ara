use crate::config::AppConfig;
use failure::Error;
use handlebars::Handlebars;
use once_cell::sync::OnceCell;
use serde_json::{Map, Value};

static HANDLEBARS: OnceCell<Handlebars> = OnceCell::INIT;

fn build_handlebars(root: &str) -> Result<Handlebars, Error> {
    let mut handlebars = Handlebars::new();
    // register template from a file and assign a name to it
    handlebars.register_templates_directory(".hbs", root)?;

    Ok(handlebars)
}

pub fn render(template: &str, data: &Map<String, Value>) -> Result<String, Error> {
    let handlebars = HANDLEBARS.get_or_init(|| {
        let config = AppConfig::get();
        build_handlebars(&config.template_dir).unwrap()
    });
    let output = handlebars.render(template, data)?;
    Ok(output)
}
