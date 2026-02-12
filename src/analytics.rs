use async_graphql::{ComplexObject, SimpleObject};

#[derive(Default, SimpleObject)]
#[graphql(complex)]
pub struct Analytics {
    campaign_costs: Vec<f32>,
}

#[ComplexObject]
impl Analytics {
    pub async fn churn_rate(&self) -> f64 {
        0.0
    }

    /// Cost per Acquisition (CPA)
    ///
    /// Formula: CPA = Total Cost of Campaign / Number of Conversions
    pub async fn cpa(&self) -> f64 {
        0.0
    }
}
