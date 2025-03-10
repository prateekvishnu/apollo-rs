# Changelog

All notable changes to `apollo-parser` will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- # [x.x.x] (unreleased) - 2021-mm-dd

> Important: X breaking changes below, indicated by **BREAKING**

## BREAKING

## Features

## Fixes

## Maintenance

## Documentation -->
# [0.2.6](https://crates.io/crates/apollo-parser/0.2.6) - 2022-05-24

## Fixes
- **lex escaped characters in StringValue tokens - [bnjjj], [pull/228] closes [issue/227], [issue/229]**

  StringValues with correctly escaped quotation marks, e.g. `{ name(id: "\"escaped\"") }`
  would error and not lex correctly. Additionally, invalid escapes in string
  values, e.g. `{ name(id: "escaped \a") }` should have an error created in the
  lexer. Both issues are fixed, and correctly bubble up to the parser.


  [bnjjj]: https://github.com/bnjjj
  [pull/228]: https://github.com/apollographql/apollo-rs/pull/228
  [issue/227]: https://github.com/apollographql/apollo-rs/issues/227
  [issue/229]: https://github.com/apollographql/apollo-rs/issues/229

# [0.2.5](https://crates.io/crates/apollo-parser/0.2.5) - 2022-04-01

> Important: 1 breaking change below, indicated by **BREAKING**

## BREAKING
- **GraphQL Int Values are cast to i32 - [bnjjj], [pull/197]**
  AST's Int Values have an `Into` implementation to their Rust type. They were
  previously converted to i64, which is not compliant with the spec. Int Values
  are now converted to i32.
  ```rust
  if let ast::Value::IntValue(val) =
      argument.value().expect("Cannot get argument value.")
  {
      let i: i32 = val.into();
  }
  ```
  [bnjjj]: https://github.com/bnjjj
  [pull/197]: https://github.com/apollographql/apollo-rs/pull/197

## Features
- **Adds a .text() method to ast::DirectiveLocation - [bnjjj], [pull/197]**
  `DirectiveLocation` can now additionally be accessed with a `.text()` method.

  ```rust
  let schema = r#"directive @example on FIELD | FRAGMENT_SPREAD | INLINE_FRAGMENT"#;
  let parser = Parser::new(schema);
  let ast = parser.parse();

  assert!(ast.errors.is_empty());

  let document = ast.document();
  for definition in document.definitions() {
      if let ast::Definition::DirectiveDefinition(dir_def) = definition {
          let dir_locations: Vec<String> = dir_def
              .directive_locations()
              .unwrap()
              .directive_locations()
              .map(|loc| loc.text().unwrap().to_string())
              .collect();
          assert_eq!(
              dir_locations,
              ["FIELD", "FRAGMENT_SPREAD", "INLINE_FRAGMENT"]
          );
          return;
      }
  }
  ```

  [bnjjj]: https://github.com/bnjjj
  [pull/197]: https://github.com/apollographql/apollo-rs/pull/197
# [0.2.4](https://crates.io/crates/apollo-parser/0.2.4) - 2022-03-07
## Fixes
- **correctly parse Arguments Definition - [bnjjj], [pull/187] closes [issue/186]**

  `apollo-parser` was creating ARGUMENTS instead of ARGUMENTS_DEFINITION nodes
  when parsing Arguments Definitions. This change fixes the incorrect parsing
  and allows to iterate over arguments definitions returned by the AST.

  [bnjjj]: https://github.com/bnjjj
  [pull/187]: https://github.com/apollographql/apollo-rs/pull/187
  [issue/186]: https://github.com/apollographql/apollo-rs/issues/186

- **Add STRING_VALUE node to DESCRIPTION - [bnjjj], [pull/188] closes [issue/185]**

  DESCRIPTION nodes are composed of STRING_VALUE nodes. The description string
  was previously simply added to the DESCRIPTION node which was not spec
  compliant.

  [bnjjj]: https://github.com/bnjjj
  [pull/188]: https://github.com/apollographql/apollo-rs/pull/188
  [issue/185]: https://github.com/apollographql/apollo-rs/issues/185

- **Schema Definition has a description - [bnjjj], [pull/188] closes [issue/185]**

  `apollo-parser` was parsing descriptions in Schema Definitions, but the
  graphql ungrammar did not account for a description node. This updates the
  ungrammar, and provides an accessor method to Schema Definition's description.

  [bnjjj]: https://github.com/bnjjj
  [pull/188]: https://github.com/apollographql/apollo-rs/pull/188
  [issue/185]: https://github.com/apollographql/apollo-rs/issues/185

