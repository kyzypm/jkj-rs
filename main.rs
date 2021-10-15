use std::io::{stdin, stdout, Write};

const HEAVENLY_STEM: &'static [&str] =
    &["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];

const EARTHLY_BRANCH: &'static [&str] = &[
    "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥",
];

const WILL_GOD: &'static [&str] = &[
    "贵人", "腾蛇", "朱雀", "六合", "勾陈", "青龙", "天空", "白虎", "太常", "玄武", "太阴", "天后",
];

const WILL_GOD_BRANCH: &'static [&str] = &[
    "丑", "巳", "午", "卯", "辰", "寅", "戌", "申", "未", "子", "酉", "亥",
];

#[derive(Debug)]
enum IndexType {
    HeavenlyStem,
    EarthlyBranch,
    WillGod,
}

fn index_of(p: &str, it: IndexType) -> usize {
    let mut j = 0;
    let x: &'static [&str] = match it {
        IndexType::HeavenlyStem => HEAVENLY_STEM,
        IndexType::EarthlyBranch => EARTHLY_BRANCH,
        IndexType::WillGod => WILL_GOD,
    };
    for i in x {
        if *i == p {
            return j;
        }
        j += 1;
    }
    0
}

#[derive(Debug)]
enum CheckType {
    SixTyJiaZi,
    DiZhi,
}

#[derive(Debug)]
struct MultipleData {
    sixty_coll: Vec<String>,
    ymdh: (u8, u8, u8, u8),
    yj: u8,
    df: u8,
}

#[derive(Debug)]
struct PaipanResult {
    df: u8,
    will_god: (u8, u8),
}

// 地盘，天盘，将神，将支，天干
type Mapping = (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
);

fn show_map(m: &Vec<Mapping>) {
    for i in m {
        println!("{} {} {} {} {}", i.0, i.1, i.2, i.3, i.4);
    }
}

fn get_item(items: &Vec<Mapping>, p: &str) -> (Mapping, usize) {
    let mut i = 0;
    while i < items.len() {
        let item = items.get(i).unwrap();
        if item.0 == p {
            return (*item, i);
        }
        i += 1;
    }
    panic!("unknown error");
}

impl MultipleData {
    fn paipan(&self) -> PaipanResult {
        let (mut mapping, dp, hp) = self.will_god();

        let (a1, a2) = (0, dp); // 将神
        let (gp, rev) = self.yuan_dun(hp); // 贵人，顺逆

        self.eval_god(gp, rev, &mut mapping);
        show_map(&mapping);

        PaipanResult {
            df: self.df,
            will_god: (a1, a2),
        }
    }

    fn title(&self) -> String {
        let a = self.ymdh.0 as usize - 1;
        let b = self.ymdh.1 as usize - 1;
        let c = self.ymdh.2 as usize - 1;
        let d = self.ymdh.3 as usize - 1;
        format!(
            "{} {} {} {} \t{}将 \t{}分",
            self.sixty_coll[a],
            self.sixty_coll[b],
            self.sixty_coll[c],
            self.sixty_coll[d],
            EARTHLY_BRANCH[self.yj as usize - 1],
            EARTHLY_BRANCH[self.df as usize - 1]
        )
    }

    // 计算，将神
    fn will_god(&self) -> (Vec<Mapping>, u8, u8) {
        let mut ds: String = self
            .sixty_coll
            .get(self.ymdh.3 as usize - 1)
            .unwrap()
            .clone();
        let hs = ds.pop().unwrap(); // 时支

        let mut a = index_of(&String::from(hs), IndexType::EarthlyBranch); // 时支次序
        let mut b = index_of(
            &String::from(EARTHLY_BRANCH[self.yj as usize - 1]),
            IndexType::EarthlyBranch,
        ); // 月将次序

        let mut item: Vec<Mapping> = Vec::new();

        for _i in 0..12 {
            item.push(
                //
                (EARTHLY_BRANCH[a], EARTHLY_BRANCH[b], "", "", ""),
            );

            a += 1;
            b += 1;

            if a == 12 {
                a = 0;
            }
            if b == 12 {
                b = 0;
            }
        }

        let (ps, _) = get_item(&item, EARTHLY_BRANCH[self.df as usize - 1]); // 找到地分
        let god = index_of(ps.1, IndexType::EarthlyBranch);

        (
            item,
            god as u8,                                                   // 将神次序
            index_of(&String::from(hs), IndexType::EarthlyBranch) as u8, // 时支次序
        )
    }

    // 贵人顺逆
    fn retrograde(&self, x: u8) -> bool {
        match x {
            3 | 4 | 5 | 6 | 7 | 8 => false,
            0 | 1 | 2 | 9 | 10 | 11 => true,
            _ => {
                panic!("unknown error");
            }
        }
    }

