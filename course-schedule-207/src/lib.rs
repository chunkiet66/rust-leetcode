#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        let num_courses = 2;
        let prerequisites = vec!(vec!(1, 0));
        assert!(Solution::can_finish(num_courses, prerequisites));

        let num_courses = 2;
        let prerequisites = vec!(vec!(1, 0), vec!(0, 1));
        assert_eq!(Solution::can_finish(num_courses, prerequisites), false);

        let num_courses = 1;
        let prerequisites = vec!();
        assert_eq!(Solution::can_finish(num_courses, prerequisites), true);
    }
}

use std::collections::HashMap;

pub struct Solution{
}

impl Solution {

    //use indegree to test, check https://www.youtube.com/watch?v=rG2-_lgcZzo&t
    pub fn can_finish_indegree(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut indegrees : Vec<i32> = vec!(0; num_courses as usize);

        for pre in prerequisites {
            graph.entry(pre[0]).or_insert(Vec::new()).push(pre[1]);
            indegrees[pre[1] as usize] = indegrees[pre[1] as usize]+1;
        }

        let mut stack: Vec<Vec<i32>> = Vec::new();

        let mut total = 0;
        for i in 0 ..num_courses {
            let index = i as usize;
            if indegrees[index] == 0 {
                if let Some(list) = graph.remove(&i) {
                    stack.push(list);
                }
                total = total + 1;
            }
        }

        while !stack.is_empty() {
            for adj in stack.pop().unwrap() {
                indegrees[adj as usize] = indegrees[adj as usize] -1;
                if  indegrees[adj as usize] == 0 {
                    if let Some(list) = graph.remove(&adj) {
                        stack.push(list);
                    }
                    total = total + 1;
                }
            }
        }

        total == num_courses
    }

    //This method is solved by checking if there's any cyclic in the graph
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();

        for pre in prerequisites {
            graph.entry(pre[1]).or_insert(vec!()).push(pre[0]);
        }

        let mut marks = vec![false; num_courses as usize];
        for i in 0..num_courses {
            if Solution::is_cyclic(i, &graph, &mut marks) {
                return false;
            }
        }

        true
    }

    pub fn is_cyclic(index: i32, graph: &HashMap<i32, Vec<i32>>, marks: &mut Vec<bool>) -> bool {
        if marks[index as usize] == true {
            return true;
        }

        marks[index as usize] = true;

        if let Some(list) = graph.get(&index) {
            for adj in list {
                if Solution::is_cyclic(*adj, graph, marks) {
                    return true;
                }
            }
        }

        marks[index as usize] = false;

        false
    }
}