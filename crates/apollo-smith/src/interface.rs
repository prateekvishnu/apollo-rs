use core::num;
use std::collections::HashSet;

use apollo_encoder::InterfaceDef;
use rand::{thread_rng, Rng};

use crate::{description::Description, field::FieldDef, name::Name, DocumentBuilder};

#[derive(Debug, Clone)]
pub struct InterfaceTypeDef {
    pub(crate) description: Option<Description>,
    pub(crate) name: Name,
    // TODO
    // interfaces: Vec<String>
    // pub(crate) directives: Vec<Directive>,
    pub(crate) fields_def: Vec<FieldDef>,
}

impl From<InterfaceTypeDef> for InterfaceDef {
    fn from(itf: InterfaceTypeDef) -> Self {
        let mut itf_def = InterfaceDef::new(itf.name.into());
        itf_def.description(itf.description.map(String::from));
        itf.fields_def
            .into_iter()
            .for_each(|f| itf_def.field(f.into()));
        // TODO interfaces + directives

        itf_def
    }
}

impl<'a> DocumentBuilder<'a> {
    pub fn interface_type_definition(&mut self) -> InterfaceTypeDef {
        let description = self
            .u
            .arbitrary()
            .unwrap_or(false)
            .then(|| self.description());
        let name = self.type_name();
        let fields_def = self.fields_definition(&[]);

        InterfaceTypeDef {
            description,
            name,
            fields_def,
        }
    }

    pub fn interface_implements(&mut self) -> HashSet<Name> {
        let mut rng = thread_rng();

        // let num_itf = self.u.arbitrary::<usize>().unwrap() % self.interface_type_defs.len();
        let num_itf = rng.gen_range(0..self.interface_type_defs.len());
        let mut interface_impls = HashSet::with_capacity(num_itf);

        for _ in 0..num_itf {
            let arbitrary_idx = rng.gen_range(0..self.interface_type_defs.len());
            interface_impls.insert(self.interface_type_defs[arbitrary_idx].name.clone());
        }

        interface_impls
    }
}
