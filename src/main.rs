use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::{collections::VecDeque, fs, sync::Mutex};

//
#[derive(Deserialize)]
struct Task {
    name: String,
    email: String,
    date: String,
}

//  List of every Task
struct AppState {
    tasks: Mutex<VecDeque<Task>>,
}

//  task: web::Form<Task> containts the form date send in method="post"
//  data: web::Data<AppState> has the list of TasK
async fn create_task(task: web::Form<Task>, data: web::Data<AppState>) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    tasks.push_back(task.into_inner());

    HttpResponse::Ok().body("Tarea creada")
}

//  Iterate list and return formatted
async fn get_tasks(data: web::Data<AppState>) -> impl Responder {
    let tasks = data.tasks.lock().unwrap();

    let task_list = tasks
        .iter()
        .map(|t| format!("Nombre: {}, Email: {}, Fecha: {}", t.name, t.email, t.date))
        .collect::<Vec<String>>()
        .join("\n");

    HttpResponse::Ok().body(task_list)
}

//  Set localhost:8080/ path to the "templates/index.html"
async fn index() -> impl Responder {
    match fs::read_to_string("templates/index.html") {
        Ok(contents) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(contents),
        Err(_) => HttpResponse::InternalServerError().body("Error al cargar el HTML"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //  Creating an instace of the task list
    let tasks = web::Data::new(AppState {
        tasks: Mutex::new(VecDeque::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(tasks.clone())
            .route("/", web::get().to(index))
            .route("/tasks", web::get().to(get_tasks)) //  view created task in "localhost:8080/task"
            .route("/tasks", web::post().to(create_task))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
