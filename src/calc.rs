use crate::consts::ZHI_DICT;
use crate::palais::Mingpan;
use crate::structs::Birthday;
use crate::tui::display_palais;
use std::process::exit;

pub fn build_palais(info: (i32, u32, u32, f64, bool)) {
    let birth = Birthday::from_gregorian(info);
    //println!("{}", birth);

    let birth_hour_idx = *ZHI_DICT.get(birth.hour.zhi()).unwrap();
    let birth_day_idx = birth.lunar.day() as usize;
    let birth_month_idx = birth.lunar.month() as usize;
    let birth_year_gan_idx = birth.year.0 as usize;
    let birth_year_zhi_idx = birth.year.1 as usize;
    let is_clockwise = match birth.gender.as_ref() {
        "陽男" | "陰女" => true,
        "陰男" | "陽女" => false,
        _ => unreachable!(),
    };

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
        .with_ms_zhuxing(birth_year_zhi_idx)
        .with_xiaoxian(birth_year_zhi_idx, info.4);

    match display_palais(mp) {
        Ok(_) => (),
        Err(e) => println!("Error occurred during display: {}", e),
    }
}

pub fn birthday_from_arg(mut d: u64) -> (i32, u32, u32, f64, bool) {
    let gender = d % 10 != 0;

    d /= 10;
    let hour = d % 100;
    if hour > 24 {
        println!("Hour should between 1-24, actually got {hour}, quit");
        exit(1)
    }

    d /= 100;
    let day = d % 100;
    if day > 31 || day == 0 {
        println!("Day should between 1-31, actually got {day}, quit");
        exit(1)
    }

    d /= 100;
    let month = d % 100;
    if month > 12 || month == 0 {
        println!("Month should between 1-12, actually got {month}, quit");
        exit(1)
    }

    let year = d / 100;
    if year < 1900 {
        println!("Year should after 1900, actually got {year}, quit");
        exit(1)
    }
    (year as i32, month as u32, day as u32, hour as f64, gender)
}
