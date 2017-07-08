use super::*;

pub struct MetadataProvider<'a> {
    metadata_dict: HashMap<&'a str, &'a str>,
}

impl<'a> metadata::Provider for MetadataProvider<'a> {
    fn tag_value(&self, key: &str) -> Option<String> {
        let entry = self.metadata_dict.get(key);
        if let Some(value) = entry {
            let s = value.to_string();
            Some(s)
        }
        else {
            None
        }
    }
}

impl<'a> MetadataProvider<'a> {
    pub fn new(metadata_dict: HashMap<&'a str, &'a str>) -> MetadataProvider<'a> {
        MetadataProvider {
            metadata_dict,
        }
    }
}

#[test]
fn test_parser_optional() {
    let formatter = Formatter::new();
    // tests with optional expressions
    {
        let expression = formatter.parser().parse("%tracknumber%. %title%[ (%composer%)]").unwrap();
        {
            let test_metadata = {
                let mut dict = HashMap::new();
                dict.insert("tracknumber", "9");
                dict.insert("title", "9th Symphony");
                dict.insert("composer", "Beethoven");
                MetadataProvider::new(dict)
            };
            let s = expression.apply(&test_metadata);
            assert_eq!("09. 9th Symphony (Beethoven)", s.to_string().as_str());
        }
        {
            let test_metadata = {
                let mut dict = HashMap::new();
                dict.insert("tracknumber", "5");
                dict.insert("title", "Greensleeves");
                MetadataProvider::new(dict)
            };
            let s = expression.apply(&test_metadata);
            assert_eq!("05. Greensleeves", s.to_string().as_str());
        }
    }
    {
        let expression = formatter.parser().parse("%tracknumber%. %title%[ (%composer%)[ '['%testfield%']'] - hop]").unwrap();
        {
            let test_metadata = {
                let mut dict = HashMap::new();
                dict.insert("tracknumber", "9");
                dict.insert("title", "9th Symphony");
                dict.insert("composer", "Beethoven");
                MetadataProvider::new(dict)
            };
            let s = expression.apply(&test_metadata);
            assert_eq!("09. 9th Symphony (Beethoven) - hop", s.to_string().as_str());
        }
        {
            let test_metadata = {
                let mut dict = HashMap::new();
                dict.insert("tracknumber", "5");
                dict.insert("title", "Greensleeves");
                dict.insert("testfield", "OK");
                MetadataProvider::new(dict)
            };
            let s = expression.apply(&test_metadata);
            assert_eq!("05. Greensleeves (?) [OK] - hop", s.to_string().as_str());
        }
        {
            let test_metadata = {
                let mut dict = HashMap::new();
                dict.insert("tracknumber", "5");
                dict.insert("title", "Greensleeves");
                MetadataProvider::new(dict)
            };
            let s = expression.apply(&test_metadata);
            assert_eq!("05. Greensleeves", s.to_string().as_str());
        }
    }
}
