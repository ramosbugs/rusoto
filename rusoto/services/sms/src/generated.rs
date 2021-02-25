// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;

use async_trait::async_trait;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::proto;
use rusoto_core::request::HttpResponse;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

impl ServerMigrationServiceClient {
    pub(crate) fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "sms", &self.region, request_uri);

        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request
    }

    pub(crate) async fn sign_and_dispatch<E>(
        &self,
        request: SignedRequest,
        from_response: fn(BufferedHttpResponse) -> RusotoError<E>,
    ) -> Result<HttpResponse, RusotoError<E>> {
        let mut response = self.client.sign_and_dispatch(request).await?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(from_response(response));
        }

        Ok(response)
    }
}

use serde_json;
/// <p>Information about the application.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AppSummary {
    /// <p>The unique ID of the application.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// <p>The creation time of the application.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The description of the application.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the application.</p>
    #[serde(rename = "importedAppId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_app_id: Option<String>,
    /// <p>The last modified time of the application.</p>
    #[serde(rename = "lastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    /// <p>The timestamp of the application's most recent successful replication.</p>
    #[serde(rename = "latestReplicationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_replication_time: Option<f64>,
    /// <p>Status of the launch configuration.</p>
    #[serde(rename = "launchConfigurationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_configuration_status: Option<String>,
    /// <p>Details about the latest launch of the application.</p>
    #[serde(rename = "launchDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_details: Option<LaunchDetails>,
    /// <p>The launch status of the application.</p>
    #[serde(rename = "launchStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_status: Option<String>,
    /// <p>A message related to the launch status of the application.</p>
    #[serde(rename = "launchStatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_status_message: Option<String>,
    /// <p>The name of the application.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Status of the replication configuration.</p>
    #[serde(rename = "replicationConfigurationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_configuration_status: Option<String>,
    /// <p>The replication status of the application.</p>
    #[serde(rename = "replicationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_status: Option<String>,
    /// <p>A message related to the replication status of the application.</p>
    #[serde(rename = "replicationStatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_status_message: Option<String>,
    /// <p>The name of the service role in the customer's account used by AWS SMS.</p>
    #[serde(rename = "roleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// <p>Status of the application.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A message related to the status of the application</p>
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>The number of server groups present in the application.</p>
    #[serde(rename = "totalServerGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_server_groups: Option<i64>,
    /// <p>The number of servers present in the application.</p>
    #[serde(rename = "totalServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_servers: Option<i64>,
}

/// <p>Configuration for validating an application.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AppValidationConfiguration {
    /// <p>The validation strategy.</p>
    #[serde(rename = "appValidationStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_validation_strategy: Option<String>,
    /// <p>The name of the configuration.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The validation parameters.</p>
    #[serde(rename = "ssmValidationParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssm_validation_parameters: Option<SSMValidationParameters>,
    /// <p>The ID of the validation.</p>
    #[serde(rename = "validationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_id: Option<String>,
}

/// <p>Output from validating an application.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AppValidationOutput {
    /// <p>Output from using SSM to validate the application.</p>
    #[serde(rename = "ssmOutput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssm_output: Option<SSMOutput>,
}

