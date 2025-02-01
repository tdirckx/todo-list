use crate::todo::todo_item::TodoItem;
use std::{collections::HashMap};
use colored::*;

pub struct TodoList {
    items: HashMap<u32, TodoItem>,
    keys: Vec::<u32>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList {
            items: HashMap::new(),
            keys: Vec::new(),
        }
    }

    pub fn add_item(&mut self, id: u32, description: String) {
        if self.items.contains_key(&id) {
            self.print_error(id)
        } else {
            self.save_item(id, description)
        }
    }
    
    pub fn remove_item(&mut self, id: u32) {
        if self.items.contains_key(&id) {
            self.delete_item(id);
        } else {    
            self.print_error(id);
        }
    }

    pub fn set_item_status(&mut self, id: u32, completed: bool) {
        if let Some(item) = self.items.get_mut(&id) {
            item.set_status(completed);
        }
    }

    pub fn list_items(&self) {
        println!("\n### ToDo List ###\n");

        for id in &self.keys {
            if let Some(item) = self.items.get(id) {
                item.display();
            }
        }
    }


    fn print_error(&self, id: u32) {
        println!("{}", format!("Todo-item id {} already exists", id).red())
    }

    fn delete_item(&mut self, id: u32) {
        self.items.remove(&id);
        self.keys.retain(|&v| v != id);
        println!("{}", format!("Todo-item id {} has been deleted", id).green());
    }
    
    fn save_item(&mut self, id: u32, description: String) {
        self.keys.push(id);
        self.items.insert(id, TodoItem::new(id, description));
        println!("{}", "Saved!!".green())
    }
}