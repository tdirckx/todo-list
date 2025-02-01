mod todo;

use todo::todo_service::TodoService;

fn main() {
    let mut service = TodoService::new();
    service.start_service();
}