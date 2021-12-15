use std::hash::Hash;

use convert_case::{Case, Casing};
use ijson::{Destructured, IValue, ValueType};
use itertools::Itertools;
use petgraph::{graph::NodeIndex, Graph};

use crate::{items::Items, regex, strategy::Strategy, Literal};

pub type TypingGraph = Graph<TypingNode, ()>;

/// Represents a typescript interface with a namespace associated with it
/// ## Examples
/// ```
/// use serde_json::json;
/// use json_typings::{Typing, Items};
///
/// let values = vec![
///     json!({
///         "a": true,
///         "b": 1,
///         "c": [1,2,3,4,5],
///     }),
///     json!({
///         "a": true,
///         "c": [1,2,3,4,5],
///     }),
/// ];
/// let typing = Typing::from_items("Example", values);
///
/// assert_eq!(
///     format!("{}", typing),
///     [
///         "export interface Example {",
///         "    a: boolean;",
///         "    b?: number;",
///         "    c: Array<number>;",
///         "}",
///     ]
///     .join("\n")
/// );
/// ```
#[derive(Debug, Clone)]
pub struct Typing {
    pub name: String,
    pub graph: TypingGraph,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum TypingNode {
    Null,
    Boolean,
    Number,
    String,
    Array {
        object_node: NodeIndex,
        key: String,
    },
    Literal(Literal),
    Object(String),
    ObjectEntry {
        key: String,
        optional: bool,
        object_node: NodeIndex,
    },
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct ObjectNode(pub usize);

impl Typing {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            graph: Graph::new(),
        }
    }

    /// Generates typing based on a vector of objects (Map<String, Value>)
    ///
    /// ## Arguments
    ///
    /// * `name` - Name of the interface and namespace for this typing
    /// * `values` - A vector of Map<String, Value> used to generate the typing
    pub fn from_items<'a, T: Into<Items>>(name: &str, items: T) -> Self {
        let mut typing = Self::new(name);
        let main_node = typing
            .graph
            .add_node(TypingNode::Object(String::from(name)));

        let items = items.into();

        let mut node_items: Vec<(NodeIndex, Items)> = vec![(main_node, items)];

