use crate::consts::{GAN, PALAIS, ZHI};
use crate::structs::Birthday;

#[derive(Debug, Default)]
pub(crate) struct Palais {
    pub(crate) name: String,
    pub(crate) gz_name: String,
    pub(crate) daxian: String,
    pub(crate) xiaoxian: String,
    pub(crate) stars_a: Vec<String>,
    pub(crate) stars_b: Vec<String>,
    pub(crate) stars_c: Vec<String>,
}

#[derive(Debug, Default)]
pub(crate) struct Mingpan {
    pub(crate) all_palais: [Palais; 12],
    pub(crate) info: String,
}

impl Mingpan {
    pub(crate) fn with_info(mut self, birth: &Birthday) -> Self {
        self.info = format!("{}", birth);
        self
    }

    /// 定十二宫天干
    pub(crate) fn with_tiangan_name(mut self, year_gan_idx: usize) -> Self {
        let start_idx = ((year_gan_idx % 5) * 2 + 2) % 10;

        (0..12).into_iter().for_each(|idx| {
            self.all_palais[(2 + idx) % 12].gz_name =
                format!("{}{}", GAN[(start_idx + idx) % 10], ZHI[(2 + idx) % 12])
        });
        self
    }

    /// 安命身宫及十二宫名
    pub(crate) fn with_shenming_palais(mut self, month: usize, hour: usize) -> Self {
        let zishi_idx = (1 + month) % 12;
        let ming_idx = (zishi_idx + 12 - hour) % 12;
        let shen_idx = (zishi_idx + hour) % 12;

        (0..12).into_iter().for_each(|idx| {
            let curr_idx = (ming_idx + idx) % 12;
            let mut curr_name = String::from(PALAIS[idx]);
            if curr_idx == shen_idx {
                curr_name.push_str("-身宫");
            }
            self.all_palais[curr_idx].name = curr_name;
        });
        self
    }
}
