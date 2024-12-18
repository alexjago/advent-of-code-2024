use anyhow::Result;
use clap::Parser;
use itertools::Itertools;
use mapgrid::*;
use nom;
use regex::{self, Match, Regex};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs::read_to_string;
use strum;

#[derive(Parser)]
pub struct Opts {
    infile: std::path::PathBuf,
}

fn main() -> Result<()> {
    let opts: Opts = clap::Parser::parse();

    let infile = read_to_string(opts.infile)?;

    println!(
        "Part 1:\n{}",
        part_1(&infile).into_iter().map(|x| x.to_string()).join(",")
    );
    println!("Part 2:\n{}", part_2(&infile));

    Ok(())
}

fn combo(op: usize, reg_a: usize, reg_b: usize, reg_c: usize) -> usize {
    match op {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => reg_a,
        5 => reg_b,
        6 => reg_c,
        _ => unimplemented!(),
    }
}

const INSTR_NAMES: [&str; 8] = ["ADV", "bxl", "BST", "jnz", "bxc", "OUT", "BDV", "CDV"];

struct State {
    ip: usize,
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
}

/// returns
/// (ip, reg_a, reg_b, reg_c, Option<out>)
/// might panic
fn p1_step(instrs: &[usize], state: State) -> (State, Option<usize>) {
    let mut ip = state.ip;
    let mut reg_a = state.reg_a;
    let mut reg_b = state.reg_b;
    let mut reg_c = state.reg_c;

    let instr = instrs[ip];

    // in theory, this could panic
    let op = instrs[ip + 1];

    // println!(
    //     "ip: {ip}\tinstr: {instr} ({})\top: {op:o}\tA: {reg_a:o}\tB: {reg_b:o}\tC: {reg_c:o}",
    //     INSTR_NAMES[instr]
    // );

    let mut instr_step = 2;

    let mut out = None;

    match instr {
        0 => {
            // adv
            reg_a = reg_a >> combo(op, reg_a, reg_b, reg_c);
        }
        1 => {
            // bxl
            reg_b ^= op;
        }
        2 => {
            // bst
            reg_b = combo(op, reg_a, reg_b, reg_c) % 8;
        }
        3 => {
            // jnz
            if reg_a > 0 {
                ip = op;
                instr_step = 0;
            }
        }
        4 => {
            // bxc
            reg_b ^= reg_c;
        }
        5 => {
            // out
            out = Some(combo(op, reg_a, reg_b, reg_c) % 8);
        }
        6 => {
            // bdv
            reg_b = reg_a >> combo(op, reg_a, reg_b, reg_c);
        }
        7 => {
            // cdv
            reg_c = reg_a >> combo(op, reg_a, reg_b, reg_c);
        }

        _ => unimplemented!(),
    };

    ip += instr_step;

    (
        State {
            ip,
            reg_a,
            reg_b,
            reg_c,
        },
        out,
    )
}

fn part_1(infile: &str) -> Vec<usize> {
    /*
    Is this the return of the infamous Intcode?

    - three bits (can store 0-7)
    - three registers (A, B, C) which can hold any integer (of any size)
    - eight instructions
        - each takes either a literal operand (3 bit number)
        - or a combo operand (0-3: literal 0-3; 4: A, 5: B, 6, C, no 7)

    for my input, 742050537 is the wrong answer grr
    */

    // println!("\n----- ----- -----\n");

    let re = Regex::new(r"\d+").unwrap();

    let mut digits = re.find_iter(infile);

    let reg_a = digits.next().unwrap().as_str().parse().unwrap();
    let reg_b = digits.next().unwrap().as_str().parse().unwrap();
    let reg_c = digits.next().unwrap().as_str().parse().unwrap();

    let instrs: Vec<usize> = digits
        .map(|s| s.as_str().parse())
        .filter_map(|x| x.ok())
        .collect();

    // println!(
    //     "Instructions:\n{}\n",
    //     instrs.clone().into_iter().join("   ")
    // );

    part_1_inner(reg_a, reg_b, reg_c, &instrs)
}

