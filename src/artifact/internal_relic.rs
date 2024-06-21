use edit_distance;
use image::RgbImage;
use log::{error, warn};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use strum_macros::Display;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum RelicStatName {
    HP,
    HPPercentage,
    ATK,
    ATKPercentage,
    DEFPercentage,
    SPD,
    CRITRate,
    CRITDMG,
    BreakEffect,
    OutgoingHealingBoost,
    EnergyRegenerationRate,
    EffectHitRate,
    PhysicalDMGBoost,
    FireDMGBoost,
    IceDMGBoost,
    LightningDMGBoost,
    WindDMGBoost,
    QuantumDMGBoost,
    ImaginaryDMGBoost,
    DEF,
    EffectRES,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum RelicSlot {
    Head,
    Hands,
    Body,
    Feet,
    PlanarSphere,
    LinkRope,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, Display)]
pub enum RelicSetName {
    PasserbyofWanderingCloud,
    MusketeerofWildWheat,
    KnightofPurityPalace,
    HunterofGlacialForest,
    ChampionofStreetwiseBoxing,
    GuardofWutheringSnow,
    FiresmithofLavaForging,
    GeniusofBrilliantStars,
    BandofSizzlingThunder,
    EagleofTwilightLine,
    ThiefofShootingMeteor,
    WastelanderofBanditryDesert,
    SpaceSealingStation,
    FleetoftheAgeless,
    PanGalacticCommercialEnterprise,
    BelobogoftheArchitects,
    CelestialDifferentiator,
    InertSalsotto,
    TaliaKingdomofBanditry,
    SprightlyVonwacq,
    RutilantArena,
    BrokenKeel,
    LongevousDisciple,
    MessengerTraversingHackerspace,
    TheAshblazingGrandDuke,
    PrisonerinDeepConfinement,
    PenaconyLandoftheDreams,
    FirmamentFronlineGlamoth,
    ZhongBiao,
    XianQu,
    ChuYun,
    WuZhu,
    ZhuLian,
    DuLan,
    TieQi,
    YongLie,
}

#[derive(Debug, Clone)]
pub struct RelicStat {
    pub name: RelicStatName,
    pub value: f64,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct InternalRelic {
    pub set_name: RelicSetName,
    pub slot: RelicSlot,
    pub star: u32,
    pub level: u32,
    pub main_stat: RelicStat,
    pub sub_stat_1: Option<RelicStat>,
    pub sub_stat_2: Option<RelicStat>,
    pub sub_stat_3: Option<RelicStat>,
    pub sub_stat_4: Option<RelicStat>,
    pub equip: Option<String>,
    pub locked: bool,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lock {
    pub save: bool,
    pub token: String,
}

#[derive(Debug, Clone)]
pub struct EquipImage {
    pub name: String,
    pub image: RgbImage,
}

impl InternalRelic {
    pub fn g_token(&mut self, lock_map: HashMap<String, Lock>) {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        self.token = format!("{:x}", hasher.finish());
        // dbg!(&lock_map);
        match lock_map.get(&self.token) {
            Some(a) => {
                // dbg!("self");
                // dbg!(&self);
                if a.save {
                    self.locked = true;
                }
            },
            None => {
                if !lock_map.is_empty() {
                    warn!("{:?}", &self);
                }
            },
        }
        // dbg!("self");
    }
}

impl Hash for RelicStat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        let v = (self.value * 1000.0) as i32;
        v.hash(state);
    }
}

impl PartialEq for RelicStat {
    fn eq(&self, other: &Self) -> bool {
        if self.name != other.name {
            return false;
        }

        let v1 = (self.value * 1000.0) as i32;
        let v2 = (other.value * 1000.0) as i32;

        v1 == v2
    }
}

impl Eq for RelicStat {}

