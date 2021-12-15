use std::collections::HashMap;
#[allow(unused_imports)]
use std::{
    cmp::Ordering,
    fmt::{self, Display},
};

#[allow(unused_imports)]
use convert_case::{Case, Casing};
#[allow(unused_imports)]
use itertools::Itertools;
use petgraph::{
    graph::NodeIndex,
    visit::{Reversed, Topo},
};
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use crate::{utils, Typing, SETTINGS};
use crate::{
    Interface, InterfaceEntry, Namespace, NamespaceEntry, NamespaceEntryValue, TypingNode,
    TypingUnion,
};

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub enum Strategy {
    Family,
    Tree,
}

pub trait TypingStrategy {
    fn fmt_typing(typing: &Typing) -> String;
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
struct Family;
impl TypingStrategy for Family {
    fn fmt_typing(_typing: &Typing) -> String {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
struct Tree;
impl TypingStrategy for Tree {
    fn fmt_typing<'a>(typing: &Typing) -> String {
        let graph = &typing.graph;
        let graph_adaptor = Reversed(graph);
        let mut topo = Topo::new(&graph_adaptor);

        let mut node_value = HashMap::new();
        let mut node_namespace = HashMap::new();
        while let Some(nx) = topo.next(&graph_adaptor) {
            match &graph[nx] {
                TypingNode::Null => {
                    node_value.insert(nx, String::from("null"));
                }
                TypingNode::Boolean => {
                    node_value.insert(nx, String::from("boolean"));
                }
                TypingNode::Number => {
                    node_value.insert(nx, String::from("number"));
                }
                TypingNode::String => {
                    node_value.insert(nx, String::from("string"));
                }
                TypingNode::Array { object_node, key } => {
                    let object_name = graph[*object_node].as_object().unwrap();
                    let values: Vec<_> = graph
                        .neighbors(nx)
                        .filter_map(|n_nx| node_value.remove(&n_nx).map(|v| (v, n_nx)))
                        .map(|(value, n_nx)| {
                            if node_namespace.contains_key(&n_nx) {
                                let namespace_value = format!("{}.{}", object_name, value);
                                let o_namespace =
                                    format!("{}", node_namespace.remove(&n_nx).unwrap());
                                node_namespace
                                    .entry(*object_node)
                                    .or_insert(Namespace::new(object_name))
                                    .entries
                                    .push(NamespaceEntry::new(
                                        &value,
                                        NamespaceEntryValue::Namespace(o_namespace),
                                    ));

                                namespace_value
                            } else {
                                value
                            }
                        })
                        .collect();

                    node_value.insert(
                        nx,
                        match values.len() {
                            0 => String::from("unknown"),
                            1 => values.into_iter().nth(0).unwrap(),
                            _ => {
                                let key = key.to_case(Case::Pascal);
                                let namespace = node_namespace
                                    .entry(*object_node)
                                    .or_insert(Namespace::new(object_name));

                                let typing_union = TypingUnion::new(values);
                                let namespace_value =
                                    NamespaceEntryValue::Alias(typing_union.to_string("    "));
                                namespace
                                    .entries
                                    .push(NamespaceEntry::new(&key, namespace_value));

                                format!("Array<{}.{}>", object_name, key)
                            }
                        },
                    );
                }
                TypingNode::Literal(l) => {
                    node_value.insert(nx, format!("{}", l));
                }
                TypingNode::Object(name) => {
                    let mut interface = Interface::new(name, None);
                    interface.entries = graph
                        .neighbors(nx)
                        .map(|n_nx| {
                            let (key, optional) = graph[n_nx].as_object_entry().unwrap();
                            let value = node_value.remove(&n_nx).unwrap();

                            InterfaceEntry {
                                key: String::from(key),
                                optional,
                                value,
                            }
                        })
                        .collect();

                    let namespace = node_namespace.entry(nx).or_insert(Namespace::new(name));
                    namespace.interface = interface;

                    node_value.insert(nx, name.clone());
                }
                TypingNode::ObjectEntry {
                    key,
                    optional: _,
                    object_node,
                } => {
                    let object_name = graph[*object_node].as_object().unwrap();
                    let values: Vec<_> = graph
                        .neighbors(nx)
                        .filter_map(|n_nx| node_value.remove(&n_nx).map(|v| (v, n_nx)))
                        .map(|(value, n_nx)| {
                            if node_namespace.contains_key(&n_nx) {
                                let namespace_value = format!("{}.{}", object_name, value);
                                let o_namespace =
                                    format!("{}", node_namespace.remove(&n_nx).unwrap());
                                node_namespace
                                    .entry(*object_node)
                                    .or_insert(Namespace::new(object_name))
                                    .entries
                                    .push(NamespaceEntry::new(
                                        &value,
                                        NamespaceEntryValue::Namespace(o_namespace),
                                    ));

                                namespace_value
                            } else {
                                value
                            }
                        })
                        .collect();

                    node_value.insert(
                        nx,
                        match values.len() {
                            1 => values.into_iter().nth(0).unwrap(),
                            _ => {
                                let key = key.to_case(Case::Pascal);
                                let namespace = node_namespace
                                    .entry(*object_node)
                                    .or_insert(Namespace::new(object_name));

                                let typing_union = TypingUnion::new(values);
                                let namespace_value =
                                    NamespaceEntryValue::Alias(typing_union.to_string("    "));
                                namespace
                                    .entries
                                    .push(NamespaceEntry::new(&key, namespace_value));

                                format!("{}.{}", object_name, key)
                            }
                        },
                    );
                }
            };
        }

        format!("{}\n", node_namespace.remove(&NodeIndex::new(0)).unwrap())
    }
}

impl Strategy {
    pub fn to_string(self, typing: &Typing) -> String {
        match self {
            Self::Tree => Tree::fmt_typing(typing),
            Self::Family => Family::fmt_typing(typing),
        }
    }
}