    // 五子元遁
    fn yuan_dun(&self, hp: u8) -> (u8, bool) {
        let mut ds: String = self
            .sixty_coll
            .get(self.ymdh.2 as usize - 1)
            .unwrap()
            .clone();
        ds.pop();
        let sp = match ds.as_str() {
            "甲" | "戊" | "庚" => (1, 7),
            "乙" => (8, 0), // 乙己、鼠猴乡
            "己" => (0, 8),
            "丙" | "丁" => (11, 9),
            "壬" => (3, 5), // 壬癸、蛇兔藏
            "癸" => (5, 3),
            "辛" => (2, 6), // 六辛逢、马虎
            _ => {
                panic!("unknown error");
            }
        };
        if self.retrograde(hp) {
            (sp.1, true)
        } else {
            (sp.0, false)
        }
    }

    fn eval_god(&self, gp: u8, rev: bool, item: &mut Vec<Mapping>) {
        let mut p: i8 = gp as i8;

        for i in 0..12 {
            let (_, ip) = get_item(item, EARTHLY_BRANCH[p as usize]);

            item.get_mut(ip).unwrap().2 = WILL_GOD[i];
            item.get_mut(ip).unwrap().3 = WILL_GOD_BRANCH[i];

            if rev {
                p -= 1;
            } else {
                p += 1;
            }
            if p == 12 {
                p = 0;
            }
            if p == -1 {
                p = 11;
            }
        }
    }
}

fn main() {
    println!("孙膑课 - 排盘工具 @ 丙杺\n");

    let mut sixty_coll: Vec<String> = Vec::new();
    print_sixty_jiazi(&mut sixty_coll);

    print!("1）干支：");
    let step1 = read_screen();
    let ymdh = parse_index(&step1, CheckType::SixTyJiaZi);

    print_earthly_branch();
    print!("2）月将：");

    let step2 = read_screen();
    let yj = parse_index(&step2, CheckType::DiZhi);

    print_earthly_branch();
    print!("3）地分：");

    let step3 = read_screen();
    let df = parse_index(&step3, CheckType::DiZhi);

    let m = MultipleData {
        sixty_coll,
        ymdh: (ymdh[0], ymdh[1], ymdh[2], ymdh[3]),
        yj: yj[0],
        df: df[0],
    };
    let r = m.paipan();
    println!("{}\n\n {:?}", m.title(), r);

    // println!(
    //     "
    //     辛丑 戊戌 壬辰 辛亥     卯将

    //     人元、干：  乙
    //     贵神、神：  丙  午
    //     将神、将：  己  酉  *
    //     地分、方：  巳
    // "
    // );
}

fn read_screen() -> String {
    let mut line = String::new();

    stdout().flush().expect("Failed to flush the screen!");
    stdin().read_line(&mut line).expect("Failed to read line!");

    if line.trim_end().len() > 0 {
        line.pop();
    }
    line
}

fn print_sixty_jiazi(sixty_coll: &mut Vec<String>) {
    let mut hi = 0;
    let mut ei = 0;

    for _i in 0..60 {
        if hi == 10 {
            hi = 0
        }
        if ei == 12 {
            ei = 0
        }
        sixty_coll.push(
            //
            format!("{}{}", HEAVENLY_STEM[hi], EARTHLY_BRANCH[ei]),
        );
        hi += 1;
        ei += 1;
    }

    let mut ips = vec![0, 10, 20, 30, 40, 50];

    for _i in 0..10 {
        let mut ip = 0;

        for j in 0..6 {
            print!("\t{:02} {}", ips[j] + 1, sixty_coll[ips[ip]]);
            ips[ip] += 1;
            ip += 1;
        }
        println!();
    }
    println!();
}

fn print_earthly_branch() {
    for i in 0..12 {
        if i == 6 {
            println!();
        }
        print!("\t{:02} {}", i + 1, EARTHLY_BRANCH[i]);
    }
    println!();
}

fn parse_index(p: &String, t: CheckType) -> Vec<u8> {
    let mut item = Vec::new();

    let mut i = 0;
    let cs: Vec<char> = p.chars().collect();
    let len = cs.len();

    while i < len {
        let mut c = cs[i];
        let mut inner = String::new();

        while c >= '0' && c <= '9' {
            inner.push(c);
            if i + 1 >= len {
                break;
            }
            i += 1;
            c = cs[i];
        }
        if inner.len() > 0 {
            item.push(inner.parse::<u8>().unwrap());
        }
        i += 1;
    }
    match t {
        CheckType::SixTyJiaZi => {
            if item.len() != 4 {
                panic!("4 elements are required");
            }
            for i in &item {
                if *i <= 0 || *i > 60 {
                    panic!("out of bounds");
                }
            }
        }
        CheckType::DiZhi => {
            if item.len() != 1 {
                panic!("1 element required");
            }
            let f = item.get(0).unwrap();
            if *f <= 0 || *f > 12 {
                panic!("out of bounds");
            }
        }
    }
    item
}
