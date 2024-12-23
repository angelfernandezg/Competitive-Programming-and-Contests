use std::fmt;
pub struct MaxSegmentTree {
    n: usize,
    t: Vec<i32>,
    lazy: Vec<Option<i32>>,  // Lazy propagation array
}

impl fmt::Debug for MaxSegmentTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format the segment tree array
        writeln!(f, "Segment Tree: {:?}", self.t)?;
        // Format the lazy propagation array
        writeln!(f, "Lazy Array: {:?}", self.lazy)
    }
}

impl MaxSegmentTree {
    // Constructor to initialize the Segment Tree and build it
    pub fn new(a: &[i32]) -> Self {
        let n = a.len();
        let t = vec![0; 4 * n];
        let lazy = vec![None; 4 * n];  // Initialize lazy array with None
        let mut seg_tree = MaxSegmentTree { n, t, lazy };
        seg_tree.build(a, 1, 0, n - 1);
        seg_tree
    }

    // Recursive function to build the Segment Tree
    fn build(&mut self, a: &[i32], v: usize, tl: usize, tr: usize) {
        if tl == tr {
            self.t[v] = a[tl];
        } else {
            let tm = (tl + tr) / 2;
            self.build(a, v * 2, tl, tm);
            self.build(a, v * 2 + 1, tm + 1, tr);
            self.t[v] = self.t[v * 2].max(self.t[v * 2 + 1]);
        }
    }

    // Function to push the lazy updates
    fn push(&mut self, v: usize, tl: usize, tr: usize) {
        if let Some(lazy_value) = self.lazy[v] {
            self.t[v] = self.t[v].min(lazy_value);
            if tl != tr {
                let _tm = (tl + tr) / 2;
                self.lazy[v * 2] = Some(self.lazy[v * 2].unwrap_or(i32::MAX).min(lazy_value));
                self.lazy[v * 2 + 1] = Some(self.lazy[v * 2 + 1].unwrap_or(i32::MAX).min(lazy_value));
            }
            self.lazy[v] = None;
        }
    }

    // Function to apply range update
    fn update(&mut self, v: usize, tl: usize, tr: usize, l: usize, r: usize, t: i32) {
        self.push(v, tl, tr);
        if l > r {
            return;
        }
        if l == tl && r == tr {
            self.lazy[v] = Some(t);
            self.push(v, tl, tr);
        } else {
            let tm = (tl + tr) / 2;
            self.update(v * 2, tl, tm, l, r.min(tm), t);
            self.update(v * 2 + 1, tm + 1, tr, l.max(tm + 1), r, t);
            self.t[v] = self.t[v * 2].max(self.t[v * 2 + 1]);
        }
    }

    // Function to find the maximum in the range [l, r]
    fn get_max(&mut self, v: usize, tl: usize, tr: usize, l: usize, r: usize) -> i32 {
        self.push(v, tl, tr);
        if l > r {
            return i32::MIN;
        }
        if l == tl && r == tr {
            return self.t[v];
        }
        let tm = (tl + tr) / 2;
        let left_max = self.get_max(v * 2, tl, tm, l, r.min(tm));
        let right_max = self.get_max(v * 2 + 1, tm + 1, tr, l.max(tm + 1), r);
        left_max.max(right_max)
    }

    // Public method to apply the Update(i, j, T)
    pub fn range_update(&mut self, l: usize, r: usize, t: i32) {
        self.update(1, 0, self.n - 1, l-1, r-1, t); //-1 just to make pos 1 be 1 in order to 0
    }

    // Public method to get the maximum in the range [i, j]
    pub fn range_max(&mut self, l: usize, r: usize) -> i32 {
        self.get_max(1, 0, self.n - 1, l-1, r-1) //same as in line 87
    }

    

}

//TASK 02
//Same structure but different implementation on exercise 2
pub struct SegmentTree {
    n: usize,
    tree: Vec<Vec<i32>>,  // Each node contains a vector that keeps frequency counts
    lazy: Vec<Option<i32>>, // Lazy propagation array using Option for efficient updates
}

impl SegmentTree {
    // Constructor to initialize the segment tree
    pub fn new(n: usize) -> Self {
        SegmentTree {
            n,
            tree: vec![vec![0; n + 1]; 4 * n], // +1 to cover all possible counts
            lazy: vec![None; 4 * n],          // Initialize lazy array with None
        }
    }

    // Helper function to build the tree
    pub fn build(&mut self, v: usize, tl: usize, tr: usize, count: &Vec<i32>) {
        if tl == tr {
            self.tree[v][count[tl] as usize] += 1;
        } else {
            let tm = (tl + tr) / 2;
            self.build(v * 2, tl, tm, count);
            self.build(v * 2 + 1, tm + 1, tr, count);
            self.tree[v] = self.merge(&self.tree[v * 2], &self.tree[v * 2 + 1]);
        }
    }

    // Function to merge two frequency vectors
    pub fn merge(&self, left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
        left.iter().zip(right).map(|(l, r)| l + r).collect()
    }

    // Function to push lazy updates to children
    fn push(&mut self, v: usize, tl: usize, tr: usize) {
        if let Some(lazy_value) = self.lazy[v] {
            // Apply the lazy update
            let shift = lazy_value as usize;
            let mut new_freq = vec![0; self.n + 1];
            for i in 0..=self.n {
                if i + shift <= self.n {
                    new_freq[i + shift] = self.tree[v][i];
                }
            }
            self.tree[v] = new_freq;
        }
    }

    // Range query function
    pub fn query(&mut self, v: usize, tl: usize, tr: usize, l: usize, r: usize, k: usize) -> bool {
        self.push(v, tl, tr);
        if l > r {
            return false;
        }
        if l == tl && r == tr {
            return self.tree[v][k] > 0;
        }
        let tm = (tl + tr) / 2;
        self.query(v * 2, tl, tm, l, r.min(tm), k)
            || self.query(v * 2 + 1, tm + 1, tr, l.max(tm + 1), r, k)
    }
}