/// <p>Represents a connector.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Connector {
    /// <p>The time the connector was associated.</p>
    #[serde(rename = "associatedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_on: Option<f64>,
    /// <p>The capabilities of the connector.</p>
    #[serde(rename = "capabilityList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_list: Option<Vec<String>>,
    /// <p>The ID of the connector.</p>
    #[serde(rename = "connectorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    /// <p>The IP address of the connector.</p>
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// <p>The MAC address of the connector.</p>
    #[serde(rename = "macAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// <p>The status of the connector.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The connector version.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>The ID of the VM manager.</p>
    #[serde(rename = "vmManagerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_manager_id: Option<String>,
    /// <p>The name of the VM manager.</p>
    #[serde(rename = "vmManagerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_manager_name: Option<String>,
    /// <p>The VM management product.</p>
    #[serde(rename = "vmManagerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_manager_type: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateAppRequest {
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of application creation.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The description of the new application</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the new application.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The name of the service role in the customer's account to be used by AWS SMS.</p>
    #[serde(rename = "roleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// <p>The server groups to include in the application.</p>
    #[serde(rename = "serverGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_groups: Option<Vec<ServerGroup>>,
    /// <p>The tags to be associated with the application.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAppResponse {
    /// <p>A summary description of the application.</p>
    #[serde(rename = "appSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_summary: Option<AppSummary>,
    /// <p>The server groups included in the application.</p>
    #[serde(rename = "serverGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_groups: Option<Vec<ServerGroup>>,
    /// <p>The tags associated with the application.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateReplicationJobRequest {
    /// <p>The description of the replication job.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Indicates whether the replication job produces encrypted AMIs.</p>
    #[serde(rename = "encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// <p>The time between consecutive replication runs, in hours.</p>
    #[serde(rename = "frequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i64>,
    /// <p>The ID of the KMS key for replication jobs that produce encrypted AMIs. This value can be any of the following:</p> <ul> <li> <p>KMS key ID</p> </li> <li> <p>KMS key alias</p> </li> <li> <p>ARN referring to the KMS key ID</p> </li> <li> <p>ARN referring to the KMS key alias</p> </li> </ul> <p> If encrypted is <i>true</i> but a KMS key ID is not specified, the customer's default KMS key for Amazon EBS is used. </p>
    #[serde(rename = "kmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The license type to be used for the AMI created by a successful replication run.</p>
    #[serde(rename = "licenseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    /// <p>The maximum number of SMS-created AMIs to retain. The oldest is deleted after the maximum number is reached and a new AMI is created.</p>
    #[serde(rename = "numberOfRecentAmisToKeep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_recent_amis_to_keep: Option<i64>,
    /// <p>The name of the IAM role to be used by the AWS SMS.</p>
    #[serde(rename = "roleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// <p>Indicates whether to run the replication job one time.</p>
    #[serde(rename = "runOnce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_once: Option<bool>,
    /// <p>The seed replication time.</p>
    #[serde(rename = "seedReplicationTime")]
    pub seed_replication_time: f64,
    /// <p>The ID of the server.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateReplicationJobResponse {
    /// <p>The unique identifier of the replication job.</p>
    #[serde(rename = "replicationJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_job_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAppLaunchConfigurationRequest {
    /// <p>The ID of the application.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteAppLaunchConfigurationResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAppReplicationConfigurationRequest {
    /// <p>The ID of the application.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteAppReplicationConfigurationResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAppRequest {
    /// <p>The ID of the application.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// <p>Indicates whether to stop all replication jobs corresponding to the servers in the application while deleting the application.</p>
    #[serde(rename = "forceStopAppReplication")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_stop_app_replication: Option<bool>,
    /// <p>Indicates whether to terminate the stack corresponding to the application while deleting the application.</p>
    #[serde(rename = "forceTerminateApp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_terminate_app: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteAppResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAppValidationConfigurationRequest {
    /// <p>The ID of the application.</p>
    #[serde(rename = "appId")]
    pub app_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteAppValidationConfigurationResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteReplicationJobRequest {
    /// <p>The ID of the replication job.</p>
    #[serde(rename = "replicationJobId")]
    pub replication_job_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteReplicationJobResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteServerCatalogRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteServerCatalogResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateConnectorRequest {
    /// <p>The ID of the connector.</p>
    #[serde(rename = "connectorId")]
    pub connector_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateConnectorResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GenerateChangeSetRequest {
    /// <p>The ID of the application associated with the change set.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// <p>The format for the change set.</p>
    #[serde(rename = "changesetFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changeset_format: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GenerateChangeSetResponse {
    /// <p>The location of the Amazon S3 object.</p>
    #[serde(rename = "s3Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_location: Option<S3Location>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GenerateTemplateRequest {
    /// <p>The ID of the application associated with the AWS CloudFormation template.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// <p>The format for generating the AWS CloudFormation template.</p>
    #[serde(rename = "templateFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_format: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GenerateTemplateResponse {
    /// <p>The location of the Amazon S3 object.</p>
    #[serde(rename = "s3Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_location: Option<S3Location>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAppLaunchConfigurationRequest {
    /// <p>The ID of the application.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAppLaunchConfigurationResponse {
    /// <p>The ID of the application.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// <p>Indicates whether the application is configured to launch automatically after replication is complete.</p>
    #[serde(rename = "autoLaunch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_launch: Option<bool>,
    /// <p>The name of the service role in the customer's account that AWS CloudFormation uses to launch the application.</p>
    #[serde(rename = "roleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// <p>The launch configurations for server groups in this application.</p>
    #[serde(rename = "serverGroupLaunchConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group_launch_configurations: Option<Vec<ServerGroupLaunchConfiguration>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAppReplicationConfigurationRequest {
    /// <p>The ID of the application.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAppReplicationConfigurationResponse {
    /// <p>The replication configurations associated with server groups in this application.</p>
    #[serde(rename = "serverGroupReplicationConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group_replication_configurations: Option<Vec<ServerGroupReplicationConfiguration>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAppRequest {
    /// <p>The ID of the application.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAppResponse {
    /// <p>Information about the application.</p>
    #[serde(rename = "appSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_summary: Option<AppSummary>,
    /// <p>The server groups that belong to the application.</p>
    #[serde(rename = "serverGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_groups: Option<Vec<ServerGroup>>,
    /// <p>The tags associated with the application.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAppValidationConfigurationRequest {
    /// <p>The ID of the application.</p>
    #[serde(rename = "appId")]
    pub app_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAppValidationConfigurationResponse {
    /// <p>The configuration for application validation.</p>
    #[serde(rename = "appValidationConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_validation_configurations: Option<Vec<AppValidationConfiguration>>,
    /// <p>The configuration for instance validation.</p>
    #[serde(rename = "serverGroupValidationConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group_validation_configurations: Option<Vec<ServerGroupValidationConfiguration>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAppValidationOutputRequest {
    /// <p>The ID of the application.</p>
    #[serde(rename = "appId")]
    pub app_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAppValidationOutputResponse {
    /// <p>The validation output.</p>
    #[serde(rename = "validationOutputList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_output_list: Option<Vec<ValidationOutput>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetConnectorsRequest {
    /// <p>The maximum number of results to return in a single call. The default value is 50. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetConnectorsResponse {
    /// <p>Information about the registered connectors.</p>
    #[serde(rename = "connectorList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_list: Option<Vec<Connector>>,
    /// <p>The token required to retrieve the next set of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetReplicationJobsRequest {
    /// <p>The maximum number of results to return in a single call. The default value is 50. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the replication job.</p>
    #[serde(rename = "replicationJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_job_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetReplicationJobsResponse {
    /// <p>The token required to retrieve the next set of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the replication jobs.</p>
    #[serde(rename = "replicationJobList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_job_list: Option<Vec<ReplicationJob>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetReplicationRunsRequest {
    /// <p>The maximum number of results to return in a single call. The default value is 50. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the replication job.</p>
    #[serde(rename = "replicationJobId")]
    pub replication_job_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetReplicationRunsResponse {
    /// <p>The token required to retrieve the next set of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the replication job.</p>
    #[serde(rename = "replicationJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_job: Option<ReplicationJob>,
    /// <p>Information about the replication runs.</p>
    #[serde(rename = "replicationRunList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_run_list: Option<Vec<ReplicationRun>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetServersRequest {
    /// <p>The maximum number of results to return in a single call. The default value is 50. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The server addresses.</p>
    #[serde(rename = "vmServerAddressList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_server_address_list: Option<Vec<VmServerAddress>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetServersResponse {
    /// <p>The time when the server was last modified.</p>
    #[serde(rename = "lastModifiedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<f64>,
    /// <p>The token required to retrieve the next set of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The status of the server catalog.</p>
    #[serde(rename = "serverCatalogStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_catalog_status: Option<String>,
    /// <p>Information about the servers.</p>
    #[serde(rename = "serverList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_list: Option<Vec<Server>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ImportAppCatalogRequest {
    /// <p>The name of the service role. If you omit this parameter, we create a service-linked role for AWS Migration Hub in your account. Otherwise, the role that you provide must have the <a href="https://docs.aws.amazon.com/migrationhub/latest/ug/new-customer-setup.html#sms-managed">policy and trust policy</a> described in the <i>AWS Migration Hub User Guide</i>.</p>
    #[serde(rename = "roleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImportAppCatalogResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ImportServerCatalogRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImportServerCatalogResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct LaunchAppRequest {
    /// <p>The ID of the application.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LaunchAppResponse {}

/// <p>Details about the latest launch of an application.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LaunchDetails {
    /// <p>The latest time that this application was launched successfully.</p>
    #[serde(rename = "latestLaunchTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_launch_time: Option<f64>,
    /// <p>The ID of the latest stack launched for this application.</p>
    #[serde(rename = "stackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    /// <p>The name of the latest stack launched for this application.</p>
    #[serde(rename = "stackName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAppsRequest {
    /// <p>The unique application IDs.</p>
    #[serde(rename = "appIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_ids: Option<Vec<String>>,
    /// <p>The maximum number of results to return in a single call. The default value is 100. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAppsResponse {
    /// <p>The application summaries.</p>
    #[serde(rename = "apps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<AppSummary>>,
    /// <p>The token required to retrieve the next set of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Contains the status of validating an application.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct NotificationContext {
    /// <p>The status of the validation.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The status message.</p>
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>The ID of the validation.</p>
    #[serde(rename = "validationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct NotifyAppValidationOutputRequest {
    /// <p>The ID of the application.</p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p>The notification information.</p>
    #[serde(rename = "notificationContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_context: Option<NotificationContext>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NotifyAppValidationOutputResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutAppLaunchConfigurationRequest {
    /// <p>The ID of the application.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// <p>Indicates whether the application is configured to launch automatically after replication is complete.</p>
    #[serde(rename = "autoLaunch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_launch: Option<bool>,
    /// <p>The name of service role in the customer's account that AWS CloudFormation uses to launch the application.</p>
    #[serde(rename = "roleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// <p>Information about the launch configurations for server groups in the application.</p>
    #[serde(rename = "serverGroupLaunchConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group_launch_configurations: Option<Vec<ServerGroupLaunchConfiguration>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutAppLaunchConfigurationResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutAppReplicationConfigurationRequest {
    /// <p>The ID of the application.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// <p>Information about the replication configurations for server groups in the application.</p>
    #[serde(rename = "serverGroupReplicationConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group_replication_configurations: Option<Vec<ServerGroupReplicationConfiguration>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutAppReplicationConfigurationResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutAppValidationConfigurationRequest {
    /// <p>The ID of the application.</p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p>The configuration for application validation.</p>
    #[serde(rename = "appValidationConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_validation_configurations: Option<Vec<AppValidationConfiguration>>,
    /// <p>The configuration for instance validation.</p>
    #[serde(rename = "serverGroupValidationConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group_validation_configurations: Option<Vec<ServerGroupValidationConfiguration>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutAppValidationConfigurationResponse {}

/// <p>Represents a replication job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReplicationJob {
    /// <p>The description of the replication job.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Indicates whether the replication job should produce encrypted AMIs.</p>
    #[serde(rename = "encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// <p>The time between consecutive replication runs, in hours.</p>
    #[serde(rename = "frequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i64>,
    /// <p>The ID of the KMS key for replication jobs that produce encrypted AMIs. This value can be any of the following: </p> <ul> <li> <p>KMS key ID</p> </li> <li> <p>KMS key alias</p> </li> <li> <p>ARN referring to the KMS key ID</p> </li> <li> <p>ARN referring to the KMS key alias</p> </li> </ul> <p>If encrypted is enabled but a KMS key ID is not specified, the customer's default KMS key for Amazon EBS is used.</p>
    #[serde(rename = "kmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The ID of the latest Amazon Machine Image (AMI).</p>
    #[serde(rename = "latestAmiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_ami_id: Option<String>,
    /// <p>The license type to be used for the AMI created by a successful replication run.</p>
    #[serde(rename = "licenseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    /// <p>The start time of the next replication run.</p>
    #[serde(rename = "nextReplicationRunStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_replication_run_start_time: Option<f64>,
    /// <p>The number of recent AMIs to keep in the customer's account for a replication job. By default, the value is set to zero, meaning that all AMIs are kept.</p>
    #[serde(rename = "numberOfRecentAmisToKeep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_recent_amis_to_keep: Option<i64>,
    /// <p>The ID of the replication job.</p>
    #[serde(rename = "replicationJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_job_id: Option<String>,
    /// <p>Information about the replication runs.</p>
    #[serde(rename = "replicationRunList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_run_list: Option<Vec<ReplicationRun>>,
    /// <p>The name of the IAM role to be used by AWS SMS.</p>
    #[serde(rename = "roleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// <p>Indicates whether to run the replication job one time.</p>
    #[serde(rename = "runOnce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_once: Option<bool>,
    /// <p>The seed replication time.</p>
    #[serde(rename = "seedReplicationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_replication_time: Option<f64>,
    /// <p>The ID of the server.</p>
    #[serde(rename = "serverId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    /// <p>The type of server.</p>
    #[serde(rename = "serverType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    /// <p>The state of the replication job.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The description of the current status of the replication job.</p>
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>Information about the VM server.</p>
    #[serde(rename = "vmServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_server: Option<VmServer>,
}

/// <p>Represents a replication run.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReplicationRun {
    /// <p>The ID of the Amazon Machine Image (AMI) from the replication run.</p>
    #[serde(rename = "amiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_id: Option<String>,
    /// <p>The completion time of the last replication run.</p>
    #[serde(rename = "completedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_time: Option<f64>,
    /// <p>The description of the replication run.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Indicates whether the replication run should produce an encrypted AMI.</p>
    #[serde(rename = "encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// <p>The ID of the KMS key for replication jobs that produce encrypted AMIs. This value can be any of the following:</p> <ul> <li> <p>KMS key ID</p> </li> <li> <p>KMS key alias</p> </li> <li> <p>ARN referring to the KMS key ID</p> </li> <li> <p>ARN referring to the KMS key alias</p> </li> </ul> <p> If encrypted is <i>true</i> but a KMS key ID is not specified, the customer's default KMS key for Amazon EBS is used. </p>
    #[serde(rename = "kmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The ID of the replication run.</p>
    #[serde(rename = "replicationRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_run_id: Option<String>,
    /// <p>The start time of the next replication run.</p>
    #[serde(rename = "scheduledStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_start_time: Option<f64>,
    /// <p>Details about the current stage of the replication run.</p>
    #[serde(rename = "stageDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_details: Option<ReplicationRunStageDetails>,
    /// <p>The state of the replication run.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The description of the current status of the replication job.</p>
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>The type of replication run.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Details of the current stage of a replication run.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReplicationRunStageDetails {
    /// <p>The current stage of a replication run.</p>
    #[serde(rename = "stage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    /// <p>The progress of the current stage of a replication run.</p>
    #[serde(rename = "stageProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_progress: Option<String>,
}

/// <p>Location of an Amazon S3 object.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct S3Location {
    /// <p>The Amazon S3 bucket name.</p>
    #[serde(rename = "bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// <p>The Amazon S3 bucket key.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

/// <p>Contains the location of validation output.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SSMOutput {
    #[serde(rename = "s3Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_location: Option<S3Location>,
}

/// <p>Contains validation parameters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SSMValidationParameters {
    /// <p>The command to run the validation script</p>
    #[serde(rename = "command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    /// <p>The timeout interval, in seconds.</p>
    #[serde(rename = "executionTimeoutSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_timeout_seconds: Option<i64>,
    /// <p>The ID of the instance. The instance must have the following tag: UserForSMSApplicationValidation=true.</p>
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The name of the S3 bucket for output.</p>
    #[serde(rename = "outputS3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_bucket_name: Option<String>,
    /// <p>The type of validation script.</p>
    #[serde(rename = "scriptType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_type: Option<String>,
    /// <p>The location of the validation script.</p>
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
}

/// <p>Represents a server.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Server {
    /// <p>The ID of the replication job.</p>
    #[serde(rename = "replicationJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_job_id: Option<String>,
    /// <p>Indicates whether the replication job is deleted or failed.</p>
    #[serde(rename = "replicationJobTerminated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_job_terminated: Option<bool>,
    /// <p>The ID of the server.</p>
    #[serde(rename = "serverId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    /// <p>The type of server.</p>
    #[serde(rename = "serverType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    /// <p>Information about the VM server.</p>
    #[serde(rename = "vmServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_server: Option<VmServer>,
}

/// <p>Logical grouping of servers.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ServerGroup {
    /// <p>The name of a server group.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of a server group.</p>
    #[serde(rename = "serverGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group_id: Option<String>,
    /// <p>The servers that belong to a server group.</p>
    #[serde(rename = "serverList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_list: Option<Vec<Server>>,
}

/// <p>Launch configuration for a server group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ServerGroupLaunchConfiguration {
    /// <p>The launch order of servers in the server group.</p>
    #[serde(rename = "launchOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_order: Option<i64>,
    /// <p>The ID of the server group with which the launch configuration is associated.</p>
    #[serde(rename = "serverGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group_id: Option<String>,
    /// <p>The launch configuration for servers in the server group.</p>
    #[serde(rename = "serverLaunchConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_launch_configurations: Option<Vec<ServerLaunchConfiguration>>,
}

