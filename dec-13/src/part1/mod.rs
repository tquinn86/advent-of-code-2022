pub enum PairType {
    Int(i32),
    Arr(Vec<PairType>)
}

impl PairType {
    pub fn parse_array(line: &str) -> PairType {
        println!("{}", line);
        // collect the chars into a vec:
        let chars: Vec<(usize, char)> = line.char_indices().collect();
        //we're parsing an array, the length better be > 0
        //and the first char should be [
        if chars.len() == 0 || chars[0].1 != '[' {
            panic!("Invalid array: {}", line);
        }

        let mut contents: Vec<PairType> = vec![];
        let mut i = 1;
        while i < chars.len() {
            if chars[i].1 == '[' {
                contents.push(Self::parse_array(&line[chars[i].0..]));
                while chars[i].1 != ',' && chars[i].1 != ']' { i += 1; }
            } else if chars[i].1 == ',' || chars[i].1 == ']' {
                //comma after an array, or array of array
                i += 1;
            } else {
                contents.push(Self::parse_int(&line[chars[i].0..]));
                while chars[i].1 != ',' && chars[i].1 != ']' { i += 1; }
            }
            i += 1;
        }
        PairType::Arr(contents)
    }

    fn parse_int(line: &str) -> PairType {
        println!("{}", line);
        let chars: Vec<(usize, char)> = line.char_indices().collect();
        //we're parsing an int, the length better be > 0
        //and the first char should be a digit
        if chars.len() == 0 || !chars[0].1.is_ascii_digit() {
            panic!("Invalid int: {}", line);
        }

        let mut number = chars[0].1.to_string();
        for i in 1..chars.len() {
            if chars[i].1.is_ascii_digit() {
                number += &chars[i].1.to_string();
            } else if chars[i].1 == ',' || chars[i].1 == ']' {
                break;
            } else {
                panic!("Invalid int: {}", line);
            }
        }
        PairType::Int(number.parse::<i32>().unwrap())
    }
}