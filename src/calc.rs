use crate::consts::GAN_DICT;
use crate::palais::Mingpan;
use crate::structs::Birthday;

pub fn build_palais(info: (i32, u32, u32, f64, bool)) {
    let birth = Birthday::from_gregorian(info);
    println!("{}", birth);

    let palais_gan_start_idx = ((birth.year.0 % 5) as usize * 2 + 2) % 10;

    let mp = Mingpan::default().with_tiangan_name(palais_gan_start_idx);
    println!("{:?}", mp);
}
