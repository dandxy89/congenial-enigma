use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{line_ending, multispace0, not_line_ending},
    combinator::recognize,
    multi::many0,
    sequence::tuple,
    IResult,
};

#[inline]
fn parse_single_comment(input: &str) -> IResult<&str, &str> {
    let (input, comment_start) = alt((tag("\\\\*"), tag("\\*"), tag("\\")))(input)?;
    let (input, content) = match comment_start {
        "\\\\*" | "\\*" => {
            let (i, content) = recognize(take_until("*\\"))(input)?;
            let (i, _) = tuple((tag("*\\"), multispace0))(i)?;
            (i, content)
        }
        "\\" => {
            let (i, content) = recognize(not_line_ending)(input)?;
            let (i, _) = line_ending(i)?;
            (i, content)
        }
        _ => unreachable!(),
    };
    Ok((input, content))
}

#[inline]
pub fn parse_comments(input: &str) -> IResult<&str, Option<&str>> {
    let (remaining, comments) = many0(parse_single_comment)(input)?;
    let last_comment = comments.last().copied();
    Ok((remaining, last_comment))
}

#[cfg(test)]
mod test {
    use super::parse_comments;

    #[test]
    fn test_parse_lp_file_comments() {
        let input = "\\* First comment *\\\n\\ENCODING=ISO-8859-1\n\\* Middle comment *\\\\Problem name: ilog.cplex\n\\* Last comment *\\";
        let (remainder, x) = parse_comments(input).unwrap();
        assert_eq!("", remainder);
        assert!(x.is_some());
    }
}
