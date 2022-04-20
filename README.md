# TaskMaster

**WIP** don't expect stability or ease of use yet.

---

A cli-based todo-list manager.

## Install
```bash
git clone https://github.com/pebblS/taskmaster.git
cd taskmaster
cargo install --path .
```

## Usage
```
USAGE:
    task [SUBCOMMAND]

    ARGS:
        <SUBCOMMAND>    [default: list] [possible values: add, list, done]
```

## Project goals

### Tasks
- [X] Add description to tasks
- [X] Toggle done state of task
- [ ] Add due date to tasks
- [ ] Add priority to tasks
- [ ] Add context to tasks
- [ ] Add comments to tasks

### Cli tool
- [X] Add tasks
- [X] Display tasks
- [X] Filter tasks based on description
- [X] Filter tasks based on done state
- [X] Save tasks on disk
- [X] Remove done tasks
- [ ] Filter tasks based on priority
- [ ] Filter tasks based on due date
- [ ] Save multiple different tasks lists for different projects
- [ ] Manage tasks on a different machine (through ssh? http?)
