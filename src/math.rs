pub(crate) fn churn_rate() -> f64 {
    todo!()
}

/// Cost per Acquisition (CPA)
///
/// Formula: CPA = Total Cost of Campaign / Number of Conversions
pub(crate) fn cpa() -> f64 {
    todo!()
}

/// Market Penetration Rate
///
/// Formula: % = (Number of Customers Who Purchased Your Product / Total Number of Potential Customers) × 100
pub(crate) fn mpr() -> f64 {
    todo!()
}

/// Market Share
///
/// Formula: (%) = (Company’s Sales / Total Market Sales) × 100
pub(crate) fn market_share() -> f64 {
    todo!()
}

/// Net Promoter Score (NPS)
///
/// Net Promoter Score (NPS) is calculated using customer feedback collected through a survey that asks respondents to rate the likelihood that they would recommend a company, product, or service to a friend or colleague. The rating is usually on a scale from 0 to 10.
///
/// Since this one is a bit more involved, here’s how you calculate NPS:
///
/// Categorize the Responses:
///
///     Promoters: Respondents giving a score of 9 or 10 are considered Promoters.
///
///     Passives: Respondents giving a score of 7 or 8 are considered Passives (and are left out of the calculation).
///
///     Detractors: Respondents giving a score between 0 and 6 are considered Detractors.
///
/// Calculate the Percentage of Promoters and Detractors:
///
///     Percentage of Promoters: (Number of Promoters / Total Number of Respondents) × 100

///     Percentage of Detractors: (Number of Detractors / Total Number of Respondents) × 100
///
/// Subtract the Percentage of Detractors from the Percentage of Promoters: This gives you the NPS, which can range from -100 (if every customer is a Detractor) to 100 (if every customer is a Promoter).
///
/// NPS = Percentage of Promoters – Percentage of Detractors
///
/// Example:
///
/// If you received responses from 100 customers, where 70 are Promoters and 10 are Detractors:
///
/// Percentage of Promoters = (70 / 100) × 100 = 70%
///
/// Percentage of Detractors = (10 / 100) × 100 = 10%
///
/// NPS = 70% - 10% = 60
///
/// This would result in an NPS of 60, which generally indicates a very healthy level of customer satisfaction and loyalty.
///
/// Remember that Passives are not factored into the final NPS calculation, although they represent a segment of customers whose loyalty is not assured.
pub(crate) fn NPS() -> f64 {
    todo!()
}
