mod todo;

use todo::todo_list::TodoList;

fn main() {
    let mut list = TodoList::new();
    list.add(1, String::from("Gi"));
    list.display();
}