        while let Some((node, items)) = node_items.pop() {
            for item_entry in items.entries() {
                let entry_node = typing.graph.add_node(TypingNode::ObjectEntry {
                    key: item_entry.key.clone(),
                    optional: item_entry.optional,
                    object_node: node,
                });
                typing.graph.add_edge(node, entry_node, ());

                let mut parent_values: Vec<(NodeIndex, Vec<IValue>)> = item_entry
                    .values
                    .into_iter()
                    .group_by(|v| std::mem::discriminant(v))
                    .into_iter()
                    .map(|(_, group)| (entry_node, group.into_iter().collect()))
                    .collect();

                while let Some((parent, values)) = parent_values.pop() {
                    let values = values;

                    match values[0].type_() {
                        ValueType::Null => {
                            let null_node = typing.graph.add_node(TypingNode::Null);
                            typing.graph.add_edge(parent, null_node, ());
                        }
                        ValueType::Bool => {
                            let bool_node = typing.graph.add_node(TypingNode::Boolean);
                            typing.graph.add_edge(parent, bool_node, ());
                        }
                        ValueType::Number => {
                            let number_node = typing.graph.add_node(TypingNode::Number);
                            typing.graph.add_edge(parent, number_node, ());
                        }
                        ValueType::String => {
                            let strs: Vec<_> = values
                                .iter()
                                .filter_map(|v| v.as_string())
                                .filter(|s| s.len() > 0)
                                .collect();

                            let uniques: Vec<_> = strs.iter().unique().collect();
                            let max_len = uniques.iter().map(|v| v.len()).max().unwrap_or(0);

                            let duplicates = strs.len() - uniques.len();
                            let numbers = uniques
                                .iter()
                                .filter_map(|v| regex::NUMBER.is_match(v).ok())
                                .filter(|v| *v)
                                .count();

                            match (max_len, duplicates) {
                                (1..=32, 1..) if numbers > uniques.len() / 2 => {
                                    uniques
                                        .iter()
                                        .map(|&&text| {
                                            let text = text.as_str();
                                            regex::NUMBER
                                                .find_iter(text)
                                                .filter_map(Result::ok)
                                                .fold(String::from(text), |acc, m| {
                                                    let text = m.as_str();
                                                    let replacement = match &text.chars().last() {
                                                        Some('n') => "${bigint}",
                                                        _ => "${number}",
                                                    };

                                                    acc.replace(text, replacement)
                                                })
                                        })
                                        .unique()
                                        .map(Literal::Template)
                                        .map(TypingNode::Literal)
                                        .for_each(|t| {
                                            let node = typing.graph.add_node(t);

                                            typing.graph.add_edge(parent, node, ());
                                        });
                                }
                                (1..=16, 1..) => uniques
                                    .into_iter()
                                    .map(|s| TypingNode::Literal(Literal::from_str(s)))
                                    .for_each(|t| {
                                        let typing_node = typing.graph.add_node(t);

                                        typing.graph.add_edge(parent, typing_node, ());
                                    }),
                                _ => {
                                    let string_node = typing.graph.add_node(TypingNode::String);
                                    typing.graph.add_edge(parent, string_node, ());
                                }
                            };
                        }
                        ValueType::Array => {
                            let array_node = typing.graph.add_node(TypingNode::Array {
                                object_node: node,
                                key: item_entry.key.clone(),
                            });
                            typing.graph.add_edge(parent, array_node, ());

                            values
                                .into_iter()
                                .filter_map(|v| match v.destructure() {
                                    Destructured::Array(a) => Some(a),
                                    _ => None,
                                })
                                .flatten()
                                .group_by(|v| std::mem::discriminant(&v.type_()))
                                .into_iter()
                                .for_each(|(_, group)| {
                                    parent_values.push((array_node, group.into_iter().collect()))
                                });
                        }
                        ValueType::Object => {
                            let object_name = item_entry.key.to_case(Case::Pascal);
                            let object_node =
                                typing.graph.add_node(TypingNode::Object(object_name));
                            typing.graph.add_edge(parent, object_node, ());

                            node_items.push((object_node, values.into()));
                        }
                    }
                }
            }
        }

        typing
    }

    pub fn as_string(&self, strategy: Strategy) -> String {
        strategy.to_string(self)
    }
}

impl TypingNode {
    pub fn is_object_entry(&self) -> bool {
        match self {
            TypingNode::ObjectEntry {
                key: _,
                optional: _,
                object_node: _,
            } => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            TypingNode::Array {
                object_node: _,
                key: _,
            } => true,
            _ => false,
        }
    }

    pub fn is_object(&self) -> bool {
        match self {
            TypingNode::Object(_) => true,
            _ => false,
        }
    }

    pub fn as_object(&self) -> Option<&str> {
        match self {
            Self::Object(name) => Some(name),
            _ => None,
        }
    }

    pub fn as_object_entry(&self) -> Option<(&str, bool)> {
        match self {
            Self::ObjectEntry {
                key,
                optional,
                object_node: _,
            } => Some((key, *optional)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use ijson::IArray;
    use petgraph::{dot::Dot, visit::Bfs};

    use crate::read_json;

    use super::*;

    #[test]
    fn test() {
        let value = read_json::file(&Path::new("./data/sample_a.json")).unwrap();

        let values = match value.into_array() {
            Ok(a) => a,
            Err(v) => {
                let mut a = IArray::new();
                a.push(v);

                a
            }
        };
        let typing = Typing::from_items("Typing", values);

        let graph = &typing.graph;
        let mut bfs = Bfs::new(&graph, NodeIndex::new(0));
        while let Some(nx) = bfs.next(&graph) {
            println!("{:?}", graph[nx]);
        }

        let graph = format!("{:?}", Dot::new(&typing.graph));
        std::fs::write("./data.graph", graph).unwrap();
    }
}
