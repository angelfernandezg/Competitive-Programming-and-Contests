mod max_segmenttree;
use max_segmenttree::MaxSegmentTree;

fn main() {
    let a = vec![1, 3, 5, 7, 9, 11];
    let mut seg_tree = MaxSegmentTree::new(&a);

    println!("Max of range [1, 3]: {}", seg_tree.range_max(1, 5));

    seg_tree.range_update(1, 3, 4);

    println!("Max of range [1, 3] after update: {}", seg_tree.range_max(1, 3));

    println!("{:?}", seg_tree);
}

mod tests {
    use super::*;

    #[test]
    fn run_handmade_tests(){
        let a1 = vec![1,5,1];
        let mut stree1 = MaxSegmentTree::new(&a1);
        stree1.range_update(1,1,6);
        assert_eq!(5,stree1.range_max(1,2));

        let a2: Vec<i32> = vec![1,2,3,4,5];
        let mut stree2: MaxSegmentTree = MaxSegmentTree::new(&a2);
        //assert_eq!(5,stree1.range_max(1,3));
    }
}