use actix_web::{web, error, HttpResponse, Result};
use tera::{Context, Tera};
use crate::util;


pub fn index(
    tmpl: web::Data<Tera>,
    ) -> Result<HttpResponse>{

    let settings = util::get_settings();
    let mut context = Context::new();
    context.insert("title", &settings["title"]);
    context.insert("version", &settings["version"]);
    let rendered = tmpl.render("index.html.tera", &context).map_err(|e| {
        error::ErrorInternalServerError(e.description().to_owned())
    })?;

    Ok(HttpResponse::Ok().body(rendered))
}
