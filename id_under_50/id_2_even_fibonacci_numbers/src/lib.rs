/*
https://www.geeksforgeeks.org/nth-even-fibonacci-number/

Fn = Fn-1 + Fn-2 [Expanding both terms]
   = Fn-2 + Fn-3 + Fn-3 + Fn-4 
   = Fn-2 + 2Fn-3 + Fn-4 [Expanding first term]
   = Fn-3 + Fn-4 + 2Fn-3 + Fn-4
   = 3Fn-3 + 2Fn-4  [Expanding one Fn-4]
   = 3Fn-3 + Fn-4 + Fn-5 + Fn-6 [Combing Fn-4 and Fn-5]
   = 4Fn-3 + Fn-6
*/

pub fn even_fibbonacci_numbers(upper_bound: i64) -> i64 {
    let mut ef_m1 = 2;
    let mut ef_m2 = 0;
    let mut sum = 0;
    while ef_m1 < upper_bound{
        sum += ef_m1;
        ef_m1 = 4 * ef_m1 + ef_m2;
        ef_m2 = (ef_m1 - ef_m2) / 4;
    }
    sum
}
