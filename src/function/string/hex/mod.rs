use super::{ Function, Error };
use ::metadata;
use ::expression::{ Expression, Evaluation, Value };

fn hex<T: metadata::Provider>(expressions: &[Box<Expression<T>>], provider: &T) -> Result<Evaluation, Error> {
    if expressions.len() < 1 {
        return Err(Error::ArgumentError);
    }
    let (int_value, truth) = expect_integer_result!(&expressions[0], provider);
    let mut result_text: String = format!("{:X}", int_value);
    if expressions.len() >= 2 {
        if let Some((zero_padding_length, _)) = try_integer_result!(&expressions[1], provider, usize) {
            let char_count: usize = result_text.chars().count();
            let padding_count = if zero_padding_length > char_count {
                zero_padding_length - char_count
            } else {
                0
            };
            result_text = format!("{}{}", "0".repeat(padding_count), result_text);
        }
    }
    Ok(Evaluation::new(Value::Text(result_text), truth))
}

function_object_maker!(hex);

#[cfg(test)]
mod test;
