use std::collections::HashMap;
use truncator::summators::W16Summator;
use truncator::utils::summator_statistics;

fn main() {
    let msg = "truncator";

    let mut hashmap: HashMap<u32, u32> = HashMap::new();
    // Sometimes we prefer initializing hashmaps with zeros per sum for easier spreadsheet import.
    // for i in 0..1024 {
    //     hashmap.insert(i as u32, 0);
    // }
    // Works with any summator implementation required for Truncator testing.
    let summator = W16Summator();

    // compute sum statistics for the selected summator.
    summator_statistics(msg, 1_000_000, &summator, &mut hashmap);

    // print results, then you can copy copy-paste to spreadsheet for data analytics + charts.
    for (k, x) in hashmap {
        println!("{}, {}", k, x);
    }
}
