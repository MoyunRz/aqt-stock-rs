pub trait CulRules {
    fn create(&self) -> Self
    where
        Self: Sized;
    fn cul_rules(&self, res: Vec<i64>) -> i64;
}

pub struct DefultRules {}

impl CulRules for DefultRules {
    fn create(&self) -> Self
    where
        Self: Sized,
    {
        DefultRules {}
    }
    fn cul_rules(&self, res: Vec<i64>) -> i64 {
        let mut cul_res: i64 = 0;
        for i in res.iter() {
            cul_res += i;
        }

        if cul_res >= 3 {
            return 1;
        }
        if cul_res <= -3 {
            return -1;
        }
        0
    }
}