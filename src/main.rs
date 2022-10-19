use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct TaskRequest {
    task_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    task_text: Vec<String>,
    test_cases: Vec<TestCase>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TestCase {
    stdin: Vec<String>,
    stdout: Vec<String>,
}

/*
 * !Make it a dependency, so we don't have to copy it anymore!
 */
#[derive(Debug, Serialize, Deserialize)]
struct ErrorOutput {
    code: i32,
    error: String,
}

impl ErrorOutput {
    pub fn new(code :i32,error :&str) -> ErrorOutput {
        return ErrorOutput { code: code, error: String::from(error) }
    }
}

/// This handler uses json extractor with limit
async fn get_task(request: web::Json<TaskRequest>, _: HttpRequest) -> HttpResponse {
    /*
     * Get path to the requested file
     */
    let dir = "./tasks/".to_string() + &request.task_name + ".json";
    let task_file_result = std::fs::read_to_string(&dir);
    let task_file = match task_file_result {
        Ok(file) => file,
        Err(_) => return HttpResponse::Ok().json(ErrorOutput::new(-1,"Could not open file with the task")),
    };

    /*
     * Make read the file into a struct
     */
    let task_result = serde_json::from_str::<Task>(&task_file);
    let task = match task_result {
        Ok(task) => task,
        Err(_) => return HttpResponse::Ok().json(ErrorOutput::new(-2,"Could not deserialize the task into a proper Task struct")),
    };

    HttpResponse::Ok().json(task) // <- send json response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new().wrap(cors).service(web::resource("/get_task").route(web::post().to(get_task)))
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