/// <p>Replication configuration for a server group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ServerGroupReplicationConfiguration {
    /// <p>The ID of the server group with which this replication configuration is associated.</p>
    #[serde(rename = "serverGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group_id: Option<String>,
    /// <p>The replication configuration for servers in the server group.</p>
    #[serde(rename = "serverReplicationConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_replication_configurations: Option<Vec<ServerReplicationConfiguration>>,
}

/// <p>Configuration for validating an instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ServerGroupValidationConfiguration {
    /// <p>The ID of the server group.</p>
    #[serde(rename = "serverGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group_id: Option<String>,
    /// <p>The validation configuration.</p>
    #[serde(rename = "serverValidationConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_validation_configurations: Option<Vec<ServerValidationConfiguration>>,
}

/// <p>Launch configuration for a server.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ServerLaunchConfiguration {
    /// <p>Indicates whether a publicly accessible IP address is created when launching the server.</p>
    #[serde(rename = "associatePublicIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associate_public_ip_address: Option<bool>,
    #[serde(rename = "configureScript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configure_script: Option<S3Location>,
    /// <p>The type of configuration script.</p>
    #[serde(rename = "configureScriptType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configure_script_type: Option<String>,
    /// <p>The name of the Amazon EC2 SSH key to be used for connecting to the launched server.</p>
    #[serde(rename = "ec2KeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_key_name: Option<String>,
    /// <p>The name of the IAM instance profile.</p>
    #[serde(rename = "iamInstanceProfileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_instance_profile_name: Option<String>,
    /// <p>The instance type to use when launching the server.</p>
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>The logical ID of the server in the AWS CloudFormation template.</p>
    #[serde(rename = "logicalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_id: Option<String>,
    /// <p>The ID of the security group that applies to the launched server.</p>
    #[serde(rename = "securityGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group: Option<String>,
    /// <p>The ID of the server with which the launch configuration is associated.</p>
    #[serde(rename = "server")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<Server>,
    /// <p>The ID of the subnet the server should be launched into.</p>
    #[serde(rename = "subnet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
    /// <p>Location of the user-data script to be executed when launching the server.</p>
    #[serde(rename = "userData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<UserData>,
    /// <p>The ID of the VPC into which the server should be launched.</p>
    #[serde(rename = "vpc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<String>,
}

/// <p>Replication configuration of a server.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ServerReplicationConfiguration {
    /// <p>The ID of the server with which this replication configuration is associated.</p>
    #[serde(rename = "server")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<Server>,
    /// <p>The parameters for replicating the server.</p>
    #[serde(rename = "serverReplicationParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_replication_parameters: Option<ServerReplicationParameters>,
}

/// <p>The replication parameters for replicating a server.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ServerReplicationParameters {
    /// <p>Indicates whether the replication job produces encrypted AMIs.</p>
    #[serde(rename = "encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// <p>The frequency of creating replication jobs for the server.</p>
    #[serde(rename = "frequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i64>,
    /// <p>The ID of the KMS key for replication jobs that produce encrypted AMIs. This value can be any of the following:</p> <ul> <li> <p>KMS key ID</p> </li> <li> <p>KMS key alias</p> </li> <li> <p>ARN referring to the KMS key ID</p> </li> <li> <p>ARN referring to the KMS key alias</p> </li> </ul> <p>If encrypted is enabled but a KMS key ID is not specified, the customer's default KMS key for Amazon EBS is used.</p>
    #[serde(rename = "kmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The license type for creating a replication job for the server.</p>
    #[serde(rename = "licenseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    /// <p>The number of recent AMIs to keep when creating a replication job for this server.</p>
    #[serde(rename = "numberOfRecentAmisToKeep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_recent_amis_to_keep: Option<i64>,
    /// <p>Indicates whether to run the replication job one time.</p>
    #[serde(rename = "runOnce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_once: Option<bool>,
    /// <p>The seed time for creating a replication job for the server.</p>
    #[serde(rename = "seedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_time: Option<f64>,
}

/// <p>Configuration for validating an instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ServerValidationConfiguration {
    /// <p>The name of the configuration.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "server")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<Server>,
    /// <p>The validation strategy.</p>
    #[serde(rename = "serverValidationStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_validation_strategy: Option<String>,
    /// <p>The validation parameters.</p>
    #[serde(rename = "userDataValidationParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data_validation_parameters: Option<UserDataValidationParameters>,
    /// <p>The ID of the validation.</p>
    #[serde(rename = "validationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_id: Option<String>,
}

/// <p>Contains output from validating an instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ServerValidationOutput {
    #[serde(rename = "server")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<Server>,
}

/// <p>Contains the location of a validation script.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Source {
    #[serde(rename = "s3Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_location: Option<S3Location>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartAppReplicationRequest {
    /// <p>The ID of the application.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartAppReplicationResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartOnDemandAppReplicationRequest {
    /// <p>The ID of the application.</p>
    #[serde(rename = "appId")]
    pub app_id: String,
    /// <p>The description of the replication run.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartOnDemandAppReplicationResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartOnDemandReplicationRunRequest {
    /// <p>The description of the replication run.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the replication job.</p>
    #[serde(rename = "replicationJobId")]
    pub replication_job_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartOnDemandReplicationRunResponse {
    /// <p>The ID of the replication run.</p>
    #[serde(rename = "replicationRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_run_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopAppReplicationRequest {
    /// <p>The ID of the application.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopAppReplicationResponse {}

/// <p>Key/value pair that can be assigned to an application.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>The tag key.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The tag value.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TerminateAppRequest {
    /// <p>The ID of the application.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TerminateAppResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateAppRequest {
    /// <p>The ID of the application.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// <p>The new description of the application.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The new name of the application.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The name of the service role in the customer's account used by AWS SMS.</p>
    #[serde(rename = "roleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// <p>The server groups in the application to update.</p>
    #[serde(rename = "serverGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_groups: Option<Vec<ServerGroup>>,
    /// <p>The tags to associate with the application.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateAppResponse {
    /// <p>A summary description of the application.</p>
    #[serde(rename = "appSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_summary: Option<AppSummary>,
    /// <p>The updated server groups in the application.</p>
    #[serde(rename = "serverGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_groups: Option<Vec<ServerGroup>>,
    /// <p>The tags associated with the application.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateReplicationJobRequest {
    /// <p>The description of the replication job.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>When true, the replication job produces encrypted AMIs. For more information, <code>KmsKeyId</code>.</p>
    #[serde(rename = "encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// <p>The time between consecutive replication runs, in hours.</p>
    #[serde(rename = "frequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i64>,
    /// <p>The ID of the KMS key for replication jobs that produce encrypted AMIs. This value can be any of the following:</p> <ul> <li> <p>KMS key ID</p> </li> <li> <p>KMS key alias</p> </li> <li> <p>ARN referring to the KMS key ID</p> </li> <li> <p>ARN referring to the KMS key alias</p> </li> </ul> <p>If encrypted is enabled but a KMS key ID is not specified, the customer's default KMS key for Amazon EBS is used.</p>
    #[serde(rename = "kmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The license type to be used for the AMI created by a successful replication run.</p>
    #[serde(rename = "licenseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    /// <p>The start time of the next replication run.</p>
    #[serde(rename = "nextReplicationRunStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_replication_run_start_time: Option<f64>,
    /// <p>The maximum number of SMS-created AMIs to retain. The oldest is deleted after the maximum number is reached and a new AMI is created.</p>
    #[serde(rename = "numberOfRecentAmisToKeep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_recent_amis_to_keep: Option<i64>,
    /// <p>The ID of the replication job.</p>
    #[serde(rename = "replicationJobId")]
    pub replication_job_id: String,
    /// <p>The name of the IAM role to be used by AWS SMS.</p>
    #[serde(rename = "roleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateReplicationJobResponse {}

/// <p>A script that runs on first launch of an Amazon EC2 instance. Used for configuring the server during launch.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct UserData {
    /// <p>Amazon S3 location of the user-data script.</p>
    #[serde(rename = "s3Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_location: Option<S3Location>,
}

/// <p>Contains validation parameters.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct UserDataValidationParameters {
    /// <p>The type of validation script.</p>
    #[serde(rename = "scriptType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_type: Option<String>,
    /// <p>The location of the validation script.</p>
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
}

/// <p>Contains validation output.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ValidationOutput {
    /// <p>The output from validating an application.</p>
    #[serde(rename = "appValidationOutput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_validation_output: Option<AppValidationOutput>,
    /// <p>The latest time that the validation was performed.</p>
    #[serde(rename = "latestValidationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_validation_time: Option<f64>,
    /// <p>The name of the validation.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The output from validation an instance.</p>
    #[serde(rename = "serverValidationOutput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_validation_output: Option<ServerValidationOutput>,
    /// <p>The status of the validation.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The status message.</p>
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>The ID of the validation.</p>
    #[serde(rename = "validationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_id: Option<String>,
}

/// <p>Represents a VM server.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VmServer {
    /// <p>The name of the VM manager.</p>
    #[serde(rename = "vmManagerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_manager_name: Option<String>,
    /// <p>The type of VM management product.</p>
    #[serde(rename = "vmManagerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_manager_type: Option<String>,
    /// <p>The name of the VM.</p>
    #[serde(rename = "vmName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_name: Option<String>,
    /// <p>The VM folder path in the vCenter Server virtual machine inventory tree.</p>
    #[serde(rename = "vmPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_path: Option<String>,
    /// <p>The VM server location.</p>
    #[serde(rename = "vmServerAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_server_address: Option<VmServerAddress>,
}

/// <p>Represents a VM server location.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VmServerAddress {
    /// <p>The ID of the VM.</p>
    #[serde(rename = "vmId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    /// <p>The ID of the VM manager.</p>
    #[serde(rename = "vmManagerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_manager_id: Option<String>,
}

