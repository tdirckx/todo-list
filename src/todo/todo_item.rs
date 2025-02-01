#[derive(Debug)]
pub struct TodoItem {
    id: u32,
    description: String,
    completed: bool,
}

impl TodoItem {
    pub fn new(id: u32, description: String) -> TodoItem {
        TodoItem {
            id,
            description,
            completed: false
        }
    }

    pub fn set_status(&mut self, completed: bool) {
        self.completed = completed;
    }

    pub fn display(&self) {
        println!("Id: {}, Description: {}, Completed: {}", self.id, self.description, self.completed);
    }
}