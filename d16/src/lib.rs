extern crate btoi;
#[macro_use]
extern crate nom;

use btoi::btoi;
use nom::character::{is_alphabetic, is_digit, is_space};
use rustc_hash::{FxHashMap, FxHashSet};
use std::hash::{Hash, Hasher};
use std::io::BufRead;
use std::str::from_utf8;

#[derive(Debug)]
pub struct NumberRange {
    min: u16,
    max: u16,
}

impl NumberRange {
    fn is_valid(&self, number: &u16) -> bool {
        number >= &self.min && number <= &self.max
    }
}

#[derive(Debug)]
pub struct Rule {
    name: String,
    range_a: NumberRange,
    range_b: NumberRange,
}

impl Rule {
    pub fn is_valid(&self, number: &u16) -> bool {
        self.range_a.is_valid(number) || self.range_b.is_valid(number)
    }
}

impl Hash for Rule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Rule {}

#[derive(Debug)]
pub struct Ticket {
    values: Box<[u16]>,
}

#[derive(Debug)]
enum RulePart {
    Result(Rule),
    Break,
}

named!(
    parse_number_range<NumberRange>,
    do_parse!(
        min: take_while1!(is_digit)
            >> char!('-')
            >> max: take_while1!(is_digit)
            >> (NumberRange {
                min: btoi(min).unwrap_or(0),
                max: btoi(max).unwrap_or(0)
            })
    )
);

fn is_alphabetic_or_space(byte: u8) -> bool {
    is_alphabetic(byte) || is_space(byte)
}

named!(
    parse_rule<RulePart>,
    alt!(
        do_parse!(
            name: take_while1!(is_alphabetic_or_space)
                >> tag!(": ")
                >> range_a: parse_number_range
                >> tag!(" or ")
                >> range_b: parse_number_range
                >> char!('\n')
                >> (RulePart::Result(Rule {
                    name: String::from(from_utf8(name).unwrap_or("")),
                    range_a,
                    range_b
                }))
        ) | do_parse!(char!('\n') >> (RulePart::Break))
    )
);

#[derive(Debug)]
enum MyTicketPart {
    Header,
    Ticket(Ticket),
    Break,
}

named!(
    parse_ticket<Ticket>,
    do_parse!(
        values: separated_list1!(tag!(","), take_while1!(is_digit))
            >> char!('\n')
            >> (Ticket {
                values: values
                    .iter()
                    .map(|v| btoi(v).unwrap_or(0))
                    .collect::<Vec<u16>>()
                    .into_boxed_slice()
            })
    )
);

named!(
    parse_my_ticket<MyTicketPart>,
    alt!(
        do_parse!(ticket: parse_ticket >> (MyTicketPart::Ticket(ticket)))
            | do_parse!(tag!("your ticket:\n") >> (MyTicketPart::Header))
            | do_parse!(char!('\n') >> (MyTicketPart::Break))
    )
);

#[derive(Debug)]
enum NearbyTicketsPart {
    Header,
    Ticket(Ticket),
    Break,
}

named!(
    parse_nearby_tickets<NearbyTicketsPart>,
    alt!(
        do_parse!(ticket: parse_ticket >> (NearbyTicketsPart::Ticket(ticket)))
            | do_parse!(tag!("nearby tickets:\n") >> (NearbyTicketsPart::Header))
            | do_parse!(char!('\n') >> (NearbyTicketsPart::Break))
    )
);

#[derive(Debug)]
enum ParserState {
    RulePart,
    MyTicketPart,
    NearbyTicketsPart,
    Done,
}

#[derive(Debug)]
pub struct Parser {
    state: ParserState,
    buffer: [u8; 1024],
    length: usize,

    result_rules: Vec<Rule>,
    result_my_ticket: Option<Ticket>,
    result_nearby_tickets: Vec<Ticket>,
}

impl Parser {
    fn new() -> Self {
        Self {
            state: ParserState::RulePart,
            buffer: [0; 1024],
            length: 0,
            result_rules: Vec::new(),
            result_my_ticket: None,
            result_nearby_tickets: Vec::new(),
        }
    }

    fn parse(&mut self, input: std::slice::Iter<u8>) {
        for c in input {
            self.buffer[self.length] = *c;
            self.length += 1;
            if c == &b'\n' {
                self.parse_line();
                self.length = 0;
            }
        }
    }

