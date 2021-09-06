// use smartstring::alias::*;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, one_of},
    combinator::{map_res, recognize},
    multi::{many0, many_till},
    sequence::{preceded, terminated},
};
pub use nom_programming_syntax::{ints::parse_int, Span, Spanning};

#[derive(Debug)]
pub struct Group<'a, T> {
    pub open:   Span<'a>,
    pub close:  Span<'a>,
    pub values: Vec<T>,
}

pub fn parse_group<'a, S, T, I>(
    open: &str,
    close: &str,
    skip_space: S,
    item: I,
    input: Span<'a>
) -> IResult<Span<'a>, Group<'a, T>>
where S: Fn(Span<'a>) -> Span<'a>,
      I: Fn(Span<'a>) -> IResult<Span<'a>, T> {
    let (input, open) = tag(open)(skip_space(input))?;
    let (input, (values, close)) =
        many_till(item, |input| tag(close)(skip_space(input)))(input)?;
    let group = Group { open, close, values };
    Ok((input,group))
}

// pub fn parse_
