I plan to use the apollo-encoder to generate graphql schema but I'm wondering what's the best way to
use them. Because I will need to implement `arbitrary` trait on it. Several solutions:

- Creating automatically with macros wrapper types:

```rust
pub type SchemaDef(apollo_encoder::SchemaDef);
impl Deref for SchemaDef {
    type Target = apollo_encoder::SchemaDef;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SchemaDef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> Arbitrary<'a> for SchemaDef {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        // My implementation

        Ok(my_schema)
    }
}
```

Ok solution 2 seems better, because we are more free to do whatever we want, having some derives, some public fields and so one
There are some missing implementations in apollo_encoder
// TODO: missing directive() method on field
// TODO: missing directive() method on InputValueField

Solution 2. On prend tous les types que je créé
Je pars du type Object et je l'implémente je descends jusqu'au bout.
J'implémente tout les Into qui vont bien avec un panic dedans directement
Et chaque fonction me renvoie bien un Node comme prévu
Pour chaque élément de la grammaire je veux une fonction
Genre par exemple pour https://spec.graphql.org/October2021/#ArgumentsDefinition
ArgumentsDefinition j'aurai arguments_definition() -- input_value_definition() -- description() -- name() -- type() -- default_value() -- directives() -- directive() --name() -- arguments() -- argument() -- name() -- value()
Je pense que je peux tout implémenter en mode Impl de DocumentBuilder
