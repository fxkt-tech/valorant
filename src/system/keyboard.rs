#[allow(dead_code)]
pub enum Shortcut {
    MainGun, // 1 主武器
    SubGun,  // 2 副武器
    Knife,   // 3 近战武器
    Skill1,  // c 技能1
    Skill2,  // q 技能2
    Skill3,  // e 技能3，免费技能
    Skill4,  // x 终极技能

    Interaction, // f 安装/拆除spike，捡枪
    DropGun,     // g 扔枪，近战武器不能扔
    View,        // v
}
