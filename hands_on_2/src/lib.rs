struct SegmentTree {
    n: u32,
    t: Vec<u32>,
}

impl SegmentTree {

    fn new(a: &[u32]) -> Self {
        let n = a.len();
        let mut t = vec![0; 4 * n];
        let mut seg_tree = SegmentTree {n, tree};
        seg_tree.build(a, 1, 0, n - 1);
        seg_tree
    }

    fn build(&mut self, a: &[u32], v: u32, tl: u32, tr: u32) {
        if tl == tr {
            self.t[v] = a[tl];
        } else {
            let tm = (tl + tr) / 2;
            self.build(a, v * 2, tl, tm);
            self.build(a, v * 2 + 1, tm + 1, tr);
            self.t[v] = self.t[v * 2] + self.t[v * 2 + 1];
        }
    }
}