impl RelicStatName {
    pub fn from_zh_cn(name: &str, is_percentage: bool) -> Option<RelicStatName> {
        match name {
            "生命值" => {
                if is_percentage {
                    Some(RelicStatName::HPPercentage)
                } else {
                    Some(RelicStatName::HP)
                }
            },
            "攻击力" => {
                if is_percentage {
                    Some(RelicStatName::ATKPercentage)
                } else {
                    Some(RelicStatName::ATK)
                }
            },
            "防御力" => {
                if is_percentage {
                    Some(RelicStatName::DEFPercentage)
                } else {
                    Some(RelicStatName::DEF)
                }
            },
            "速度" => Some(RelicStatName::SPD),
            "暴击率" => Some(RelicStatName::CRITRate),
            "暴击伤害" => Some(RelicStatName::CRITDMG),
            "击破特攻" => Some(RelicStatName::BreakEffect),
            "治疗量加成" => Some(RelicStatName::OutgoingHealingBoost),
            "能量恢复效率" => Some(RelicStatName::EnergyRegenerationRate),
            "效果命中" => Some(RelicStatName::EffectHitRate),
            "物理属性伤害提高" => Some(RelicStatName::PhysicalDMGBoost),
            "火属性伤害提高" => Some(RelicStatName::FireDMGBoost),
            "冰属性伤害提高" => Some(RelicStatName::IceDMGBoost),
            "雷属性伤害提高" => Some(RelicStatName::LightningDMGBoost),
            "风属性伤害提高" => Some(RelicStatName::WindDMGBoost),
            "量子属性伤害提高" => Some(RelicStatName::QuantumDMGBoost),
            "虚数属性伤害提高" => Some(RelicStatName::ImaginaryDMGBoost),
            "效果抵抗" => Some(RelicStatName::EffectRES),
            _ => None,
        }
    }
}

impl RelicStat {
    // e.g "生命值+4,123", "暴击率+10%"
    pub fn from_zh_cn_raw(s: &str) -> Option<RelicStat> {
        let temp: Vec<&str> = s.split("+").collect();
        if temp.len() != 2 {
            return None;
        }

        let is_percentage = temp[1].contains("%");
        let stat_name = match RelicStatName::from_zh_cn(temp[0], is_percentage) {
            Some(v) => v,
            None => return None,
        };

        let re = Regex::new("[%,]").unwrap();
        let mut value = match re.replace_all(temp[1], "").parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                error!("stat `{}` parse error", s);
                return None;
            },
        };
        if is_percentage {
            value /= 100.0;
        }

        Some(RelicStat {
            name: stat_name,
            value,
        })
    }
}

