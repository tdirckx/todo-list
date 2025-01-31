use crate::todo::todo_item::TodoItem;

pub struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList {
            items: Vec::new(),
        }
    }

    pub fn add(&mut self, id: u32, desciption: String) {
        self.items.push(TodoItem::new(id, desciption));
    }

    pub fn display(&self) {
        println!("### ToDo List ###\n");

        for item in &self.items {
            item.display();
        }
    }
}