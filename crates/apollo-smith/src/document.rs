use apollo_encoder::Schema;

use crate::object::ObjectTypeDef;

pub struct Document {
    object_types: Vec<ObjectTypeDef>,
}

impl From<Document> for Schema {
    fn from(val: Document) -> Self {
        let mut schema = Schema::new();
        val.object_types
            .into_iter()
            .for_each(|obj| schema.object(obj.into()));

        schema
    }
}
