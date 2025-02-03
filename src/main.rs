use task_manager::clear_screen;
use task_manager::input;
use task_manager::load_from_json;
use task_manager::save_to_json;
use task_manager::time;
use task_manager::uuid;
use task_manager::Task;
fn main() {
    let mut data: Vec<Task> = load_from_json();

    loop {
        println!("");
        println!("please Enter this cammands");
        println!(" add   : to add task");
        println!("delete : to delete task ");
        println!("update : to update task");
        println!(" clear : to clear screen ");
        println!(" save  : to save task");
        println!("  q    : to quit program  ⚠️ warning before quit toh insure you save task");
        println!("-------------------------------------------------------------------------");
        println!("");

        let first_input = input().to_string();
        match first_input.as_str().trim() {
            "add" => {
                println!("enter your task title");
                let title = input().to_string();
                println!("enter your task text");
                let text = input().to_string();
                let task = Task {
                    id: uuid(),
                    title: title,
                    text: text,
                    date: time(),
                };
                data.push(task);
                println!("task added");
            }
            "update" => {
                println!("enter id to update task");
                let update_id = input().as_str().trim().to_string();

                let mut found = false;
                for task in data.iter_mut() {
                    if task.id == update_id {
                        found = true;
                        println!("Enter new title (leave blank to keep the same):");
                        let new_title = input().to_string();
                        if !new_title.is_empty() {
                            task.title = new_title;
                        }

                        println!("Enter new text (leave blank to keep the same):");
                        let new_text = input().to_string();
                        if !new_text.is_empty() {
                            task.text = new_text;
                        }
                        println!("task updated");
                        break;
                    }
                }
                if !found {
                    println!("id not found");
                }
            }
            "display" => {
                println!("{:?}", data);
            }
            "delete" => {
                println!("enter id to delete task");
                let delete = input().as_str().trim().to_string();
                let orignal_len = data.len();
                data.retain(|task| task.id != delete);
                if data.len() == orignal_len {
                    println!("your id not found please enter a valid id");
                } else {
                    println!("task deleted with id {}", delete);
                }
            }
            "save" => {
                save_to_json(&data);
                println!("Tasks saved successfully.");
            }
            "q" => {
                break;
            }
            "clear" => {
                clear_screen();
            }
            _ => println!("camand not found"),
        }
    }
}
