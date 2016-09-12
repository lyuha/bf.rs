pub struct BFMemory {
    list: Vec<u8>,
    pointer: usize,
}

impl BFMemory {
    pub fn new() -> BFMemory {
        BFMemory {
            list: vec![0; 100],
            pointer: 0,
        }
    }
    pub fn get_value(&self) -> (u8) { self.list[self.pointer] }
    pub fn set_value(&mut self, value: u8) -> () { self.list[self.pointer] = value; }
    pub fn increase(&mut self) -> () {
        if self.list[self.pointer].checked_add(1).is_none() {
            // err
        } else {
            self.list[self.pointer] += 1;
        }
    }
    pub fn decrease(&mut self) -> () {
        if self.list[self.pointer].checked_sub(1).is_none() {
            // err
        } else {
            self.list[self.pointer] -= 1;
        }
    }
    pub fn move_left(&mut self) -> () {
        if self.pointer.checked_sub(1).is_none() {
            // err
        } else {
            self.pointer -= 1;
        }
    }
    pub fn move_right(&mut self) -> () {
        self.pointer += 1;
        if self.pointer >= self.list.len() {
            self.list.push(0);
        }
    }
}
