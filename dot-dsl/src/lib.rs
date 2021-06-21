pub mod graph {
    use std::collections::HashMap;

    use self::graph_items::{edge::Edge, node::Node};

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
                attrs: HashMap::with_capacity(1),
            }
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            self.nodes.extend_from_slice(nodes);

            self
        }

        pub fn get_node(&self, node_value: &str) -> Option<Node> {
            self.nodes.iter().find(|n| n.value == node_value).map(|n| n.clone())
        }

        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            self.edges.extend_from_slice(edges);

            self
        }

        pub fn with_attrs(mut self, attrs: &[(&'static str, &'static str)]) -> Self {
            attrs.iter().for_each(|(k, v)| {
                self.attrs.insert(k.to_string(), v.to_string());
            });

            self
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq, Eq)]
            pub struct Edge {
                first_value: &'static str,
                second_value: &'static str,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(a: &'static str, b: &'static str) -> Self {
                    Self {
                        first_value: a,
                        second_value: b,
                        attrs: HashMap::with_capacity(1),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&'static str, &'static str)]) -> Self {
                    attrs.iter().for_each(|(k, v)| {
                        self.attrs.insert(k.to_string(), v.to_string());
                    });

                    self
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq, Eq)]
            pub struct Node {
                pub value: &'static str,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(a: &'static str) -> Self {
                    Self {
                        value: a,
                        attrs: HashMap::with_capacity(1),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&'static str, &'static str)]) -> Self {
                    attrs.iter().for_each(|(k, v)| {
                        self.attrs.insert(k.to_string(), v.to_string());
                    });

                    self
                }
                
                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|s| s.as_str())
                }
            }
        }
    }
}
