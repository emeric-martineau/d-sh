///
/// Module to create standard handlebars.
///
/// Release under MIT License.
///
use handlebars::*;

handlebars_helper!(ends_width_helper: |text: str, pattern: str| text.ends_with(pattern));

pub struct Template;

impl Template {
    pub fn new() -> Handlebars {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("ends_width", Box::new(ends_width_helper));

        handlebars
    }
}
