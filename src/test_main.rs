#[cfg(test)]
mod test_solutions {
    use crate::solution;

    #[test]
    fn test_solution_part_1() {
        assert_eq!(solution("test_data.txt", 2020), 436)
    }

    #[test]
    fn test_solution_part_2() {
        assert_eq!(solution("test_data.txt", 30000000), 175594)
    }
}