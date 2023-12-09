pub struct History
{
    values: Vec<i64>
}

fn next_value_in_sequence(values: &Vec<i64>) -> i64
{
    if values.iter().all(|v| *v == 0) { return 0 }

    let mut diff_seq = vec![];

    for i in 0..values.len()-1
    {
        diff_seq.push(values[i+1] - values[i])
    }

    values.last().unwrap() + next_value_in_sequence(&diff_seq)
}

impl History
{
    pub fn next_value(&self) -> i64
    {
        return next_value_in_sequence(&self.values);
    }

    pub fn previous_value(&self) -> i64
    {
        let rev_values = self.values.iter()
            .rev()
            .map(|v| v.clone())
            .collect();

        next_value_in_sequence(&rev_values)
    }
}

impl From<String> for History
{
    fn from(value: String) -> Self
    {
        let values = value.split(' ').map(|c| c.parse::<i64>().unwrap()).collect();

        History { values: values }
    }
}

#[cfg(test)]
mod tests
{
    use crate::oasis::next_value_in_sequence;

    use super::History;

    #[test]
    fn test_next_value_in_sequence()
    {
        let initial_seq = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(next_value_in_sequence(&initial_seq), 28)
    }

    #[test]
    fn test_next_value_in_diff_sequence_reversed()
    {
        let mut seq: Vec<i64> = vec![10, 13, 16, 21, 30, 45];
        seq.reverse();

        assert_eq!(next_value_in_sequence(&seq), 5);
    }

    #[test]
    fn test_example()
    {
        let histories = vec![
            History::from("0 3 6 9 12 15".to_string()),
            History::from("1 3 6 10 15 21".to_string()),
            History::from("10 13 16 21 30 45".to_string())
        ];

        let next_values: Vec<i64> = histories.iter().map(|h| h.next_value()).collect();
        let result: i64 = next_values.iter().sum();

        assert_eq!(result, 114);
    }
}
