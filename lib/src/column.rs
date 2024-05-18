pub struct Column {
    name: &'static str,
    foreign: bool,
}

impl Column {
    #[must_use]
    pub const fn new(name: &'static str, foreign: bool) -> Self {
        Self { name, foreign }
    }

    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    #[must_use]
    pub const fn foreign(&self) -> bool {
        self.foreign
    }
}
