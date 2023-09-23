use lazy_static::lazy_static;
use std::collections::HashMap;

pub(crate) const GAN: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
pub(crate) const ZHI: [&str; 12] = [
    "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥",
];

lazy_static! {
    pub(crate) static ref GAN_DICT: HashMap<String, usize> = GAN
        .iter()
        .enumerate()
        .map(|(idx, &s)| (String::from(s), idx))
        .collect();
    pub(crate) static ref ZHI_DICT: HashMap<String, usize> = ZHI
        .iter()
        .enumerate()
        .map(|(idx, &s)| (String::from(s), idx))
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dict() {
        assert_eq!(GAN_DICT.get("丙"), Some(&2));
        assert_eq!(GAN_DICT.get("庚"), Some(&6));
        assert_eq!(ZHI_DICT.get("寅"), Some(&2));
        assert_eq!(ZHI_DICT.get("申"), Some(&8));
    }
}
