use crate::calculates::base_calculate::BaseCalculate;
use crate::computes::defult_rules::CulRules;

pub struct Calculate {
    calculators: Vec<Box<dyn BaseCalculate>>,
    rules: Box<dyn CulRules>,
}

impl Calculate {
    // 接受已经创建的 CulRules 实例
    pub fn new(rules: Box<dyn CulRules>) -> Self {
        Self {
            calculators: Vec::new(),
            rules,
        }
    }

    pub fn add_calculator(&mut self, calculator: Box<dyn BaseCalculate>) {
        self.calculators.push(calculator);
    }

    pub fn execute_rules(&self) -> i64 {
        // 传递对 calculators 的引用
        let mut  cul_res = Vec::new();
        for  calculator in &self.calculators {
            let cul = calculator.calculate();
            cul_res.push(cul);
        }
        self.rules.cul_rules(cul_res)
    }
}