use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::{collections::VecDeque, fs, sync::Mutex};

#[derive(Deserialize, Clone)]
struct Task {
    id: usize,
    name: String,
    email: String,
    date: String,
    description: String,
}

//  List of every Task
struct AppState {
    tasks: Mutex<VecDeque<Task>>,
    next_id: Mutex<usize>,
}

//  containts the form date
#[derive(Deserialize)]
struct TaskForm {
    name: String,
    email: String,
    date: String,
    description: String,
}

//  form: web::Form<TaskForm> containts the form date send in method="post"
//  data: web::Data<AppState> has the list of TasK
async fn create_task(form: web::Form<TaskForm>, data: web::Data<AppState>) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    let mut next_id = data.next_id.lock().unwrap();

    let new_task = Task {
        id: *next_id,
        name: form.name.clone(),
        email: form.email.clone(),
        date: form.date.clone(),
        description: form.description.clone(),
    };

    *next_id += 1;
    tasks.push_back(new_task);

    HttpResponse::SeeOther()
        .append_header(("Location", "/tasks"))
        .finish()
}

//  Iterate list and return formatted
async fn get_tasks(data: web::Data<AppState>) -> impl Responder {
    let tasks = data.tasks.lock().unwrap();
    let mut table_html = String::new();

    for task in tasks.iter() {
        table_html.push_str(&format!(
            "<tr>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
            </tr>",
            task.id, task.name, task.email, task.date, task.description
        ));
    }

    //  replace field in index.html to display table
    let template = std::fs::read_to_string("templates/index.html").unwrap_or_default();
    let full_page = template.replace("{{TASK_TABLE}}", &table_html);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(full_page)
}

//  Set localhost:8080/ path to the "templates/index.html"
async fn index(data: web::Data<AppState>) -> impl Responder {
    //  generated empty list
    let tasks = data.tasks.lock().unwrap();
    let mut table_html = String::new();

    for task in tasks.iter() {
        table_html.push_str(&format!(
            "<tr>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
            </tr>",
            task.id, task.name, task.email, task.date, task.description
        ));
    }

    match fs::read_to_string("templates/index.html") {
        Ok(contents) => {
            let full_page = contents.replace("{{TASK_TABLE}}", &table_html);

            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(full_page)
        }
        Err(_) => HttpResponse::InternalServerError().body("Error al cargar el HTML"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //  Creating an instace of the task list
    let app_tasks = web::Data::new(AppState {
        tasks: Mutex::new(VecDeque::new()),
        next_id: Mutex::new(1),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_tasks.clone())
            .route("/", web::get().to(index))
            .route("/tasks", web::get().to(get_tasks))
            .route("/tasks", web::post().to(create_task))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
