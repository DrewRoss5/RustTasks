# RustTodo
A CLI ToDo list utility created in Rust
# Roadmap:
<ul>
  <li>Create an install script</li>
</ul>

# Task File Format
Files for this program follow a very simple formt:<br>
Each line in the file represents a task, if the first character is an X, the task is complete, otherwise, it is incomplete.<br>
## Example:
`XA complete task` <br>
 `An incomplete task`<br>
`XA thrid task`<br>
Task ID's are generated at runtime: At the moment, the program looks for a "tasks.txt" file in the current working directory, though this will be updated to a more accessible location in the future.

# Installation:
At the time of writing, there is no official install script, so the best way to use the program is to create a file called `tasks.txt` in the directory you will call the program from, and generate a binary using `cargo run` or  `cargo build`.

# Usage:
### Commands:
<dl>
  <dt>help</dt>
  <dd>Prints a list of available commands and their functions.</dd>
  <dt>view</dt>
  <dd>Prints each task's id, name and completion status.</dd>
  <dt>new &lt;task name(s)&gt;</dt>
  <dd>Creates a new task for each name. Names should be passed as strings in quotations and seperated by spaces</dd>
  <dt>complete &lt;task id(s)&gt;</dt>
  <dd>Completes the task for each provided id. Accepts any number of ids, seperated with spaces</dd>
  <dt>del &lt;task id&gt;</dt>
  <dd>Deletes the task with the given id. Currently accepts only one ID</dd>
</dl>
