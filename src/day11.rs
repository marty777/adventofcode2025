use std::collections::{HashMap};

type Addr = [char;3];

/// Count paths to `dst` from `curr` to `dst` recursively using memoization
pub fn path_count(outputs:&HashMap<Addr, Vec<Addr>>, cache:&mut HashMap<Addr, usize>, curr:Addr, dst:Addr) -> usize {
    if cache.contains_key(&curr) {
        return *cache.get(&curr).unwrap();
    }
    if curr == dst {
        return 1;
    }
    if !outputs.contains_key(&curr) {
        return 0;
    }
    let mut paths = 0;
    for next in outputs.get(&curr).unwrap() {
        paths += path_count(outputs, cache, *next, dst);
    }
    cache.insert(curr, paths);
    return paths;
}

// try to convert a 3 character &str to a [char;3]
fn to_addr(string:&str) -> Addr {
    let result:Addr = string.chars().collect::<Vec<char>>().try_into().unwrap();
    return result;
}

/// Print the solutions to day 11 for the given input `lines`
pub fn run(lines:&Vec<String>) {
    let part1;
    let part2;
    let mut outputs:HashMap<Addr, Vec<Addr>> = HashMap::new();
    // I had some difficulty with lifetimes using &str keys in the cache 
    // HashMap, thus the constant size Addr type.
    for i in 0..lines.len() {
        let parts = lines[i].split(": ").collect::<Vec<&str>>();
        let src:Addr = to_addr(parts[0]);
        let outs = parts[1].split(" ").map(|x| to_addr(x)).collect::<Vec<Addr>>();
        outputs.insert(src, outs);
    }
    let you = to_addr("you");
    let out = to_addr("out");
    let svr = to_addr("svr");
    let dac = to_addr("dac");
    let fft = to_addr("fft");
    let mut cache:HashMap<Addr, usize> = HashMap::new();
    // Part 1 - find all paths from you to out
    part1 = path_count(&outputs, &mut cache, you, out);
    cache.clear();

    // Part 2 - the paths from svr to out that pass through both dac and fft
    // are the sum of the products of paths that path between the nodes in each
    // order.
    let svr_dac = path_count(&outputs, &mut cache, svr, dac);
    cache.clear();
    let dac_fft = path_count(&outputs, &mut cache, dac, fft);
    cache.clear();
    let fft_out = path_count(&outputs, &mut cache, fft, out);
    cache.clear();
    let svr_fft = path_count(&outputs, &mut cache, svr, fft);
    cache.clear();
    let fft_dac = path_count(&outputs, &mut cache, fft, dac);
    cache.clear();
    let dac_out = path_count(&outputs, &mut cache, dac, out);
    cache.clear();
    part2 = (svr_dac * dac_fft * fft_out) + (svr_fft * fft_dac * dac_out);
    
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