pub fn get_real_relic_name_chs(raw: &str) -> Option<String> {
    let all_relic_chs = [
        "过客的逢春木簪",
        "过客的游龙臂鞲",
        "过客的残绣风衣",
        "过客的冥途游履",
        "快枪手的野穗毡帽",
        "快枪手的粗革手套",
        "快枪手的猎风披肩",
        "快枪手的铆钉马靴",
        "圣骑的宽恕盔面",
        "圣骑的沉默誓环",
        "圣骑的肃穆胸甲",
        "圣骑的秩序铁靴",
        "雪猎的荒神兜帽",
        "雪猎的巨蜥手套",
        "雪猎的冰龙披风",
        "雪猎的鹿皮软靴",
        "拳王的冠军护头",
        "拳王的重炮拳套",
        "拳王的贴身护胸",
        "拳王的弧步战靴",
        "铁卫的铸铁面盔",
        "铁卫的银鳞手甲",
        "铁卫的旧制军服",
        "铁卫的白银护胫",
        "火匠的黑耀目镜",
        "火匠的御火戒指",
        "火匠的阻燃围裙",
        "火匠的合金义肢",
        "天才的超距遥感",
        "天才的频变捕手",
        "天才的元域深潜",
        "天才的引力漫步",
        "乐队的偏光墨镜",
        "乐队的巡演手绳",
        "乐队的钉刺皮衣",
        "乐队的铆钉短靴",
        "翔鹰的长喙头盔",
        "翔鹰的鹰击指环",
        "翔鹰的翼装束带",
        "翔鹰的绒羽绑带",
        "怪盗的千人假面",
        "怪盗的绘纹手套",
        "怪盗的纤钢爪钩",
        "怪盗的流星快靴",
        "废土客的呼吸面罩",
        "废土客的荒漠终端",
        "废土客的修士长袍",
        "废土客的动力腿甲",
        "「黑塔」的空间站点",
        "「黑塔」的漫历轨迹",
        "罗浮仙舟的天外楼船",
        "罗浮仙舟的建木枝蔓",
        "公司的巨构总部",
        "公司的贸易航道",
        "贝洛伯格的存护堡垒",
        "贝洛伯格的铁卫防线",
        "螺丝星的机械烈阳",
        "螺丝星的环星孔带",
        "萨尔索图的移动城市",
        "萨尔索图的晨昏界线",
        "塔利亚的钉壳小镇",
        "塔利亚的裸皮电线",
        "翁瓦克的诞生之岛",
        "翁瓦克的环岛海岸",
        "泰科铵的镭射球场",
        "泰科铵的弧光赛道",
        "伊须磨洲的残船鲸落",
        "伊须磨洲的坼裂缆索",
        "莳者的复明义眼",
        "莳者的机巧木手",
        "莳者的承露羽衣",
        "莳者的天人丝履",
        "信使的全息目镜",
        "信使的百变义手",
        "信使的密信挎包",
        "信使的酷跑板鞋",
        "匹诺康尼的堂皇酒店",
        "匹诺康尼的逐梦轨道",
        "大公的冥焰冠冕",
        "大公的绅雅礼靴",
        "大公的绒火指套",
        "大公的蒙恩长袍",
        "格拉默的寂静坟碑",
        "格拉默的铁骑兵团",
        "系囚的合啮拘笼",
        "系囚的幽闭缚束",
        "系囚的绝足锁桎",
        "系囚的铅石梏铐",
        "钟表匠的极目透镜",
        "钟表匠的交运腕表",
        "钟表匠的空幻礼服",
        "钟表匠的隐梦革履",
        "先驱的绝热围壳",
        "先驱的虚极罗盘",
        "先驱的密合铅衣",
        "先驱的泊星桩锚",
        "出云的祸津众神",
        "出云的终始一刀",
        "茨冈尼亚的母神卧榻",
        "茨冈尼亚的轮回纽结",
        "铸炼宫的莲华灯芯",
        "铸炼宫的焰轮天绸",
        "都蓝的穹窿金帐",
        "都蓝的器兽缰辔",
        "铁骑的索敌战盔",
        "铁骑的摧坚铁腕",
        "铁骑的银影装甲",
        "铁骑的行空护胫",
        "勇烈的玄枵面甲",
        "勇烈的钩爪腕甲",
        "勇烈的飞翎瓷甲",
        "勇烈的逐猎腿甲",
    ];

    let mut min_index = 0;
    let mut min_dis = edit_distance::edit_distance(raw, all_relic_chs[0]);
    let mut same_flag = false;
    for (i, &val) in all_relic_chs.iter().enumerate().skip(1) {
        let dis = edit_distance::edit_distance(val, raw);
        if dis < min_dis {
            min_dis = dis;
            min_index = i;
            same_flag = false;
        } else if dis == min_dis {
            same_flag = true;
        }
    }

    if same_flag {
        None
    } else {
        Some(String::from(all_relic_chs[min_index]))
    }
}

