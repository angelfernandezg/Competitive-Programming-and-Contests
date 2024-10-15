// greedely assing bigger frequencies to larger items

pub fn little_girl_sum<T>(list: &[T], queries: &[(usize, usize)]) -> T
where
    T: Ord + Copy + Mul<i32, Output = T> + Default + AddAssign,
{
    let mut sorted = list.to_vec();


}