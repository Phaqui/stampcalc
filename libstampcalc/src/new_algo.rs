fn dot(a: &[usize], b: &[usize]) -> usize {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

fn increment_current(current: &mut Vec<usize>, max_factors: &Vec<usize>) -> (bool, bool) {
    let mut did_increment = false;
    let mut did_carry = false;
    
    //println!("  -- increment_current()");
    //println!("     from: {:?}", current);
    for i in (0..max_factors.len()).rev() {
        if current[i] < max_factors[i] {
            current[i] += 1;
            did_increment = true;
            break;
            // TODO notify that we only went up one without carry
        } else {
            did_carry = true;
            current[i] = 0;
        }
    }
    //println!("      to: {:?}", current);
    
    return (did_increment, did_carry);
}

pub fn solutions(price: usize, input: &Vec<usize>) {
    let len = input.len();
    
    let mut stamps = input.clone();
    stamps.sort_unstable();
    stamps.reverse();
    
    let (biggest, rest) = stamps.split_first().unwrap();
    
    let mut current = vec![0; len - 1];
    
    //let mut answers: Vec<Vec<usize>> = Vec::new();
    
    let old_n_checks: usize = stamps.iter().map(|x| (price / x) + 1).product();
    //println!("OLD number of checks to do: {}\n", old_n_checks);
    
    let max_outer = price / biggest;
    
    let mut nchecks: usize = 0;
    let mut nsolutions: usize = 0;
    
    for big in 0 .. max_outer + 1 {
        // is this necessary?
        current.fill(0);
        
        let price_to_hit = price - big * biggest;
        let max_factors = rest.iter().map(|rs| price_to_hit / rs).collect::<Vec<usize>>();
        
        loop {
            let (did_increment, carried) = increment_current(&mut current, &max_factors);
            if (!did_increment) { break; }
            
            nchecks += 1;
            if price_to_hit == dot(&rest, &current) {
                // somehow return this result..
                nsolutions += 1;
            }
        }
    }

}

fn increment_current2(current: &mut Vec<usize>, max_factors: &Vec<usize>) -> Option<(i32)> {
    for i in (0..max_factors.len()).rev() {
        if current[i] < max_factors[i] {
            current[i] += 1;
            return Some(i);
        } else {
            current[i] = 0;
        }
    }
    None
}

fn calc_max_factors(price_to_hit: usize, factors: &Vec<usize>) -> Vec<usize> {
    factors.iter().map(|rs| price_to_hit / rs).collect::<Vec<usize>>();
}

pub fn solutions2(price: usize, input: &Vec<usize>) {
    let len = input.len();
    let mut stamps = input.clone();
    stamps.sort_unstable();
    stamps.reverse();

    let mut current: Vec<usize> = vec![0; len - 1];
    let mut tmpstamps = stamps.clone();
    let mut price_to_hit: usize = price;
    let mut local_max_factors = calc_max_factors(price_to_hit, &stamps);

    while let Some(imd) = increment_current2(&mut current, &max_factors) {
        // imd = Index of Max Digit = the index of the biggest digit that was incremented

        // if we incremented any digit except the least significant digit
        if imd < len - 1 {
            // it could be that we carried over several digits
            // max_place_increased is the index of the highest digit that rolled over,
            // so, now, recalculate a new "local" max_factors, it being the max_factors
            // with these new big digits
            price_to_hit = price - dot(&current[..imd], &stamps[..imd]);
            local_max_factors = calc_max_factors(price_to_hit, &stamps[..imd]);

            if price_to_hit == dot(&current, &stamps) {
                nsolutions += 1;
            }
        } else if imd == len - 1 {
            let last_digit = try_solve_for_last_digit(&current, &stamps);
            if last_digit > -1 {
                // solution found, last digit was "found"
            }
        }
    }

    /*
    for i in 0..stamps.len() {
        let remaining = len - i - 1;

        let mut current = vec![0; remaining];
        let (biggest, rest) = stamps.split_first().unwrap();
        let this_max = price / biggest;

        for big in 0..this_max + 1 {
            // calculate new max_factors array for this "instance"
            let price_to_hit = price - XXXX;
            let max_factors = rest.iter().map(|rs| price_to_hit / rs).collect::<Vec<usize>>();

            while let Some(_) = increment_current2(&mut current, &max_factors) {
                if price_hit_hit == dot(&rest, &current) {
                    // somehow return this result
                    nsolutions += 1;
                }
            }
        }
    }
    */
}
