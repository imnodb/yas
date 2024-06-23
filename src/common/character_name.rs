use std::collections::HashSet;

use crate::artifact::internal_relic::EquipImage;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CHARACTER_NAMES: HashSet<&'static str> = HashSet::from([
        "迪卢克",
        "可莉",
        "胡桃",
        "宵宫",
        "安柏",
        "班尼特",
        "香菱",
        "辛焱",
        "烟绯",
        "托马",
        "莫娜",
        "达达利亚",
        "珊瑚宫心海",
        "神里绫人",
        "夜兰",
        "妮露",
        "芭芭拉",
        "行秋",
        "坎蒂丝",
        "琴",
        "温迪",
        "魈",
        "旅行者",
        "枫原万叶",
        "流浪者",
        "砂糖",
        "早柚",
        "鹿野院平藏",
        "珐露珊",
        "刻晴",
        "雷电将军",
        "八重神子",
        "赛诺",
        "北斗",
        "丽莎",
        "雷泽",
        "菲谢尔",
        "九条裟罗",
        "久岐忍",
        "多莉",
        "七七",
        "甘雨",
        "神里绫华",
        "优菈",
        "埃洛伊",
        "申鹤",
        "凯亚",
        "重云",
        "迪奥娜",
        "罗莎莉亚",
        "莱依拉",
        "钟离",
        "阿贝多",
        "荒泷一斗",
        "诺艾尔",
        "凝光",
        "云堇",
        "五郎",
        "提纳里",
        "纳西妲",
        "柯莱",
        "白术",
        "卡维",
        "瑶瑶",
        "艾尔海森",
        "迪希雅",
        "米卡",
        "琳妮特",
        "林尼",
        "菲米尼",
        "芙宁娜",
        "那维莱特",
        "娜维娅",
    ]);
    pub static ref CHARACTER_IMAGES: Vec<EquipImage> = Vec::from([
        EquipImage {
            name: "开拓者".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/开拓者.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "白露".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/白露.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "符玄".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/符玄.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "青雀".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/青雀.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "素裳".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/素裳.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "停云".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/停云.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "驭空".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/驭空.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "玲可".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/玲可.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "克拉拉".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/克拉拉.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "佩拉".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/佩拉.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "杰帕德".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/杰帕德.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "希露瓦".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/希露瓦.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "希儿".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/希儿.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "布洛妮娅".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/布洛妮娅.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "艾丝妲".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/艾丝妲.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "银狼".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/银狼.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "卡芙卡".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/卡芙卡.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "丹恒".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/丹恒.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "三月七".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/三月七.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "藿藿".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/藿藿.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "阮•梅".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/阮•梅.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "黑天鹅".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/黑天鹅.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "真理".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/真理.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "黑塔".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/黑塔.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "花火".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/花火.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "黄泉".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/黄泉.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "砂金".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/砂金.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "知更鸟".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/知更鸟.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "托帕&账账".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/托帕&账账.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "加拉赫".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/加拉赫.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "流萤".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/流萤.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "姬子".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/姬子.png"))
                .unwrap()
                .into_rgb8(),
        },
        EquipImage {
            name: "瓦尔特".to_string(),
            image: image::load_from_memory(include_bytes!("../../models/images/瓦尔特.png"))
                .unwrap()
                .into_rgb8(),
        },
    ]);
}