    fn parse_line(&mut self) {
        match self.state {
            ParserState::RulePart => {
                if let Ok((_, result)) = parse_rule(&self.buffer[..self.length]) {
                    match result {
                        RulePart::Break => self.state = ParserState::MyTicketPart,
                        RulePart::Result(rule) => self.result_rules.push(rule),
                    }
                }
            }
            ParserState::MyTicketPart => {
                if let Ok((_, result)) = parse_my_ticket(&self.buffer[..self.length]) {
                    match result {
                        MyTicketPart::Break => self.state = ParserState::NearbyTicketsPart,
                        MyTicketPart::Header => (),
                        MyTicketPart::Ticket(ticket) => self.result_my_ticket = Some(ticket),
                    }
                }
            }
            ParserState::NearbyTicketsPart => {
                if let Ok((_, result)) = parse_nearby_tickets(&self.buffer[..self.length]) {
                    match result {
                        NearbyTicketsPart::Break => self.state = ParserState::Done,
                        NearbyTicketsPart::Header => (),
                        NearbyTicketsPart::Ticket(ticket) => {
                            self.result_nearby_tickets.push(ticket)
                        }
                    }
                }
            }
            ParserState::Done => {}
        }
    }
}

pub fn parse<R>(mut reader: R) -> (Box<[Rule]>, Ticket, Box<[Ticket]>)
where
    R: BufRead,
{
    let mut page: [u8; 4096] = [0; 4096];
    let mut parser = Parser::new();
    while let Ok(page_len) = reader.read(&mut page) {
        if page_len == 0 {
            break;
        }
        parser.parse(page[..page_len].iter());
    }
    parser.parse(b"\n".iter());
    (
        parser.result_rules.into_boxed_slice(),
        parser.result_my_ticket.expect("my ticket"),
        parser.result_nearby_tickets.into_boxed_slice(),
    )
}

pub fn p1_solve((rules, _, nearby_tickets): &(Box<[Rule]>, Ticket, Box<[Ticket]>)) -> Option<u64> {
    let mut invalid_sum: u64 = 0;
    for ticket in nearby_tickets.iter() {
        invalid_sum += ticket.values.iter().fold(0, |acc, value| {
            if rules.iter().any(|rule| rule.is_valid(value)) {
                acc
            } else {
                acc + *value as u64
            }
        });
    }

    Some(invalid_sum)
}

pub fn p2_solve(
    (rules, my_ticket, nearby_tickets): &(Box<[Rule]>, Ticket, Box<[Ticket]>),
) -> Option<u64> {
    // step 1: remove obviously invalid tickets
    let valid_tickets: Vec<&Ticket> = nearby_tickets
        .iter()
        .filter(|ticket| {
            ticket
                .values
                .iter()
                .all(|value| rules.iter().any(|rule| rule.is_valid(value)))
        })
        .collect();

    let mut remaining_rules = FxHashSet::default();
    remaining_rules.reserve(rules.len());
    for rule in rules.iter() {
        remaining_rules.insert(rule);
    }

    // step 2: skip values that match _every_ rule
    let mut values_by_field = FxHashMap::default();
    values_by_field.reserve(my_ticket.values.len());
    for ticket in valid_tickets.iter() {
        for (field, value) in ticket.values.iter().enumerate() {
            let mut always_valid = true;
            for rule in remaining_rules.iter() {
                if !rule.is_valid(value) {
                    always_valid = false;
                    break;
                }
            }
            if !always_valid {
                let field_values = values_by_field
                    .entry(field)
                    .or_insert_with(FxHashSet::default);
                field_values.insert(value);
            }
        }
    }

    let mut mappings = FxHashMap::default();

    loop {
        let mut invalid_per_field_per_rule = FxHashMap::default();
        invalid_per_field_per_rule.reserve(values_by_field.len());
        for (field, values) in values_by_field.iter() {
            let mut invalid_per_rule = FxHashMap::default();
            invalid_per_rule.reserve(remaining_rules.len());
            for rule in remaining_rules.iter() {
                let count =
                    values.iter().fold(
                        0,
                        |acc, value| if !rule.is_valid(value) { acc + 1 } else { acc },
                    );
                invalid_per_rule.insert(*rule, count);
            }
            invalid_per_field_per_rule.insert(*field, invalid_per_rule);
        }

        let mut changes = 0;

        for (field, invalid_per_rule) in invalid_per_field_per_rule.iter() {
            let sum: usize = invalid_per_rule.iter().map(|(_, v)| v).sum();
            if (invalid_per_rule.len() - sum) == 1 {
                for (rule, count) in invalid_per_rule {
                    if count == &0 {
                        mappings.insert(*rule, *field);
                        remaining_rules.remove(*rule);
                        values_by_field.remove(field);
                        changes += 1;
                        break;
                    }
                }
            }
        }

        if changes == 0 {
            break;
        }
    }

    let mut result = 1;

    for (rule, field) in mappings {
        if rule.name.len() > 9 && &rule.name[..9] == "departure" {
            result *= my_ticket.values[field] as u64;
        }
    }

    Some(result)
}