- **Add `repeatable` keyword to GraphQL ungrammar - [bnjjj], [pull/189]**

  `repeatable` keyword was not able to be accessed programmatically from the
  parsed AST for Directive Definitions, this is now fixed.

  [bnjjj]: https://github.com/bnjjj
  [pull/189]: https://github.com/apollographql/apollo-rs/pull/189

# [0.2.3](https://crates.io/crates/apollo-parser/0.2.3) - 2022-02-17
## Features
- **expose Lexer as a pub struct - [bnjjj], [pull/168]**

  The `Lexer` in `apollo-parser` is now a publicly available interface.

  ```rust
  use apollo_parser::Lexer;

  let query = "
  {
      animal
      ...snackSelection
      ... on Pet {
        playmates {
          count
        }
      }
  }
  ";
  let lexer = Lexer::new(query);
  assert_eq!(lexer.errors().len(), 0);

  let tokens = lexer.tokens();
  ```

  [bnjjj]: https://github.com/bnjjj
  [pull/168]: https://github.com/apollographql/apollo-rs/pull/168

## Fixes
- **add a getter for Directives in Variable Definitions - [lrlna], [pull/172]**

  While the parser was correctly parsing and accounting for directives in a
  variable definition, the getter for Directives in VariableDefinition type in the
  AST was missing. This commit makes an addition to the graphql ungrammar, and by
  extension the generated AST nodes API.

  [lrlna]: https://github.com/lrlna
  [pull/172]: https://github.com/apollographql/apollo-rs/pull/172

# [0.2.2](https://crates.io/crates/apollo-parser/0.2.2) - 2022-02-11
## Fixes
- **create an error when description preceeds operation definition and proceed parsing - [MidasLamb], [pull/158]/ [lrlna], [pull/160]**

  According to the spec Operation Definitions don't currently allow for
  descriptions.

  ```graphql
  "this description is not allowed"
  {
    name
    age
  }
  ```

  When a description was added before an operation, the parser
  would continuously try to register the error without removing it from the list
  of valid tokens. This fix removes the incorrect token, and continuous parsing
  an OperationDefinition.

  [MidasLamb]: https://github.com/MidasLamb
  [lrlna]: https://github.com/lrlna
  [pull/158]: https://github.com/apollographql/apollo-rs/pull/158
  [pull/160]: https://github.com/apollographql/apollo-rs/pull/160

- **Correctly parse an Inline Fragment when type condition is absent - [bnjjj], [pull/164]**

  The following inline fragment would previously be incorrectly parsed as a FragmentSpread when in reality it's an Inline Fragment:
  ```graphql
  query HeroForEpisode {
    ... @tag(name: "team-customers") { # an inline fragment
      primaryFunction
    }
  }
  ```

  This has now been fixed.

  [bnjjj]: https://github.com/bnjjj
  [pull/164]: https://github.com/apollographql/apollo-rs/pull/164
# [0.2.1](https://crates.io/crates/apollo-parser/0.2.1) - 2022-01-26
## Fixes
- **fix(apollo-parser): add ignored tokens to TYPE nodes in correct place - [lrlna], [issue/143] [pull/153]**

  This fixes the location of ignored tokens (COMMA, WHITESPACE) inside a TYPE node.

  Before this commit this sort of query

  ```graphql
  mutation MyMutation($custId: Int!, $b: String) {
    myMutation(custId: $custId)
  }
  ```

  would result the `ast.document.to_string()` to have this output:

  ```graphql
  mutation MyMutation($custId: , Int!$b:  String) {
      myMutation(custId: $custId)
  }
  ```

  which is incorrect. The `to_string()` now results in the exact same output, as
  the AST created is correct.

  [lrlna]: https://github.com/lrlna
  [issue/143]: https://github.com/apollographql/apollo-rs/issues/143
  [pull/153]: https://github.com/apollographql/apollo-rs/pull/153

- **fix(apollo-parser): bump BANG token when creating NON_NULL_TYPE - [lrlna], [issue/142] [pull/146]**

  We are missing BANG token in the AST when a NON_NULL_TYPE gets created.
  Although the node created is indeed NON_NULL_TYPE, it's also important to keep
  the original set of tokens. The followin example now works:

  ```rust
  let mutation = r#"
  mutation MyMutation($custId: Int!) {
    myMutation(custId: $custId)
  }"#;

  let parser = Parser::new(mutation);
  let ast = parser.parse();
  assert_eq!(ast.errors.len(), 0);

  let doc = ast.document();
  assert_eq(&doc, &mutation);
  ```

  [lrlna]: https://github.com/lrlna
  [issue/142]: https://github.com/apollographql/apollo-rs/issues/142
  [pull/146]: https://github.com/apollographql/apollo-rs/pull/146

