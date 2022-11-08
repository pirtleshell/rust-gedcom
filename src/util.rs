use crate::{
    tokenizer::{Token, Tokenizer},
    types::CustomData,
};

/// Macro for displaying `Option`s in debug mode without the text wrapping.
#[macro_export]
macro_rules! fmt_optional_value {
    ($debug_struct: ident, $prop: literal, $val: expr) => {
        if let Some(value) = $val {
            $debug_struct.field($prop, value);
        } else {
            $debug_struct.field($prop, &"None");
        }
    };
}

/// Debug function displaying GEDCOM line number of error message.
pub fn dbg(tokenizer: &Tokenizer) -> String {
    format!("line {}:", tokenizer.line)
}

/// Grabs and returns to the end of the current line as a String
pub fn take_line_value(tokenizer: &mut Tokenizer) -> String {
    let value: String;
    tokenizer.next_token();

    if let Token::LineValue(val) = &tokenizer.current_token {
        value = val.to_string();
    } else {
        panic!(
            "{} Expected LineValue, found {:?}",
            dbg(&tokenizer),
            tokenizer.current_token
        );
    }
    tokenizer.next_token();
    value
}

pub fn parse_custom_tag(tokenizer: &mut Tokenizer, tag: String) -> CustomData {
    let value = take_line_value(tokenizer);
    CustomData { tag, value }
}

/// Takes the value of the current line including handling
/// multi-line values from CONT & CONC tags.
pub fn take_continued_text(tokenizer: &mut Tokenizer, level: u8) -> String {
    let mut value = take_line_value(tokenizer);

    loop {
        if let Token::Level(cur_level) = tokenizer.current_token {
            if cur_level <= level {
                break;
            }
        }
        match &tokenizer.current_token {
            Token::Tag(tag) => match tag.as_str() {
                "CONT" => {
                    value.push('\n');
                    value.push_str(&take_line_value(tokenizer))
                }
                "CONC" => {
                    value.push(' ');
                    value.push_str(&take_line_value(tokenizer))
                }
                _ => panic!(
                    "{} Unhandled Continuation Tag: {}",
                    dbg(tokenizer),
                    tag
                ),
            },
            Token::Level(_) => tokenizer.next_token(),
            _ => panic!(
                "Unhandled Continuation Token: {:?}",
                tokenizer.current_token
            ),
        }
    }
    value
}
