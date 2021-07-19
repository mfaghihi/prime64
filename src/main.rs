use primegen::mathfuncs::*;

fn main() {

    // Doing some tests
    //let a :u64 =3;
    //let b :u64 =6;
    //let prime :u64 =5;
    
    // let base: u64 = 2;
    // let twop: u64 = base.pow(31);
    // let twop2: u64 = base.pow(31)+1;
    // let mayprime :u64 = base.pow(31)+32;
    //println!("a+b =  {:?}",field_add(&a,&b,&prime));
    //println!("a/b =  {:?}",field_div(&a,&b,&prime));

    //println!("twop= {:?}, twop2= {:?}, mayprime = {:?}",twop, twop2, mayprime);
   // println!("twop * twop2 mod mayprime = {:?}",field_mul(&twop,&twop2,&mayprime));

    //let factor1 : u64 =256;
    //let factor2 : u64 = 81;
    //println!("factor1= {:?}, factor2= {:?}",factor1, factor2);
    //println!("find_prime_with_factors= {:?}", find_prime_with_factors(&factor1,&factor2));


    // Generating the parameters
    let basetwo: u64 = 2;
    let basethree: u64 = 3;

    let twofactor = basetwo.pow(19);
    let threefactor = basethree.pow(10);

    let the_prime = find_prime_with_factors(&twofactor,&threefactor);
    let n_minus_one = the_prime - 1;
    println!("the prime is = {:?}", the_prime);

    let the_generator = find_generator(&the_prime);
    //println!("the generator is = {:?}", the_generator);
    
    // size of input vector to NTT

    // here we define which root of unity we want. For example 512th root of unity for root2
    // and 729th root of unity for root3 
    let n_for_root2 = basetwo.pow(9);
    let n_for_root3 = basethree.pow(6);

    let k2 = field_div(&n_minus_one, &n_for_root2, &the_prime);
    let root2 = field_pow(&the_generator, &k2, &the_prime);
    println!("the {:?}th root of unity is {:?}",n_for_root2, root2);
    // println!(" {:?}th_root to the power of  {:?} is {:?}",
    //     n_for_root2,n_for_root2, 
    //     field_pow(&root2,&n_for_root2,&the_prime));

    let k3 = field_div(&n_minus_one, &n_for_root3, &the_prime);
    let root3=field_pow(&the_generator, &k3, &the_prime);
    println!("the {:?}th root of unity is {:?}",n_for_root3, root3);
    // println!(" {:?}th_root to the power of  {:?} is {:?}",
    //     n_for_root3,n_for_root3, 
    //     field_pow(&root3,&n_for_root3,&the_prime));

    // for i in 1..n_for_root2{
    //     if field_pow(&root2, &i, &the_prime) == 1{
    //         println!("root2 not a desired root");
    //     }
    // }
    
    // for i in 1..n_for_root3{
    //     if field_pow(&root3, &i, &the_prime) == 1{
    //         println!("root3 not a desired root");
    //     }
    // }
    
}
 