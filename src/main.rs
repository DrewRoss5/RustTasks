use std::{env::args, fmt::Error, fs, io::{BufRead, BufReader}, num::ParseIntError, process::exit};

struct Task{
    complete: bool,
    name: String,
    id: i32
}

impl Task {
    // prints the task out 
    fn as_string(&self) -> String{
        let complete_str: String;
        if self.complete{
            complete_str = "COMPLETE".to_string();
        }
        else{
            complete_str = "INCOMPLETE".to_string();
        }
        format!("{0:<8} {1:<80}{2:<20}", self.id, self.name, complete_str)
    }

    fn export(&self) -> String{
        let complete_char: char;
        if self.complete{
            complete_char = 'X';
        }
        else{
            complete_char = ' ';
        }
        format!("{}{}", complete_char, self.name)
    }
}

fn read_task(task_str: String, id: i32) -> Result<Task, Error>{
    // determine if the task is complete based on the first character
    let mut task_chars = task_str.chars();
    let complete_char: char = task_chars.next().unwrap();
    let complete: bool;
    match complete_char{
        ' '=>complete = false,
        'X'=>complete = true,
        _=> return Err(Error)
    }
    // read the task name and construct the task
    let task = task_chars.as_str().to_string();
    Ok(Task{
        complete,
        name: task,
        id
    })   
}

fn read_task_file(file: &fs::File) -> Result<Vec<Task>, Error>{
    let mut tasks: Vec<Task> = Vec::new();
    // create an iterator for the file lines
    let lines = BufReader::new(file).lines();
    // parse each line and attempt to make a task file of it
    let mut tmp: Result<Task, Error>;
    let mut id = 0;
    for line in lines.flatten(){
        tmp = read_task(line, id);
        match tmp{
            Ok(task) => tasks.push(task),
            Err(_) => return Err(Error)
        }
        id += 1;
    }
    Ok(tasks)
}

fn write_task_file(filepath: &String, tasks: &Vec<Task>) -> Result<(), std::io::Error>{
    // create a string represetning all taks
    let mut task_str: String = "".to_string();
    for task in tasks{
        task_str += task.export().as_str();
        task_str += "\n"
    }
    fs::write(filepath, task_str)
}

// takes a vector of arguments and converts each of them to an i32, returns an error if a non-integer value is provided or any valueis greater than the provided maximum
fn parse_task_ids(arguments: &Vec<String>, parsed: &mut Vec<i32>, max: i32 ) -> Result<(), Error>{
    let mut tmp: Result<i32, ParseIntError>;
    // skip the first two arguments
    let it = arguments.iter().skip(2);
    for arg in it{
        tmp = arg.parse::<i32>();
        match tmp{
            Ok(e)=>parsed.push(e),
            Err(_)=>return Err(Error)
        }
        if tmp.unwrap() > max{
            return  Err(Error)
        }
    }
    Ok(())
}

fn main() {
    let file_path = "tasks.txt";
    // attempt to open the task file 
    let task_file = fs::File::open(file_path).expect("The task file could not be found.");
    // parse the task file
    let tmp = read_task_file(&task_file);
    if tmp.is_err() {println!("Invalid task file!")}
    else{
        // validate user arguments
        let arguments: Vec<String> = args().collect();
        if arguments.len() < 2{
            println!("Please provide at least one argument");
            exit(-1)
        }
        // run the user's chosen command
        let mut tasks = tmp.unwrap();
        match arguments[1].as_str() {
            "help"=>{
                let task_count = 5;
                let task_names = ["help", "view", "new", "complete", "del"];
                let task_params = ["", "", "<task name(s)>", "<task id(s)>", "<task id>"];
                let task_descriptions = ["Display a list of available commands", "Display all current tasks and their statuses",
                                                    "Create a new task for each of the provided names", "Mark the task at each given id complete", 
                                                    "Delete the task at the given id"];
                println!("{0:<10} {1:<20}{2}\n", "Command", "Parameters", "Description");
                for i in 0..task_count{
                    println!("{0:<9} {1:<20}{2}", task_names[i], task_params[i], task_descriptions[i])
                }
            }
            "view"=>{
                println!("{0:<8} {1:<80}{2:<20}", "ID:", "Name:", "Status:");
                for task in &tasks{
                    println!("{}", &task.as_string());
                }
            }
            "new"=>{
                // ensure that the user provided at least one task name, and create a task for each provided name if so
                if arguments.len() > 2{
                    let mut index: i32 = tasks.len() as i32;
                    for arg in arguments.iter().skip(2){
                        if arg.len() > 80{
                            println!("Invalid task name: \"{}\"\nPlease ensure that all task name are under 80 characters", arg);
                            exit(-1);
                        }
                        tasks.push(Task { complete: false, name: arg.to_string(), id: index});
                        index += 1;
                    }
                    if arguments.len() == 3{
                        println!("Task created successfully")
                    }
                    else{
                        println!("Tasks created successfully")
                    }
                }
                else{
                    println!("Please provide at least one task to create");
                    exit(-1)
                }

    
            },
            "complete"=>{
                // ensure that at least one task id was provided and that each task id is valid
                if arguments.len() < 3{
                    println!("Please provide at least one task ID to complete")
                }
                let mut id_list: Vec<i32> = Vec::new(); 
                let result = parse_task_ids(&arguments, &mut id_list, (tasks.len() - 1) as i32);
                match result{
                    Ok(_)=>{
                        for i in id_list{
                            tasks[i as usize].complete = true;
                        }
                        if arguments.len() == 3{
                            println!("Task marked as complete successfuly")
                        }
                        else{
                            println!("Tasks marked as complete successfuly")
                        }
                    }
                    Err(_)=>{
                        println!("Invalid Task ID");
                        exit(-1)
                    }
                }
            },
            "del"=>{
                // ensure exacty one ID was provided and that it's valid
                if arguments.len() != 3{
                    println!("Please provide exactly one task ID to delete");
                    exit(-1)
                }
                let result = arguments[2].parse::<i32>();
                match result{
                    Ok(id)=>{
                        tasks.remove(id as usize);
                        println!("Task deleted succesfully")
                    }
                    Err(_)=>{
                        println!("Invalid task ID");
                        exit(-1)
                    }
                }
            }
            _=>{
                print!("\"{}\" is not a valid command", arguments[1]);
            }
        }
        write_task_file(&file_path.to_string(), &tasks).unwrap()
    }
}