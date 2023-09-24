use crate::consts::ZHI_DICT;
use crate::palais::Mingpan;
use crate::structs::Birthday;
use crate::tui::display_palais;

pub fn build_palais(info: (i32, u32, u32, f64, bool)) {
    let birth = Birthday::from_gregorian(info);
    //println!("{}", birth);

    let mp = Mingpan::default()
        .with_info(&birth)
        .with_tiangan_name(birth.year.0 as usize)
        .with_shenming_palais(
            birth.lunar.month() as usize,
            *ZHI_DICT.get(birth.hour.zhi()).unwrap(),
        )
        .with_wuxingju()
        .with_ziwei(birth.lunar.day() as usize)
        .with_tianfu();

    match display_palais(mp) {
        Ok(_) => (),
        Err(e) => println!("Error occurred during display: {}", e),
    }
}
