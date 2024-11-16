mod max_segmenttree;
use max_segmenttree::MaxSegmentTree;
use max_segmenttree::SegmentTree;

use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    // Read n and m
    let first_line = input.next().unwrap().unwrap();
    let parts: Vec<usize> = first_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let n = parts[0];
    let m = parts[1];

    // Read array A
    let second_line = input.next().unwrap().unwrap();
    let a: Vec<i32> = second_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Initialize the segment tree
    let mut seg_tree = MaxSegmentTree::new(&a);

    // Process each query
    for _ in 0..m {
        let line = input.next().unwrap().unwrap();
        let query: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();

        if query[0] == 0 {
            // Update query: 0 i j T
            let i = query[1] as usize;
            let j = query[2] as usize;
            let t = query[3];
            seg_tree.range_update(i, j, t);
        } else if query[0] == 1 {
            // Max query: 1 i j
            let i = query[1] as usize;
            let j = query[2] as usize;
            let result = seg_tree.range_max(i, j);
            println!("{}", result);
        }
    }
    // Task 2 main
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let first_line = lines.next().unwrap().unwrap();
    let parts: Vec<usize> = first_line.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let n = parts[0];
    let m = parts[1];

    let second_line = lines.next().unwrap().unwrap();
    let count: Vec<i32> = second_line.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut seg_tree = SegmentTree::new(n);
    seg_tree.build(1, 0, n - 1, &count);

    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let query: Vec<usize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        if query[0] == 0 {
            // Update query: 0 i j delta
            let (i, j, delta) = (query[1], query[2], query[3] as i32);
            seg_tree.update(1, 0, n - 1, i, j, delta);
        } else {
            // IsThere query: 1 i j k
            let (i, j, k) = (query[1], query[2], query[3]);
            if seg_tree.query(1, 0, n - 1, i, j, k) {
                println!("1");
            } else {
                println!("0");
            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn run_handmade_tests(){
        let a1 = vec![1, 3, 5, 7, 9, 11];
        let mut stree1 = MaxSegmentTree::new(&a1);
        assert_eq!(9,stree1.range_max(1,5));

        stree1.range_update(1, 3, 4);
        assert_eq!(4,stree1.range_max(1,3));
    }

    #[test]
    fn run_testset_01() {
        for i in 1..=10 {
            let input_file = format!("testset_01/input{:}.txt", i);
            let output_file = format!("testset_01/output{:}.txt", i);
    
            // Read input and output from files
            let input: String = fs::read_to_string(&input_file)
                .expect(&format!("Failed to read from {}", input_file));
            let expected_output = fs::read_to_string(&output_file)
                .expect(&format!("Failed to read from {}", output_file));
    
            // Split input into lines
            let mut input_lines = input.lines();
    
            // Read n and m
            let first_line = input_lines.next().unwrap();
            let parts: Vec<usize> = first_line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            let n = parts[0];
            let m = parts[1];
    
            // Read array A
            let second_line = input_lines.next().unwrap();
            let a: Vec<i32> = second_line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
    
            // Initialize the segment tree
            let mut seg_tree = MaxSegmentTree::new(&a);
    
            // Process each query and collect output
            let mut output = String::new();
            for _ in 0..m {
                let line = input_lines.next().unwrap();
                let query: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
    
                if query[0] == 0 {
                    // Update query: 0 i j T
                    let i = query[1] as usize;
                    let j = query[2] as usize;
                    let t = query[3];
                    seg_tree.range_update(i, j, t);
                } else if query[0] == 1 {
                    // Max query: 1 i j
                    let i = query[1] as usize;
                    let j = query[2] as usize;
                    let result = seg_tree.range_max(i, j);
                    output.push_str(&format!("{}\n", result));
                }
            }
    
            // Compare output with expected output
            let output = output.trim();
            let expected_output = expected_output.trim();
    
            if output == expected_output {
                println!("Test case {}: Passed", i);
            } else {
                println!("Test case {}: Failed", i);
                println!("Expected:\n{}", expected_output);
                println!("Got:\n{}", output);
            }
        }
    }

    #[test]
    fn run_testset_02() {
        let mut all_tests_passed = true;
    
        for i in 0..8 {
            let input_file = format!("testset_02/input{}.txt", i);
            let output_file = format!("testset_02/output{}.txt", i);
    
            // Read input and expected output from files
            let input = fs::read_to_string(&input_file)
                .expect(&format!("Failed to read from {}", input_file));
            let expected_output = fs::read_to_string(&output_file)
                .expect(&format!("Failed to read from {}", output_file))
                .trim() // Trim to remove extra newlines
                .to_string();
    
            // Split input into lines
            let mut input_lines = input.lines();
            let first_line = input_lines.next().unwrap();
            let first_line_parts: Vec<usize> = first_line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
    
            let n = first_line_parts[0];
            let m = first_line_parts[1];
    
            // Parse segments
            let mut segments = vec![];
            for _ in 0..n {
                let segment: Vec<usize> = input_lines
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();
                segments.push((segment[0], segment[1]));
            }
    
            let mut seg_tree = SegmentTree::new(n);
    
            // Build initial counts based on segments
            let mut count = vec![0; n];
            for (l, r) in &segments {
                for i in *l..=*r {
                    count[i] += 1;
                }
            }
            seg_tree.build(1, 0, n - 1, &count);
    
            // Process queries
            let mut output = Vec::new();
            for _ in 0..m {
                let query: Vec<usize> = input_lines
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();
                let (i, j, k) = (query[0], query[1], query[2]);
                if seg_tree.query(1, 0, n - 1, i, j, k) {
                    output.push("1".to_string());
                } else {
                    output.push("0".to_string());
                }
            }
    
            // Compare output with expected output
            let output_string = output.join("\n");
            if output_string == expected_output {
                println!("Test {} passed.", i);
            } else {
                println!("Test {} failed.", i);
                all_tests_passed = false;
            }
        }
    
        if all_tests_passed {
            println!("All tests passed!");
        } else {
            println!("Some tests failed.");
        }
    }
}

