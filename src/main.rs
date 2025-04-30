use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, fs, sync::Mutex};

//  Representing a single task
#[derive(Serialize, Deserialize, Clone)]
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

//  Containts the form data
#[derive(Deserialize)]
struct TaskForm {
    name: String,
    email: String,
    date: String,
    description: String,
}

//  json: web::Json<TaskForm> containts the form date send in method="post"
//  data: web::Data<AppState> has the list of TasK
async fn create_task_json(json: web::Json<TaskForm>, data: web::Data<AppState>) -> impl Responder {
    //  Lock task list and ID for safe access
    let mut tasks = data.tasks.lock().unwrap();
    let mut next_id = data.next_id.lock().unwrap();

    //  Create a new task using the form data
    let new_task = Task {
        id: *next_id,
        name: json.name.clone(),
        email: json.email.clone(),
        date: json.date.clone(),
        description: json.description.clone(),
    };

    *next_id += 1;

    //  Add new task to the queue
    tasks.push_back(new_task.clone());

    //  Return JSON with HTTP 201
    HttpResponse::Created().json(new_task)
}

//  Iterate list and return as a JSON array
async fn get_tasks_json(data: web::Data<AppState>) -> impl Responder {
    //  Lock and access task list
    let tasks = data.tasks.lock().unwrap();

    //  Clone tasks to return them
    let task_list: Vec<_> = tasks.iter().cloned().collect();

    //  Return cloned list as JSON
    HttpResponse::Ok().json(task_list)
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

//  Entry point
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
            .route("/api/tasks", web::get().to(get_tasks_json))
            .route("/api/tasks", web::post().to(create_task_json))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
