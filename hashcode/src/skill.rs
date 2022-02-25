#[derive(Debug, Clone, PartialEq)]
pub struct Skill {
    name: String,
    level: isize,
}

impl Skill {
    pub fn new(name: String, level: isize) -> Self {
        Self { name, level }
    }

    pub fn fullfills(&self, other: &Skill) -> bool {
        if self.name != other.name {
            false
        } else {
            self.level >= other.level
        }
    }

    pub fn level(&self) -> isize {
        self.level
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn inc(&mut self) {
        self.level += 1;
    }
}

impl From<&(isize, String)> for Skill {
    fn from(input: &(isize, String)) -> Self {
        Self::new(input.1.clone(), input.0)
    }
}