impl RelicSetName {
    pub fn from_zh_cn(s: &str) -> Option<RelicSetName> {
        // let s = match get_real_relic_name_chs(s) {
        //     Some(v) => v,
        //     None => return None,
        // };
        // println!("name: {}", s);
        match s {
            "过客的逢春木簪" | "过客的游龙臂鞲" | "过客的残绣风衣" | "过客的冥途游履" => {
                Some(RelicSetName::PasserbyofWanderingCloud)
            },
            "快枪手的野穗毡帽" | "快枪手的粗革手套" | "快枪手的猎风披肩" | "快枪手的铆钉马靴" => {
                Some(RelicSetName::MusketeerofWildWheat)
            },
            "圣骑的宽恕盔面" | "圣骑的沉默誓环" | "圣骑的肃穆胸甲" | "圣骑的秩序铁靴" => {
                Some(RelicSetName::KnightofPurityPalace)
            },
            "雪猎的荒神兜帽" | "雪猎的巨蜥手套" | "雪猎的冰龙披风" | "雪猎的鹿皮软靴" => {
                Some(RelicSetName::HunterofGlacialForest)
            },
            "拳王的冠军护头" | "拳王的重炮拳套" | "拳王的贴身护胸" | "拳王的弧步战靴" => {
                Some(RelicSetName::ChampionofStreetwiseBoxing)
            },
            "铁卫的铸铁面盔" | "铁卫的银鳞手甲" | "铁卫的旧制军服" | "铁卫的白银护胫" => {
                Some(RelicSetName::GuardofWutheringSnow)
            },
            "火匠的黑耀目镜" | "火匠的御火戒指" | "火匠的阻燃围裙" | "火匠的合金义肢" => {
                Some(RelicSetName::FiresmithofLavaForging)
            },
            "天才的超距遥感" | "天才的频变捕手" | "天才的元域深潜" | "天才的引力漫步" => {
                Some(RelicSetName::GeniusofBrilliantStars)
            },
            "乐队的偏光墨镜" | "乐队的巡演手绳" | "乐队的钉刺皮衣" | "乐队的铆钉短靴" => {
                Some(RelicSetName::BandofSizzlingThunder)
            },
            "翔鹰的长喙头盔" | "翔鹰的鹰击指环" | "翔鹰的翼装束带" | "翔鹰的绒羽绑带" => {
                Some(RelicSetName::EagleofTwilightLine)
            },
            "怪盗的千人假面" | "怪盗的绘纹手套" | "怪盗的纤钢爪钩" | "怪盗的流星快靴" => {
                Some(RelicSetName::ThiefofShootingMeteor)
            },
            "废土客的呼吸面罩" | "废土客的荒漠终端" | "废土客的修士长袍" | "废土客的动力腿甲" => {
                Some(RelicSetName::WastelanderofBanditryDesert)
            },
            "「黑塔」的空间站点" | "「黑塔」的漫历轨迹" => {
                Some(RelicSetName::SpaceSealingStation)
            },
            "罗浮仙舟的天外楼船" | "罗浮仙舟的建木枝蔓" => {
                Some(RelicSetName::FleetoftheAgeless)
            },
            "公司的巨构总部" | "公司的贸易航道" => {
                Some(RelicSetName::PanGalacticCommercialEnterprise)
            },
            "贝洛伯格的存护堡垒" | "贝洛伯格的铁卫防线" => {
                Some(RelicSetName::BelobogoftheArchitects)
            },
            "螺丝星的机械烈阳" | "螺丝星的环星孔带" => {
                Some(RelicSetName::CelestialDifferentiator)
            },
            "萨尔索图的移动城市" | "萨尔索图的晨昏界线" => {
                Some(RelicSetName::InertSalsotto)
            },
            "塔利亚的钉壳小镇" | "塔利亚的裸皮电线" => {
                Some(RelicSetName::TaliaKingdomofBanditry)
            },
            "翁瓦克的诞生之岛" | "翁瓦克的环岛海岸" => {
                Some(RelicSetName::SprightlyVonwacq)
            },
            "泰科铵的镭射球场" | "泰科铵的弧光赛道" => {
                Some(RelicSetName::RutilantArena)
            },
            "伊须磨洲的残船鲸落" | "伊须磨洲的坼裂缆索" => {
                Some(RelicSetName::BrokenKeel)
            },
            "莳者的复明义眼" | "莳者的机巧木手" | "莳者的承露羽衣" | "莳者的天人丝履" => {
                Some(RelicSetName::LongevousDisciple)
            },
            "信使的全息目镜" | "信使的百变义手" | "信使的密信挎包" | "信使的酷跑板鞋" => {
                Some(RelicSetName::MessengerTraversingHackerspace)
            },
            "大公的冥焰冠冕" | "大公的绅雅礼靴" | "大公的绒火指套" | "大公的蒙恩长袍" => {
                Some(RelicSetName::TheAshblazingGrandDuke)
            },
            "系囚的合啮拘笼" | "系囚的幽闭缚束" | "系囚的绝足锁桎" | "系囚的铅石梏铐" => {
                Some(RelicSetName::PrisonerinDeepConfinement)
            },
            "匹诺康尼的堂皇酒店" | "匹诺康尼的逐梦轨道" => {
                Some(RelicSetName::PenaconyLandoftheDreams)
            },
            "格拉默的寂静坟碑" | "格拉默的铁骑兵团" => {
                Some(RelicSetName::FirmamentFronlineGlamoth)
            },
            "钟表匠的极目透镜" | "钟表匠的交运腕表" | "钟表匠的空幻礼服" | "钟表匠的隐梦革履" => {
                Some(RelicSetName::ZhongBiao)
            },
            "先驱的绝热围壳" | "先驱的虚极罗盘" | "先驱的密合铅衣" | "先驱的泊星桩锚" => {
                Some(RelicSetName::XianQu)
            },
            "出云的祸津众神" | "出云的终始一刀" => {
                Some(RelicSetName::ChuYun)
            },
            "茨冈尼亚的母神卧榻" | "茨冈尼亚的轮回纽结" => {
                Some(RelicSetName::WuZhu)
            },
            "铸炼宫的莲华灯芯" | "铸炼宫的焰轮天绸" => {
                Some(RelicSetName::ZhuLian)
            },
            "都蓝的穹窿金帐" | "都蓝的器兽缰辔" => {
                Some(RelicSetName::DuLan)
            },
            "铁骑的索敌战盔" | "铁骑的摧坚铁腕" | "铁骑的银影装甲" | "铁骑的行空护胫" => {
                Some(RelicSetName::TieQi)
            },
            "勇烈的玄枵面甲" | "勇烈的钩爪腕甲" | "勇烈的飞翎瓷甲" | "勇烈的逐猎腿甲" => {
                Some(RelicSetName::YongLie)
            },
            _ => None,
        }
    }
}

