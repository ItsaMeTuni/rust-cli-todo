# CLI Todo

![](https://i.imgur.com/EIA68l8.png)

This is a simple todo list CLI application. It was made for me to get familiar with Rust, not to be a feature-complete working application.

If you want to test it out build it with cargo and execute it.

Data is stored in ~/.cli-todo as JSON.

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

# Spec

Here I'll define how the app should work in its final version, if I ever get to finish it.


Standard output
```
ID    Status   Title                      Deadline    Tags              Repeat
#a0f  done     Implement feature #24bd7f  06/20/2020  work development  no
#a11  done     Finish article             ----------  personal          no
#a12  pending  Write weekly report        fri 10PM    work boring       every friday
```

Notes:
- Columns can be reordered (flag and cfg)
- Columns can be hidden (flag and cfg)

## Commands

### Listing tasks

`cli-todo`

Executing cli todo without any arguments will list the tasks.

Flags:
- `--order <column-name>` Order the list by the specified column name. Case-indepentent. If the column name has spaces it must be in quotes.
- `--rev-order` Reverse the order of the column.
- `--hide <column-names>` Hide columns with name. Case-indepentent. Comma-separated. Must be in quotes if any column name has spaces.
- `--show <column-names` Show columns with name (useful if the user has hidden any columns by default). Case-indepentent. Comma-separated. Must be in quotes if any column names have spaces.
- `--only <tags>` Filter tasks with the specified tags. Case-indepentent. Comma-separated. Must be in quotes if any tags have spaces.
- `--except <tags>` Filter tasks that do not have the specified tags. Case-indepentent. Comma-separated. Must be in quotes if any tags have spaces.

### Create task

Syntax
`cli-todo create <title> [tags] [deadline] [--repeat]`

Notes:
- Optional commands can be rearranged in any order the user wants, the program will determine what is what in the parsing phase.
- Titles have to be in quotes if they contain spaces
- Tags are comma separated, if any have spaces the whole tags group must be in quotes
- The `done`, `pending` and `progress` tags are special tags, if any of them is added the status of the task will be set to that. If more than one is added the status will be set to the last one. If none are set the status will default to `pending`.
- See the date rule chapter for information on date arguments
- If the `--repeat` flag is set the task will repeat on the same day of the week and at the same time set in the deadline. Deadline is required to be set for the `--repeat` flag to be used.

Examples:

`cli-todo create "my task"`
`cli-todo create "other task" "work,development,boring"`
`cli-todo create "another task" personal`

TODO: add examples with repeat and deadline

### Change status of a task

`cli-todo status <task-id> [new-status]`
`cli-todo done <task-id>`

Notes:
- If no `new-status` argument is provided the task will be set to `done`.
- Possible values for `new-status` are `done`, `progress`, `pending`.
- `task-id` is the ID of the task with or without the `#` prefix.
- The done command is an alias for `cli-todo <task-id> done`

### Delete a task

`cli-todo delete <task-id>`
`cli-todo del <task-id>`

Notes:
- `del` is an alias for `delete`

### Configure a tag

Tags don't have to be configured, they're just strings. However, if the user adds a color to a tag that tag will appear colored in the output.

`cli-todo tag <tag-name> <hex-color>`

`cli-todo tag work #7748cd`


## Date syntax

Here the syntax for specifying dates in arguments is defined.

Syntax `[<mm>/<dd>/[yyyy]|<special-keyword>] [hh:mm]`

Date special keywords are `today`, `tomorrow`, `mon`, `tue`, `wed`, `thu`, `fri`, `sat`, `sun`.

Examples:

`04/06/2020 14:45`
`04/06 16:45`
`today 16:45` 

`tomorrow`: Next day

`sun 20:00`: Next sunday at 20:00

## Extra notes

Make statuses as special tags. That way we can implement new statuses easily later, maybe even let the user create their own.