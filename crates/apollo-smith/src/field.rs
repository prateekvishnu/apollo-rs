use std::collections::HashSet;

use apollo_encoder::Field;
use rand::{thread_rng, Rng};

use crate::{
    argument::ArgumentsDef, description::Description, name::Name, ty::Ty, DocumentBuilder,
};

#[derive(Debug, Clone)]
pub struct FieldDef {
    pub(crate) description: Option<Description>,
    pub(crate) name: Name,
    pub(crate) arguments_definition: Option<ArgumentsDef>,
    pub(crate) ty: Ty,
    // directives: Vec<Directive>
}

impl From<FieldDef> for Field {
    fn from(val: FieldDef) -> Self {
        let mut field = Field::new(val.name.into(), val.ty.into());
        if let Some(args) = val.arguments_definition {
            // TODO add arg fields.arg(....)
        }
        field.description(val.description.map(String::from));

        field
    }
}

impl<'a> DocumentBuilder<'a> {
    pub fn fields_definition(&mut self, exclude: &[&Name]) -> Vec<FieldDef> {
        let mut rng = thread_rng();
        let num_fields = rng.gen_range(2..50);
        let mut fields_names = HashSet::with_capacity(num_fields);

        // TODO switch to arbitrary
        for i in 0..num_fields {
            let name = self.name_with_index(i);
            if !exclude.contains(&&name) {
                fields_names.insert(self.name_with_index(i));
            }
        }

        // TODO add mechanism to add own type for recursive type
        let available_types: Vec<Ty> = self
            .object_type_defs
            .iter()
            .map(|o| Ty::Named(o.name.clone()))
            .collect();

        fields_names
            .into_iter()
            .map(|field_name| FieldDef {
                description: self
                    .u
                    .arbitrary()
                    .unwrap_or(false)
                    .then(|| self.description()),
                name: field_name,
                // TODO
                arguments_definition: None,
                ty: self.choose_ty(&available_types),
            })
            .collect()
    }
}
