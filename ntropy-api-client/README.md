# Rust API client for ntropy-api-client

The world's most powerful, multi-geo, multi-lingual transaction categorization API.

For more information, please visit [https://ntropy.com/developers](https://ntropy.com/developers)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 
- Package version: 0.0.1
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `ntropy-api-client` and add the following to `Cargo.toml` under `[dependencies]`:

```
ntropy-api-client = { path = "./ntropy-api-client" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.ntropy.network*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AccountHolderApi* | [**ntropy_server_period_api_period_v22_period_create_account_holder**](docs/AccountHolderApi.md#ntropy_server_period_api_period_v22_period_create_account_holder) | **POST** /v2/account-holder | Create an account holder.
*AccountHolderApi* | [**ntropy_server_period_api_period_v22_period_get_account_holder**](docs/AccountHolderApi.md#ntropy_server_period_api_period_v22_period_get_account_holder) | **GET** /v2/account-holder/{account_holder_id} | Retrieve the information of an account holder.
*AccountHolderApi* | [**ntropy_server_period_api_period_v22_period_get_account_holder_income_check**](docs/AccountHolderApi.md#ntropy_server_period_api_period_v22_period_get_account_holder_income_check) | **POST** /v2/account-holder/{account_holder_id}/income | Categorize an account holder's histories income by their frequency and type
*AccountHolderApi* | [**ntropy_server_period_api_period_v22_period_get_account_holder_metrics**](docs/AccountHolderApi.md#ntropy_server_period_api_period_v22_period_get_account_holder_metrics) | **POST** /v2/account-holder/{account_holder_id}/query | Query the transaction ledger of an account holder.
*AccountHolderApi* | [**ntropy_server_period_api_period_v22_period_get_account_holder_transactions**](docs/AccountHolderApi.md#ntropy_server_period_api_period_v22_period_get_account_holder_transactions) | **GET** /v2/account-holder/{account_holder_id}/transactions | List transactions of an account holder.
*EnrichmentApi* | [**ntropy_server_period_api_period_v22_period_enrich_transactions_async**](docs/EnrichmentApi.md#ntropy_server_period_api_period_v22_period_enrich_transactions_async) | **POST** /v2/transactions/async | Enrich and add transactions to the ledgers of account holders asynchronously.
*EnrichmentApi* | [**ntropy_server_period_api_period_v22_period_enrich_transactions_sync**](docs/EnrichmentApi.md#ntropy_server_period_api_period_v22_period_enrich_transactions_sync) | **POST** /v2/transactions/sync | Enrich and add transactions to the ledger of account holders synchronously.
*EnrichmentApi* | [**ntropy_server_period_api_period_v22_period_get_async_enrichment_result**](docs/EnrichmentApi.md#ntropy_server_period_api_period_v22_period_get_async_enrichment_result) | **GET** /v2/transactions/async/{id} | Fetch the result of a batch transaction enrichment.
*MiscApi* | [**ntropy_server_period_api_period_v2_period_get_chart_of_accounts**](docs/MiscApi.md#ntropy_server_period_api_period_v2_period_get_chart_of_accounts) | **GET** /v2/chart-of-accounts | Get the chart of accounts that the Ntropy API can return
*MiscApi* | [**ntropy_server_period_api_period_v2_period_get_labels_hierarchy**](docs/MiscApi.md#ntropy_server_period_api_period_v2_period_get_labels_hierarchy) | **GET** /v2/labels/hierarchy/{account_holder_type} | Get the hierarchy of Ntropy labels
*MiscApi* | [**ntropy_server_period_api_period_v2_period_health_get**](docs/MiscApi.md#ntropy_server_period_api_period_v2_period_health_get) | **GET** /health | Check API health.
*MiscApi* | [**ntropy_server_period_api_period_v2_period_report_enrichment**](docs/MiscApi.md#ntropy_server_period_api_period_v2_period_report_enrichment) | **POST** /v2/report | Report a wrongly classified transaction.


## Documentation For Models

 - [AccountHolder](docs/AccountHolder.md)
 - [AccountHolderMetricName](docs/AccountHolderMetricName.md)
 - [AccountHolderMetricsQuery](docs/AccountHolderMetricsQuery.md)
 - [AccountHolderMetricsQueryResult](docs/AccountHolderMetricsQueryResult.md)
 - [AccountHolderMetricsQueryResultStatsByLabelValue](docs/AccountHolderMetricsQueryResultStatsByLabelValue.md)
 - [AccountHolderOverallStatsMetric](docs/AccountHolderOverallStatsMetric.md)
 - [AccountHolderTransaction](docs/AccountHolderTransaction.md)
 - [AccountHolderTransactionResult](docs/AccountHolderTransactionResult.md)
 - [AccountHolderType](docs/AccountHolderType.md)
 - [BatchResult](docs/BatchResult.md)
 - [ChartOfAccountsValue](docs/ChartOfAccountsValue.md)
 - [EnrichedTransaction](docs/EnrichedTransaction.md)
 - [EnrichmentReport](docs/EnrichmentReport.md)
 - [IncomeGroup](docs/IncomeGroup.md)
 - [LabelledTransaction](docs/LabelledTransaction.md)
 - [LabelsHierarchyValue](docs/LabelsHierarchyValue.md)
 - [MetricStatisticsValue](docs/MetricStatisticsValue.md)
 - [MissingAccountHoldersError](docs/MissingAccountHoldersError.md)
 - [Model](docs/Model.md)
 - [ModelCreateRequest](docs/ModelCreateRequest.md)
 - [ModelStatus](docs/ModelStatus.md)
 - [NtropyServerApiV22EnrichTransactionsSync200ResponseInner](docs/NtropyServerApiV22EnrichTransactionsSync200ResponseInner.md)
 - [RecurrenceGroup](docs/RecurrenceGroup.md)
 - [RecurrenceGroupOneOf](docs/RecurrenceGroupOneOf.md)
 - [Transaction](docs/Transaction.md)
 - [TransactionError](docs/TransactionError.md)
 - [TransactionErrorErrorDetails](docs/TransactionErrorErrorDetails.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

api@ntropy.com

