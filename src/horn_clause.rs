pub struct HornClause {
    literals: Vec<i8>,
}

impl HornClause {
    pub fn new(literals: Vec<i8>) -> Self {
        HornClause {
            literals
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, i8> {
        self.literals.iter()
    }
}