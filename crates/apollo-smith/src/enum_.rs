use std::collections::HashSet;

use arbitrary::Arbitrary;
use rand::{thread_rng, Rng};

use crate::{name::Name, DocumentBuilder};

#[derive(Debug, Clone)]
pub struct EnumTypeDef {
    pub(crate) name: Name,
    pub(crate) variants: HashSet<Name>,
}

impl EnumTypeDef {
    pub fn arbitrary_variant(&self) -> &Name {
        let mut rng = thread_rng();

        self.variants
            .iter()
            .nth(rng.gen_range(0..self.variants.len()))
            .expect("cannot get variant")
    }
}

impl<'a> DocumentBuilder<'a> {
    pub fn enum_type_definition(&mut self) -> EnumTypeDef {
        //  --------------------------------- HERE
        todo!()
    }

    pub fn choose_enum(&mut self) -> &EnumTypeDef {
        let mut rng = thread_rng();

        self.enum_type_defs
            .get(rng.gen_range(0..self.enum_type_defs.len()))
            .expect("cannot get enum")
    }
}
