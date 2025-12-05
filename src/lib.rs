use unicode_width::UnicodeWidthChar;

#[derive(Debug, Default)]
pub enum DecoType {
    #[default]
    Normal,
}

pub trait Kakomi {
    fn decorate(&self, deco: DecoType) -> String;
}

impl Kakomi for String {
    fn decorate(&self, deco: DecoType) -> String {
        match deco {
            DecoType::Normal => normal_deco(self),
        }
    }
}

fn get_str_len<T: AsRef<str>>(strg: T) -> u32 {
    let mut len = 0;
    for c in strg.as_ref().chars() {
        len += c.width().unwrap_or(0);
    }
    len as u32
}

fn mul_str<T: AsRef<str>>(msg: &T, mul: u32) -> String {
    let mut res = String::new();
    for _ in 0..mul {
        res.push_str(msg.as_ref());
    }
    res
}

fn get_max_len<T: AsRef<str>>(strg: T) -> u32 {
    let mut max = 0;
    for l in strg.as_ref().lines() {
        let len = get_str_len(l);
        if max < len {
            max = len;
        }
    }
    max
}

fn normal_deco<T: AsRef<str>>(s: T) -> String {
    let max_chars = get_max_len(&s);
    let start_line = format!("╭{}╮", mul_str(&"─", max_chars + 2));
    let end_line = format!("╰{}╯", mul_str(&"─", max_chars + 2));

    let mut result = start_line;
    for l in s.as_ref().lines() {
        let rem = max_chars - get_str_len(l);
        let fill_space = mul_str(&" ", rem);
        result.push_str(format!("\n│ {l}{fill_space} │").as_str());
    }
    result.push_str(format!("\n{}", end_line.as_str()).as_str());
    result
}

#[cfg(test)]
mod tests {
    use crate::{DecoType, Kakomi};

    #[test]
    fn normal_test() {
        let content = "foo".to_string();
        let formatted = content.decorate(DecoType::default());

        assert_eq!("╭─────╮\n│ foo │\n╰─────╯", formatted);
    }
}
