use sea_orm::Set;

use super::prelude::AccountActiveModel;

#[derive(Debug, Default)]
pub struct AccountConditions {
    pub goal_reached: Option<bool>,
    pub id: Option<i32>,
}

impl AccountActiveModel {
    pub fn new(
        pk: String,
        proxy: Option<String>,
        address: String,
        target_ambient_swaps_count: i32,
        target_apriori_deposit_count: i32,
        target_bean_swaps_count: i32,
        target_hashflow_swaps_count: i32,
        target_kinza_deposit_count: i32,
        target_shmonad_deposit_count: i32,
    ) -> Self {
        Self {
            address: Set(address),
            private_key: Set(pk),
            proxy: Set(proxy),
            target_ambient_swaps_count: Set(target_ambient_swaps_count),
            target_apriori_deposit_count: Set(target_apriori_deposit_count),
            target_bean_swaps_count: Set(target_bean_swaps_count),
            target_hashflow_swaps_count: Set(target_hashflow_swaps_count),
            target_kinza_deposit_count: Set(target_kinza_deposit_count),
            target_shmonad_deposit_count: Set(target_shmonad_deposit_count),
            ..Default::default()
        }
    }
}
