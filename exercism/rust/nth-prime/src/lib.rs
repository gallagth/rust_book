use std::borrow::BorrowMut;

static mut PRIME_SIEVE: Option<Vec<bool>> = None;

fn update_sieve(n: usize) {
    unsafe {
        if PRIME_SIEVE.is_none() || PRIME_SIEVE.borrow_mut().as_mut().unwrap().len() < n {
            PRIME_SIEVE = Some(vec!(true; n));
        } else {
            return;
        }
        let value = PRIME_SIEVE.borrow_mut().as_mut().unwrap();
        value[0] = false;
        value[1] = false;
        for i in 2..n {
            let mut j = i;
            if !value[j] { continue; }
            value[j] = true;
            j += i;
            while j < n {
                value[j] = false;
                j += i;
            }
        }
    }
}

pub fn nth(n: usize) -> Result<usize, &'static str> {
    // Update the prime number sieve
    update_sieve(20*n);
    unsafe {
        let sieve = PRIME_SIEVE.borrow_mut().as_mut().unwrap();
        let mut j = 0;
        for i in 2..sieve.len() {
            if sieve[i] {
                j += 1;
                if j == n { return Ok(i); }
            }
        }
        return Err("Error");
    }
}