use std::cmp::max;
use std::io::{self, BufRead};
use std::fs;
use std::cmp::Ordering;

struct HolidayPlanification {
    dp: Vec<Vec<i32>>,
}

impl HolidayPlanification {
    fn new(n: usize, d: usize) -> Self {
        Self {
            dp: vec![vec![0; d + 1]; n + 1],
        }
    }

    fn calculate_max_attractions(&mut self, city_attractions: Vec<Vec<i32>>, n: usize, d: usize) -> i32 {
        for i in 1..=n {
            for j in 1..=d {

                self.dp[i][j] = self.dp[i - 1][j]; 
                let mut sum = 0;

                for k in 1..=city_attractions[i - 1].len().min(j) {
                    sum += city_attractions[i - 1][k - 1];
                    self.dp[i][j] = max(self.dp[i][j], self.dp[i - 1][j - k] + sum);

                    //println!("i: {}, j: {}, k: {}, sum: {}, dp[{}][{}]: {}", i, j, k, sum, i, j, self.dp[i][j]);
                }
            }
        }
        self.dp[n][d]
    }
}

struct LessonPlanification {
    lessons: Vec<(i32, i32)>,
    tails: Vec<i32>,
}

impl LessonPlanification {
    fn new(n: usize) -> Self {
        Self {
            lessons: vec![(0, 0); n],
            tails: Vec::with_capacity(n + 1),
        }
    }

    fn sort_by_beauty(&mut self) -> Vec<i32> {

        //self.lessons.sort_by(|a, b| {
        //    if a.0 == b.0 {
        //        a.1.cmp(&b.1)
        //    } else {
        //        a.0.cmp(&b.0)
        //    }
        //});

        self.lessons.sort_by(|a, b| {
            if a.0 == b.0 {
                b.1.cmp(&a.1) // If same beauty the one with higher difficulty first so repetition never happens
            } else {
                a.0.cmp(&b.0)
            }
        });
        //println!("tails: {:?}", self.lessons);
        self.lessons.iter().map(|&(_, difficulty)| difficulty).collect()
    }

    fn dynamic_longest_increasing_subsequence(&mut self, difficulties: &[i32]) -> usize {
        let n = difficulties.len();
        let mut lis = vec![1; n];

        for i in (0..n - 1).rev() {
            for j in i + 1..n {
                if difficulties[i] < difficulties[j] {
                    lis[i] = lis[i].max(lis[j] + 1);
                }   
            }
        }

        let max_lis = *lis.iter().max().unwrap();
        println!("LIS array: {:?}", lis);
        max_lis
    }

    fn longest_increasing_subsequence_geeks4g(&mut self, difficulties: &[i32]) -> usize {
        for &difficulty in difficulties {
            match self.tails.binary_search(&difficulty) {
                Ok(_) => continue,
                Err(pos) => {
                    if pos < self.tails.len() {
                        self.tails[pos] = difficulty;
                    } else {
                        self.tails.push(difficulty);
                    }
                }
            }
        }
        self.tails.len()
    }

    fn longest_increasing_subsequence(&mut self, difficulties: &[i32]) -> usize {
        let mut max_len = 0;
    
        for &difficulty in difficulties {
            match self.tails.binary_search(&difficulty) {
                Ok(_) => continue,
                Err(pos) => {
                    if pos == self.tails.len() {
                        self.tails.push(difficulty);
                    } else {
                        self.tails[pos] = difficulty;
                    }
                    // Update max_len if the current length of tails is greater
                    max_len = max_len.max(self.tails.len());
                }
            }
        }
        //println!("tails: {:?}", self.tails); // Print the tails vector
        max_len
    }    
}

fn main(){
    
}

mod tests {
    use super::*;

    #[test]
    fn run_handmade_tests(){
        let city_attractions = vec![vec![3, 2, 1], vec![3, 1, 1]];
        let n = 2;
        let d = 3;

        let mut holiday_plan = HolidayPlanification::new(n, d);
        let max_attractions = holiday_plan.calculate_max_attractions(city_attractions, n, d);

        assert_eq!(max_attractions, 8);


        let mut lesson_plan = LessonPlanification::new(5);
        let lessons = [(0, 3), (99, 1), (11, 20), (1, 2), (10, 5)];
        lesson_plan.lessons = lessons.to_vec();
        let sorted_difficulties = lesson_plan.sort_by_beauty();
        let lis = lesson_plan.longest_increasing_subsequence(&sorted_difficulties);
        
        assert_eq!(lis, 3);
    }

    #[test]
    fn run_testset_01(){
        for i in 0..=4 {
            let input_file = format!("testset_01/input{:}.txt", i);
            let output_file = format!("testset_01/output{:}.txt", i);
    
            let input: String = fs::read_to_string(&input_file)
                .expect(&format!("Failed to read from {}", input_file));
            let expected_output = fs::read_to_string(&output_file)
                .expect(&format!("Failed to read from {}", output_file));
    
            let mut lines = input.lines();

            let first_line = lines.next().unwrap();
            let mut parts = first_line.split_whitespace();
            let n: usize = parts.next().unwrap().parse().unwrap();
            let d: usize = parts.next().unwrap().parse().unwrap();

            let mut city_attractions = Vec::new();
            for _ in 0..n {
                let line = lines.next().unwrap();
                let attractions: Vec<i32> = line.split_whitespace()
                                    .map(|x| x.parse().unwrap())
                                    .collect();
                city_attractions.push(attractions);
            }

            let mut holiday_plan = HolidayPlanification::new(n, d);
            let max_attractions = holiday_plan.calculate_max_attractions(city_attractions, n, d);
            
            let expected_output = expected_output.trim().parse().unwrap();
    
            if max_attractions == expected_output {
                println!("Test case {}: Passed", i);
            } else {
                println!("Test case {}: Failed", i);
                println!("Expected:\n{}", expected_output);
                println!("Got:\n{}", max_attractions);
            }
        }
    }

    #[test]
    fn run_testset_02(){

        for i in 0..=10 {
            let input_file = format!("testset_02/input{:}.txt", i);
            let output_file = format!("testset_02/output{:}.txt", i);
    
            let input: String = fs::read_to_string(&input_file)
                .expect(&format!("Failed to read from {}", input_file));
            let expected_output = fs::read_to_string(&output_file)
                .expect(&format!("Failed to read from {}", output_file));
    
            let mut lines = input.lines();
    
            let n: usize = lines.next().unwrap().parse().unwrap();
            let mut lesson_plan = LessonPlanification::new(n);
    
            for i in 0..n {
                let line = lines.next().unwrap();
                let mut parts = line.split_whitespace();
                let beauty: i32 = parts.next().unwrap().parse().unwrap();
                let difficulty: i32 = parts.next().unwrap().parse().unwrap();
                lesson_plan.lessons[i as usize] = (beauty, difficulty);
            }
    
            let sorted_difficulties = lesson_plan.sort_by_beauty();
            let maximun_selected_topics = lesson_plan.longest_increasing_subsequence(&sorted_difficulties);
            
            let expected_output = expected_output.trim().parse().unwrap();
    
            if maximun_selected_topics == expected_output {
                println!("Test case {}: Passed", i);
            } else {
                println!("Test case {}: Failed", i);
                println!("Expected:\n{}", expected_output);
                println!("Got:\n{}", maximun_selected_topics);
            }
        }
    }
} 