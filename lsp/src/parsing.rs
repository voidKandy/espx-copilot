use lsp_types::Position as LspPos;
use nom::{
    self,
    bytes::complete::{take_till, take_until},
    character::is_newline,
    sequence::preceded,
    IResult,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Position {
    UserPrompt(String),
    // AttributeValue { name: String, value: String },
}

pub const PREFIX: &str = "#$";

pub fn parse_for_prompt(input: &str) -> IResult<&str, &str> {
    let (r, o) = preceded(
        take_until(PREFIX),
        take_till(|c| is_newline(c as u8)), // tag("p"),
    )(input)?;
    let o = o.strip_prefix(PREFIX).unwrap();
    Ok((r, o))
}

pub fn get_prompt_and_position_on_line(input: &str, line: usize) -> Option<(String, LspPos)> {
    if let Some(l) = input.lines().nth(line) {
        if let Some(idx) = l.find(PREFIX) {
            let (_, o) = parse_for_prompt(l).unwrap();
            let pos = LspPos {
                line: line as u32,
                character: (idx + o.len()) as u32,
            };
            return Some((o.to_string(), pos));
        }
    }
    None
}

pub fn get_all_prompts_and_positions(input: &str) -> Vec<(String, LspPos)> {
    let mut tuple_vec = vec![];
    for (i, l) in input.lines().into_iter().enumerate() {
        if let Some(idx) = l.find(PREFIX) {
            let (_, o) = parse_for_prompt(l).unwrap();
            let pos = LspPos {
                line: i as u32,
                character: (idx + o.len()) as u32,
            };
            tuple_vec.push((o.to_string(), pos));
        }
    }
    tuple_vec
}

// pub fn parse_for_prompt_prefix<'a>(i: &'a str) -> IResult<&'a str, &'a str> {
//     terminated(tag("π"), is_not("\n\r")).parse(i)
// }

// pub fn get_position_from_lsp_completion(
//     text_params: &TextDocumentPositionParams,
// ) -> Option<Position> {
//     debug!(
//         "get_position_from_lsp_completion: uri {}",
//         text_params.text_document.uri
//     );
//     let text = get_text_document_current(&text_params.text_document.uri)?;
//     debug!("get_position_from_lsp_completion: text {}", text);
//     let pos = text_params.position;
//     debug!("get_position_from_lsp_completion: pos {:?}", pos);
//
//     match parse_for_prompt(&text) {
//         Ok((_, out)) => {
//             // debug!("Parsed text output!: {:?}", t);
//             Some(Position::UserPrompt(out.to_string()))
//         }
//         Err(err) => {
//             debug!("Error parsing text: {:?}", err);
//             None
//         }
//     }
//
// }

#[cfg(test)]
mod tests {
    use crate::parsing::get_prompt_and_position_on_line;

    use super::parse_for_prompt;

    #[test]
    fn parse_for_prompt_gets_correct_prompt() {
        let input = "not a prompt #$ This is a prompt 
            notAprompt";
        // let input = "#$phij";
        let (i, o) = parse_for_prompt(&input).unwrap();
        println!("I: {},O: {}", i, o);
        assert_eq!(" This is a prompt", o);
        assert!(false);
    }

    #[test]
    fn output_pos_is_correct() {
        let input = "
not a prompt
Not a prompt
#$ This is a prompt 
notAprompt";
        let (_, pos) = get_prompt_and_position_on_line(input, 3usize).unwrap();
        println!("{:?}", pos);
        assert_eq!((3, 18), (pos.line, pos.character));
    }
}