fn part_1_inner(reg_a: usize, reg_b: usize, reg_c: usize, instrs: &[usize]) -> Vec<usize> {
    let mut out: Vec<usize> = vec![];
    let mut ip = 0;

    let mut reg_a = reg_a;
    let mut reg_b = reg_b;
    let mut reg_c = reg_c;

    // println!("EXECUTION TRACE:\nNote that all numbers (except ip) should be in OCTAL.\nInstructions with combo operands in CAPS.");

    while ip < instrs.len() {
        let rez = p1_step(
            &instrs,
            State {
                ip,
                reg_a,
                reg_b,
                reg_c,
            },
        );

        if let Some(o) = rez.1 {
            out.push(o);
        }

        // println!("\tout: {out:?}");

        ip = rez.0.ip;
        reg_a = rez.0.reg_a;
        reg_b = rez.0.reg_b;
        reg_c = rez.0.reg_c;
    }

    out
}

fn part_2_orig(infile: &str) -> usize {
    // we're making a quine!
    // answer for this one:
    // the correct value of register A

    // ok so for  MY PARTICULAR PROGRAM

    /*
     0. bst(a) // b = a % 8
     2. bxl(1) // b = b ^ 1
     4. cdv(b) // c = a >> (b % 8)
     6. bxc(4) // b = b ^ c // == b ^ (a >> (b % 8))
     8. bxl(4) // b = b ^ 4
    10. adv(3) // a = a >> 3
    12. out(b) // output (b % 8)
    14. jnz(0) // if (a==0), jump to 0 (else halt)

     */

    /* So structurally my program is

    generate a value for b from a
    shift a
    loop

    and B, C are overwritten each time

    insight: since we output b mod 8, its higher order bits (acquired at instr 6) don't matter for the result

    */

    /*

    (( (a >> ((a % 8) ^ 1) ) ^ ((a % 8) ^ 1) ) ^ 4 ) % 8

    so when we have an (octal) output digit D

    D = (a >> ((a % 8) ^ 1) ) ^ ((a % 8) ^ 1)  ^ 4 // implicitly,  all % 8

    (D ^ 4) = (a >> ((a % 8) ^ 1) ) ^ ((a % 8) ^ 1)

    // I suppose at this point we have 7 or 8 options for D
    // or more accurately we can try 8 different values for ((a % 8) ^ 1)
    // which then implies some value for the relevant digit when a >> ((a % 8) ^ 1)
    */

    // suppose we have to generate instruction D=2

    /*

    then D ^ 4 = 6

    (a >> ((a % 8) ^ 1)) ^ ((a % 8) ^ 1) == 6, solve for `a`
    (there are only 64 options to check: 8 for the lower bits and 8 for the higher bits)

    now, this also sets some higher bits in A, which... i'm not sure how to deal with yet

    */

    // for target in 0_usize..8 {
    //     for lo in 0_usize..8 {
    //         for hi in 0_usize..8 {
    //             if (hi >> (lo ^ 1)) ^ (lo ^ 1) == target {
    //                 println!(
    //                     "0o{:04o} ({hi}, {lo}) works for {target}",
    //                     (hi << (lo ^ 1)) | lo
    //                 );
    //             }
    //         }
    //     }
    // }

    // can we solve this from the last instruction-digit (MSBs of A), or do we have to go from the first digit? (LSBs of A)

    // we do have to consider non-multiples-of-three-bit shifts too

    // consider a recursive? algo which takes as input {bit_idx: u1} to indicate already-set bits? Plus a target digit.
    // Ideally we would normalise so that the LSB for our shifted digit is idx 0?
    // Or we build from the MSB of A down
    // eg our final digit is 0, so we can make that as 4, (generated as hi = 2, lo = 0) or 6 (via hi = 3, lo = 0) or 1 (via hi = 0, lo = 1)
    // the lowest of these is 1, but we can't be super sure that lower digits can't use this
    // we have 16 digits total so this is bits 45-47
    // going with 0o1 for now
    // set {45: 1, 46: 0, 47: 0} and recurse to ...
    // ... next digit 3
    // we can build 3 in several different ways
    // 0o10 (4, 0), 0o12 (5, 0), 0o3 (3, 1), 0o12 (1, 2) and many more
    // of these only 0o10 and 0o12 match our existing constraints...
    // set either {42: 0, 43: 0, 44: 0, 45: 1, 46: 0, 47: 0} for 0o10xxxxxxxxxxxxxx
    // or maybe   {42: 0, 43: 1, 44: 0, 45: 1, 46: 0, 47: 0} for 0o12xxxxxxxxxxxxxx

    // YOLO 1, let's try it
    // huh, that didn't work
    // what happened here is that by the time we got to building the second digit, A had been shifted down to only one digit
    // so maybe we need another digit on A?
    // no, we need the same number of digits on A as instructions in the output

    let mut outcomes: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for register_a in 0_usize..0o200 {
        let sss = format!("{register_a} 0 0    2,4,1,1,7,5,4,4,1,4,0,3,5,5,3,0");
        let rez = part_1(&sss);
        outcomes.insert(register_a, rez);
    }
    for (k, v) in outcomes {
        println!("{k:04o} ==> {v:?}");
    }

    println!("{:?} => [4]", set_bits_to_output(&BTreeMap::new(), vec![4]));
    println!(
        "{:?} => [3, 0]",
        set_bits_to_output(&BTreeMap::new(), vec![3, 0])
    );
    println!(
        "{:?} => [1, 4]",
        set_bits_to_output(&BTreeMap::new(), vec![1, 4])
    );

    todo!()
}

