use std::{io::{BufReader, BufRead}, fs::File, collections::HashMap};

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum State
{
    Operational,
    Damaged,
    Unknown
}

#[derive(Clone)]
struct Record
{
    pub state: Vec<State>,
    pub damaged_groups: Vec<usize>,
}

fn main()
{
    let file = File::open("./day-12/input.txt").unwrap();
    let buffer: BufReader<File>  = BufReader::new(file);

    let records = parse(buffer);

    let mut cache = HashMap::new();

    let result: usize = records.iter().map(|r| possible_solutions(&r.state, &r.damaged_groups, &mut cache)).sum();
    println!("Result: {}", result);

    let result2: usize = records.iter()
        .map(|r| expand_record(&r, 5))
        .map(|r| possible_solutions(&r.state, &r.damaged_groups, &mut cache))
        .sum();

    println!("Result 2: {}", result2);
}

fn parse(buffer: BufReader<File>) -> Vec<Record>
{
    let mut result = vec![];

    for line in buffer.lines()
    {
        match line
        {
            Ok(content) => result.push(parse_line(&content)),
            Err(e) => panic!("Error while reading file: {}", e)
        }
    }

    result
}

fn parse_line(content: &str) -> Record
{
    let mut parts = content.split(' ');

    let state = parts.next().unwrap().chars().map(|c| {
        match c
        {
            '.' => State::Operational,
            '#' => State::Damaged,
            _ => State::Unknown
        }
    }).collect();

    let damaged_groups = parts.next().unwrap().split(',').map(|n| n.parse::<usize>().unwrap()).collect();

    Record { state, damaged_groups }
}

fn expand_record(record: &Record, times: usize) -> Record
{
    let mut state = record.state.clone();
    let damaged_groups = record.damaged_groups.clone().repeat(times);

    for _ in 0..(times-1)
    {
        state.push(State::Unknown);
        state.append(&mut record.state.clone());
    }

    Record { state, damaged_groups }
}

fn possible_solutions(state: &Vec<State>, groups: &Vec<usize>, mut cache: &mut HashMap<(Vec<State>, Vec<usize>), usize>) -> usize
{
    if let Some(result) = cache.get(&(state.clone(), groups.clone())) { return *result; }

    match state.first()
    {
        Some(State::Operational) => {
            let remaining: Vec<_> = state.iter().skip_while(|s| **s == State::Operational).map(|s| *s).collect();

            // If remaining is smaller than next group, impossible case
            if remaining.len() < *groups.first().unwrap_or(&0)
            {
                cache.insert((state.clone(), groups.clone()), 0);
                return 0;
            }

            return possible_solutions(&remaining, &groups, &mut cache);
        },
        Some(State::Damaged) => {
            match groups.first()
            {
                Some(size) => {
                    let matched_group: Vec<_> = state.iter().take(*size).collect();

                    if matched_group.len() == *size && matched_group.iter().all(|s| **s != State::Operational)
                    {
                        // If next is a unknown, we need to explicitely set it to a Operational (to have a delimiter)
                        let mut remaining : Vec<_> = state.iter().skip(*size).map(|s| *s).collect();
                        let remaining_groups = groups.iter().skip(1).map(|i| *i).collect();

                        match remaining.first_mut()
                        {
                            // We have another damaged after, so we didn't match the group
                            Some(State::Damaged) => {
                                cache.insert((state.clone(), groups.clone()), 0);
                                return 0;
                            }
                            // If the next elem is unknown we can set it to operational (as if we have damaged it doesn't work)
                            Some(first) if *first == State::Unknown => *first = State::Operational,
                            _ => ()
                        }

                        return possible_solutions(&remaining, &remaining_groups, &mut cache);
                    }
                    else
                    {
                        cache.insert((state.clone(), groups.clone()), 0);
                        return 0;
                    }
                }
                None => {
                    cache.insert((state.clone(), groups.clone()), 0);
                    return 0
                }
            }
        },
        Some(State::Unknown) => {
            let mut state_operational = state.clone();
            state_operational[0] = State::Operational;

            let mut state_damaged = state.clone();
            state_damaged[0] = State::Damaged;

            let result = possible_solutions(&state_operational, &groups, &mut cache) +
                                possible_solutions(&state_damaged, &groups, &mut cache);

            cache.insert((state.clone(), groups.clone()), result);

            return result;
        },
        None => {
            let result = if groups.is_empty() {  1 }
                else { return 0 };

            cache.insert((state.clone(), groups.clone()), result);
            return result;
        }
    }
}

#[cfg(test)]
mod tests
{
    use std::{collections::HashMap, io::BufReader, fs::File};

    use crate::{Record, State, possible_solutions, parse, parse_line, expand_record};

    #[test]
    fn test_possible_solutions()
    {
        let record = Record { state: vec![State::Operational, State::Unknown, State::Operational], damaged_groups: vec![1] };

        assert_eq!(possible_solutions(&record.state, &record.damaged_groups, &mut HashMap::new()), 1);

        let record = Record { state: vec![
            State::Operational,
            State::Unknown,
            State::Unknown,
            State::Unknown,
            State::Operational], damaged_groups: vec![1] };

        assert_eq!(possible_solutions(&record.state, &record.damaged_groups, &mut HashMap::new()), 3);
    }

    #[test]
    fn test_possible_solutions_custom()
    {
        // Expected: 5
        let record = parse_line("?.?#.?###??.#???? 2,4,1,2");
        assert_eq!(possible_solutions(&record.state, &record.damaged_groups, &mut HashMap::new()), 5);

        let record = parse_line(".??#?????.???????# 4,5,2");
        assert_eq!(possible_solutions(&record.state, &record.damaged_groups, &mut HashMap::new()), 3);
    }

    #[test]
    fn test_possible_solutions_example1()
    {
        let records = parse(BufReader::new(File::open("./input2.txt").unwrap()));

        let solutions: Vec<_> = records.iter()
            .map(|r| possible_solutions(&r.state, &r.damaged_groups, &mut HashMap::new()))
            .collect();

        assert_eq!(solutions, vec![
            1, 4, 1, 1, 4, 10
        ]);
    }

    #[test]
    fn test_possible_solutions_input()
    {
        let records = parse(BufReader::new(File::open("./input.txt").unwrap()));

        let mut cache = HashMap::new();

        let solutions: Vec<_> = records.iter()
            .map(|r| possible_solutions(&r.state, &r.damaged_groups, &mut cache))
            .collect();

        assert_eq!(solutions.iter().sum::<usize>(), 7361);
    }

    #[test]
    fn test_expand_record()
    {
        let record = Record { state: vec![State::Operational], damaged_groups: vec![1] };

        let expanded_record = expand_record(&record, 5);

        assert_eq!(expanded_record.state.len(), 9);
        assert_eq!(expanded_record.damaged_groups.len(), 5);
    }
}