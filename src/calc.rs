use crate::consts::ZHI_DICT;
use crate::palais::Mingpan;
use crate::structs::Birthday;
use crate::tui::display_palais;

pub fn build_palais(info: (i32, u32, u32, f64, bool)) {
    let birth = Birthday::from_gregorian(info);
    //println!("{}", birth);

    let birth_hour_idx = *ZHI_DICT.get(birth.hour.zhi()).unwrap();
    let birth_day_idx = birth.lunar.day() as usize;
    let birth_month_idx = birth.lunar.month() as usize;
    let birth_year_gan_idx = birth.year.0 as usize;
    let birth_year_zhi_idx = birth.year.1 as usize;
    let is_clockwise;
    match birth.gender.as_ref() {
        "陽男" | "陰女" => is_clockwise = true,
        "陰男" | "陽女" => is_clockwise = false,
        _ => unreachable!(),
    }

    let mp = Mingpan::default()
        .with_info(&birth)
        .with_tiangan_name(birth_year_gan_idx)
        .with_shenming_palais(birth_month_idx, birth_hour_idx)
        .with_wuxingju()
        .with_ziwei(birth_day_idx)
        .with_tianfu()
        .with_hour_based(birth_hour_idx, birth_year_zhi_idx)
        .with_month_based(birth_month_idx)
        .with_day_based(birth_day_idx, birth_month_idx, birth_hour_idx)
        .with_year_gan_based(birth_year_gan_idx, is_clockwise)
        .with_year_zhi_based(birth_year_zhi_idx)
        .with_changsheng(is_clockwise)
        .with_daxian(is_clockwise)
        .with_ms_zhuxing(birth_year_zhi_idx);

    match display_palais(mp) {
        Ok(_) => (),
        Err(e) => println!("Error occurred during display: {}", e),
    }
}
