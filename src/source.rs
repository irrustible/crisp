use nom_locate::LocatedSpan;

pub struct Int<'a>(LocatedSpan<&'a str, isize>);

pub struct Symbol<'a>(LocatedSpan<&'a str, String>);

pub struct Str<'a>(LocatedSpan<&'a str, String>);

pub struct List<'a> {
    open:  LocatedSpan<&'a str>,
    close: LocatedSpan<&'a str>,
}
