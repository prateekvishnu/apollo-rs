use rand::{thread_rng, Rng};

use crate::{input_value::InputValue, name::Name, DocumentBuilder};

#[derive(Debug, Clone)]
pub struct ArgumentsDef {}

#[derive(Debug, Clone)]
pub struct Argument {
    pub(crate) name: Name,
    pub(crate) value: InputValue,
}
// TODO implement From

impl<'a> DocumentBuilder<'a> {
    pub fn arguments(&mut self) -> Vec<Argument> {
        let mut rng = thread_rng();
        let num_arguments = rng.gen_range(0..5);
        let arguments = (0..num_arguments).map(|_| self.argument());

        arguments.collect()
    }

    pub fn argument(&mut self) -> Argument {
        let name = self.name();
        let value = self.input_value();

        Argument { name, value }
    }
}
