use super::*;
use super::value::Value;

fn if2<T: metadata::Provider>(provider: &T, expressions: &[Box<expression::Expression<T>>]) -> Result<Value, Error> {
    if expressions.len() != 2 {
        return Err(Error::TypeError);
    }
    let expr_value = expressions[0].apply(provider);
    match expr_value {
        Value::Empty | Value::Boolean(false) => Ok(expressions[1].apply(provider)),
        _ => Ok(expr_value),
    }
}

pub fn make_function_object<T: metadata::Provider>() -> super::Function<T> {
    Function::new(
        "if2",
        Box::new(|provider: &T, expressions: &[Box<expression::Expression<T>>]| -> Result<Value, Error> { if2(provider, expressions) })
    )
}

#[test]
fn test_function_if2()
{
    let formatter = super::super::Formatter::new();
    // tests with functions
    {
        let test_metadata = {
            let mut dict = HashMap::new();
            dict.insert("tracknumber", "9");
            dict.insert("title", "9th Symphony");
            dict.insert("composer", "Beethoven");
            super::super::tests::MetadataProvider::new(dict)
        };
        {
            let expression = formatter.parser().parse("%tracknumber%. $if2(%composer%, %tracknumber%) - %title%").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("9. Beethoven - 9th Symphony", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("%tracknumber%. $if2(%artist%, %composer%) - %title%").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("9. Beethoven - 9th Symphony", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("%tracknumber%. $if2(%composer%, %artist%) - %title%").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("9. Beethoven - 9th Symphony", s.to_string().as_str());
        }
        {
            let expression = formatter.parser().parse("%tracknumber%. $if2(%albumartist%, %artist%) - %title%").unwrap();
            let s = expression.apply(&test_metadata);
            assert_eq!("9. ? - 9th Symphony", s.to_string().as_str());
        }
    }
}