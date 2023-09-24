use crate::consts::ZHI_DICT;
use crate::palais::Mingpan;
use crate::structs::Birthday;
use crate::tui::display_palais;

pub fn build_palais(info: (i32, u32, u32, f64, bool)) {
    let birth = Birthday::from_gregorian(info);
    //println!("{}", birth);

    let birth_hour_idx = *ZHI_DICT.get(birth.hour.zhi()).unwrap();

    let mp = Mingpan::default()
        .with_info(&birth)
        .with_tiangan_name(birth.year.0 as usize)
        .with_shenming_palais(birth.lunar.month() as usize, birth_hour_idx)
        .with_wuxingju()
        .with_ziwei(birth.lunar.day() as usize)
        .with_tianfu()
        .with_hour_based(birth_hour_idx, birth.year.1 as usize);

    match display_palais(mp) {
        Ok(_) => (),
        Err(e) => println!("Error occurred during display: {}", e),
    }
}
