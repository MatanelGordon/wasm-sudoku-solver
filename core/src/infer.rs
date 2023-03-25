use crate::analyze::AnalyzedCell;

// first number is the index, second number is the value.
pub type InferType = (usize, usize);

fn infer_group(group: &Vec<AnalyzedCell>) -> Vec<InferType> {
    let known: Vec<usize> = group.iter().filter_map(|x| x.get_value()).collect();

    let flattened: Vec<usize> = group
        .iter()
        .filter_map(|x| x.get_undetermined())
        .flatten()
        .filter(|&x| !known.contains(x))
        .map(|x| *x)
        .collect();

    let single_repeating_values: Vec<usize> = flattened
        .iter()
        .filter(|&val| flattened.iter().filter(|&val1| val == val1).count() == 1)
        .map(|x| *x)
        .collect();

    let mut results: Vec<InferType> = single_repeating_values
        .into_iter()
        .map(|val| {
            let index = group
                .iter()
                .position(|cell| {
                    cell.get_undetermined()
                        .unwrap_or(&Vec::new())
                        .contains(&val)
                })
                .unwrap();

            (index, val)
        })
        .collect();

    results.sort_by(|&(i1),(i2)| i1.cmp(i2));
    results.dedup_by(|(i1),(i2)| i1 == i2);

    // recursive search after found solutions - are there any other solutions after setting this value?
    if results.len() > 0 {
        let mut cloned = group.to_vec();

        // no need to worry about updating the Undetermined options
        // i already filter known numbers from the flattened
        for &(i,v) in results.iter(){
            cloned[i] = AnalyzedCell::Value(v);
        }

        results.extend(infer_group(&cloned));
    }

    results
}
