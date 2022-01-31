use rand::{thread_rng, Rng};

use crate::{name::Name, DocumentBuilder};

#[derive(Debug, Clone)]
pub enum InputValue {
    // TODO
    // Variable()
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
    Enum(Name),
    List(Vec<InputValue>),
    Object(Vec<(Name, InputValue)>),
}

impl From<InputValue> for apollo_encoder::InputValue {
    fn from(input_value: InputValue) -> Self {
        // InputValue as we use it is not implemented in apollo_encoder
        todo!()
    }
}

impl<'a> DocumentBuilder<'a> {
    pub fn input_value(&mut self) -> InputValue {
        let mut rng = thread_rng();
        match rng.gen_range(0..8usize) {
            // Int
            0 => InputValue::Int(rng.gen()),
            // Float
            1 => InputValue::Float(rng.gen()),
            // String
            2 => InputValue::String(self.limited_string(40)),
            // Boolean
            3 => InputValue::Boolean(rng.gen()),
            // Null
            4 => InputValue::Null,
            // Enum
            5 => InputValue::Enum(self.choose_enum().arbitrary_variant().clone()),
            // List
            6 => {
                // FIXME: it's wrong it should always be the same type inside
                InputValue::List(
                    (1..rng.gen_range(2..5usize))
                        .map(|_| self.input_value())
                        .collect(),
                )
            }
            // Object
            7 => InputValue::Object(
                (1..rng.gen_range(2..5usize))
                    .map(|_| (self.name(), self.input_value()))
                    .collect(),
            ),
            _ => unreachable!(),
        }
    }
}
