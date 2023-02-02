pub mod sheriff;

pub trait Weapon {
    fn new() -> Self;
    // 获取伤害值
    fn get_atk() -> u32;
    fn reload(&mut self) -> &mut Self;
    // 开火，主要用于计算子弹/弹夹相关逻辑
    fn fire(&mut self) -> &mut Self;
    // 辅助开火
    fn sub_fire(&mut self) -> &mut Self;
    // 弹夹情况
    fn show_magazine(self) -> String;
}
