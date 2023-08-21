//! Put everything in one file as suggested by the given starting state.

pub mod graph {
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;
    use std::iter::FromIterator;

    /// Converts the given slice of couples of `&str`s into a `HashMap` where the keys are the
    /// firsts and the values are the seconds.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// # use dot_dsl::graph::collect_attrs;
    /// # use maplit::hashmap;
    /// assert_eq!(collect_attrs(&vec![("hello", "world"),
    ///                                ("good", "day")]),
    ///            hashmap!{
    ///                "hello".to_string() => "world".to_string(),
    ///                "good".to_string() => "day".to_string(),
    ///            });
    /// ```
    pub fn collect_attrs(attrs: &[(&str, &str)]) -> HashMap<String, String> {
        HashMap::from_iter(
            attrs
                .iter()
                .map(|&(key, value)| (key.to_string(), value.to_string())),
        )
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = collect_attrs(attrs);
            self
        }

        pub fn node(&self, id: &str) -> Option<&Node> {
            self.nodes.iter().find(|&node| node.id == id)
        }
    }

    pub mod graph_items {
        use super::*;

        pub mod node {
            use super::*;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Node {
                pub id: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(id: &str) -> Self {
                    Self {
                        id: id.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = collect_attrs(attrs);
                    self
                }

                pub fn attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).map(String::as_str)
                }
            }
        }

        pub mod edge {
            use super::*;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge {
                start_id: String,
                end_id: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(start_id: &str, end_id: &str) -> Self {
                    Self {
                        start_id: start_id.to_string(),
                        end_id: end_id.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = collect_attrs(attrs);
                    self
                }

                pub fn attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).map(String::as_str)
                }
            }
        }
    }
}
