use rocket::fairing::AdHoc;

use crate::app::controllers::answer as answer_controller;
use crate::app::controllers::form as form_controller;
use crate::app::controllers::question as question_controller;
use crate::app::controllers::user as user_controller;
use crate::app::controllers::paper as paper_controller;

#[get("/health")]
fn index() -> &'static str {
    "questions api ready"
}

pub fn router() -> AdHoc {
    AdHoc::on_ignite("Mounting routes", |rocket| async {
        rocket
            .mount("/", routes![index])
            .mount(
                "/api/v1/question",
                routes![
                    question_controller::index,
                    question_controller::show,
                    question_controller::store,
                    question_controller::destroy,
                    question_controller::update,
                ],
            )
            .mount(
                "/api/v1/form",
                routes![
                    form_controller::index,
                    form_controller::show,
                    form_controller::store,
                    form_controller::destroy,
                    form_controller::update,
                ],
            )
            .mount(
                "/api/v1/user",
                routes![
                    user_controller::index,
                    user_controller::show,
                    user_controller::update,
                ],
            )
            .mount(
                "/api/v1/answer",
                routes![
                    answer_controller::index,
                    answer_controller::show,
                    answer_controller::store,
                    answer_controller::destroy,
                    answer_controller::update,
                ],
            ).mount("/api/v1/paper",
                routes![
                    paper_controller::index,
                    paper_controller::show,
                    paper_controller::store,
                    paper_controller::destroy,
                    paper_controller::update,
                ],
            )
    })
}
