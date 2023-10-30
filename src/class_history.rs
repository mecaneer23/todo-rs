use std::fmt::{self, write};

use crate::class_todo::Todo;

struct Restorable {
    stored: String,
    selected: usize,
}

impl Restorable {
    pub fn new(todos: Vec<Todo>, selected: usize) -> Restorable {
        Restorable {
            stored: todos
                .iter()
                .map(|todo: &Todo| todo.text.clone())
                .collect::<Vec<String>>()
                .join(" |SEP|"),
            selected,
        }
    }

    pub fn get(&self) -> (Vec<Todo>, usize) {
        (
            self.stored
                .split(" |SEP|")
                .map(|text| Todo::new(text))
                .collect::<Vec<Todo>>(),
            self.selected,
        )
    }
}

impl fmt::Display for Restorable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            self.stored.replace(" |SEP|", ", "),
            self.selected
        )
    }
}

pub struct UndoRedo {
    history: Vec<Restorable>,
    index: usize,
}

impl UndoRedo {
    pub fn new() -> UndoRedo {
        UndoRedo {
            history: vec![],
            index: 0,
        }
    }

    pub fn add(&mut self, todos: Vec<Todo>, selected: usize) {
        self.history.push(Restorable::new(todos, selected));
        self.index = self.history.len() - 1;
    }

    pub fn undo(&mut self) -> (Vec<Todo>, usize) {
        if self.index > 0 {
            self.index -= 1;
        }
        self.history[self.index].get()
    }

    pub fn redo(&mut self) -> (Vec<Todo>, usize) {
        if self.index < self.history.len() - 1 {
            self.index += 1;
        }
        self.history[self.index].get()
    }
}

impl fmt::Display for UndoRedo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let history_display: Vec<String> = self.history.iter().enumerate()
        .map(|(i, v)| {
            if i == self.index {
                format!(">  {}", v)
            } else {
                format!("   {}", v)
            }
        })
        .collect();

        let history_str = history_display.join("\n");
        let length_str = format!("length: ({})", self.history.len());
        let index_str = format!("index: ({})", self.index);

        write!(f, "{}\n{}\n{}", history_str, length_str, index_str)
    }
}