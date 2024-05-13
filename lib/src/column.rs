pub struct Column {
    name: &'static str,
}

impl Column {
    #[must_use]
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }

    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }
}
