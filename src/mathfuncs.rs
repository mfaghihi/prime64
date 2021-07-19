/// A function to add two u64 numbers mod a u64 number (a+b mod prime)
/// prime does not need to be an actual prime for this function
pub fn field_add(a: &u64, b: &u64, prime: &u64)-> u64 {
    (a+b)%prime
}

use std::convert::{From, TryInto};
/// A function to multiply two u64 numbers mod a u64 number (a*b mod prime)
/// prime does not need to be an actual prime for this function
pub fn field_mul(a: &u64, b: &u64, prime: &u64)-> u64 {
    // Convert from u64 to u128 to avoid overflow during the computation.
    // Since the output will be computed mod prime, we can later convert back 
    // the output from u128 to u64
    let a64:u128 = u128::from(*a);
    let b64:u128 = u128::from(*b);
    let prime64:u128 = u128::from(*prime);

    // since our inputs are u64, this line will return a reasonable value
    ((a64*b64)%prime64).try_into().unwrap()
}
/// A function to raise a u64 number to another u64 number mod a u64 number (a.pow(b) mod prime)
/// prime does not need to be an actual prime for this function
pub fn field_pow(base_number: &u64, exponent: &u64, prime: &u64)-> u64{
    let mut result: u64 = 1;
    let mut base = *base_number;
    let mut exp =*exponent;
    loop
    {
        if exp % 2 == 1 {
            result = field_mul(&result, &base, &prime);
        }
        exp = exp >> 1;
        if exp == 0 {
            break result
        }
        base = field_mul(&base, &base, &prime);
    }
}

/// A function to invert a u64 number mod a u64 number (b.pow(-1) mod prime)
/// It is better to make sure prime is an actual prime number. Otherwise, the 
/// output may not be an inverse of the input.
pub fn field_inv(b: &u64, prime: &u64)-> u64 {
    let pm2=prime-2;
    
    // Computing the inverse: b.pow(-1) mod prime = b.pow(pm2) mod prime
    field_pow(b,&pm2, prime)
}
/// A function to compute division of a u64 number by another u64 number mod a
//  u64 number (a/b mod prime)
/// It is better to make sure prime is an actual prime number. Otherwise, the 
/// output may not be an actual division of the inputs.
pub fn field_div(num: &u64, den: &u64, prime: &u64)-> u64 {
    let a32 = *num;
    let b32 = *den;
    let prime32=*prime;

    // find inverse of the denum
    let b_inv = field_inv(&b32,&prime32);
    //println!("iverse of {:?} = {:?}",b32,b_inv);

    // a/b = a*b.pow(-1)= a*b_inv
    field_mul(&a32,&b_inv,&prime32)
}

/// Function to test primality of a u64 number deterministicly. Using a deterministic test may not be efficient.
/// But it is good enough for our porpuse of testing a u64 number 
/// Code from : https://riptutorial.com/rust/example/20557/a-short-primality-test
fn is_prime(n: u64) -> bool {
    (2..n)
    .take_while(|divisor| divisor * divisor <= n)
    .all(|divisor| n % divisor != 0)
}

use std::vec::Vec;

/// A function to generate a prime such that (prime - 1) has factors factor1 and factor2
pub fn find_prime_with_factors(factor1: &u64,factor2: &u64)->u64
{
    // multiply factor1 and factor2 for a potential acceptable (p-1)=factor1*factor2
    // store the result in a temporary variable
	let tmp = factor1*factor2;
    
    // the potential prime
	let mut p = tmp+1;

    // A variable (vector) to store the actual output prime(s)
	let mut primes: Vec<u64> = vec![];// In case that we may want more than 1 
                                    // prime number. In that case, we need to 
                                    // make changes to the rest of the code

    // If p is a prime, this flag will become true
	let mut isprime = false;

	while !isprime
	{
		for i in 1..50000000// we can change the starting i to adjust the 
                                // desired number of bits in the prime number
		{
            // lets check if the current number p is a prime
            if is_prime(p){
        
                primes.push(p);
                isprime = true;
            
                break; // If we want more than 1 prime, we should remove this break.
            }
        // if p is not prime, add a new factor to it
        // By doing so, the value of p-1 will still have factor1 and factor2 as its factors
        p = tmp * i + 1;

		}
	}
	primes[0]
}

//use factor::factor::factor;
use primes::factors_uniq;
pub fn find_generator(prime: &u64)->u64{
    let nm1 = *prime -1;
    
    // Find the unique prime factors of nm1. The used library receives a u64 input
    // and outputs a u64 number. Since prime is a u64 number, its factors are less 
    // than 64 bits. 
    let prime_factors = factors_uniq(nm1);

    // How many prime factors exists in nm1
	let length = prime_factors.len();
	
    // We go through all numbers starting from 2 until we find a generator. 
    // Since a random number can be a generator with high probability, this
    // approach should give the output after trying a few numbers
    let mut result = 1;
    loop{
        result = result + 1;

        // If the current result is not a generator, this flag will become false
        let mut is_gen= true;

		for i in 0..length
		{
            // we need to compute result to the power of (nm1/factor[i]) and check if it is equal to 1
            // which means that the result is not a generator 
            let denum:u64= prime_factors[i].try_into().unwrap();
            let div = field_div(&nm1, &denum, &prime);
            //println!("{:?} div {:?} = {:?}",nm1,denum,prime);
			if  field_pow(&result,&div, prime) == 1
			{
				is_gen = false;
				break;
			}

		}	
        if is_gen {
            break;
        }
    }
    result
}