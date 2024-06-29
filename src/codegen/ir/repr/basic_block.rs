use super::Triple;

#[derive(Default, Clone, Debug)]
pub struct BasicBlock {
    pub triples: Vec<Triple>,
}

impl BasicBlock {
    pub fn add_triple(&mut self, triple: Triple) -> usize {
        let id = self.triples.len();
        self.triples.push(triple);

        id
    }
}
