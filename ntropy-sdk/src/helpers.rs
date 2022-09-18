use ntropy_api_client::models::{
    enriched_transaction, ntropy_server_api_v2_2_enrich_transactions_sync_200_response_inner,
    EnrichedTransaction, NtropyServerApiV22EnrichTransactionsSync200ResponseInner,
    TransactionError,
};

pub fn map_enriched_transaction(
    response: &NtropyServerApiV22EnrichTransactionsSync200ResponseInner,
) -> Result<EnrichedTransaction, TransactionError> {
    if let Some(ref error) = response.error {
        Err(TransactionError {
            error: error.clone(),
            error_details: response.error_details.clone(),
        })
    } else {
        Ok(EnrichedTransaction {
            transaction_id: response.transaction_id.clone(),
            logo: response.logo.clone(),
            website: response.website.clone(),
            location: response.location.clone(),
            merchant: response.merchant.clone(),
            merchant_id: response.merchant_id.clone(),
            labels: response.labels.clone(),
            chart_of_accounts: response.chart_of_accounts.clone(),
            recurrence: map_recurrence(&response.recurrence),
            recurrence_group: response.recurrence_group.clone(),
            recurrence_group_id: response.recurrence_group_id.clone(),
            mcc: response.mcc.clone(),
            person: response.person.clone(),
            transaction_type: map_transaction_type(&response.transaction_type),
        })
    }
}

fn map_recurrence(
    value: &Option<ntropy_server_api_v2_2_enrich_transactions_sync_200_response_inner::Recurrence>,
) -> Option<enriched_transaction::Recurrence> {
    match value {
        Some(
            ntropy_server_api_v2_2_enrich_transactions_sync_200_response_inner::Recurrence::OneOff,
        ) => Some(enriched_transaction::Recurrence::OneOff),
        Some(
            ntropy_server_api_v2_2_enrich_transactions_sync_200_response_inner::Recurrence::Recurring,
        ) => Some(enriched_transaction::Recurrence::Recurring),
        Some(
            ntropy_server_api_v2_2_enrich_transactions_sync_200_response_inner::Recurrence::Repeating,
        ) => Some(enriched_transaction::Recurrence::Repeating),
        Some(
            ntropy_server_api_v2_2_enrich_transactions_sync_200_response_inner::Recurrence::Subscription,
        ) => Some(enriched_transaction::Recurrence::Subscription),
        _ => None,
    }
}

fn map_transaction_type(
    value: &Option<
        ntropy_server_api_v2_2_enrich_transactions_sync_200_response_inner::TransactionType,
    >,
) -> Option<enriched_transaction::TransactionType> {
    match value {
        Some(
            ntropy_server_api_v2_2_enrich_transactions_sync_200_response_inner::TransactionType::Business,
        ) => Some(enriched_transaction::TransactionType::Business),
        Some(
            ntropy_server_api_v2_2_enrich_transactions_sync_200_response_inner::TransactionType::Consumer,
        ) => Some(enriched_transaction::TransactionType::Consumer),
        Some(
            ntropy_server_api_v2_2_enrich_transactions_sync_200_response_inner::TransactionType::Unknown,
        ) => Some(enriched_transaction::TransactionType::Unknown),

        _ => None,
    }
}