# [0.2.0](https://crates.io/crates/apollo-parser/0.2.0) - 2021-12-22
## Breaking
- **impl Iterator for ast.errors() - [o0Ignition0o], [issue/119] [pull/120]**

  `ast.errors()` now return an Iterator. This makes it a bit easier for users to process any errors returned by the Parser. Below is the new usage example:

  ```rust

  let query = r#"
  type Query {
      "A simple type for getting started!"
      hello: String
      cats(cat: [String]! = ): [String]!
  }"#;


  let parser = Parser::new(&query);
  let ast = parser.parse();

  assert!(ast.errors.len(), 1);

  for err in ast.errors() { // no longer need to .iter() on this
      // process errors in a way that's useful for your implementation
      dbg!(&err);
  }
  ```

  [o0Ignition0o]: https://github.com/o0Ignition0o
  [issue/119]: https://github.com/apollographql/apollo-rs/issues/119
  [pull/120]: https://github.com/apollographql/apollo-rs/pull/120

## Fixes
- **fix: properly create TYPE's NAMED_TYPE, LIST_TYPE, NON_NULL_TYPE - [lrlna], [issue/125] [pull/127]**

  Whenever a NAMED_TYPED, LIST_TYPE, NON_NULL_TYPE are created, they are
  automatically get created as part of the TYPE node, so we do not need to start
  it manually. This fix makes it possible to once again do:

  ```rust
  if let ast::Type::NamedType(name) = var.ty().unwrap() {
      assert_eq!(name.name().unwrap().text().as_ref(), "Int")
  }
  ```
  [lrlna]: https://github.com/lrlna
  [issue/125]: https://github.com/apollographql/apollo-rs/issues/125
  [pull/127]: https://github.com/apollographql/apollo-rs/pull/127

- **fix: create an error when SelectionSet is empty in operation definition - [lrlna], [pull/134]**

  An Operation Definition must have a selection set with values, so this query
  `query {}` should also come with an error.

  [lrlna]: https://github.com/lrlna
  [pull/134]: https://github.com/apollographql/apollo-rs/pull/134

- **fix: variable definition can have a LIST_TYPE - [lrlna], [issue/131] [pull/135]**

  Variable definition was previously not accepting a LIST_TYPE, which is
  incorrect. This commit fixes this issue.

  [lrlna]: https://github.com/lrlna
  [issue/131]: https://github.com/apollographql/apollo-rs/issues/131
  [pull/135]: https://github.com/apollographql/apollo-rs/pull/135

## Maintenance
- **chore: typo in README - [lrlna], [c598d3]**

  [lrlna]: https://github.com/lrlna
  [c598d3]: https://github.com/apollographql/apollo-rs/commit/c598d33bc9e80f767804bf5a88a7a1d6f400e832

- **fuzzing for apollo-parser - [Geal], [pull/122]**

  The fuzz test checks for lexer and parser errors and stops early.

  The following fuzz-encountered errors are fixed:
  - panics on the following input:
  ```
  "
  ```
  - crash on partial block string opening token
  ```
  ""
  ```
  - infinite loop on unfollowed 'extend' ident

  The parser fuzzer catches errors in the lexer and returns early. It
  will not avoid infinite loops and running out of memory in the lexer.

  [Geal]: https://github.com/Geal
  [pull/122]: https://github.com/apollographql/apollo-rs/pull/122

- **chore: run clippy in CI on benchmark directories - [lrlna], [pull/123]**

  [lrlna]: https://github.com/lrlna
  [pull/123]: https://github.com/apollographql/apollo-rs/pull/123

- **chore: add tests for untermiated strings and invalid type system extensions - [lrlna], [pull/124]**

  Follows up on [#122] and adds tests for the incorrectly lexed and parsed
  inputs that fuzzing discovered.

  This commit also changes logic around having an "unexpected end of data" for
  `""` string. This now gets lexed into a `StringValue` token.

  [lrlna]: https://github.com/lrlna
  [pull/124]: https://github.com/apollographql/apollo-rs/pull/124
  [#122]: https://github.com/apollographql/apollo-rs/pull/122

- **chore: allow dead code in xtask's ast_src - [lrlna], [pull/128]**

  [lrlna]: https://github.com/lrlna
  [pull/128]: https://github.com/apollographql/apollo-rs/pull/128

- **chore: add a test for nested SELECTION_SETs - [lrlna], [pull/137]**

  This will mostly act as an example in case users are looking for how to work
  with nested selections and get their FIELD/INLINE_FRAGMENT/FRAGMENT_SPREAD.

  [lrlna]: https://github.com/lrlna
  [pull/137]: https://github.com/apollographql/apollo-rs/pull/137