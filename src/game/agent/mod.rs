mod neon;

use crate::system::keyboard::Shortcut;

pub trait Agent {
    fn new() -> Self;
    // 切换武器
    fn switch_weapon(key: Shortcut);
    // 购买物品
    fn buy_item(key: Shortcut);
    // 捡枪
    fn pick_weapon(key: Shortcut);
    // 扔枪
    fn drop_weapon(key: Shortcut);
    // 开火
    fn fire();
    // 辅助开火
    fn sub_fire();
}
