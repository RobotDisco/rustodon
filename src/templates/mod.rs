use maud::{self, html, Markup, Render};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use GIT_REV;

/// Type to store data about a templated page in. Used to insert each page's markup into
/// a base template which sets up stuff like stylesheets and the general html structure.
#[derive(Debug, Builder)]
#[builder(setter(into))]
pub struct Page {
    #[builder(default = "None")]
    title: Option<String>,
    content: Markup,
}

/// Allows returning `Page`s from Rocket routes.
impl<'r> Responder<'r> for Page {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        self.render().respond_to(req)
    }
}

impl Render for Page {
    fn render(&self) -> Markup {
        html! {
            (maud::DOCTYPE)

            head {
                meta charset="utf-8";
                title {
                    @if let Some(title) = self.title.as_ref() {
                        "Rustodon | " (title)
                    } @else {
                        "Rustodon"
                    }
                }
            }

            body {
                (self.content)

                footer {
                    div {
                        "Running commit "
                        a href=(format!("https://github.com/rustodon/rustodon/commit/{}", GIT_REV))
                            code (GIT_REV)
                    }
                }
            }
        }
    }
}
