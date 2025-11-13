# Iced Todo App

A TodoMVC-style application built with iced.

## Features

- Add/remove tasks
- Mark tasks as complete
- Filter tasks (All/Active/Completed)
- Clear completed tasks
- Persistent input state

## Architecture

Uses The Elm Architecture pattern:
- **Model**: TodoApp state with tasks and filter
- **Message**: User actions (add, toggle, delete, filter)
- **Update**: State transitions based on messages
- **View**: Declarative UI rendering

## Running

```bash
cargo run
```

## References

- [TodoMVC](https://todomvc.com/)
- [iced Todos Example](https://github.com/iced-rs/iced/tree/master/examples/todos)
