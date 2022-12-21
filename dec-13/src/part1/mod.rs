pub enum PairType {
    Int(i32),
    Arr(Vec<PairType>)
}

impl PairType {
    fn parse_array(line: &str) -> PairType {
        //we're parsing an array, the length better be > 0
        //and the first char should be [
        if line.len() == 0 || line.chars().nth(0).unwrap() != '[' {
            panic("Invalid array: {}", line);
        }
        
        let contents: Vec<PairType> = vec![];

        PairType::Arr(contents)
    }
}