fn set_bits_to_output(input: &BTreeMap<usize, bool>, target: Vec<usize>) -> Option<usize> {
    let mut mask_out = usize::MAX;
    let mut mask_in = 0;

    for (k, v) in input.iter() {
        if *k < 64 {
            mask_out ^= 1 << (*k);
            mask_in |= (*v as usize) << (*k);
        }
    }

    // our hunting license extends for 7 bits above the set-bits assuming just the one relevant shift
    let bits = target.len() * 3 + 7;

    for i in 0_usize..(1 << bits) {
        let register = (i & mask_out) | mask_in;
        if part_1(&format!("{} 0 0 2,4,1,1,7,5,4,4,1,4,0,3,5,5,3,0", register)) == target {
            return Some(register);
        }
    }

    None
}

/*
fn i_love_recursion(
    input: BTreeMap<usize, bool>,
    target: Vec<usize>,
    done: usize,
) -> Option<usize> {
    for digit_count in done..target.len() {
        let mut mask_out = usize::MAX;
        let mut mask_in = 0;

        for (k, v) in input.iter() {
            if *k < 64 {
                mask_out ^= 1 << (*k);
                mask_in |= (*v as usize) << (*k);
            }
        }

        // our hunting license extends for 7 bits above the set-bits assuming just the one relevant shift

        let low_bit = digit_count * 3;
        let high_bit = digit_count * 3 + 8;

        for i in ((1 << low_bit) - 1)..(1 << high_bit) {
            let register = (i & mask_out) | mask_in;

            if register < i {
                continue;
            }
            if part_1(&format!("{} 0 0 2,4,1,1,7,5,4,4,1,4,0,3,5,5,3,0", register))
                == target[(target.len() - done - 1)..]
            {
                return Some(register);
            }
        }
    }
    None
}
*/

/*

I love recursion!


things we know

suppose register A is some N bit octal number, A

which produces some output O: [u3; N]

then if register A is A' = A >> 3

its output will be O[1..]

if reg A is A" = A >> 6

the output will be O[2..]

etc

we know that output[0] depends on bits 0-2 and potentially bits 3-5 also, output[1] on bits 3-5 and potentially bits 6-8

but crucially, the final output, output[N-1], can *only* depend on bits (N-1)*3 .. N*3,
 because after the `adv` the register has to be zero so the machine can halt!


So we only have 3 bits for a one-digit output, 6 bits for a two-digit, etc

we can productionise this


*/