/// Errors returned by CreateApp
#[derive(Debug, PartialEq)]
pub enum CreateAppError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl CreateAppError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAppError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(CreateAppError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateAppError::InvalidParameter(err.msg))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(CreateAppError::MissingRequiredParameter(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(CreateAppError::OperationNotPermitted(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(CreateAppError::UnauthorizedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateAppError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAppError::InternalError(ref cause) => write!(f, "{}", cause),
            CreateAppError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateAppError::MissingRequiredParameter(ref cause) => write!(f, "{}", cause),
            CreateAppError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            CreateAppError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateAppError {}
/// Errors returned by CreateReplicationJob
#[derive(Debug, PartialEq)]
pub enum CreateReplicationJobError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>There are no connectors available.</p>
    NoConnectorsAvailable(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>The specified replication job already exists.</p>
    ReplicationJobAlreadyExists(String),
    /// <p>The specified server cannot be replicated.</p>
    ServerCannotBeReplicated(String),
    /// <p>The service is temporarily unavailable.</p>
    TemporarilyUnavailable(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl CreateReplicationJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateReplicationJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(CreateReplicationJobError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateReplicationJobError::InvalidParameter(
                        err.msg,
                    ))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        CreateReplicationJobError::MissingRequiredParameter(err.msg),
                    )
                }
                "NoConnectorsAvailableException" => {
                    return RusotoError::Service(CreateReplicationJobError::NoConnectorsAvailable(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(CreateReplicationJobError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "ReplicationJobAlreadyExistsException" => {
                    return RusotoError::Service(
                        CreateReplicationJobError::ReplicationJobAlreadyExists(err.msg),
                    )
                }
                "ServerCannotBeReplicatedException" => {
                    return RusotoError::Service(
                        CreateReplicationJobError::ServerCannotBeReplicated(err.msg),
                    )
                }
                "TemporarilyUnavailableException" => {
                    return RusotoError::Service(CreateReplicationJobError::TemporarilyUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(CreateReplicationJobError::UnauthorizedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateReplicationJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateReplicationJobError::InternalError(ref cause) => write!(f, "{}", cause),
            CreateReplicationJobError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateReplicationJobError::MissingRequiredParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationJobError::NoConnectorsAvailable(ref cause) => write!(f, "{}", cause),
            CreateReplicationJobError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            CreateReplicationJobError::ReplicationJobAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationJobError::ServerCannotBeReplicated(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationJobError::TemporarilyUnavailable(ref cause) => write!(f, "{}", cause),
            CreateReplicationJobError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateReplicationJobError {}
/// Errors returned by DeleteApp
#[derive(Debug, PartialEq)]
pub enum DeleteAppError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl DeleteAppError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAppError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(DeleteAppError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteAppError::InvalidParameter(err.msg))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(DeleteAppError::MissingRequiredParameter(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(DeleteAppError::OperationNotPermitted(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(DeleteAppError::UnauthorizedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAppError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAppError::InternalError(ref cause) => write!(f, "{}", cause),
            DeleteAppError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteAppError::MissingRequiredParameter(ref cause) => write!(f, "{}", cause),
            DeleteAppError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            DeleteAppError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAppError {}
/// Errors returned by DeleteAppLaunchConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteAppLaunchConfigurationError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl DeleteAppLaunchConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteAppLaunchConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(DeleteAppLaunchConfigurationError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DeleteAppLaunchConfigurationError::InvalidParameter(err.msg),
                    )
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        DeleteAppLaunchConfigurationError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        DeleteAppLaunchConfigurationError::OperationNotPermitted(err.msg),
                    )
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(
                        DeleteAppLaunchConfigurationError::UnauthorizedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAppLaunchConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAppLaunchConfigurationError::InternalError(ref cause) => write!(f, "{}", cause),
            DeleteAppLaunchConfigurationError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteAppLaunchConfigurationError::MissingRequiredParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteAppLaunchConfigurationError::OperationNotPermitted(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteAppLaunchConfigurationError::UnauthorizedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteAppLaunchConfigurationError {}
/// Errors returned by DeleteAppReplicationConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteAppReplicationConfigurationError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl DeleteAppReplicationConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteAppReplicationConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(
                        DeleteAppReplicationConfigurationError::InternalError(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DeleteAppReplicationConfigurationError::InvalidParameter(err.msg),
                    )
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        DeleteAppReplicationConfigurationError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        DeleteAppReplicationConfigurationError::OperationNotPermitted(err.msg),
                    )
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(
                        DeleteAppReplicationConfigurationError::UnauthorizedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAppReplicationConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAppReplicationConfigurationError::InternalError(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteAppReplicationConfigurationError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteAppReplicationConfigurationError::MissingRequiredParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteAppReplicationConfigurationError::OperationNotPermitted(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteAppReplicationConfigurationError::UnauthorizedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteAppReplicationConfigurationError {}
/// Errors returned by DeleteAppValidationConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteAppValidationConfigurationError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl DeleteAppValidationConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteAppValidationConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(
                        DeleteAppValidationConfigurationError::InternalError(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DeleteAppValidationConfigurationError::InvalidParameter(err.msg),
                    )
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        DeleteAppValidationConfigurationError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        DeleteAppValidationConfigurationError::OperationNotPermitted(err.msg),
                    )
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(
                        DeleteAppValidationConfigurationError::UnauthorizedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAppValidationConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAppValidationConfigurationError::InternalError(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteAppValidationConfigurationError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteAppValidationConfigurationError::MissingRequiredParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteAppValidationConfigurationError::OperationNotPermitted(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteAppValidationConfigurationError::UnauthorizedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteAppValidationConfigurationError {}
/// Errors returned by DeleteReplicationJob
#[derive(Debug, PartialEq)]
pub enum DeleteReplicationJobError {
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>The specified replication job does not exist.</p>
    ReplicationJobNotFound(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl DeleteReplicationJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteReplicationJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteReplicationJobError::InvalidParameter(
                        err.msg,
                    ))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        DeleteReplicationJobError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(DeleteReplicationJobError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "ReplicationJobNotFoundException" => {
                    return RusotoError::Service(DeleteReplicationJobError::ReplicationJobNotFound(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(DeleteReplicationJobError::UnauthorizedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteReplicationJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteReplicationJobError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteReplicationJobError::MissingRequiredParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteReplicationJobError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            DeleteReplicationJobError::ReplicationJobNotFound(ref cause) => write!(f, "{}", cause),
            DeleteReplicationJobError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteReplicationJobError {}
/// Errors returned by DeleteServerCatalog
#[derive(Debug, PartialEq)]
pub enum DeleteServerCatalogError {
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl DeleteServerCatalogError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteServerCatalogError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteServerCatalogError::InvalidParameter(
                        err.msg,
                    ))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        DeleteServerCatalogError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(DeleteServerCatalogError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(DeleteServerCatalogError::UnauthorizedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteServerCatalogError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteServerCatalogError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteServerCatalogError::MissingRequiredParameter(ref cause) => write!(f, "{}", cause),
            DeleteServerCatalogError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            DeleteServerCatalogError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteServerCatalogError {}
/// Errors returned by DisassociateConnector
#[derive(Debug, PartialEq)]
pub enum DisassociateConnectorError {
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl DisassociateConnectorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateConnectorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DisassociateConnectorError::InvalidParameter(
                        err.msg,
                    ))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        DisassociateConnectorError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(DisassociateConnectorError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(DisassociateConnectorError::UnauthorizedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateConnectorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateConnectorError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DisassociateConnectorError::MissingRequiredParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateConnectorError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            DisassociateConnectorError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateConnectorError {}
/// Errors returned by GenerateChangeSet
#[derive(Debug, PartialEq)]
pub enum GenerateChangeSetError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl GenerateChangeSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GenerateChangeSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(GenerateChangeSetError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GenerateChangeSetError::InvalidParameter(err.msg))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(GenerateChangeSetError::MissingRequiredParameter(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(GenerateChangeSetError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(GenerateChangeSetError::UnauthorizedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GenerateChangeSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GenerateChangeSetError::InternalError(ref cause) => write!(f, "{}", cause),
            GenerateChangeSetError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GenerateChangeSetError::MissingRequiredParameter(ref cause) => write!(f, "{}", cause),
            GenerateChangeSetError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            GenerateChangeSetError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GenerateChangeSetError {}
/// Errors returned by GenerateTemplate
#[derive(Debug, PartialEq)]
pub enum GenerateTemplateError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl GenerateTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GenerateTemplateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(GenerateTemplateError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GenerateTemplateError::InvalidParameter(err.msg))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(GenerateTemplateError::MissingRequiredParameter(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(GenerateTemplateError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(GenerateTemplateError::UnauthorizedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GenerateTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GenerateTemplateError::InternalError(ref cause) => write!(f, "{}", cause),
            GenerateTemplateError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GenerateTemplateError::MissingRequiredParameter(ref cause) => write!(f, "{}", cause),
            GenerateTemplateError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            GenerateTemplateError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GenerateTemplateError {}
/// Errors returned by GetApp
#[derive(Debug, PartialEq)]
pub enum GetAppError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl GetAppError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAppError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(GetAppError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetAppError::InvalidParameter(err.msg))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(GetAppError::MissingRequiredParameter(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(GetAppError::OperationNotPermitted(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(GetAppError::UnauthorizedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAppError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAppError::InternalError(ref cause) => write!(f, "{}", cause),
            GetAppError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetAppError::MissingRequiredParameter(ref cause) => write!(f, "{}", cause),
            GetAppError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            GetAppError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAppError {}
/// Errors returned by GetAppLaunchConfiguration
#[derive(Debug, PartialEq)]
pub enum GetAppLaunchConfigurationError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl GetAppLaunchConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAppLaunchConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(GetAppLaunchConfigurationError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetAppLaunchConfigurationError::InvalidParameter(
                        err.msg,
                    ))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        GetAppLaunchConfigurationError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        GetAppLaunchConfigurationError::OperationNotPermitted(err.msg),
                    )
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(
                        GetAppLaunchConfigurationError::UnauthorizedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAppLaunchConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAppLaunchConfigurationError::InternalError(ref cause) => write!(f, "{}", cause),
            GetAppLaunchConfigurationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetAppLaunchConfigurationError::MissingRequiredParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAppLaunchConfigurationError::OperationNotPermitted(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAppLaunchConfigurationError::UnauthorizedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetAppLaunchConfigurationError {}
/// Errors returned by GetAppReplicationConfiguration
#[derive(Debug, PartialEq)]
pub enum GetAppReplicationConfigurationError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl GetAppReplicationConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetAppReplicationConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(
                        GetAppReplicationConfigurationError::InternalError(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        GetAppReplicationConfigurationError::InvalidParameter(err.msg),
                    )
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        GetAppReplicationConfigurationError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        GetAppReplicationConfigurationError::OperationNotPermitted(err.msg),
                    )
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(
                        GetAppReplicationConfigurationError::UnauthorizedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAppReplicationConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAppReplicationConfigurationError::InternalError(ref cause) => write!(f, "{}", cause),
            GetAppReplicationConfigurationError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAppReplicationConfigurationError::MissingRequiredParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAppReplicationConfigurationError::OperationNotPermitted(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAppReplicationConfigurationError::UnauthorizedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetAppReplicationConfigurationError {}
/// Errors returned by GetAppValidationConfiguration
#[derive(Debug, PartialEq)]
pub enum GetAppValidationConfigurationError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl GetAppValidationConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetAppValidationConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(GetAppValidationConfigurationError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        GetAppValidationConfigurationError::InvalidParameter(err.msg),
                    )
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        GetAppValidationConfigurationError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        GetAppValidationConfigurationError::OperationNotPermitted(err.msg),
                    )
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(
                        GetAppValidationConfigurationError::UnauthorizedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAppValidationConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAppValidationConfigurationError::InternalError(ref cause) => write!(f, "{}", cause),
            GetAppValidationConfigurationError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAppValidationConfigurationError::MissingRequiredParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAppValidationConfigurationError::OperationNotPermitted(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAppValidationConfigurationError::UnauthorizedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetAppValidationConfigurationError {}
/// Errors returned by GetAppValidationOutput
#[derive(Debug, PartialEq)]
pub enum GetAppValidationOutputError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl GetAppValidationOutputError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAppValidationOutputError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(GetAppValidationOutputError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetAppValidationOutputError::InvalidParameter(
                        err.msg,
                    ))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        GetAppValidationOutputError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        GetAppValidationOutputError::OperationNotPermitted(err.msg),
                    )
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(
                        GetAppValidationOutputError::UnauthorizedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAppValidationOutputError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAppValidationOutputError::InternalError(ref cause) => write!(f, "{}", cause),
            GetAppValidationOutputError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetAppValidationOutputError::MissingRequiredParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            GetAppValidationOutputError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            GetAppValidationOutputError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAppValidationOutputError {}
/// Errors returned by GetConnectors
#[derive(Debug, PartialEq)]
pub enum GetConnectorsError {
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl GetConnectorsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetConnectorsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(GetConnectorsError::UnauthorizedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetConnectorsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetConnectorsError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetConnectorsError {}
/// Errors returned by GetReplicationJobs
#[derive(Debug, PartialEq)]
pub enum GetReplicationJobsError {
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl GetReplicationJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetReplicationJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(GetReplicationJobsError::InvalidParameter(err.msg))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(GetReplicationJobsError::MissingRequiredParameter(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(GetReplicationJobsError::UnauthorizedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetReplicationJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetReplicationJobsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetReplicationJobsError::MissingRequiredParameter(ref cause) => write!(f, "{}", cause),
            GetReplicationJobsError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetReplicationJobsError {}
/// Errors returned by GetReplicationRuns
#[derive(Debug, PartialEq)]
pub enum GetReplicationRunsError {
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl GetReplicationRunsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetReplicationRunsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(GetReplicationRunsError::InvalidParameter(err.msg))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(GetReplicationRunsError::MissingRequiredParameter(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(GetReplicationRunsError::UnauthorizedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetReplicationRunsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetReplicationRunsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetReplicationRunsError::MissingRequiredParameter(ref cause) => write!(f, "{}", cause),
            GetReplicationRunsError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetReplicationRunsError {}
/// Errors returned by GetServers
#[derive(Debug, PartialEq)]
pub enum GetServersError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl GetServersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetServersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(GetServersError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetServersError::InvalidParameter(err.msg))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(GetServersError::MissingRequiredParameter(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(GetServersError::UnauthorizedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetServersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetServersError::InternalError(ref cause) => write!(f, "{}", cause),
            GetServersError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetServersError::MissingRequiredParameter(ref cause) => write!(f, "{}", cause),
            GetServersError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetServersError {}
/// Errors returned by ImportAppCatalog
#[derive(Debug, PartialEq)]
pub enum ImportAppCatalogError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl ImportAppCatalogError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ImportAppCatalogError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(ImportAppCatalogError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ImportAppCatalogError::InvalidParameter(err.msg))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(ImportAppCatalogError::MissingRequiredParameter(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(ImportAppCatalogError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(ImportAppCatalogError::UnauthorizedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ImportAppCatalogError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ImportAppCatalogError::InternalError(ref cause) => write!(f, "{}", cause),
            ImportAppCatalogError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ImportAppCatalogError::MissingRequiredParameter(ref cause) => write!(f, "{}", cause),
            ImportAppCatalogError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            ImportAppCatalogError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ImportAppCatalogError {}
/// Errors returned by ImportServerCatalog
#[derive(Debug, PartialEq)]
pub enum ImportServerCatalogError {
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>There are no connectors available.</p>
    NoConnectorsAvailable(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl ImportServerCatalogError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ImportServerCatalogError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(ImportServerCatalogError::InvalidParameter(
                        err.msg,
                    ))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        ImportServerCatalogError::MissingRequiredParameter(err.msg),
                    )
                }
                "NoConnectorsAvailableException" => {
                    return RusotoError::Service(ImportServerCatalogError::NoConnectorsAvailable(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(ImportServerCatalogError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(ImportServerCatalogError::UnauthorizedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ImportServerCatalogError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ImportServerCatalogError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ImportServerCatalogError::MissingRequiredParameter(ref cause) => write!(f, "{}", cause),
            ImportServerCatalogError::NoConnectorsAvailable(ref cause) => write!(f, "{}", cause),
            ImportServerCatalogError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            ImportServerCatalogError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ImportServerCatalogError {}
/// Errors returned by LaunchApp
#[derive(Debug, PartialEq)]
pub enum LaunchAppError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl LaunchAppError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<LaunchAppError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(LaunchAppError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(LaunchAppError::InvalidParameter(err.msg))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(LaunchAppError::MissingRequiredParameter(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(LaunchAppError::OperationNotPermitted(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(LaunchAppError::UnauthorizedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for LaunchAppError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LaunchAppError::InternalError(ref cause) => write!(f, "{}", cause),
            LaunchAppError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            LaunchAppError::MissingRequiredParameter(ref cause) => write!(f, "{}", cause),
            LaunchAppError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            LaunchAppError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for LaunchAppError {}
/// Errors returned by ListApps
#[derive(Debug, PartialEq)]
pub enum ListAppsError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl ListAppsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAppsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(ListAppsError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListAppsError::InvalidParameter(err.msg))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(ListAppsError::MissingRequiredParameter(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(ListAppsError::OperationNotPermitted(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(ListAppsError::UnauthorizedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAppsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAppsError::InternalError(ref cause) => write!(f, "{}", cause),
            ListAppsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListAppsError::MissingRequiredParameter(ref cause) => write!(f, "{}", cause),
            ListAppsError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            ListAppsError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAppsError {}
/// Errors returned by NotifyAppValidationOutput
#[derive(Debug, PartialEq)]
pub enum NotifyAppValidationOutputError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl NotifyAppValidationOutputError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<NotifyAppValidationOutputError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(NotifyAppValidationOutputError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(NotifyAppValidationOutputError::InvalidParameter(
                        err.msg,
                    ))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        NotifyAppValidationOutputError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        NotifyAppValidationOutputError::OperationNotPermitted(err.msg),
                    )
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(
                        NotifyAppValidationOutputError::UnauthorizedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for NotifyAppValidationOutputError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NotifyAppValidationOutputError::InternalError(ref cause) => write!(f, "{}", cause),
            NotifyAppValidationOutputError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            NotifyAppValidationOutputError::MissingRequiredParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            NotifyAppValidationOutputError::OperationNotPermitted(ref cause) => {
                write!(f, "{}", cause)
            }
            NotifyAppValidationOutputError::UnauthorizedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for NotifyAppValidationOutputError {}
/// Errors returned by PutAppLaunchConfiguration
#[derive(Debug, PartialEq)]
pub enum PutAppLaunchConfigurationError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl PutAppLaunchConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutAppLaunchConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(PutAppLaunchConfigurationError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(PutAppLaunchConfigurationError::InvalidParameter(
                        err.msg,
                    ))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        PutAppLaunchConfigurationError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        PutAppLaunchConfigurationError::OperationNotPermitted(err.msg),
                    )
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(
                        PutAppLaunchConfigurationError::UnauthorizedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutAppLaunchConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutAppLaunchConfigurationError::InternalError(ref cause) => write!(f, "{}", cause),
            PutAppLaunchConfigurationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            PutAppLaunchConfigurationError::MissingRequiredParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            PutAppLaunchConfigurationError::OperationNotPermitted(ref cause) => {
                write!(f, "{}", cause)
            }
            PutAppLaunchConfigurationError::UnauthorizedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutAppLaunchConfigurationError {}
/// Errors returned by PutAppReplicationConfiguration
#[derive(Debug, PartialEq)]
pub enum PutAppReplicationConfigurationError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl PutAppReplicationConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutAppReplicationConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(
                        PutAppReplicationConfigurationError::InternalError(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        PutAppReplicationConfigurationError::InvalidParameter(err.msg),
                    )
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        PutAppReplicationConfigurationError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        PutAppReplicationConfigurationError::OperationNotPermitted(err.msg),
                    )
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(
                        PutAppReplicationConfigurationError::UnauthorizedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutAppReplicationConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutAppReplicationConfigurationError::InternalError(ref cause) => write!(f, "{}", cause),
            PutAppReplicationConfigurationError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            PutAppReplicationConfigurationError::MissingRequiredParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            PutAppReplicationConfigurationError::OperationNotPermitted(ref cause) => {
                write!(f, "{}", cause)
            }
            PutAppReplicationConfigurationError::UnauthorizedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutAppReplicationConfigurationError {}
/// Errors returned by PutAppValidationConfiguration
#[derive(Debug, PartialEq)]
pub enum PutAppValidationConfigurationError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl PutAppValidationConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutAppValidationConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(PutAppValidationConfigurationError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        PutAppValidationConfigurationError::InvalidParameter(err.msg),
                    )
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        PutAppValidationConfigurationError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        PutAppValidationConfigurationError::OperationNotPermitted(err.msg),
                    )
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(
                        PutAppValidationConfigurationError::UnauthorizedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutAppValidationConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutAppValidationConfigurationError::InternalError(ref cause) => write!(f, "{}", cause),
            PutAppValidationConfigurationError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            PutAppValidationConfigurationError::MissingRequiredParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            PutAppValidationConfigurationError::OperationNotPermitted(ref cause) => {
                write!(f, "{}", cause)
            }
            PutAppValidationConfigurationError::UnauthorizedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutAppValidationConfigurationError {}
/// Errors returned by StartAppReplication
#[derive(Debug, PartialEq)]
pub enum StartAppReplicationError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl StartAppReplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartAppReplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(StartAppReplicationError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartAppReplicationError::InvalidParameter(
                        err.msg,
                    ))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        StartAppReplicationError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(StartAppReplicationError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(StartAppReplicationError::UnauthorizedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartAppReplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartAppReplicationError::InternalError(ref cause) => write!(f, "{}", cause),
            StartAppReplicationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartAppReplicationError::MissingRequiredParameter(ref cause) => write!(f, "{}", cause),
            StartAppReplicationError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            StartAppReplicationError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartAppReplicationError {}
/// Errors returned by StartOnDemandAppReplication
#[derive(Debug, PartialEq)]
pub enum StartOnDemandAppReplicationError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl StartOnDemandAppReplicationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StartOnDemandAppReplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(StartOnDemandAppReplicationError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        StartOnDemandAppReplicationError::InvalidParameter(err.msg),
                    )
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        StartOnDemandAppReplicationError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        StartOnDemandAppReplicationError::OperationNotPermitted(err.msg),
                    )
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(
                        StartOnDemandAppReplicationError::UnauthorizedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartOnDemandAppReplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartOnDemandAppReplicationError::InternalError(ref cause) => write!(f, "{}", cause),
            StartOnDemandAppReplicationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartOnDemandAppReplicationError::MissingRequiredParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            StartOnDemandAppReplicationError::OperationNotPermitted(ref cause) => {
                write!(f, "{}", cause)
            }
            StartOnDemandAppReplicationError::UnauthorizedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for StartOnDemandAppReplicationError {}
/// Errors returned by StartOnDemandReplicationRun
#[derive(Debug, PartialEq)]
pub enum StartOnDemandReplicationRunError {
    /// <p>The user has the required permissions, so the request would have succeeded, but a dry run was performed.</p>
    DryRunOperation(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You have exceeded the number of on-demand replication runs you can request in a 24-hour period.</p>
    ReplicationRunLimitExceeded(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl StartOnDemandReplicationRunError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StartOnDemandReplicationRunError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DryRunOperationException" => {
                    return RusotoError::Service(StartOnDemandReplicationRunError::DryRunOperation(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        StartOnDemandReplicationRunError::InvalidParameter(err.msg),
                    )
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        StartOnDemandReplicationRunError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        StartOnDemandReplicationRunError::OperationNotPermitted(err.msg),
                    )
                }
                "ReplicationRunLimitExceededException" => {
                    return RusotoError::Service(
                        StartOnDemandReplicationRunError::ReplicationRunLimitExceeded(err.msg),
                    )
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(
                        StartOnDemandReplicationRunError::UnauthorizedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartOnDemandReplicationRunError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartOnDemandReplicationRunError::DryRunOperation(ref cause) => write!(f, "{}", cause),
            StartOnDemandReplicationRunError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartOnDemandReplicationRunError::MissingRequiredParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            StartOnDemandReplicationRunError::OperationNotPermitted(ref cause) => {
                write!(f, "{}", cause)
            }
            StartOnDemandReplicationRunError::ReplicationRunLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            StartOnDemandReplicationRunError::UnauthorizedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for StartOnDemandReplicationRunError {}
/// Errors returned by StopAppReplication
#[derive(Debug, PartialEq)]
pub enum StopAppReplicationError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl StopAppReplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopAppReplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(StopAppReplicationError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StopAppReplicationError::InvalidParameter(err.msg))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(StopAppReplicationError::MissingRequiredParameter(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(StopAppReplicationError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(StopAppReplicationError::UnauthorizedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopAppReplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopAppReplicationError::InternalError(ref cause) => write!(f, "{}", cause),
            StopAppReplicationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StopAppReplicationError::MissingRequiredParameter(ref cause) => write!(f, "{}", cause),
            StopAppReplicationError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            StopAppReplicationError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopAppReplicationError {}
/// Errors returned by TerminateApp
#[derive(Debug, PartialEq)]
pub enum TerminateAppError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl TerminateAppError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TerminateAppError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(TerminateAppError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(TerminateAppError::InvalidParameter(err.msg))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(TerminateAppError::MissingRequiredParameter(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(TerminateAppError::OperationNotPermitted(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(TerminateAppError::UnauthorizedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TerminateAppError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TerminateAppError::InternalError(ref cause) => write!(f, "{}", cause),
            TerminateAppError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            TerminateAppError::MissingRequiredParameter(ref cause) => write!(f, "{}", cause),
            TerminateAppError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            TerminateAppError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TerminateAppError {}
/// Errors returned by UpdateApp
#[derive(Debug, PartialEq)]
pub enum UpdateAppError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl UpdateAppError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAppError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(UpdateAppError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateAppError::InvalidParameter(err.msg))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(UpdateAppError::MissingRequiredParameter(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(UpdateAppError::OperationNotPermitted(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(UpdateAppError::UnauthorizedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateAppError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAppError::InternalError(ref cause) => write!(f, "{}", cause),
            UpdateAppError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateAppError::MissingRequiredParameter(ref cause) => write!(f, "{}", cause),
            UpdateAppError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            UpdateAppError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateAppError {}
/// Errors returned by UpdateReplicationJob
#[derive(Debug, PartialEq)]
pub enum UpdateReplicationJobError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>The specified replication job does not exist.</p>
    ReplicationJobNotFound(String),
    /// <p>The specified server cannot be replicated.</p>
    ServerCannotBeReplicated(String),
    /// <p>The service is temporarily unavailable.</p>
    TemporarilyUnavailable(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl UpdateReplicationJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateReplicationJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(UpdateReplicationJobError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateReplicationJobError::InvalidParameter(
                        err.msg,
                    ))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        UpdateReplicationJobError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(UpdateReplicationJobError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "ReplicationJobNotFoundException" => {
                    return RusotoError::Service(UpdateReplicationJobError::ReplicationJobNotFound(
                        err.msg,
                    ))
                }
                "ServerCannotBeReplicatedException" => {
                    return RusotoError::Service(
                        UpdateReplicationJobError::ServerCannotBeReplicated(err.msg),
                    )
                }
                "TemporarilyUnavailableException" => {
                    return RusotoError::Service(UpdateReplicationJobError::TemporarilyUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(UpdateReplicationJobError::UnauthorizedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateReplicationJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateReplicationJobError::InternalError(ref cause) => write!(f, "{}", cause),
            UpdateReplicationJobError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateReplicationJobError::MissingRequiredParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateReplicationJobError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            UpdateReplicationJobError::ReplicationJobNotFound(ref cause) => write!(f, "{}", cause),
            UpdateReplicationJobError::ServerCannotBeReplicated(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateReplicationJobError::TemporarilyUnavailable(ref cause) => write!(f, "{}", cause),
            UpdateReplicationJobError::UnauthorizedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateReplicationJobError {}
/// Trait representing the capabilities of the SMS API. SMS clients implement this trait.
#[async_trait]
pub trait ServerMigrationService {
    /// <p>Creates an application. An application consists of one or more server groups. Each server group contain one or more servers.</p>
    async fn create_app(
        &self,
        input: CreateAppRequest,
    ) -> Result<CreateAppResponse, RusotoError<CreateAppError>>;

    /// <p>Creates a replication job. The replication job schedules periodic replication runs to replicate your server to AWS. Each replication run creates an Amazon Machine Image (AMI).</p>
    async fn create_replication_job(
        &self,
        input: CreateReplicationJobRequest,
    ) -> Result<CreateReplicationJobResponse, RusotoError<CreateReplicationJobError>>;

    /// <p>Deletes the specified application. Optionally deletes the launched stack associated with the application and all AWS SMS replication jobs for servers in the application.</p>
    async fn delete_app(
        &self,
        input: DeleteAppRequest,
    ) -> Result<DeleteAppResponse, RusotoError<DeleteAppError>>;

    /// <p>Deletes the launch configuration for the specified application.</p>
    async fn delete_app_launch_configuration(
        &self,
        input: DeleteAppLaunchConfigurationRequest,
    ) -> Result<DeleteAppLaunchConfigurationResponse, RusotoError<DeleteAppLaunchConfigurationError>>;

    /// <p>Deletes the replication configuration for the specified application.</p>
    async fn delete_app_replication_configuration(
        &self,
        input: DeleteAppReplicationConfigurationRequest,
    ) -> Result<
        DeleteAppReplicationConfigurationResponse,
        RusotoError<DeleteAppReplicationConfigurationError>,
    >;

    /// <p>Deletes the validation configuration for the specified application.</p>
    async fn delete_app_validation_configuration(
        &self,
        input: DeleteAppValidationConfigurationRequest,
    ) -> Result<
        DeleteAppValidationConfigurationResponse,
        RusotoError<DeleteAppValidationConfigurationError>,
    >;

    /// <p>Deletes the specified replication job.</p> <p>After you delete a replication job, there are no further replication runs. AWS deletes the contents of the Amazon S3 bucket used to store AWS SMS artifacts. The AMIs created by the replication runs are not deleted.</p>
    async fn delete_replication_job(
        &self,
        input: DeleteReplicationJobRequest,
    ) -> Result<DeleteReplicationJobResponse, RusotoError<DeleteReplicationJobError>>;

    /// <p>Deletes all servers from your server catalog.</p>
    async fn delete_server_catalog(
        &self,
    ) -> Result<DeleteServerCatalogResponse, RusotoError<DeleteServerCatalogError>>;

    /// <p>Disassociates the specified connector from AWS SMS.</p> <p>After you disassociate a connector, it is no longer available to support replication jobs.</p>
    async fn disassociate_connector(
        &self,
        input: DisassociateConnectorRequest,
    ) -> Result<DisassociateConnectorResponse, RusotoError<DisassociateConnectorError>>;

    /// <p>Generates a target change set for a currently launched stack and writes it to an Amazon S3 object in the customer’s Amazon S3 bucket.</p>
    async fn generate_change_set(
        &self,
        input: GenerateChangeSetRequest,
    ) -> Result<GenerateChangeSetResponse, RusotoError<GenerateChangeSetError>>;

    /// <p>Generates an AWS CloudFormation template based on the current launch configuration and writes it to an Amazon S3 object in the customer’s Amazon S3 bucket.</p>
    async fn generate_template(
        &self,
        input: GenerateTemplateRequest,
    ) -> Result<GenerateTemplateResponse, RusotoError<GenerateTemplateError>>;

    /// <p>Retrieve information about the specified application.</p>
    async fn get_app(
        &self,
        input: GetAppRequest,
    ) -> Result<GetAppResponse, RusotoError<GetAppError>>;

    /// <p>Retrieves the application launch configuration associated with the specified application.</p>
    async fn get_app_launch_configuration(
        &self,
        input: GetAppLaunchConfigurationRequest,
    ) -> Result<GetAppLaunchConfigurationResponse, RusotoError<GetAppLaunchConfigurationError>>;

    /// <p>Retrieves the application replication configuration associated with the specified application.</p>
    async fn get_app_replication_configuration(
        &self,
        input: GetAppReplicationConfigurationRequest,
    ) -> Result<
        GetAppReplicationConfigurationResponse,
        RusotoError<GetAppReplicationConfigurationError>,
    >;

    /// <p>Retrieves information about a configuration for validating an application.</p>
    async fn get_app_validation_configuration(
        &self,
        input: GetAppValidationConfigurationRequest,
    ) -> Result<
        GetAppValidationConfigurationResponse,
        RusotoError<GetAppValidationConfigurationError>,
    >;

    /// <p>Retrieves output from validating an application.</p>
    async fn get_app_validation_output(
        &self,
        input: GetAppValidationOutputRequest,
    ) -> Result<GetAppValidationOutputResponse, RusotoError<GetAppValidationOutputError>>;

    /// <p>Describes the connectors registered with the AWS SMS.</p>
    async fn get_connectors(
        &self,
        input: GetConnectorsRequest,
    ) -> Result<GetConnectorsResponse, RusotoError<GetConnectorsError>>;

    /// <p>Describes the specified replication job or all of your replication jobs.</p>
    async fn get_replication_jobs(
        &self,
        input: GetReplicationJobsRequest,
    ) -> Result<GetReplicationJobsResponse, RusotoError<GetReplicationJobsError>>;

    /// <p>Describes the replication runs for the specified replication job.</p>
    async fn get_replication_runs(
        &self,
        input: GetReplicationRunsRequest,
    ) -> Result<GetReplicationRunsResponse, RusotoError<GetReplicationRunsError>>;

    /// <p>Describes the servers in your server catalog.</p> <p>Before you can describe your servers, you must import them using <a>ImportServerCatalog</a>.</p>
    async fn get_servers(
        &self,
        input: GetServersRequest,
    ) -> Result<GetServersResponse, RusotoError<GetServersError>>;

    /// <p>Allows application import from AWS Migration Hub.</p>
    async fn import_app_catalog(
        &self,
        input: ImportAppCatalogRequest,
    ) -> Result<ImportAppCatalogResponse, RusotoError<ImportAppCatalogError>>;

    /// <p>Gathers a complete list of on-premises servers. Connectors must be installed and monitoring all servers to import.</p> <p>This call returns immediately, but might take additional time to retrieve all the servers.</p>
    async fn import_server_catalog(
        &self,
    ) -> Result<ImportServerCatalogResponse, RusotoError<ImportServerCatalogError>>;

    /// <p>Launches the specified application as a stack in AWS CloudFormation.</p>
    async fn launch_app(
        &self,
        input: LaunchAppRequest,
    ) -> Result<LaunchAppResponse, RusotoError<LaunchAppError>>;

    /// <p>Retrieves summaries for all applications.</p>
    async fn list_apps(
        &self,
        input: ListAppsRequest,
    ) -> Result<ListAppsResponse, RusotoError<ListAppsError>>;

    /// <p>Provides information to AWS SMS about whether application validation is successful.</p>
    async fn notify_app_validation_output(
        &self,
        input: NotifyAppValidationOutputRequest,
    ) -> Result<NotifyAppValidationOutputResponse, RusotoError<NotifyAppValidationOutputError>>;

    /// <p>Creates or updates the launch configuration for the specified application.</p>
    async fn put_app_launch_configuration(
        &self,
        input: PutAppLaunchConfigurationRequest,
    ) -> Result<PutAppLaunchConfigurationResponse, RusotoError<PutAppLaunchConfigurationError>>;

    /// <p>Creates or updates the replication configuration for the specified application.</p>
    async fn put_app_replication_configuration(
        &self,
        input: PutAppReplicationConfigurationRequest,
    ) -> Result<
        PutAppReplicationConfigurationResponse,
        RusotoError<PutAppReplicationConfigurationError>,
    >;

    /// <p>Creates or updates a validation configuration for the specified application.</p>
    async fn put_app_validation_configuration(
        &self,
        input: PutAppValidationConfigurationRequest,
    ) -> Result<
        PutAppValidationConfigurationResponse,
        RusotoError<PutAppValidationConfigurationError>,
    >;

    /// <p>Starts replicating the specified application by creating replication jobs for each server in the application.</p>
    async fn start_app_replication(
        &self,
        input: StartAppReplicationRequest,
    ) -> Result<StartAppReplicationResponse, RusotoError<StartAppReplicationError>>;

    /// <p>Starts an on-demand replication run for the specified application.</p>
    async fn start_on_demand_app_replication(
        &self,
        input: StartOnDemandAppReplicationRequest,
    ) -> Result<StartOnDemandAppReplicationResponse, RusotoError<StartOnDemandAppReplicationError>>;

    /// <p>Starts an on-demand replication run for the specified replication job. This replication run starts immediately. This replication run is in addition to the ones already scheduled.</p> <p>There is a limit on the number of on-demand replications runs that you can request in a 24-hour period.</p>
    async fn start_on_demand_replication_run(
        &self,
        input: StartOnDemandReplicationRunRequest,
    ) -> Result<StartOnDemandReplicationRunResponse, RusotoError<StartOnDemandReplicationRunError>>;

    /// <p>Stops replicating the specified application by deleting the replication job for each server in the application.</p>
    async fn stop_app_replication(
        &self,
        input: StopAppReplicationRequest,
    ) -> Result<StopAppReplicationResponse, RusotoError<StopAppReplicationError>>;

    /// <p>Terminates the stack for the specified application.</p>
    async fn terminate_app(
        &self,
        input: TerminateAppRequest,
    ) -> Result<TerminateAppResponse, RusotoError<TerminateAppError>>;

    /// <p>Updates the specified application.</p>
    async fn update_app(
        &self,
        input: UpdateAppRequest,
    ) -> Result<UpdateAppResponse, RusotoError<UpdateAppError>>;

    /// <p>Updates the specified settings for the specified replication job.</p>
    async fn update_replication_job(
        &self,
        input: UpdateReplicationJobRequest,
    ) -> Result<UpdateReplicationJobResponse, RusotoError<UpdateReplicationJobError>>;
}
/// A client for the SMS API.
#[derive(Clone)]
pub struct ServerMigrationServiceClient {
    client: Client,
    region: region::Region,
}

impl ServerMigrationServiceClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ServerMigrationServiceClient {
        ServerMigrationServiceClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ServerMigrationServiceClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ServerMigrationServiceClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ServerMigrationServiceClient {
        ServerMigrationServiceClient { client, region }
    }
}

#[async_trait]
impl ServerMigrationService for ServerMigrationServiceClient {
    /// <p>Creates an application. An application consists of one or more server groups. Each server group contain one or more servers.</p>
    async fn create_app(
        &self,
        input: CreateAppRequest,
    ) -> Result<CreateAppResponse, RusotoError<CreateAppError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.CreateApp",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateAppError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateAppResponse, _>()
    }

    /// <p>Creates a replication job. The replication job schedules periodic replication runs to replicate your server to AWS. Each replication run creates an Amazon Machine Image (AMI).</p>
    async fn create_replication_job(
        &self,
        input: CreateReplicationJobRequest,
    ) -> Result<CreateReplicationJobResponse, RusotoError<CreateReplicationJobError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.CreateReplicationJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateReplicationJobError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateReplicationJobResponse, _>()
    }

    /// <p>Deletes the specified application. Optionally deletes the launched stack associated with the application and all AWS SMS replication jobs for servers in the application.</p>
    async fn delete_app(
        &self,
        input: DeleteAppRequest,
    ) -> Result<DeleteAppResponse, RusotoError<DeleteAppError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.DeleteApp",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteAppError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteAppResponse, _>()
    }

    /// <p>Deletes the launch configuration for the specified application.</p>
    async fn delete_app_launch_configuration(
        &self,
        input: DeleteAppLaunchConfigurationRequest,
    ) -> Result<DeleteAppLaunchConfigurationResponse, RusotoError<DeleteAppLaunchConfigurationError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.DeleteAppLaunchConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteAppLaunchConfigurationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteAppLaunchConfigurationResponse, _>()
    }

    /// <p>Deletes the replication configuration for the specified application.</p>
    async fn delete_app_replication_configuration(
        &self,
        input: DeleteAppReplicationConfigurationRequest,
    ) -> Result<
        DeleteAppReplicationConfigurationResponse,
        RusotoError<DeleteAppReplicationConfigurationError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.DeleteAppReplicationConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                DeleteAppReplicationConfigurationError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteAppReplicationConfigurationResponse, _>()
    }

    /// <p>Deletes the validation configuration for the specified application.</p>
    async fn delete_app_validation_configuration(
        &self,
        input: DeleteAppValidationConfigurationRequest,
    ) -> Result<
        DeleteAppValidationConfigurationResponse,
        RusotoError<DeleteAppValidationConfigurationError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.DeleteAppValidationConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                DeleteAppValidationConfigurationError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteAppValidationConfigurationResponse, _>()
    }

    /// <p>Deletes the specified replication job.</p> <p>After you delete a replication job, there are no further replication runs. AWS deletes the contents of the Amazon S3 bucket used to store AWS SMS artifacts. The AMIs created by the replication runs are not deleted.</p>
    async fn delete_replication_job(
        &self,
        input: DeleteReplicationJobRequest,
    ) -> Result<DeleteReplicationJobResponse, RusotoError<DeleteReplicationJobError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.DeleteReplicationJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteReplicationJobError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteReplicationJobResponse, _>()
    }

    /// <p>Deletes all servers from your server catalog.</p>
    async fn delete_server_catalog(
        &self,
    ) -> Result<DeleteServerCatalogResponse, RusotoError<DeleteServerCatalogError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.DeleteServerCatalog",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, DeleteServerCatalogError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteServerCatalogResponse, _>()
    }

    /// <p>Disassociates the specified connector from AWS SMS.</p> <p>After you disassociate a connector, it is no longer available to support replication jobs.</p>
    async fn disassociate_connector(
        &self,
        input: DisassociateConnectorRequest,
    ) -> Result<DisassociateConnectorResponse, RusotoError<DisassociateConnectorError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.DisassociateConnector",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DisassociateConnectorError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DisassociateConnectorResponse, _>()
    }

    /// <p>Generates a target change set for a currently launched stack and writes it to an Amazon S3 object in the customer’s Amazon S3 bucket.</p>
    async fn generate_change_set(
        &self,
        input: GenerateChangeSetRequest,
    ) -> Result<GenerateChangeSetResponse, RusotoError<GenerateChangeSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GenerateChangeSet",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GenerateChangeSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GenerateChangeSetResponse, _>()
    }

    /// <p>Generates an AWS CloudFormation template based on the current launch configuration and writes it to an Amazon S3 object in the customer’s Amazon S3 bucket.</p>
    async fn generate_template(
        &self,
        input: GenerateTemplateRequest,
    ) -> Result<GenerateTemplateResponse, RusotoError<GenerateTemplateError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GenerateTemplate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GenerateTemplateError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GenerateTemplateResponse, _>()
    }

    /// <p>Retrieve information about the specified application.</p>
    async fn get_app(
        &self,
        input: GetAppRequest,
    ) -> Result<GetAppResponse, RusotoError<GetAppError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GetApp",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetAppError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetAppResponse, _>()
    }

    /// <p>Retrieves the application launch configuration associated with the specified application.</p>
    async fn get_app_launch_configuration(
        &self,
        input: GetAppLaunchConfigurationRequest,
    ) -> Result<GetAppLaunchConfigurationResponse, RusotoError<GetAppLaunchConfigurationError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GetAppLaunchConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetAppLaunchConfigurationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetAppLaunchConfigurationResponse, _>()
    }

    /// <p>Retrieves the application replication configuration associated with the specified application.</p>
    async fn get_app_replication_configuration(
        &self,
        input: GetAppReplicationConfigurationRequest,
    ) -> Result<
        GetAppReplicationConfigurationResponse,
        RusotoError<GetAppReplicationConfigurationError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GetAppReplicationConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetAppReplicationConfigurationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetAppReplicationConfigurationResponse, _>()
    }

    /// <p>Retrieves information about a configuration for validating an application.</p>
    async fn get_app_validation_configuration(
        &self,
        input: GetAppValidationConfigurationRequest,
    ) -> Result<
        GetAppValidationConfigurationResponse,
        RusotoError<GetAppValidationConfigurationError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GetAppValidationConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetAppValidationConfigurationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetAppValidationConfigurationResponse, _>()
    }

    /// <p>Retrieves output from validating an application.</p>
    async fn get_app_validation_output(
        &self,
        input: GetAppValidationOutputRequest,
    ) -> Result<GetAppValidationOutputResponse, RusotoError<GetAppValidationOutputError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GetAppValidationOutput",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetAppValidationOutputError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetAppValidationOutputResponse, _>()
    }

    /// <p>Describes the connectors registered with the AWS SMS.</p>
    async fn get_connectors(
        &self,
        input: GetConnectorsRequest,
    ) -> Result<GetConnectorsResponse, RusotoError<GetConnectorsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GetConnectors",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetConnectorsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetConnectorsResponse, _>()
    }

    /// <p>Describes the specified replication job or all of your replication jobs.</p>
    async fn get_replication_jobs(
        &self,
        input: GetReplicationJobsRequest,
    ) -> Result<GetReplicationJobsResponse, RusotoError<GetReplicationJobsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GetReplicationJobs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetReplicationJobsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetReplicationJobsResponse, _>()
    }

    /// <p>Describes the replication runs for the specified replication job.</p>
    async fn get_replication_runs(
        &self,
        input: GetReplicationRunsRequest,
    ) -> Result<GetReplicationRunsResponse, RusotoError<GetReplicationRunsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GetReplicationRuns",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetReplicationRunsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetReplicationRunsResponse, _>()
    }

    /// <p>Describes the servers in your server catalog.</p> <p>Before you can describe your servers, you must import them using <a>ImportServerCatalog</a>.</p>
    async fn get_servers(
        &self,
        input: GetServersRequest,
    ) -> Result<GetServersResponse, RusotoError<GetServersError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GetServers",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetServersError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetServersResponse, _>()
    }

    /// <p>Allows application import from AWS Migration Hub.</p>
    async fn import_app_catalog(
        &self,
        input: ImportAppCatalogRequest,
    ) -> Result<ImportAppCatalogResponse, RusotoError<ImportAppCatalogError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.ImportAppCatalog",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ImportAppCatalogError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ImportAppCatalogResponse, _>()
    }

    /// <p>Gathers a complete list of on-premises servers. Connectors must be installed and monitoring all servers to import.</p> <p>This call returns immediately, but might take additional time to retrieve all the servers.</p>
    async fn import_server_catalog(
        &self,
    ) -> Result<ImportServerCatalogResponse, RusotoError<ImportServerCatalogError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.ImportServerCatalog",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, ImportServerCatalogError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ImportServerCatalogResponse, _>()
    }

    /// <p>Launches the specified application as a stack in AWS CloudFormation.</p>
    async fn launch_app(
        &self,
        input: LaunchAppRequest,
    ) -> Result<LaunchAppResponse, RusotoError<LaunchAppError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.LaunchApp",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, LaunchAppError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<LaunchAppResponse, _>()
    }

    /// <p>Retrieves summaries for all applications.</p>
    async fn list_apps(
        &self,
        input: ListAppsRequest,
    ) -> Result<ListAppsResponse, RusotoError<ListAppsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.ListApps",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListAppsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListAppsResponse, _>()
    }

    /// <p>Provides information to AWS SMS about whether application validation is successful.</p>
    async fn notify_app_validation_output(
        &self,
        input: NotifyAppValidationOutputRequest,
    ) -> Result<NotifyAppValidationOutputResponse, RusotoError<NotifyAppValidationOutputError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.NotifyAppValidationOutput",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, NotifyAppValidationOutputError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<NotifyAppValidationOutputResponse, _>()
    }

    /// <p>Creates or updates the launch configuration for the specified application.</p>
    async fn put_app_launch_configuration(
        &self,
        input: PutAppLaunchConfigurationRequest,
    ) -> Result<PutAppLaunchConfigurationResponse, RusotoError<PutAppLaunchConfigurationError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.PutAppLaunchConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutAppLaunchConfigurationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<PutAppLaunchConfigurationResponse, _>()
    }

    /// <p>Creates or updates the replication configuration for the specified application.</p>
    async fn put_app_replication_configuration(
        &self,
        input: PutAppReplicationConfigurationRequest,
    ) -> Result<
        PutAppReplicationConfigurationResponse,
        RusotoError<PutAppReplicationConfigurationError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.PutAppReplicationConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutAppReplicationConfigurationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<PutAppReplicationConfigurationResponse, _>()
    }

    /// <p>Creates or updates a validation configuration for the specified application.</p>
    async fn put_app_validation_configuration(
        &self,
        input: PutAppValidationConfigurationRequest,
    ) -> Result<
        PutAppValidationConfigurationResponse,
        RusotoError<PutAppValidationConfigurationError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.PutAppValidationConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutAppValidationConfigurationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<PutAppValidationConfigurationResponse, _>()
    }

    /// <p>Starts replicating the specified application by creating replication jobs for each server in the application.</p>
    async fn start_app_replication(
        &self,
        input: StartAppReplicationRequest,
    ) -> Result<StartAppReplicationResponse, RusotoError<StartAppReplicationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.StartAppReplication",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StartAppReplicationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<StartAppReplicationResponse, _>()
    }

    /// <p>Starts an on-demand replication run for the specified application.</p>
    async fn start_on_demand_app_replication(
        &self,
        input: StartOnDemandAppReplicationRequest,
    ) -> Result<StartOnDemandAppReplicationResponse, RusotoError<StartOnDemandAppReplicationError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.StartOnDemandAppReplication",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StartOnDemandAppReplicationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<StartOnDemandAppReplicationResponse, _>()
    }

    /// <p>Starts an on-demand replication run for the specified replication job. This replication run starts immediately. This replication run is in addition to the ones already scheduled.</p> <p>There is a limit on the number of on-demand replications runs that you can request in a 24-hour period.</p>
    async fn start_on_demand_replication_run(
        &self,
        input: StartOnDemandReplicationRunRequest,
    ) -> Result<StartOnDemandReplicationRunResponse, RusotoError<StartOnDemandReplicationRunError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.StartOnDemandReplicationRun",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StartOnDemandReplicationRunError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<StartOnDemandReplicationRunResponse, _>()
    }

    /// <p>Stops replicating the specified application by deleting the replication job for each server in the application.</p>
    async fn stop_app_replication(
        &self,
        input: StopAppReplicationRequest,
    ) -> Result<StopAppReplicationResponse, RusotoError<StopAppReplicationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.StopAppReplication",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StopAppReplicationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<StopAppReplicationResponse, _>()
    }

    /// <p>Terminates the stack for the specified application.</p>
    async fn terminate_app(
        &self,
        input: TerminateAppRequest,
    ) -> Result<TerminateAppResponse, RusotoError<TerminateAppError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.TerminateApp",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TerminateAppError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TerminateAppResponse, _>()
    }

    /// <p>Updates the specified application.</p>
    async fn update_app(
        &self,
        input: UpdateAppRequest,
    ) -> Result<UpdateAppResponse, RusotoError<UpdateAppError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.UpdateApp",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateAppError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateAppResponse, _>()
    }

    /// <p>Updates the specified settings for the specified replication job.</p>
    async fn update_replication_job(
        &self,
        input: UpdateReplicationJobRequest,
    ) -> Result<UpdateReplicationJobResponse, RusotoError<UpdateReplicationJobError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.UpdateReplicationJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateReplicationJobError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateReplicationJobResponse, _>()
    }
}
