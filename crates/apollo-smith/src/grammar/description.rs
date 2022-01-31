// use crate::{Parser, SyntaxKind};

use apollo_encoder::Schema;
use arbitrary::Unstructured;

use crate::{Node, SchemaBuilder};

use super::name::limited_string;

/// See: https://spec.graphql.org/October2021/#Description
///
/// *Description*:
///     StringValue
pub(crate) fn description(
    u: &mut Unstructured,
    builder: &mut SchemaBuilder,
    // It's maybe better to return Node::Description
) -> Node {
    Node::Description(limited_string(300, u))
    // Create description

    // let _g = p.start_node(SyntaxKind::DESCRIPTION);
    // p.bump(SyntaxKind::STRING_VALUE)
}