fn do_it(
    known_bits: usize,                                 // prefix of register A
    digits_done: usize,                                // qty of known digits (starting at the last)
    target: &[usize],                                  // program text to match against
    program: &[usize],                                 // actual program text
    memo: &mut HashMap<(usize, usize), Option<usize>>, // cache by `known_bits` and `digits_done`
) -> Option<usize> {
    println!(
        "done: {digits_done}, known: 0o{known_bits:o}, targeting: {:?}",
        &target[..(target.len() - digits_done)]
    );

    // memoizing on known_bits and start_digit
    if let Some(rez) = memo.get(&(known_bits % 8, digits_done)) {
        println!(
            "~~~ omg a hit tweet ~~~\t ({}, {}) => {rez:?}",
            known_bits % 8,
            digits_done
        );
        return rez.map(|x| (known_bits << (digits_done * 3)) | x);
    }
    if digits_done == target.len() {
        return Some(known_bits);
    }

    // loop over digit count
    for digit_count in 1..=(target.len() - digits_done) {
        let tt = &target[target.len() - (digits_done + digit_count)..(target.len() - digits_done)];
        println!("done: {digits_done}, trying next {digit_count} digits");

        for trial in 0_usize..(1 << (digit_count * 3)) {
            let register = (known_bits % 8) << 3 | trial;

            // println!("\t{register:016o}");

            if part_1_inner(register, 0, 0, program) == tt {
                println!("done: {digits_done}, width: {digit_count}, trialled: 0o{register:o}, targeting {tt:?}  was SUCCESSFUL, moving on");

                let rez = do_it(register, digits_done + digit_count, target, program, memo);

                println!("subquery result: {rez:?}");

                memo.insert(
                    (known_bits % 8, digits_done),
                    rez.map(|x| x % 2_usize.pow(3 * (target.len() - digits_done) as u32)),
                );

                if let Some(r) = rez {
                    return Some((known_bits << (digits_done * 3)) | r);
                }
            }
        }
        memo.insert((known_bits % 8, digits_done), None);
    }
    None
}

fn part_2(infile: &str) -> usize {
    let mut memo = HashMap::new();

    let re = Regex::new(r"\d+").unwrap();

    let digits = re.find_iter(infile);

    let instrs: Vec<usize> = digits
        .skip(3)
        .map(|s| s.as_str().parse())
        .filter_map(|x| x.ok())
        .collect();

    do_it(0, 0, &instrs, &instrs, &mut memo).expect("This should be solveable!")
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const EXAMPLE_2: &str = r"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    const PROGRAM: &[usize; 16] = &[2, 4, 1, 1, 7, 5, 4, 4, 1, 4, 0, 3, 5, 5, 3, 0];

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1), vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }

    #[test]
    fn part_1_small_examples() {
        assert_eq!(part_1("0 0 9         2 6"), vec![]);
        assert_eq!(part_1("10 0 0        5 0 5 1 5 4"), vec![0, 1, 2]);
        assert_eq!(
            part_1("2024 0 0      0 1 5 4 3 0"),
            vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]
        );
        assert_eq!(part_1("0 29 0        1 7"), vec![]);
        assert_eq!(part_1("0 2024 43690  4 0"), vec![]);
    }

    #[test]
    fn part_2_yolo() {
        assert_ne!(
            part_1("8 0 0   2,4,1,1,7,5,4,4,1,4,0,3,5,5,3,0"),
            vec![3, 0]
        );
        assert_ne!(
            part_1("144 0 0   2,4,1,1,7,5,4,4,1,4,0,3,5,5,3,0"),
            vec![3, 0]
        );
        assert_eq!(
            part_1("46 0 0   2,4,1,1,7,5,4,4,1,4,0,3,5,5,3,0"),
            vec![3, 0]
        );
    }

    #[test]
    fn part_2_do_it() {
        let mut memo = HashMap::new();
        assert_eq!(do_it(0, 0, &[3, 0], PROGRAM, &mut memo), Some(0o56));

        let mut memo = HashMap::new();
        let rez = do_it(0, 0, &PROGRAM[6..], PROGRAM, &mut memo);
        println!("{memo:?}");
        assert_eq!(rez, Some(771968555));
    }

    #[test]
    fn cant_count_to_four() {
        for i in 0..64 {
            assert_ne!(part_1_inner(i, 0, 0, PROGRAM), vec![4]);
        }
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_2), 117440);
    }
}
