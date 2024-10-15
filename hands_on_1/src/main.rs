mod tree;

use tree::Tree;

fn main() {
    println!("Hello, world!");
}

mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let mut tree_1 = Tree::with_root(10);

        assert_eq!(tree_1.sum(), 10);

        tree_1.add_node(0, 5, true); // id 1
        tree_1.add_node(0, 22, false); // id 2

        assert_eq!(tree_1.sum(), 37);

        tree_1.add_node(1, 7, false); // id 3
        tree_1.add_node(2, 20, true); // id 4

        assert_eq!(tree_1.sum(), 64);

        let mut tree_2 = Tree::with_root(8); // id 0
        tree_2.add_node(0, 4, true); // id 1
        tree_2.add_node(0, 20, false); // id 2
        tree_2.add_node(1, 3, true); // id 3
        tree_2.add_node(2, 26, false); // id 4
        tree_2.add_node(1, 7, false); // id 5
        tree_2.add_node(2, 16, true); // id 6
        tree_2.add_node(6, 17, false); // id 7
        tree_2.add_node(7, 19, false); // id 8
        tree_2.add_node(8, 18, true); // id 9
        tree_2.add_node(5, 6, true); // id 10

        let mut tree_3 = Tree::with_root(8); // id 0
        tree_3.add_node(0, 4, true); // id 1
        tree_3.add_node(0, 20, false); // id 2
        tree_3.add_node(1, 3, true); // id 3
        tree_3.add_node(2, 26, false); // id 4
        tree_3.add_node(1, 7, false); // id 5
        tree_3.add_node(2, 16, true); // id 6
        tree_3.add_node(6, 17, false); // id 7
        tree_3.add_node(7, 19, false); // id 8
        tree_3.add_node(8, 18, true); // id 9
        tree_3.add_node(5, 8, true); // id 10

        let mut tree_4 = Tree::with_root(8); // id 0
        tree_4.add_node(0, 4, true); // id 1
        tree_4.add_node(0, 20, false); // id 2
        tree_4.add_node(1, 3, true); // id 3
        tree_4.add_node(2, 26, false); // id 4
        tree_4.add_node(1, 7, false); // id 5
        tree_4.add_node(2, 16, true); // id 6
        tree_4.add_node(6, 17, false); // id 7
        tree_4.add_node(7, 19, false); // id 8
        tree_4.add_node(8, 18, false); // id 9
        tree_4.add_node(5, 6, true); // id 10

        assert_eq!(tree_2.sum(), 144);
        assert_eq!(tree_3.sum(), 146);
        assert_eq!(tree_4.sum(), 144);

        assert_eq!(tree_1.is_bst(), true);
        assert_eq!(tree_2.is_bst(), true);
        assert_eq!(tree_3.is_bst(), false);
        assert_eq!(tree_4.is_bst(), false);

        assert_eq!(tree_2.max_path_sum(), 116);
        assert_eq!(tree_3.max_path_sum(), 117);
        assert_eq!(tree_4.max_path_sum(), 116);
    }
}
