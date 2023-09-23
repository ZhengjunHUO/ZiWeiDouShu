use crate::consts::ZHI_DICT;
use crate::palais::Mingpan;
use crate::structs::Birthday;

pub fn build_palais(info: (i32, u32, u32, f64, bool)) {
    let birth = Birthday::from_gregorian(info);
    println!("{}", birth);

    let mp = Mingpan::default()
        .with_tiangan_name(birth.year.0 as usize)
        .with_shenming_palais(
            birth.lunar.month() as usize,
            *ZHI_DICT.get(birth.hour.zhi()).unwrap(),
        );
    println!("{:?}", mp);
}
