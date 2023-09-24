use crate::consts::{GAN, PALAIS, WUXINGJU, WUXINGJU_DICT, ZHI, ZIWEI_SYSTEM};
use crate::structs::Birthday;

#[derive(Debug, Default)]
pub(crate) struct Palais {
    pub(crate) name: String,
    pub(crate) gz_name: String,
    pub(crate) gz_idx: (usize, usize),
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
    pub(crate) ming_idx: usize,
    pub(crate) wxj_idx: usize,
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
                format!("{}{}", GAN[(start_idx + idx) % 10], ZHI[(2 + idx) % 12]);
            self.all_palais[(2 + idx) % 12].gz_idx = ((start_idx + idx) % 10, (2 + idx) % 12);
        });
        self
    }

    /// 安命身宫及十二宫名
    pub(crate) fn with_shenming_palais(mut self, month: usize, hour: usize) -> Self {
        let zishi_idx = (1 + month) % 12;
        let ming_idx = (zishi_idx + 12 - hour) % 12;
        self.ming_idx = ming_idx;
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

    /// 定五行局，dep: with_tiangan_name & with_shenming_palais
    pub(crate) fn with_wuxingju(mut self) -> Self {
        let gz = self.all_palais[self.ming_idx].gz_idx;
        let gan = gz.0;
        let mut zhi = gz.1;
        if zhi >= 6 {
            zhi = zhi - 6;
        }

        self.wxj_idx = *WUXINGJU_DICT.get(&(gan / 2, zhi / 2)).unwrap();
        self.info.push_str("\n");
        self.info.push_str(WUXINGJU[self.wxj_idx]);
        self
    }

    /// 安紫微星系，dep: with_wuxingju
    pub(crate) fn with_ziwei(mut self, day: usize) -> Self {
        let ju = self.wxj_idx + 2;
        let ziwei_idx;
        if day % ju == 0 {
            ziwei_idx = (day / ju + 1) % 12;
        } else {
            let delta = ju - day % ju;
            let temp = day / ju + 2;
            if delta % 2 == 1 {
                ziwei_idx = (temp - delta + 12) % 12;
            } else {
                ziwei_idx = (temp + delta) % 12;
            }
        }

        ZIWEI_SYSTEM.iter().for_each(|&(idx, star)| {
            self.all_palais[(ziwei_idx + idx) % 12]
                .stars_a
                .push(star.to_owned());
        });
        self
    }
}
