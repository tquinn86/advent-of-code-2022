pub enum PairType {
    Int(i32),
    Arr(Vec<PairType>)
}

impl PairType {
    fn parse_array(line: &str) -> PairType {
        if line.len() > 0 {
            PairType::Arr(vec![])
        } else {
            //empty vec is a thing
            PairType::Arr(vec![])
        }
    }
}