# CLI Todo
This is a simple todo list CLI application. It was made for me to get familiar with Rust, not to be a feature-complete working application.

If you want to test it out build it with cargo and execute it.

## How to use

### List tasks
To see a list of the current tasks just execute the program without any parameters.

### Create task
```
./cli-todo create <task-title>
```

E.g.: `./cli-todo create "My new task"`

### To change the status of a task
```
./cli-todo task <task-id> [done|progress|pending]
```

E.g.: `./cli-todo task 0 done`

## Stuff to improve

Adding columns to the program's output requires modification of code in many different parts. This could be improved by generalizing the implementation of columns.

Error handling is rubbish (user input validation, IO errors).