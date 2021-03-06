use super::Error;
use crate::expression::{Evaluation, Expression, Value};
use crate::metadata;
use std::ops::Deref;
use std::path::Path;

pub fn filename<T: metadata::Provider>(
    expressions: &[Box<Expression<T>>],
    provider: &T,
) -> Result<Evaluation, Error> {
    if expressions.len() < 1 {
        return Err(Error::ArgumentError);
    }
    let (original_string, truth) = expect_string_result!(&expressions[0], provider);
    let file_path = Path::new(original_string.as_str());
    let result_text: String = {
        match file_path.file_stem() {
            Some(file_stem) => file_stem.to_string_lossy().deref().to_owned(),
            None => "".to_owned(),
        }
    };
    Ok(Evaluation::new(Value::Text(result_text), truth))
}

#[cfg(test)]
mod test;
