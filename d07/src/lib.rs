#[macro_use]
extern crate nom;

use nom::character::is_digit;
use std::io::BufRead;
use std::str::from_utf8;
use ustr::{Ustr, UstrMap, UstrSet};

#[derive(Debug)]
pub struct BagCountRule {
    pub count: u8,
    pub color: Ustr,
}

type BagCountRuleStatement = (Ustr, Vec<BagCountRule>);

named!(
    bag_count_rule<BagCountRule>,
    do_parse!(
        count: delimited!(char!(' '), take_while!(is_digit), char!(' '))
            >> color: take_until!(" bag")
            >> pair!(tag!(" bag"), opt!(char!('s')))
            >> (BagCountRule {
                count: from_utf8(count).unwrap_or("").parse().unwrap_or(0),
                color: Ustr::from(from_utf8(color).unwrap_or("undefined"))
            })
    )
);
named!(
    bag_count_rule_sequence<Vec<BagCountRule>>,
    terminated!(separated_list0!(char!(','), bag_count_rule), char!('.'))
);

named!(
    bag_count_rule_statement<BagCountRuleStatement>,
    do_parse!(
        color: take_until!(" bags contain")
            >> tag!(" bags contain")
            >> rules: bag_count_rule_sequence
            >> (Ustr::from(from_utf8(color).unwrap_or("undefined")), rules)
    )
);

pub fn parse<R>(mut reader: R) -> Vec<BagCountRuleStatement>
where
    R: BufRead,
{
    let mut result = Vec::new();
    let mut buf: [u8; 4096] = [0; 4096];
    let mut line_buf: [u8; 512] = [0; 512];
    let mut line_len = 0;
    while let Ok(len) = reader.read(&mut buf) {
        if len == 0 {
            break;
        }
        for c in buf[..len].iter() {
            if c == &b'\n' {
                if let Ok((_, r)) = bag_count_rule_statement(&line_buf[..line_len]) {
                    result.push(r);
                }
                line_buf = [0; 512];
                line_len = 0;
            } else {
                line_buf[line_len] = *c;
                line_len += 1;
            }
        }
    }

    if let Ok((_, r)) = bag_count_rule_statement(&line_buf[..line_len]) {
        result.push(r);
    }
    result
}

pub fn p1_solve(ruleset: &[BagCountRuleStatement]) -> usize {
    let mut child_2_parent = UstrMap::default();
    for (color, rules) in ruleset {
        for rule in rules {
            let parents = child_2_parent.entry(rule.color).or_insert_with(Vec::new);
            parents.push(color);
        }
    }
    count_parents(&Ustr::from("shiny gold"), &child_2_parent, UstrSet::default()).len()
}

fn count_parents(color: &Ustr, child_2_parent: &UstrMap<Vec<&Ustr>>, set: UstrSet) -> UstrSet {
    if let Some(parents) = child_2_parent.get(color) {
        return parents.iter().fold(set, |mut acc, parent| {
            if acc.contains(parent) {
                return acc;
            }
            acc.insert(**parent);
            count_parents(parent, child_2_parent, acc)
        });
    }
    set
}

pub fn p2_solve(ruleset: &[BagCountRuleStatement]) -> usize {
    let mut rule_map = UstrMap::default();
    for (color, rules) in ruleset {
        rule_map.insert(Ustr::from(color), rules.as_slice());
    }
    *count_children(&Ustr::from("shiny gold"), &rule_map, UstrMap::default())
        .get(&Ustr::from("shiny gold"))
        .unwrap_or(&0)
}

fn count_children(
    color: &Ustr,
    rule_map: &UstrMap<&[BagCountRule]>,
    mut counts: UstrMap<usize>,
) -> UstrMap<usize> {
    if counts.contains_key(color) {
        return counts;
    }
    if let Some(rules) = rule_map.get(color) {
        counts = rules.iter().fold(counts, |mut acc, rule| {
            acc = count_children(&rule.color, rule_map, acc);
            acc
        });
        counts.insert(
            *color,
            rules.iter().fold(0, |mut acc, rule| {
                let sum = counts.get(&rule.color).unwrap_or(&0) + 1;
                let c = rule.count as usize;
                acc += sum * c;
                acc
            }),
        );
    }
    counts
}
