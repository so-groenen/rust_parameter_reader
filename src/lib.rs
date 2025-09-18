use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ParameterReader
{
    content: String,
    parameters: &'static[&'static str]
}

#[derive(Debug, PartialEq)]
pub enum ParameterError
{
    MissingParam(Vec<String>),
    BadDelimiter(String),
    ReadContentError(String),
}

impl std::fmt::Display for ParameterError 
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result 
    {
        match self
        {
            ParameterError::MissingParam(missing_params) =>  
            {
                let missing_params = missing_params.join(", ");
                write!(f, "ParameterReaderError: Missing parameters: \"{missing_params}\"")
            }   
            ParameterError::BadDelimiter(line) => write!(f, "ParameterReaderError: Bad Delimiter at: \"{line}\""),
            ParameterError::ReadContentError(io_runtime_error) => write!(f, "{io_runtime_error}"),
        }
    }
}


impl ParameterReader
{
    pub fn build(file_name: &str, parameters: &'static[&'static str]) -> Result<Self, ParameterError>
    {
        let content   = match fs::read_to_string(file_name)
        {
            Ok(value) => value,
            Err(err)  => return Err(ParameterError::ReadContentError(err.to_string()))
        };
        
        let reader = Self {content, parameters};
        Ok(reader)
    }

    pub fn parse_parameters(&self, delimiter: &'static str) -> Result<HashMap<&'static str, String>, ParameterError>
    {
        let mut parameter_map: HashMap<&'static str, String> = HashMap::new();

        for line in self.content.lines()
        {
            for name in self.parameters
            {
                if line.contains(name)
                {
                    let Some((_, value)) = line.split_once(delimiter) else
                    {
                        return Err(ParameterError::BadDelimiter(line.to_string()));
                    };
                    let value = value.trim();

                    parameter_map.insert(name, value.to_owned());
                }
            }
        }
        let missing              = self.parameters.iter().filter(|&&name| !parameter_map.contains_key(name));
        let missing: Vec<String> = missing.map(|&name| name.to_string()).collect();

        if !missing.is_empty()
        {
            return Err(ParameterError::MissingParam(missing));
        }
        Ok(parameter_map)
    }
}
 
#[cfg(test)]
mod tests {
    use super::*;
    const PARAMS: [&str; 4] =
    [
        "my_float",
        "my_int",
        "my_array",
        "my_bool"];

    const TEST_FOLDER: &str = "test_files";
    const DELIM: &str       = ":";
    const ARRAY_SPLIT: &str = ",";
    
    
    #[test]
    fn it_works() 
    {
        let mut file_name = std::path::PathBuf::new();
        file_name.push(TEST_FOLDER);
        file_name.push("test_good_format.txt");

        let file_name = file_name.display().to_string();
        let reader    = ParameterReader::build(&file_name, &PARAMS);
        assert!(reader.is_ok(), "Reader is ok!");

        let reader = reader.unwrap();

        let parameters = reader.parse_parameters(DELIM);
        assert!(parameters.is_ok(), "Parameters should be ok!");
        let parameters = parameters.unwrap();

        let my_float = parameters["my_float"].parse::<f32>();
        assert!(my_float.is_ok(), "my_float should be ok!");
        let my_float = my_float.unwrap();

        let my_int = parameters["my_int"].parse::<i32>();
        assert!(my_int.is_ok(), "my_int should be ok!");
        let my_int = my_int.unwrap();

        let my_bool = parameters["my_bool"].parse::<bool>();
        assert!(my_bool.is_ok(), "my_bool should be ok!");
        let my_bool = my_bool.unwrap();

        let my_array = parameters["my_array"].split(ARRAY_SPLIT).map(|val| val.trim().parse::<i32>().expect("array should be ok"));
        let my_array: Vec<i32> = my_array.collect();

        dbg!(my_float);
        dbg!(my_int);
        dbg!(my_bool);
        dbg!(my_array);
    }


    #[test]
    fn bad_delim() 
    {
        let mut file_name = std::path::PathBuf::new();
        file_name.push(TEST_FOLDER);
        file_name.push("test_bad_delim.txt");

        let error_line = "my_int= 42".to_string();

        let file_name = file_name.display().to_string();
        let reader    = ParameterReader::build(&file_name, &PARAMS);
        assert!(reader.is_ok(), "Reader should be ok!");

        let reader = reader.unwrap();

        let parameters = reader.parse_parameters(DELIM);
        assert!(parameters.is_err_and(|e| e == ParameterError::BadDelimiter(error_line)), "Parameters should be BadDelim!");
    }

    #[test]
    fn missing_bool() 
    {
        let mut file_name = std::path::PathBuf::new();
        file_name.push(TEST_FOLDER);
        file_name.push("test_missing_bool.txt");

        let file_name = file_name.display().to_string();
        let reader    = ParameterReader::build(&file_name, &PARAMS);
        assert!(reader.is_ok(), "Reader should be ok!");

        let reader = reader.unwrap();

        let parameters = reader.parse_parameters(DELIM);
        assert!(parameters.is_err(), "Parameters should be missing!");
        
        let error = parameters.unwrap_err();
        match error
        {
            ParameterError::MissingParam(mut missing_names) => 
            {
                let missing = missing_names.pop().expect("Missing parameter should be here!");
                assert_eq!(missing.clone(), "my_bool".to_string(), "The missing parameter should be \"my_bool\"");
                dbg!(missing);
            }
            _ => panic!("Error must be of MissingParam type!")
        }
    }

     #[test]
    fn non_existant_file() 
    {
        let mut file_name = std::path::PathBuf::new();
        file_name.push(TEST_FOLDER);
        file_name.push("i_love_javascript.txt");

        let file_name = file_name.display().to_string();
        let reader    = ParameterReader::build(&file_name, &PARAMS);
        assert!(reader.is_err(), "Reader should be error!");

        let error = reader.unwrap_err();
        match error
        {
            ParameterError::ReadContentError(run_time_error) => 
            {
                dbg!(run_time_error);
            },
            _ => panic!("Error must be ReadContentError!")    
        }
      
    }
}
