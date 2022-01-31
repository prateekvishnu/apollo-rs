use rand::{thread_rng, Rng};

use crate::{argument::Argument, name::Name, DocumentBuilder};

pub struct Directive {
    pub(crate) name: Name,
    pub(crate) arguments: Vec<Argument>,
}

impl<'a> DocumentBuilder<'a> {
    pub fn directives(&mut self) -> Vec<Directive> {
        let mut rng = thread_rng();
        let num_directives = rng.gen_range(0..5);
        let directives = (0..num_directives).map(|_| self.directive());

        directives.collect()
    }

    pub fn directive(&mut self) -> Directive {
        let name = self.name();

        todo!()
    }
}
