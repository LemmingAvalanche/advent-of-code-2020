const MAX_SIZE: usize = 26;
const ASCII_VALUE_LOWER_CASE_A: usize = 97;

pub fn solve_part_1(input: &str) -> u64 {
    let mut answers = [false; MAX_SIZE];
    let mut total: u64 = 0;
    for l in input.lines() {
        if l == "" {
            total += count_yeses(&answers);
            reset_answers(&mut answers);
        } else {
            questionaire_any(l, &mut answers);
        }
    }
    total += count_yeses(&answers);
    total
}

#[inline]
fn count_yeses(answers: &[bool; MAX_SIZE]) -> u64 {
    answers.iter().fold(0, |acc, b| acc + if *b { 1 } else { 0 })
}

// g: multi-line string, one answer per line
fn group_any_yes(g: &str, answers: &mut [bool; MAX_SIZE]) -> () {
    g.lines().for_each(|l| questionaire_any(l, answers));
}

fn questionaire_any(qs: &str, answers: &mut [bool; MAX_SIZE]) -> () {
    for q in qs.chars() {
        answers[answer_to_idx(q)] = true;
    }
}

fn reset_answers(answers: &mut [bool; MAX_SIZE]) -> () {
    for i in 0..MAX_SIZE {
        answers[i] = false;
    }
}

#[inline]
fn answer_to_idx(ch: char) -> usize {
    (ch as usize) - ASCII_VALUE_LOWER_CASE_A
}

#[test]
fn questionaire_any_abcx() {
    let answer = [true, true, true, false, false, false,
                  false, false, false, false, false, false,
                  false, false, false, false, false, false,
                  false, false, false, false, false, true,
                  false, false];
    let mut work = [false; MAX_SIZE];
    questionaire_any(&"abcx", &mut work);
    assert_eq!(answer, work);
}

#[test]
fn questionaire_any_xqhf() {
    let answer = [false, false, false, false, false, true,
                  false, true, false, false, false, false,
                  false, false, false, false, true, false,
                  false, false, false, false, false, true,
                  false, false];
    let mut work = [false; MAX_SIZE];
    questionaire_any(&"xqhf", &mut work);
    assert_eq!(answer, work);
}
#[test]
fn group_any_yes_first_group_example() {
    let group = "abcx\nabcy\nabcz";
    let answer = [true, true, true, false, false, false,
                  false, false, false, false, false, false,
                  false, false, false, false, false, false,
                  false, false, false, false, false, true,
                  true, true];
    let mut work = [false; MAX_SIZE];
    group_any_yes(&group, &mut work);
    assert_eq!(answer, work);
}
#[test]
fn count_yeses_abcx() {
    let mut work = [false; MAX_SIZE];
    questionaire_any(&"abcx", &mut work);
    assert_eq!(4, count_yeses(&work));
}
#[test]
fn count_yeses_two_questionaires_any_abcx_xqhf() {
    let mut work = [false; MAX_SIZE];
    questionaire_any(&"abcx", &mut work);
    questionaire_any(&"xqhf", &mut work);
    assert_eq!(7, count_yeses(&work));
}
#[test]
fn solve_part_1_first_example() {
    let s = "abcx\nabcy\nabcz";
    assert_eq!(6, solve_part_1(&s));
}
#[test]
fn solve_part_1_multi_group_example() {
    let s = "abc

a
b
c

ab
ac

a
a
a
a

b
";
    assert_eq!(11, solve_part_1(&s));
}
#[test]
fn solve_part_1_one_group_all_yeses() {
    let s = "abc
def
ghi
jkl
mno
pqr
stu
vwx
yz";
    assert_eq!(26, solve_part_1(&s))
}
#[test]
fn solve_part_1_several_groups_26_yeses() {
    let s = "abc

def

ghi

jkl

mno

pqr

stu

vwx

yz";
    assert_eq!(26, solve_part_1(&s))
}
#[test]
fn solve_part_1_many_groups_no_yeses() {
    let s = "







";
    assert_eq!(0, solve_part_1(&s))
}
