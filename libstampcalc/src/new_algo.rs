fn dot(a: &[usize], b: &[usize]) -> usize {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

#[inline(always)]
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
        //current[0] = big;
        
        let price_to_hit = price - big * biggest;
        //println!("* big == {}, which means price to hit == {}", big, price_to_hit);
        let max_factors = rest.iter().map(|rs| price_to_hit / rs).collect::<Vec<usize>>();
        //println!("  max_factors for this round: {:?}", max_factors);
        
        let n_to_check: usize = max_factors.iter().product();
        nchecks += n_to_check;
        
        //println!("  combinations to check: {}", n_to_check);
        let mut ninner_checks: usize = 0;
        
        loop {
            let (did_increment, carried) = increment_current(&mut current, &max_factors);
            if (!did_increment) { break; }
            
            //println!("    checking factors: {:?}", &current);
            ninner_checks += 1;
            if price_to_hit == dot(&rest, &current) {
                nsolutions += 1;
                //println!("  --SOLUTION FOUND: {:?}", current);
            }
        }
        
        //println!("Did {} inner checks", ninner_checks);
    }
    
    //println!("Found {} solutions (doing {} checks)", nsolutions, nchecks);
}
