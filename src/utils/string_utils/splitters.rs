pub struct Splitters;
#[allow(dead_code)]
pub enum Split<'a> {
    Split(Vec<String>),
    IncompleteBrackets,
    Failed(&'a str),
}

#[allow(dead_code)]
impl Splitters {
    pub fn bracket(in_string: &str, br_open: char) -> Split {
        let open_brs = "({[<";
        let close_brs = ")}]>";
        let closing_bracket = match open_brs.find(br_open) {
            Some(i) => close_brs.as_bytes()[i] as char,
            None => {
                '\0'
            }
        };
        if closing_bracket == '\0' {
            return Split::Failed("Not a valid bracket");
        } else {
            let mut container = Vec::<String>::new();
            let mut form_vec = Vec::<String>::new();
            let mut check: u8 = 0;
            let mut nbuff: i16 = 0;
            for c in in_string.chars() {
                if check == 1 {
                    form_vec.push(c.to_string());
                }
                if c.eq(&br_open) {
                    check = 1;
                    nbuff+=1;
                }
                if c == closing_bracket {
                    nbuff-=1;
                    if nbuff == 0 {check = 0;}
                    if nbuff < 0 {
                        check = 1;
                        print!("b1");
                        break;
                    }
                }
                if check == 0 {
                    if !form_vec.is_empty() {
                        form_vec.pop();
                        container.push(form_vec.join("").to_string());
                        form_vec.clear();
                    }
                }
            }
            if check == 0 {
                return Split::Split(container);
            } else if nbuff >= 0 {
                    return Split::IncompleteBrackets;
            } else {
                return Split::Failed("Extra closing bracket quitting...");
            }
        }
    }

    pub fn dbreaker(string: &str, delimiter: char) -> Split {
        if delimiter.is_alphanumeric() {
            return Split::Failed("delimiter cannot be alpha numeric");
        }
        let mut form_vec = Vec::<String>::new();
        let mut container = Vec::<String>::new();
        let mut check: u8 = 0;
        for c in string.chars() {
            if c.eq(&'\'') {
                if check == 2 {}
                else if check == 1 {check = 0;}
                else if check == 0 {check = 1;}
            }
            if c.eq(&'"') {
                if check == 1 {}
                else if check == 2 {check = 0;}
                else if check == 0 {check = 2;}
            }
            if !(c.eq(&delimiter) && check == 0) {
                form_vec.push(c.to_string());
            }
            else {
                container.push(form_vec.join(""));
                form_vec.clear();
            }
        }
        if !form_vec.is_empty() {
            container.push(form_vec.join(""));
            form_vec.clear();
        }
        return Split::Split(container);
    }

    pub fn quote(string: &str, delimiter: char) -> Split {
        if delimiter.is_alphanumeric() {
            return Split::Failed("delimiter cannot be alpha-numeric");
        }
        let mut form_vec = Vec::<String>::new();
        let mut quote_vec = Vec::<String>::new();
        let mut container = Vec::<String>::new();
        let mut check: u8 = 0;
        let mut _previous_state: u8 = 0;
        for c in string.chars() {
            _previous_state = check;
            if c.eq(&'\'') {
                if check == 2 {}
                else if check == 1 {check = 0}
                else if check == 0 {check = 1}
            }
            else if c.eq(&'"') {
                if check == 1 {}
                else if check == 2 {check = 0}
                else if check == 0 {check = 2}
            }
            if check == 0 {
                if _previous_state != check {
                    container.push(quote_vec.join(""));
                    quote_vec.clear();
                }
                else if !c.eq(&delimiter){
                    form_vec.push(c.to_string());
                }
                else {
                    if !form_vec.is_empty() {
                        container.push(form_vec.join(""));
                        form_vec.clear();
                    }
                }
            }
            else {
                if !form_vec.is_empty() {
                    container.push(form_vec.join(""));
                    form_vec.clear();
                }
                if check == 2 && c.eq(&'"') {}
                if check == 1 && c.eq(&'\'') {}
                else {
                    quote_vec.push(c.to_string());
                }
            }
        }
        if !form_vec.is_empty() {
            container.push(form_vec.join(""));
        }
        return Split::Split(container);
    }
}
