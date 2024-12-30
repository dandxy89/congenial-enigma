use std::{borrow::Cow, collections::HashMap};

use nom::{branch::alt, error::ErrorKind, sequence::tuple, IResult};

use crate::nom::{
    decoder::{
        constraint::{parse_constraint_header, parse_constraints},
        objective::parse_objectives,
        problem_name::parse_problem_name,
        sense::parse_sense,
    },
    model::{Constraint, Objective, Sense, Variable},
};

#[cfg_attr(feature = "diff", derive(diff::Diff), diff(attr(#[derive(Debug, PartialEq)])))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, PartialEq)]
pub struct LPProblem<'a> {
    pub name: Option<&'a str>,
    pub sense: Sense,
    pub objectives: HashMap<&'a str, Objective<'a>>,
    pub constraints: HashMap<Cow<'a, str>, Constraint<'a>>,
    pub variables: HashMap<&'a str, Variable<'a>>,
}

impl LPProblem<'_> {
    #[inline]
    /// Returns the name of the LP Problem
    pub fn name(&self) -> Option<&str> {
        self.name
    }

    #[inline]
    /// Returns `true` if the `Self` a Minimize LP Problem
    pub fn is_minimization(&self) -> bool {
        self.sense.is_minimization()
    }

    #[inline]
    /// Returns the number of constraints contained within the Problem
    pub fn constraint_count(&self) -> usize {
        self.constraints.len()
    }

    #[inline]
    /// Returns the number of objectives contained within the Problem
    pub fn objective_count(&self) -> usize {
        self.objectives.len()
    }
}

fn take_until_no_case<'a>(tag: &'a str) -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
    move |input: &str| {
        let mut index = 0;
        let tag_lower = tag.to_lowercase();
        let chars: Vec<char> = input.chars().collect();

        while index <= chars.len() - tag.len() {
            let window: String = chars[index..index + tag.len()].iter().collect();
            if window.to_lowercase() == tag_lower {
                return Ok((&input[index..], &input[..index]));
            }
            index += 1;
        }

        Err(nom::Err::Error(nom::error::Error::new(input, ErrorKind::TakeUntil)))
    }
}

impl<'a> TryFrom<&'a str> for LPProblem<'a> {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        // Extract the Sense, problem name and objectives slice
        let (input, (name, sense, obj_section, _cons_header)) = tuple((
            parse_problem_name,
            parse_sense,
            // First find where the constraint section starts by looking for any valid header
            alt((take_until_no_case("subject to"), take_until_no_case("such that"), take_until_no_case("s.t."), take_until_no_case("st:"))),
            parse_constraint_header,
        ))(input)?;

        // Parse objectives from the section before constraints
        let (_, (objs, mut variables)) = parse_objectives(obj_section)?;

        // Parse the constraints
        let (_remaining, (constraints, constraint_vars)) = parse_constraints(input)?;
        variables.extend(constraint_vars);

        // Parse Variable Bounds (Integer, General, Bounded, Free, Semi-continuous and SOS)
        //

        Ok(LPProblem { name, sense, objectives: objs, constraints, variables })
    }
}

#[cfg(test)]
mod test {
    const SMALL_INPUT: &str = "\\ This file has been generated by Author
\\ ENCODING=ISO-8859-1
\\Problem name: diet
Minimize
 obj1: -0.5 x - 2y - 8z
 obj2: y + x + z
 obj3: 10z - 2.5x
       + y
subject to:
c1:  3 x1 + x2 + 2 x3 = 30
c2:  2 x1 + x2 + 3 x3 + x4 >= 15
c3:  2 x2 + 3 x4 <= 25";

    #[cfg(feature = "serde")]
    #[test]
    fn test_minified_example() {
        let problem = crate::nom::lp_problem::LPProblem::try_from(SMALL_INPUT).expect("test case not to fail");

        assert_eq!(problem.objectives.len(), 3);
        assert_eq!(problem.constraints.len(), 3);
        insta::assert_yaml_snapshot!(&problem, {
            ".objectives" => insta::sorted_redaction(),
            ".constraints" => insta::sorted_redaction(),
            ".variables" => insta::sorted_redaction()
        });
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialization_lifecycle() {
        let problem = crate::nom::lp_problem::LPProblem::try_from(SMALL_INPUT).expect("test case not to fail");

        // Serialized
        let serialized_problem = serde_json::to_string(&problem).expect("test case not to fail");

        // Deserialize
        let _: crate::nom::lp_problem::LPProblem<'_> = serde_json::from_str(&serialized_problem).expect("test case not to fail");
    }
}
