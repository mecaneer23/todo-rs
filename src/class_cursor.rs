use std::vec;

use crate::class_todo::Todo;
pub struct Cursor {
    pub positions: Vec<usize>,
    direction: Option<String>,
}

impl Cursor {
    pub fn new(positions: Vec<usize>) -> Cursor {
        Self {
            positions,
            direction: None,
        }
    }

    pub fn len(&self) -> usize {
        self.positions.len()
    }

    pub fn to_string(&self) -> String {
        self.positions[0].to_string()
    }

    pub fn to_int(&self) -> usize {
        self.positions[0]
    }

    pub fn contains(&self, child: &usize) -> bool {
        self.positions.contains(child)
    }

    pub fn get(&self) -> &Vec<usize> {
        &self.positions
    }

    pub fn get_first(&self) -> usize {
        self.positions[0]
    }

    pub fn get_last(&self) -> usize {
        self.positions[self.len() - 1]
    }

    pub fn set_to(&mut self, position: usize) {
        self.positions = vec![position]
    }

    pub fn todo_set_to(&mut self, todo_position: (Vec<Todo>, usize)) -> Vec<Todo> {
        self.positions[0] = todo_position.1;
        todo_position.0
    }

    pub fn todos_override(&mut self, todos: Vec<Todo>, positions: Vec<usize>) -> Vec<Todo> {
        self.positions = positions;
        todos
    }

    pub fn slide_up(&mut self) {
        let min = self.positions.iter().min().unwrap();
        if min == &0 {
            return;
        }
        self.positions.insert(0, min - &1);
        self.positions.pop();
    }

    pub fn slide_down(&mut self, max_len: &usize) {
        let max = *self.positions.iter().max().unwrap();
        if max >= max_len - 1 {
            return;
        }
        self.positions.push(max + 1);
        self.positions.rotate_left(1);
        self.positions.pop();
    }

    pub fn select_next(&mut self) {
        self.positions
        .push(*self.positions.iter().max().unwrap() + 1);
    self.positions.sort();
}

    pub fn deselect_next(&mut self) {
        if self.positions.len() > 1 {
            self.positions.pop();
        }
    }

    pub fn deselect_prev(&mut self) {
        if self.positions.len() > 1 {
            self.positions.rotate_left(1);
            self.positions.pop();
        }
    }

    pub fn select_prev(&mut self) {
        self.positions.push(*self.positions.iter().min().unwrap() - 1);
        self.positions.sort();
    }

    pub fn get_deletable(self) -> Vec<usize> {
        vec![*self.positions.iter().min().unwrap(); self.len()]
    }

    pub fn multiselect_down(&mut self, max_len: &usize) {
        if *self.positions.iter().max().unwrap() >= max_len - 1 {
            return;
        }
        let down = Some(String::from("down"));
        if self.positions.len() == 1 || self.direction == down {
            self.select_next();
            self.direction = down;
            return;
        }
        self.deselect_prev();
    }

    pub fn multiselect_up(&mut self) {
        let up = Some(String::from("up"));
        if *self.positions.iter().min().unwrap() == 0 && self.direction == up {
            return;
        }
        if self.positions.len() == 1 || self.direction == up {
            self.select_prev();
            self.direction = up;
            return;
        }
        self.deselect_next();
    }

    pub fn multiselect_top(&mut self) {
        for _ in self.positions[0]..0 {
            self.multiselect_up();
        }
    }

    pub fn multiselect_bottom(&mut self, max_len: usize) {
        for _ in self.positions[0]..max_len {
            self.multiselect_down(&max_len);
        }
    }

    pub fn multiselect_to(&mut self, position: usize, max_len: &usize) {
        for _ in self.positions[0]..position {
            if position >= self.positions[0] {
                self.multiselect_down(max_len);
                continue;
            }
            self.multiselect_up();
        }
    }

    pub fn multiselect_from(&mut self) {
        unimplemented!();
    }

    pub fn multiselect_all(&mut self, max_len: usize) {
        self.positions = (0..max_len).collect();
    }
}
