pub struct Node {
    key: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: u32) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

pub struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn with_root(key: u32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    /// Adds a child to the node with `parent_id` and returns the id of the new node.
    /// The new node has the specified `key`. The new node is the left  child of the  
    /// node `parent_id` iff `is_left` is `true`, the right child otherwise.
    ///
    /// # Panics
    /// Panics if the `parent_id` does not exist, or if the node `parent_id ` has  
    /// the child already set.
    pub fn add_node(&mut self, parent_id: usize, key: u32, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert!(
                self.nodes[parent_id].id_left == None,
                "Parent node has the left child already set"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right == None,
                "Parent node has the right child already set"
            );
        }

        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);

        child_id
    }

    /// Returns the sum of all the keys in the tree
    pub fn sum(&self) -> u32 {
        self.rec_sum(Some(0))
    }

    /// A private recursive function that computes the sum of
    /// nodes in the subtree rooted at `node_id`.
    fn rec_sum(&self, node_id: Option<usize>) -> u32 {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let sum_left = self.rec_sum(node.id_left);
            let sum_right = self.rec_sum(node.id_right);

            return sum_left + sum_right + node.key;
        }
        0
    }

    pub fn is_bst(&self) -> bool {
        self.is_bst_in_range(Some(0), None, None)
    }

    // use (min, max) intervals to if it really is a BST
    fn is_bst_in_range(&self, node_id: Option<usize>, min: Option<u32>, max: Option<u32>) -> bool {
        if let Some(id) = node_id {
            let node = &self.nodes[id];

            // Check if node.key is in the range (min, node.key)
            if let Some(min_val) = min {
                if node.key <= min_val {
                    return false;
                }
            }

            // Check if node.key is in the range (node.key, max)
            if let Some(max_val) = max {
                if node.key >= max_val {
                    return false;
                }
            }

            // Recursive
            let is_left_bst = self.is_bst_in_range(node.id_left, min, Some(node.key));
            let is_right_bst = self.is_bst_in_range(node.id_right, Some(node.key), max);

            return is_left_bst && is_right_bst;
        }

        // Base case
        true
    }

    pub fn max_path_sum(&self) -> u32 {
        let mut global_max_sum = u32::MIN;
        self.rec_max_path_sum(Some(0), &mut global_max_sum);
        global_max_sum
    }

    fn rec_max_path_sum(&self, node_id: Option<usize>, global_max_sum: &mut u32) -> u32 {
        if let Some(id) = node_id {
            let node = &self.nodes[id];

            // No child nodes then return the key
            if node.id_left.is_none() && node.id_right.is_none() {
                return node.key;
            }

            // Find the maximum path sum in the left subtree
            let left_sum = if let Some(left_id) = node.id_left {
                self.rec_max_path_sum(Some(left_id), global_max_sum)
            } else {
                u32::MIN
            };

            // Find the maximum path sum in the right subtree
            let right_sum = if let Some(right_id) = node.id_right {
                self.rec_max_path_sum(Some(right_id), global_max_sum)
            } else {
                u32::MIN
            };

            // If both children exist, update the global maximum with the sum passing through the current node
            if node.id_left.is_some() && node.id_right.is_some() {
                *global_max_sum = (*global_max_sum).max(left_sum + right_sum + node.key);
            }

            // Return the maximum path sum going through either left or right child (plus current node's key)
            return left_sum.max(right_sum) + node.key;
        }

        // If node is None, return a very small value (since we're looking for maximum)
        u32::MIN
    }
}