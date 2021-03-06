use super::Error;
use crate::expression::{Evaluation, Expression, Value};
use crate::metadata;

pub fn caps<T: metadata::Provider>(
    expressions: &[Box<Expression<T>>],
    provider: &T,
) -> Result<Evaluation, Error> {
    if expressions.len() != 1 {
        return Err(Error::ArgumentError);
    }
    let (original_string, truth) = expect_string_result!(&expressions[0], provider);
    let result_text = {
        let mut caps_next_character = true;
        let mut result_chars = Vec::new();
        for c in original_string.chars() {
            let is_whitespace = c.is_whitespace() || (c == '(');
            match (caps_next_character, is_whitespace) {
                (true, false) => {
                    result_chars.extend(c.to_uppercase());
                    caps_next_character = false;
                }
                (false, false) => {
                    result_chars.extend(c.to_lowercase());
                }
                (_, true) => {
                    result_chars.extend(c.to_lowercase());
                    caps_next_character = true;
                }
            }
        }
        result_chars.into_iter().collect()
    };
    Ok(Evaluation::new(Value::Text(result_text), truth))
}

#[cfg(test)]
mod test;
