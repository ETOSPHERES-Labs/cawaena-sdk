use sdk::types::transactions::PurchaseDetails;

#[derive(Debug, serde::Serialize)]
pub struct PurchaseDetailsEntity {
    /// The address where the amount should be paid.
    pub system_address: String,
    /// The amount to be paid.
    pub amount: f64,
    /// The status of transaction
    pub status: String,
    /// Any reasons attached to the status
    pub invalid_reasons: Vec<String>,
    /// The id of the network that the transaction is sent in
    pub network_id: String,
}

impl TryFrom<PurchaseDetails> for PurchaseDetailsEntity {
    type Error = sdk::Error;
    fn try_from(val: PurchaseDetails) -> Result<Self, Self::Error> {
        let (status, status_reasons) = match val.status {
            sdk::types::ApiTxStatus::Pending => ("Pending", Vec::new()),
            sdk::types::ApiTxStatus::WaitingForVerification(vec) => ("WaitingForVerification", vec),
            sdk::types::ApiTxStatus::Valid => ("Valid", Vec::new()),
            sdk::types::ApiTxStatus::Invalid(vec) => ("Invalid", vec),
            sdk::types::ApiTxStatus::ProcessingIncoming => ("ProcessingIncoming", Vec::new()),
            sdk::types::ApiTxStatus::ProcessingOutgoing => ("ProcessingOutgoing", Vec::new()),
            sdk::types::ApiTxStatus::Completed => ("Completed", Vec::new()),
            sdk::types::ApiTxStatus::Failed => ("Failed", Vec::new()),
        };

        Ok(PurchaseDetailsEntity {
            system_address: val.system_address,
            amount: f64::try_from(val.amount)?,
            status: status.to_string(),
            network_id: val.network.id,
            invalid_reasons: status_reasons,
        })
    }
}
