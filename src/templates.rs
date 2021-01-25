use crate::{
    data::{get_projects, Projects},
    languages::{Language, PageTexts},
    utils::{get_current_directory, get_language_texts, get_scss_content},
};
use actix_web::{error::InternalError, http::StatusCode, web, HttpRequest, HttpResponse};
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "page.stpl")]
pub struct Page<'a> {
    pub page_id: &'a str,
    pub page_styles: String,

    pub page_lang: String,
    pub page_texts: PageTexts,
    pub texts: Language,

    pub page_url: String,
    pub page_keywords: &'a str,
    pub page_image: String,

    pub projects: Projects,

    pub heart_svg: &'a str,
}

fn get_header_keys<'a>(page_id: &str, page_url: &String) -> (&'a str, String) {
    let mut page_keywords: &'a str = "";
    let mut page_image = String::new();

    if page_id == "home" {
        page_keywords = "Daniel Solarte Chaverra, Developer, ReactJS, TypeScript, JavaScript, js, programmer, software, NodeJS, Deno, Rust, Ionic, Figma, danielsolartech, 100DaysOfCode, portfolio, it, technology, service workers, pwa, ts, react";
        page_image = format!("{}assets/images/avatar.png", page_url);
    }

    (page_keywords, page_image)
}

pub fn render(
    page_id: &str,
    page_url: web::Data<String>,
    req: HttpRequest,
) -> actix_web::Result<HttpResponse> {
    let (page_lang, texts) = get_language_texts(&req);
    let page_url: String = page_url.into_inner().to_string();
    let (page_keywords, page_image) = get_header_keys(page_id, &page_url);

    let page_texts: &PageTexts = texts
        .pages
        .get(page_id.clone())
        .expect("Cannot get page texts.");

    let scss_rute: String = format!("{}assets/scss/{}.scss", get_current_directory(), page_id);

    let template = Page {
        page_id,
        page_styles: get_scss_content(&scss_rute).expect("Cannot parse SCSS file."),

        page_lang,
        page_texts: page_texts.clone(),
        texts,

        page_url,
        page_keywords,
        page_image,

        projects: if page_id == "projects" {
            get_projects().expect("Cannot parse projects data.")
        } else {
            Projects::new()
        },

        heart_svg: include_str!("heart.svg"),
    };

    Ok(HttpResponse::Ok().content_type("text/html").body(
        template
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?,
    ))
}
