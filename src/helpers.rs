use rocket_contrib::templates::handlebars::{Helper, Handlebars, Context, RenderContext, Output, HelperResult, JsonRender};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn url_encode(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut Output
) -> HelperResult {
    if let Some(param) = h.param(0) {
        let param_val = &param.value().render();
        let encoded = utf8_percent_encode(param_val, FRAGMENT).to_string();
        out.write(&encoded)?;
    }

    Ok(())
}