impl RelicSlot {
    pub fn from_zh_cn(s: &str) -> Option<RelicSlot> {
        // let s = match get_real_relic_name_chs(s) {
        //     Some(v) => v,
        //     None => return None,
        // };
        match s {
            "过客的逢春木簪"
            | "快枪手的野穗毡帽"
            | "圣骑的宽恕盔面"
            | "雪猎的荒神兜帽"
            | "拳王的冠军护头"
            | "铁卫的铸铁面盔"
            | "火匠的黑耀目镜"
            | "天才的超距遥感"
            | "乐队的偏光墨镜"
            | "翔鹰的长喙头盔"
            | "怪盗的千人假面"
            | "废土客的呼吸面罩"
            | "莳者的复明义眼"
            | "大公的冥焰冠冕"
            | "系囚的合啮拘笼"
            | "信使的全息目镜"
            | "钟表匠的极目透镜"
            | "铁骑的索敌战盔"
            | "勇烈的玄枵面甲"
            | "先驱的绝热围壳" => Some(RelicSlot::Head),
            "过客的游龙臂鞲"
            | "快枪手的粗革手套"
            | "圣骑的沉默誓环"
            | "雪猎的巨蜥手套"
            | "拳王的重炮拳套"
            | "铁卫的银鳞手甲"
            | "火匠的御火戒指"
            | "天才的频变捕手"
            | "乐队的巡演手绳"
            | "翔鹰的鹰击指环"
            | "怪盗的绘纹手套"
            | "废土客的荒漠终端"
            | "莳者的机巧木手"
            | "系囚的铅石梏铐"
            | "大公的绒火指套"
            | "信使的百变义手"
            | "钟表匠的交运腕表"
            | "铁骑的摧坚铁腕"
            | "勇烈的钩爪腕甲"
            | "先驱的虚极罗盘" => Some(RelicSlot::Hands),
            "过客的残绣风衣"
            | "快枪手的猎风披肩"
            | "圣骑的肃穆胸甲"
            | "雪猎的冰龙披风"
            | "拳王的贴身护胸"
            | "铁卫的旧制军服"
            | "火匠的阻燃围裙"
            | "天才的元域深潜"
            | "乐队的钉刺皮衣"
            | "翔鹰的翼装束带"
            | "怪盗的纤钢爪钩"
            | "废土客的修士长袍"
            | "莳者的承露羽衣"
            | "大公的蒙恩长袍"
            | "系囚的幽闭缚束"
            | "信使的密信挎包"
            | "钟表匠的空幻礼服"
            | "铁骑的银影装甲"
            | "勇烈的飞翎瓷甲"
            | "先驱的密合铅衣" => Some(RelicSlot::Body),
            "过客的冥途游履"
            | "快枪手的铆钉马靴"
            | "圣骑的秩序铁靴"
            | "雪猎的鹿皮软靴"
            | "拳王的弧步战靴"
            | "铁卫的白银护胫"
            | "火匠的合金义肢"
            | "天才的引力漫步"
            | "乐队的铆钉短靴"
            | "翔鹰的绒羽绑带"
            | "怪盗的流星快靴"
            | "废土客的动力腿甲"
            | "莳者的天人丝履"
            | "系囚的绝足锁桎"
            | "大公的绅雅礼靴"
            | "信使的酷跑板鞋"
            | "钟表匠的隐梦革履"
            | "铁骑的行空护胫"
            | "勇烈的逐猎腿甲"
            | "先驱的泊星桩锚" => Some(RelicSlot::Feet),
            "「黑塔」的空间站点"
            | "罗浮仙舟的天外楼船"
            | "公司的巨构总部"
            | "贝洛伯格的存护堡垒"
            | "螺丝星的机械烈阳"
            | "萨尔索图的移动城市"
            | "塔利亚的钉壳小镇"
            | "翁瓦克的诞生之岛"
            | "泰科铵的镭射球场"
            | "格拉默的铁骑兵团"
            | "匹诺康尼的堂皇酒店"
            | "出云的祸津众神"
            | "茨冈尼亚的母神卧榻"
            | "铸炼宫的莲华灯芯"
            | "都蓝的穹窿金帐"
            | "伊须磨洲的残船鲸落" => Some(RelicSlot::PlanarSphere),
            "「黑塔」的漫历轨迹"
            | "罗浮仙舟的建木枝蔓"
            | "公司的贸易航道"
            | "贝洛伯格的铁卫防线"
            | "螺丝星的环星孔带"
            | "萨尔索图的晨昏界线"
            | "塔利亚的裸皮电线"
            | "翁瓦克的环岛海岸"
            | "泰科铵的弧光赛道"
            | "匹诺康尼的逐梦轨道"
            | "格拉默的寂静坟碑"
            | "出云的终始一刀"
            | "茨冈尼亚的轮回纽结"
            | "铸炼宫的焰轮天绸"
            | "都蓝的器兽缰辔"
            | "伊须磨洲的坼裂缆索" => Some(RelicSlot::LinkRope),
            _ => None,
        }
    }
}
