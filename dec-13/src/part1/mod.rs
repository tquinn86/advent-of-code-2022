use std::fmt;
use std::fmt::Display;
use std::cmp::Ordering;

pub enum PairType {
    Int(i32),
    Arr(Vec<PairType>)
}

impl PairType {
    pub fn parse_array(line: &str) -> (usize, PairType) {
        //println!("{}", line);
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
                let result = Self::parse_array(&line[chars[i].0..]);
                contents.push(result.1);
                i += result.0;
            } else if chars[i].1 == ',' {
                //conusme the comma
                i += 1;
            } else if chars[i].1 == ']' {
                //consume the bracket and break
                i += 1;
                break;
            } else {
                let result = Self::parse_int(&line[chars[i].0..]);
                contents.push(result.1);
                i += result.0;
            }
        }
        (i, PairType::Arr(contents))
    }

    pub fn is_arr(&self) -> bool {
        match self {
            PairType::Arr(_) => true,
            _ => false
        }
    }

    pub fn is_int(&self) -> bool {
        match self {
            PairType::Int(_) => true,
            _ => false
        }
    }

    fn parse_int(line: &str) -> (usize, PairType) {
        //println!("{}", line);
        let chars: Vec<(usize, char)> = line.char_indices().collect();
        //we're parsing an int, the length better be > 0
        //and the first char should be a digit
        if chars.len() == 0 || !chars[0].1.is_ascii_digit() {
            panic!("Invalid int: {}", line);
        }

        let mut number = chars[0].1.to_string();
        let mut i = 1;
        while i < chars.len() {
            if chars[i].1.is_ascii_digit() {
                number += &chars[i].1.to_string();
            } else if chars[i].1 == ',' || chars[i].1 == ']' {
                break;
            } else {
                panic!("Invalid int: {}", line);
            }
            i += 1;
        }
        (i, PairType::Int(number.parse::<i32>().unwrap()))
    }

    //comparisons of Int and Arr need to turn
    //Ints into Arrs so helper to do just that
    fn into_arr(&self) -> Self {
        if let PairType::Int(iself) = self {
            let v = vec![PairType::Int(*iself)];
            PairType::Arr(v)
        }
        else{
            panic!("This method only works for PairType::Int");
        }
    }
}

impl Display for PairType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PairType::Int(i) => write!(f, "{}", i),
            PairType::Arr(c) => {
                let mut i = 0;
                let mut res = write!(f, "[");
                if res.is_err() { return res; }
                while i < c.len() {
                    res = write!(f, "{}", c[i]);
                    if res.is_err() { return res; }

                    if i < c.len() - 1 {
                        res = write!(f, ",");
                        if res.is_err() { return res; }
                    }
                    i += 1;
                }
                write!(f, "]")
            }
        }
    }
}

impl PartialEq for PairType {
    fn eq(&self, other: &PairType) -> bool {
        match self {
            PairType::Int(iself) => {
                if let PairType::Int(iother) = other {
                    return iself == iother;
                } else {
                    //turn self into an Arr and compare
                    //recursively
                    let aself = self.into_arr();
                    return aself == *other;
                }
            },
            PairType::Arr(aself) => {
                if let PairType::Arr(aother) = other {
                    if aself.len() == aother.len() {
                        //call this recursively on each item
                        for i in 0..aself.len() {
                            if aself[i] != aother[i] {
                                return false;
                            }
                        }
                        return true;
                    } else {
                        return false;
                    }
                } else {
                    //turn other into Arr and compare recursively
                    let aother = other.into_arr();
                    return *self == aother;
                }
            }
        }
    }
}

impl PartialOrd for PairType {
    fn partial_cmp(&self, other: &PairType) -> Option<Ordering> {
        match self {
            PairType::Int(iself) => {
                if let PairType::Int(iother) = other {
                    return Some(iself.cmp(iother));
                } else {
                    //make iself into a Arr and compare recursively
                    let aself = self.into_arr();
                    return aself.partial_cmp(other);
                }
            },
            PairType::Arr(aself) => {
                if let PairType::Arr(aother) = other {
                    let mut i_max = aself.len();
                    let mut tentative_order = Some(Ordering::Equal);
                    if aself.len() > aother.len() {
                        tentative_order = Some(Ordering::Greater);
                        i_max = aother.len();
                    } else if aother.len() > aself.len() {
                        tentative_order = Some(Ordering::Less);
                    }

                    //now loop through, if we find unequal values,
                    //return that, otherwise keep going.
                    for i in 0..i_max {
                        let res = aself[i].partial_cmp(&aother[i]);
                        if res != Some(Ordering::Equal) {
                            return res;
                        }
                    }
                    tentative_order
                } else {
                    //convert other into an Arr and call recursively
                    let aother = other.into_arr();
                    self.partial_cmp(&aother)
                }
            }
        }
    }
}