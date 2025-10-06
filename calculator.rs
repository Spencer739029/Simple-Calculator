use axum::{
    response::Html,
    extract::Form,
};

#[derive(serde::Deserialize)]
pub struct CalcForm {
    num1: f64,
    num2: f64,
    op: String,
}

pub async fn calculate(Form(form): Form<CalcForm>) -> Html<String> {
    let result = match form.op.as_str() {
        "+" => form.num1 + form.num2,
        "-" => form.num1 - form.num2,
        "*" => form.num1 * form.num2,
        "/" => {
            if form.num2 != 0.0 {
                form.num1 / form.num2
            } else {
                return Html("Division by zero!".to_string());
            }
        }
        _ => return Html("Invalid operation!".to_string()),
    };

    let html = format!("<h1>Result: {}</h1>", result);
    Html(html)
}
