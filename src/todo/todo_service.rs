use colored::Colorize;

use crate::todo::todo_list::TodoList;
use std::{collections::HashMap, io};

pub struct TodoService {
    todo: HashMap<String, TodoList>,
}

impl TodoService {
    pub fn new() -> TodoService {
        TodoService {
            todo: HashMap::new(),
        }
    }

    pub fn start_service(&mut self) {
        println!("{}", format!("\n########################################").purple());
        println!("{}", format!("###### Welcome to Todo-list tool. ######").purple());
        println!("{}", format!("########################################\n\n").purple());
        
        self.start();
    }

    pub fn start(&mut self) {
        let mut todo = TodoList::new();

        let mut input = self.ask_question(String::from("# Please enter one of these commands: new, delete, enter, leave, list"));

        self.execute_command(input.trim());

        // todo.add_item(1, String::from("Call Bill!"));
        // todo.add_item(2, String::from("Call Dana!"));
        // todo.add_item(2, String::from("Call Dana!"));
        // todo.add_item(3, String::from("Call Tom!"));
        // todo.remove_item(1);

        // todo.set_item_status(2, true);
        // todo.list_items();
    }

    fn new_todo_list(&mut self) {
        let mut input = self.ask_question(String::from("# Please enter a name for your list!"));
        self.todo.insert(input, TodoList::new());

        self.start();
    }

    fn delete_todo_list(&mut self) {}
    fn enter_todo_list(&mut self) {}
    fn leave_todo_list(&mut self) {}

    fn list_todo_list(&mut self) {
        println!("#### Todo list:\n");
        for (key, _value) in &self.todo {
            print!("- {}", key);
        }

        self.start();
    }

    fn unknown_command(&mut self, command: &str) {
        println!("{}", format!("Unknown command: {}, please use one of these commands: new, delete, enter, leave, list\n\n", command).red());
        self.start();
    }

    fn execute_command(&mut self, command: &str) {
        match command {
            "new" => self.new_todo_list(),
            "delete" => self.delete_todo_list(),
            "enter" => self.enter_todo_list(),
            "leave" => self.leave_todo_list(),
            "list" => self.list_todo_list(),
            _ => self.unknown_command(command),
        }
    }

    fn ask_question(&self, question: String) -> String {
        println!("{}", format!("{}", question).italic());

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        println!("\n");

        input
    }
}

//enter commands
//listen to command and ask follow up questions
//when finished print result and go back to ask command
//commands:

// list todo-lists
// create new todo-list
// enter todo-list
// exit todo-list
// delete todo-list

// call crud from todo-list

//### todo::
//- refactor big time
//- create command mod
//- create list service
//- create command + attributes possible. Like: new Book, creates a new todo-list called book
//- create enter <list name>
//- add item
//- remove item
//- delete item
//- set item status
//- create leave <list name>
//- save options?
//- load options?