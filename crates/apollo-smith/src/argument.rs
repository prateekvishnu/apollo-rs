use rand::{thread_rng, Rng};

use crate::{name::Name, value::Value, DocumentBuilder};

#[derive(Debug, Clone)]
pub struct ArgumentsDef {}

#[derive(Debug, Clone)]
pub struct Argument {
    pub(crate) name: Name,
    pub(crate) value: Value,
}

impl<'a> DocumentBuilder<'a> {
    pub fn arguments(&mut self) -> Vec<Argument> {
        let mut rng = thread_rng();
        let num_directives = rng.gen_range(0..5);
        let directives = (0..num_directives).map(|_| self.argument());

        directives.collect()
    }

    pub fn argument(&mut self) -> Argument {
        let name = self.name();

        todo!()
    }
}
