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

impl GameLiftClient {
    pub(crate) fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "gamelift", &self.region, request_uri);

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
/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AcceptMatchInput {
    /// <p>Player response to the proposed match.</p>
    #[serde(rename = "AcceptanceType")]
    pub acceptance_type: String,
    /// <p>A unique identifier for a player delivering the response. This parameter can include one or multiple player IDs.</p>
    #[serde(rename = "PlayerIds")]
    pub player_ids: Vec<String>,
    /// <p>A unique identifier for a matchmaking ticket. The ticket must be in status <code>REQUIRES_ACCEPTANCE</code>; otherwise this request will fail.</p>
    #[serde(rename = "TicketId")]
    pub ticket_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AcceptMatchOutput {}

/// <p><p>Properties that describe an alias resource.</p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>DescribeAlias</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ResolveAlias</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Alias {
    /// <p>Amazon Resource Name (<a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a>) that is assigned to a GameLift alias resource and uniquely identifies it. ARNs are unique across all Regions. In a GameLift alias ARN, the resource ID matches the alias ID value.</p>
    #[serde(rename = "AliasArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_arn: Option<String>,
    /// <p>A unique identifier for an alias. Alias IDs are unique within a Region.</p>
    #[serde(rename = "AliasId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_id: Option<String>,
    /// <p>A time stamp indicating when this data object was created. Format is a number expressed in Unix time as milliseconds (for example "1469498468.057").</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>A human-readable description of an alias.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The time that this data object was last modified. Format is a number expressed in Unix time as milliseconds (for example "1469498468.057").</p>
    #[serde(rename = "LastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>A descriptive label that is associated with an alias. Alias names do not need to be unique.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The routing configuration, including routing type and fleet target, for the alias. </p>
    #[serde(rename = "RoutingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_strategy: Option<RoutingStrategy>,
}

/// <p>Values for use in <a>Player</a> attribute key-value pairs. This object lets you specify an attribute value using any of the valid data types: string, number, string array, or data map. Each <code>AttributeValue</code> object can use only one of the available properties.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AttributeValue {
    /// <p>For number values, expressed as double.</p>
    #[serde(rename = "N")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<f64>,
    /// <p>For single string values. Maximum string length is 100 characters.</p>
    #[serde(rename = "S")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    /// <p>For a map of up to 10 data type:value pairs. Maximum length for each string value is 100 characters. </p>
    #[serde(rename = "SDM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdm: Option<::std::collections::HashMap<String, f64>>,
    /// <p>For a list of up to 10 strings. Maximum length for each string is 100 characters. Duplicate values are not recognized; all occurrences of the repeated value after the first of a repeated value are ignored.</p>
    #[serde(rename = "SL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl: Option<Vec<String>>,
}

/// <p>Temporary access credentials used for uploading game build files to Amazon GameLift. They are valid for a limited time. If they expire before you upload your game build, get a new set by calling <a>RequestUploadCredentials</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AwsCredentials {
    /// <p>Temporary key allowing access to the Amazon GameLift S3 account.</p>
    #[serde(rename = "AccessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    /// <p>Temporary secret key allowing access to the Amazon GameLift S3 account.</p>
    #[serde(rename = "SecretAccessKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,
    /// <p>Token used to associate a specific build ID with the files uploaded using these credentials.</p>
    #[serde(rename = "SessionToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_token: Option<String>,
}

/// <p><p>Properties describing a custom game build.</p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateBuild</a> </p> </li> <li> <p> <a>ListBuilds</a> </p> </li> <li> <p> <a>DescribeBuild</a> </p> </li> <li> <p> <a>UpdateBuild</a> </p> </li> <li> <p> <a>DeleteBuild</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Build {
    /// <p>Amazon Resource Name (<a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a>) that is assigned to a GameLift build resource and uniquely identifies it. ARNs are unique across all Regions. In a GameLift build ARN, the resource ID matches the <i>BuildId</i> value.</p>
    #[serde(rename = "BuildArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_arn: Option<String>,
    /// <p>A unique identifier for a build.</p>
    #[serde(rename = "BuildId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_id: Option<String>,
    /// <p>Time stamp indicating when this data object was created. Format is a number expressed in Unix time as milliseconds (for example "1469498468.057").</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>A descriptive label that is associated with a build. Build names do not need to be unique. It can be set using <a>CreateBuild</a> or <a>UpdateBuild</a>.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Operating system that the game server binaries are built to run on. This value determines the type of fleet resources that you can use for this build.</p>
    #[serde(rename = "OperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    /// <p>File size of the uploaded game build, expressed in bytes. When the build status is <code>INITIALIZED</code>, this value is 0.</p>
    #[serde(rename = "SizeOnDisk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_on_disk: Option<i64>,
    /// <p><p>Current status of the build.</p> <p>Possible build statuses include the following:</p> <ul> <li> <p> <b>INITIALIZED</b> -- A new build has been defined, but no files have been uploaded. You cannot create fleets for builds that are in this status. When a build is successfully created, the build status is set to this value. </p> </li> <li> <p> <b>READY</b> -- The game build has been successfully uploaded. You can now create new fleets for this build.</p> </li> <li> <p> <b>FAILED</b> -- The game build upload failed. You cannot create new fleets for this build. </p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Version information that is associated with a build or script. Version strings do not need to be unique. This value can be set using <a>CreateBuild</a> or <a>UpdateBuild</a>.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Information about the use of a TLS/SSL certificate for a fleet. TLS certificate generation is enabled at the fleet level, with one certificate generated for the fleet. When this feature is enabled, the certificate can be retrieved using the <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/reference-serversdk.html">GameLift Server SDK</a> call <code>GetInstanceCertificate</code>. All instances in a fleet share the same certificate.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CertificateConfiguration {
    /// <p>Indicates whether a TLS/SSL certificate was generated for a fleet. </p> <p> </p> <p> </p>
    #[serde(rename = "CertificateType")]
    pub certificate_type: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ClaimGameServerInput {
    /// <p>A set of custom game server properties, formatted as a single string value. This data is passed to a game client or service when it requests information on game servers using <a>ListGameServers</a> or <a>ClaimGameServer</a>. </p>
    #[serde(rename = "GameServerData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_server_data: Option<String>,
    /// <p>A unique identifier for the game server group where the game server is running. Use either the <a>GameServerGroup</a> name or ARN value.. If you are not specifying a game server to claim, this value identifies where you want GameLift FleetIQ to look for an available game server to claim. </p>
    #[serde(rename = "GameServerGroupName")]
    pub game_server_group_name: String,
    /// <p>A custom string that uniquely identifies the game server to claim. If this parameter is left empty, GameLift FleetIQ searches for an available game server in the specified game server group.</p>
    #[serde(rename = "GameServerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_server_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ClaimGameServerOutput {
    /// <p>Object that describes the newly claimed game server.</p>
    #[serde(rename = "GameServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_server: Option<GameServer>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateAliasInput {
    /// <p>A human-readable description of the alias.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A descriptive label that is associated with an alias. Alias names do not need to be unique.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The routing configuration, including routing type and fleet target, for the alias. </p>
    #[serde(rename = "RoutingStrategy")]
    pub routing_strategy: RoutingStrategy,
    /// <p>A list of labels to assign to the new alias resource. Tags are developer-defined key-value pairs. Tagging AWS resources are useful for resource management, access management and cost allocation. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html"> Tagging AWS Resources</a> in the <i>AWS General Reference</i>. Once the resource is created, you can use <a>TagResource</a>, <a>UntagResource</a>, and <a>ListTagsForResource</a> to add, remove, and view tags. The maximum tag limit may be lower than stated. See the AWS General Reference for actual tagging limits.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAliasOutput {
    /// <p>The newly created alias resource.</p>
    #[serde(rename = "Alias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Alias>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateBuildInput {
    /// <p>A descriptive label that is associated with a build. Build names do not need to be unique. You can use <a>UpdateBuild</a> to change this value later. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The operating system that the game server binaries are built to run on. This value determines the type of fleet resources that you can use for this build. If your game build contains multiple executables, they all must run on the same operating system. If an operating system is not specified when creating a build, Amazon GameLift uses the default value (WINDOWS_2012). This value cannot be changed later.</p>
    #[serde(rename = "OperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    /// <p>The location where your game build files are stored. Use this parameter only when creating a build using files that are stored in an S3 bucket that you own. Identify an S3 bucket name and key, which must in the same Region where you're creating a build. This parameter must also specify the ARN for an IAM role that you've set up to give Amazon GameLift access your S3 bucket. To call this operation with a storage location, you must have IAM PassRole permission. For more details on IAM roles and PassRole permissions, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/setting-up-role.html"> Set up a role for GameLift access</a>. </p>
    #[serde(rename = "StorageLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_location: Option<S3Location>,
    /// <p>A list of labels to assign to the new build resource. Tags are developer-defined key-value pairs. Tagging AWS resources are useful for resource management, access management and cost allocation. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html"> Tagging AWS Resources</a> in the <i>AWS General Reference</i>. Once the resource is created, you can use <a>TagResource</a>, <a>UntagResource</a>, and <a>ListTagsForResource</a> to add, remove, and view tags. The maximum tag limit may be lower than stated. See the AWS General Reference for actual tagging limits.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>Version information that is associated with a build or script. Version strings do not need to be unique. You can use <a>UpdateBuild</a> to change this value later. </p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateBuildOutput {
    /// <p>The newly created build resource, including a unique build IDs and status. </p>
    #[serde(rename = "Build")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<Build>,
    /// <p>Amazon S3 location for your game build file, including bucket name and key.</p>
    #[serde(rename = "StorageLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_location: Option<S3Location>,
    /// <p>This element is returned only when the operation is called without a storage location. It contains credentials to use when you are uploading a build file to an S3 bucket that is owned by Amazon GameLift. Credentials have a limited life span. To refresh these credentials, call <a>RequestUploadCredentials</a>. </p>
    #[serde(rename = "UploadCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_credentials: Option<AwsCredentials>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateFleetInput {
    /// <p>A unique identifier for a build to be deployed on the new fleet. You can use either the build ID or ARN value. The custom game server build must have been successfully uploaded to Amazon GameLift and be in a <code>READY</code> status. This fleet setting cannot be changed once the fleet is created. </p>
    #[serde(rename = "BuildId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_id: Option<String>,
    /// <p><p>Indicates whether to generate a TLS/SSL certificate for the new fleet. TLS certificates are used for encrypting traffic between game clients and game servers running on GameLift. If this parameter is not specified, the default value, DISABLED, is used. This fleet setting cannot be changed once the fleet is created. Learn more at <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-howitworks.html#gamelift-howitworks-security">Securing Client/Server Communication</a>. </p> <p>Note: This feature requires the AWS Certificate Manager (ACM) service, which is available in the AWS global partition but not in all other partitions. When working in a partition that does not support this feature, a request for a new fleet with certificate generation results fails with a 4xx unsupported Region error.</p> <p>Valid values include: </p> <ul> <li> <p> <b>GENERATED</b> - Generate a TLS/SSL certificate for this fleet.</p> </li> <li> <p> <b>DISABLED</b> - (default) Do not generate a TLS/SSL certificate for this fleet.</p> </li> </ul></p>
    #[serde(rename = "CertificateConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_configuration: Option<CertificateConfiguration>,
    /// <p>A human-readable description of a fleet.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Range of IP addresses and port settings that permit inbound traffic to access game sessions that are running on the fleet. For fleets using a custom game build, this parameter is required before game sessions running on the fleet can accept connections. For Realtime Servers fleets, Amazon GameLift automatically sets TCP and UDP ranges for use by the Realtime servers. You can specify multiple permission settings or add more by updating the fleet.</p>
    #[serde(rename = "EC2InboundPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_inbound_permissions: Option<Vec<IpPermission>>,
    /// <p>The name of an EC2 instance type that is supported in Amazon GameLift. A fleet instance type determines the computing resources of each instance in the fleet, including CPU, memory, storage, and networking capacity. Amazon GameLift supports the following EC2 instance types. See <a href="http://aws.amazon.com/ec2/instance-types/">Amazon EC2 Instance Types</a> for detailed descriptions.</p>
    #[serde(rename = "EC2InstanceType")]
    pub ec2_instance_type: String,
    /// <p>Indicates whether to use On-Demand instances or Spot instances for this fleet. If empty, the default is <code>ON_DEMAND</code>. Both categories of instances use identical hardware and configurations based on the instance type selected for this fleet. Learn more about <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-ec2-instances.html#gamelift-ec2-instances-spot"> On-Demand versus Spot Instances</a>. </p>
    #[serde(rename = "FleetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_type: Option<String>,
    /// <p>A unique identifier for an AWS IAM role that manages access to your AWS services. Fleets with an instance role ARN allow applications that are running on the fleet's instances to assume the role. Learn more about using on-box credentials for your game servers at <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-resources.html"> Access external resources from a game server</a>. To call this operation with instance role ARN, you must have IAM PassRole permissions. See <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-iam-policy-examples.html">IAM policy examples for GameLift</a>. </p>
    #[serde(rename = "InstanceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_role_arn: Option<String>,
    /// <p>This parameter is no longer used. Instead, to specify where Amazon GameLift should store log files once a server process shuts down, use the Amazon GameLift server API <code>ProcessReady()</code> and specify one or more directory paths in <code>logParameters</code>. See more information in the <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api-ref.html#gamelift-sdk-server-api-ref-dataypes-process">Server API Reference</a>. </p>
    #[serde(rename = "LogPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_paths: Option<Vec<String>>,
    /// <p>The name of an Amazon CloudWatch metric group to add this fleet to. A metric group aggregates the metrics for all fleets in the group. Specify an existing metric group name, or provide a new name to create a new metric group. A fleet can only be included in one metric group at a time. </p>
    #[serde(rename = "MetricGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_groups: Option<Vec<String>>,
    /// <p>A descriptive label that is associated with a fleet. Fleet names do not need to be unique.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>A game session protection policy to apply to all instances in this fleet. If this parameter is not set, instances in this fleet default to no protection. You can change a fleet&#39;s protection policy using <a>UpdateFleetAttributes</a>, but this change will only affect sessions created after the policy change. You can also set protection for individual instances using <a>UpdateGameSession</a>.</p> <ul> <li> <p> <b>NoProtection</b> - The game session can be terminated during a scale-down event.</p> </li> <li> <p> <b>FullProtection</b> - If the game session is in an <code>ACTIVE</code> status, it cannot be terminated during a scale-down event.</p> </li> </ul></p>
    #[serde(rename = "NewGameSessionProtectionPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_game_session_protection_policy: Option<String>,
    /// <p>A unique identifier for the AWS account with the VPC that you want to peer your Amazon GameLift fleet with. You can find your account ID in the AWS Management Console under account settings. </p>
    #[serde(rename = "PeerVpcAwsAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_vpc_aws_account_id: Option<String>,
    /// <p>A unique identifier for a VPC with resources to be accessed by your Amazon GameLift fleet. The VPC must be in the same Region as your fleet. To look up a VPC ID, use the <a href="https://console.aws.amazon.com/vpc/">VPC Dashboard</a> in the AWS Management Console. Learn more about VPC peering in <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/vpc-peering.html">VPC Peering with Amazon GameLift Fleets</a>. </p>
    #[serde(rename = "PeerVpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_vpc_id: Option<String>,
    /// <p>A policy that limits the number of game sessions an individual player can create over a span of time for this fleet.</p>
    #[serde(rename = "ResourceCreationLimitPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_creation_limit_policy: Option<ResourceCreationLimitPolicy>,
    /// <p>Instructions for launching server processes on each instance in the fleet. Server processes run either a custom game build executable or a Realtime script. The runtime configuration defines the server executables or launch script file, launch parameters, and the number of processes to run concurrently on each instance. When creating a fleet, the runtime configuration must have at least one server process configuration; otherwise the request fails with an invalid request exception. (This parameter replaces the parameters <code>ServerLaunchPath</code> and <code>ServerLaunchParameters</code>, although requests that contain values for these parameters instead of a runtime configuration will continue to work.) This parameter is required unless the parameters <code>ServerLaunchPath</code> and <code>ServerLaunchParameters</code> are defined. Runtime configuration replaced these parameters, but fleets that use them will continue to work. </p>
    #[serde(rename = "RuntimeConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_configuration: Option<RuntimeConfiguration>,
    /// <p>A unique identifier for a Realtime script to be deployed on the new fleet. You can use either the script ID or ARN value. The Realtime script must have been successfully uploaded to Amazon GameLift. This fleet setting cannot be changed once the fleet is created.</p>
    #[serde(rename = "ScriptId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_id: Option<String>,
    /// <p>This parameter is no longer used. Instead, specify server launch parameters in the <code>RuntimeConfiguration</code> parameter. (Requests that specify a server launch path and launch parameters instead of a runtime configuration will continue to work.)</p>
    #[serde(rename = "ServerLaunchParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_launch_parameters: Option<String>,
    /// <p>This parameter is no longer used. Instead, specify a server launch path using the <code>RuntimeConfiguration</code> parameter. Requests that specify a server launch path and launch parameters instead of a runtime configuration will continue to work.</p>
    #[serde(rename = "ServerLaunchPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_launch_path: Option<String>,
    /// <p>A list of labels to assign to the new fleet resource. Tags are developer-defined key-value pairs. Tagging AWS resources are useful for resource management, access management and cost allocation. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html"> Tagging AWS Resources</a> in the <i>AWS General Reference</i>. Once the resource is created, you can use <a>TagResource</a>, <a>UntagResource</a>, and <a>ListTagsForResource</a> to add, remove, and view tags. The maximum tag limit may be lower than stated. See the AWS General Reference for actual tagging limits.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateFleetOutput {
    /// <p>Properties for the newly created fleet.</p>
    #[serde(rename = "FleetAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_attributes: Option<FleetAttributes>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateGameServerGroupInput {
    /// <p>Configuration settings to define a scaling policy for the Auto Scaling group that is optimized for game hosting. The scaling policy uses the metric <code>"PercentUtilizedGameServers"</code> to maintain a buffer of idle game servers that can immediately accommodate new games and players. After the Auto Scaling group is created, update this value directly in the Auto Scaling group using the AWS console or APIs.</p>
    #[serde(rename = "AutoScalingPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_policy: Option<GameServerGroupAutoScalingPolicy>,
    /// <p><p>Indicates how GameLift FleetIQ balances the use of Spot Instances and On-Demand Instances in the game server group. Method options include the following:</p> <ul> <li> <p> <code>SPOT<em>ONLY</code> - Only Spot Instances are used in the game server group. If Spot Instances are unavailable or not viable for game hosting, the game server group provides no hosting capacity until Spot Instances can again be used. Until then, no new instances are started, and the existing nonviable Spot Instances are terminated (after current gameplay ends) and are not replaced.</p> </li> <li> <p> <code>SPOT</em>PREFERRED</code> - (default value) Spot Instances are used whenever available in the game server group. If Spot Instances are unavailable, the game server group continues to provide hosting capacity by falling back to On-Demand Instances. Existing nonviable Spot Instances are terminated (after current gameplay ends) and are replaced with new On-Demand Instances.</p> </li> <li> <p> <code>ON<em>DEMAND</em>ONLY</code> - Only On-Demand Instances are used in the game server group. No Spot Instances are used, even when available, while this balancing strategy is in force.</p> </li> </ul></p>
    #[serde(rename = "BalancingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balancing_strategy: Option<String>,
    /// <p>An identifier for the new game server group. This value is used to generate unique ARN identifiers for the EC2 Auto Scaling group and the GameLift FleetIQ game server group. The name must be unique per Region per AWS account.</p>
    #[serde(rename = "GameServerGroupName")]
    pub game_server_group_name: String,
    /// <p>A flag that indicates whether instances in the game server group are protected from early termination. Unprotected instances that have active game servers running might be terminated during a scale-down event, causing players to be dropped from the game. Protected instances cannot be terminated while there are active game servers running except in the event of a forced game server group deletion (see ). An exception to this is with Spot Instances, which can be terminated by AWS regardless of protection status. This property is set to <code>NO_PROTECTION</code> by default.</p>
    #[serde(rename = "GameServerProtectionPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_server_protection_policy: Option<String>,
    /// <p>The EC2 instance types and sizes to use in the Auto Scaling group. The instance definitions must specify at least two different instance types that are supported by GameLift FleetIQ. For more information on instance types, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html">EC2 Instance Types</a> in the <i>Amazon EC2 User Guide</i>. You can optionally specify capacity weighting for each instance type. If no weight value is specified for an instance type, it is set to the default value "1". For more information about capacity weighting, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-instance-weighting.html"> Instance Weighting for Amazon EC2 Auto Scaling</a> in the Amazon EC2 Auto Scaling User Guide.</p>
    #[serde(rename = "InstanceDefinitions")]
    pub instance_definitions: Vec<InstanceDefinition>,
    /// <p>The EC2 launch template that contains configuration settings and game server code to be deployed to all instances in the game server group. You can specify the template using either the template name or ID. For help with creating a launch template, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/create-launch-template.html">Creating a Launch Template for an Auto Scaling Group</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>. After the Auto Scaling group is created, update this value directly in the Auto Scaling group using the AWS console or APIs.</p>
    #[serde(rename = "LaunchTemplate")]
    pub launch_template: LaunchTemplateSpecification,
    /// <p>The maximum number of instances allowed in the EC2 Auto Scaling group. During automatic scaling events, GameLift FleetIQ and EC2 do not scale up the group above this maximum. After the Auto Scaling group is created, update this value directly in the Auto Scaling group using the AWS console or APIs.</p>
    #[serde(rename = "MaxSize")]
    pub max_size: i64,
    /// <p>The minimum number of instances allowed in the EC2 Auto Scaling group. During automatic scaling events, GameLift FleetIQ and EC2 do not scale down the group below this minimum. In production, this value should be set to at least 1. After the Auto Scaling group is created, update this value directly in the Auto Scaling group using the AWS console or APIs.</p>
    #[serde(rename = "MinSize")]
    pub min_size: i64,
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) for an IAM role that allows Amazon GameLift to access your EC2 Auto Scaling groups.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>A list of labels to assign to the new game server group resource. Tags are developer-defined key-value pairs. Tagging AWS resources is useful for resource management, access management, and cost allocation. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html"> Tagging AWS Resources</a> in the <i>AWS General Reference</i>. Once the resource is created, you can use <a>TagResource</a>, <a>UntagResource</a>, and <a>ListTagsForResource</a> to add, remove, and view tags, respectively. The maximum tag limit may be lower than stated. See the AWS General Reference for actual tagging limits.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>A list of virtual private cloud (VPC) subnets to use with instances in the game server group. By default, all GameLift FleetIQ-supported Availability Zones are used. You can use this parameter to specify VPCs that you've set up. This property cannot be updated after the game server group is created, and the corresponding Auto Scaling group will always use the property value that is set with this request, even if the Auto Scaling group is updated directly.</p>
    #[serde(rename = "VpcSubnets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_subnets: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateGameServerGroupOutput {
    /// <p>The newly created game server group object, including the new ARN value for the GameLift FleetIQ game server group and the object's status. The EC2 Auto Scaling group ARN is initially null, since the group has not yet been created. This value is added once the game server group status reaches <code>ACTIVE</code>. </p>
    #[serde(rename = "GameServerGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_server_group: Option<GameServerGroup>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateGameSessionInput {
    /// <p>A unique identifier for an alias associated with the fleet to create a game session in. You can use either the alias ID or ARN value. Each request must reference either a fleet ID or alias ID, but not both.</p>
    #[serde(rename = "AliasId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_id: Option<String>,
    /// <p>A unique identifier for a player or entity creating the game session. This ID is used to enforce a resource protection policy (if one exists) that limits the number of concurrent active game sessions one player can have.</p>
    #[serde(rename = "CreatorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    /// <p>A unique identifier for a fleet to create a game session in. You can use either the fleet ID or ARN value. Each request must reference either a fleet ID or alias ID, but not both.</p>
    #[serde(rename = "FleetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_id: Option<String>,
    /// <p>Set of custom properties for a game session, formatted as key:value pairs. These properties are passed to a game server process in the <a>GameSession</a> object with a request to start a new game session (see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession">Start a Game Session</a>).</p>
    #[serde(rename = "GameProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_properties: Option<Vec<GameProperty>>,
    /// <p>Set of custom game session properties, formatted as a single string value. This data is passed to a game server process in the <a>GameSession</a> object with a request to start a new game session (see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession">Start a Game Session</a>).</p>
    #[serde(rename = "GameSessionData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_data: Option<String>,
    /// <p> <i>This parameter is no longer preferred. Please use <code>IdempotencyToken</code> instead.</i> Custom string that uniquely identifies a request for a new game session. Maximum token length is 48 characters. If provided, this string is included in the new game session's ID. (A game session ARN has the following format: <code>arn:aws:gamelift:&lt;region&gt;::gamesession/&lt;fleet ID&gt;/&lt;custom ID string or idempotency token&gt;</code>.) </p>
    #[serde(rename = "GameSessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_id: Option<String>,
    /// <p>Custom string that uniquely identifies a request for a new game session. Maximum token length is 48 characters. If provided, this string is included in the new game session's ID. (A game session ARN has the following format: <code>arn:aws:gamelift:&lt;region&gt;::gamesession/&lt;fleet ID&gt;/&lt;custom ID string or idempotency token&gt;</code>.) Idempotency tokens remain in use for 30 days after a game session has ended; game session objects are retained for this time period and then deleted.</p>
    #[serde(rename = "IdempotencyToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    /// <p>The maximum number of players that can be connected simultaneously to the game session.</p>
    #[serde(rename = "MaximumPlayerSessionCount")]
    pub maximum_player_session_count: i64,
    /// <p>A descriptive label that is associated with a game session. Session names do not need to be unique.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateGameSessionOutput {
    /// <p>Object that describes the newly created game session record.</p>
    #[serde(rename = "GameSession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session: Option<GameSession>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateGameSessionQueueInput {
    /// <p>A list of fleets that can be used to fulfill game session placement requests in the queue. Fleets are identified by either a fleet ARN or a fleet alias ARN. Destinations are listed in default preference order.</p>
    #[serde(rename = "Destinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<GameSessionQueueDestination>>,
    /// <p>A descriptive label that is associated with game session queue. Queue names must be unique within each Region.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A collection of latency policies to apply when processing game sessions placement requests with player latency information. Multiple policies are evaluated in order of the maximum latency value, starting with the lowest latency values. With just one policy, the policy is enforced at the start of the game session placement for the duration period. With multiple policies, each policy is enforced consecutively for its duration period. For example, a queue might enforce a 60-second policy followed by a 120-second policy, and then no policy for the remainder of the placement. A player latency policy must set a value for <code>MaximumIndividualPlayerLatencyMilliseconds</code>. If none is set, this API request fails.</p>
    #[serde(rename = "PlayerLatencyPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_latency_policies: Option<Vec<PlayerLatencyPolicy>>,
    /// <p>A list of labels to assign to the new game session queue resource. Tags are developer-defined key-value pairs. Tagging AWS resources are useful for resource management, access management and cost allocation. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html"> Tagging AWS Resources</a> in the <i>AWS General Reference</i>. Once the resource is created, you can use <a>TagResource</a>, <a>UntagResource</a>, and <a>ListTagsForResource</a> to add, remove, and view tags. The maximum tag limit may be lower than stated. See the AWS General Reference for actual tagging limits.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The maximum time, in seconds, that a new game session placement request remains in the queue. When a request exceeds this time, the game session placement changes to a <code>TIMED_OUT</code> status.</p>
    #[serde(rename = "TimeoutInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_seconds: Option<i64>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateGameSessionQueueOutput {
    /// <p>An object that describes the newly created game session queue.</p>
    #[serde(rename = "GameSessionQueue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_queue: Option<GameSessionQueue>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateMatchmakingConfigurationInput {
    /// <p>A flag that determines whether a match that was created with this configuration must be accepted by the matched players. To require acceptance, set to <code>TRUE</code>. With this option enabled, matchmaking tickets use the status <code>REQUIRES_ACCEPTANCE</code> to indicate when a completed potential match is waiting for player acceptance. </p>
    #[serde(rename = "AcceptanceRequired")]
    pub acceptance_required: bool,
    /// <p>The length of time (in seconds) to wait for players to accept a proposed match, if acceptance is required. If any player rejects the match or fails to accept before the timeout, the tickets are returned to the ticket pool and continue to be evaluated for an acceptable match.</p>
    #[serde(rename = "AcceptanceTimeoutSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance_timeout_seconds: Option<i64>,
    /// <p>The number of player slots in a match to keep open for future players. For example, assume that the configuration's rule set specifies a match for a single 12-person team. If the additional player count is set to 2, only 10 players are initially selected for the match. This parameter is not used if <code>FlexMatchMode</code> is set to <code>STANDALONE</code>.</p>
    #[serde(rename = "AdditionalPlayerCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_player_count: Option<i64>,
    /// <p>The method used to backfill game sessions that are created with this matchmaking configuration. Specify <code>MANUAL</code> when your game manages backfill requests manually or does not use the match backfill feature. Specify <code>AUTOMATIC</code> to have GameLift create a <a>StartMatchBackfill</a> request whenever a game session has one or more open slots. Learn more about manual and automatic backfill in <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-backfill.html"> Backfill Existing Games with FlexMatch</a>. Automatic backfill is not available when <code>FlexMatchMode</code> is set to <code>STANDALONE</code>.</p>
    #[serde(rename = "BackfillMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backfill_mode: Option<String>,
    /// <p>Information to be added to all events related to this matchmaking configuration. </p>
    #[serde(rename = "CustomEventData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_event_data: Option<String>,
    /// <p>A human-readable description of the matchmaking configuration. </p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p><p>Indicates whether this matchmaking configuration is being used with GameLift hosting or as a standalone matchmaking solution. </p> <ul> <li> <p> <b>STANDALONE</b> - FlexMatch forms matches and returns match information, including players and team assignments, in a <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-events.html#match-events-matchmakingsucceeded"> MatchmakingSucceeded</a> event.</p> </li> <li> <p> <b>WITH_QUEUE</b> - FlexMatch forms matches and uses the specified GameLift queue to start a game session for the match. </p> </li> </ul></p>
    #[serde(rename = "FlexMatchMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex_match_mode: Option<String>,
    /// <p>A set of custom properties for a game session, formatted as key-value pairs. These properties are passed to a game server process in the <a>GameSession</a> object with a request to start a new game session (see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession">Start a Game Session</a>). This information is added to the new <a>GameSession</a> object that is created for a successful match. This parameter is not used if <code>FlexMatchMode</code> is set to <code>STANDALONE</code>.</p>
    #[serde(rename = "GameProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_properties: Option<Vec<GameProperty>>,
    /// <p>A set of custom game session properties, formatted as a single string value. This data is passed to a game server process in the <a>GameSession</a> object with a request to start a new game session (see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession">Start a Game Session</a>). This information is added to the new <a>GameSession</a> object that is created for a successful match. This parameter is not used if <code>FlexMatchMode</code> is set to <code>STANDALONE</code>.</p>
    #[serde(rename = "GameSessionData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_data: Option<String>,
    /// <p>Amazon Resource Name (<a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a>) that is assigned to a GameLift game session queue resource and uniquely identifies it. ARNs are unique across all Regions. Queues can be located in any Region. Queues are used to start new GameLift-hosted game sessions for matches that are created with this matchmaking configuration. If <code>FlexMatchMode</code> is set to <code>STANDALONE</code>, do not set this parameter. </p>
    #[serde(rename = "GameSessionQueueArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_queue_arns: Option<Vec<String>>,
    /// <p>A unique identifier for a matchmaking configuration. This name is used to identify the configuration associated with a matchmaking request or ticket.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>An SNS topic ARN that is set up to receive matchmaking notifications.</p>
    #[serde(rename = "NotificationTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_target: Option<String>,
    /// <p>The maximum duration, in seconds, that a matchmaking ticket can remain in process before timing out. Requests that fail due to timing out can be resubmitted as needed.</p>
    #[serde(rename = "RequestTimeoutSeconds")]
    pub request_timeout_seconds: i64,
    /// <p>A unique identifier for a matchmaking rule set to use with this configuration. You can use either the rule set name or ARN value. A matchmaking configuration can only use rule sets that are defined in the same Region.</p>
    #[serde(rename = "RuleSetName")]
    pub rule_set_name: String,
    /// <p>A list of labels to assign to the new matchmaking configuration resource. Tags are developer-defined key-value pairs. Tagging AWS resources are useful for resource management, access management and cost allocation. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html"> Tagging AWS Resources</a> in the <i>AWS General Reference</i>. Once the resource is created, you can use <a>TagResource</a>, <a>UntagResource</a>, and <a>ListTagsForResource</a> to add, remove, and view tags. The maximum tag limit may be lower than stated. See the AWS General Reference for actual tagging limits.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateMatchmakingConfigurationOutput {
    /// <p>Object that describes the newly created matchmaking configuration.</p>
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<MatchmakingConfiguration>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateMatchmakingRuleSetInput {
    /// <p>A unique identifier for a matchmaking rule set. A matchmaking configuration identifies the rule set it uses by this name value. Note that the rule set name is different from the optional <code>name</code> field in the rule set body.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A collection of matchmaking rules, formatted as a JSON string. Comments are not allowed in JSON, but most elements support a description field.</p>
    #[serde(rename = "RuleSetBody")]
    pub rule_set_body: String,
    /// <p>A list of labels to assign to the new matchmaking rule set resource. Tags are developer-defined key-value pairs. Tagging AWS resources are useful for resource management, access management and cost allocation. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html"> Tagging AWS Resources</a> in the <i>AWS General Reference</i>. Once the resource is created, you can use <a>TagResource</a>, <a>UntagResource</a>, and <a>ListTagsForResource</a> to add, remove, and view tags. The maximum tag limit may be lower than stated. See the AWS General Reference for actual tagging limits.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateMatchmakingRuleSetOutput {
    /// <p>The newly created matchmaking rule set.</p>
    #[serde(rename = "RuleSet")]
    pub rule_set: MatchmakingRuleSet,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreatePlayerSessionInput {
    /// <p>A unique identifier for the game session to add a player to.</p>
    #[serde(rename = "GameSessionId")]
    pub game_session_id: String,
    /// <p>Developer-defined information related to a player. Amazon GameLift does not use this data, so it can be formatted as needed for use in the game.</p>
    #[serde(rename = "PlayerData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_data: Option<String>,
    /// <p>A unique identifier for a player. Player IDs are developer-defined.</p>
    #[serde(rename = "PlayerId")]
    pub player_id: String,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreatePlayerSessionOutput {
    /// <p>Object that describes the newly created player session record.</p>
    #[serde(rename = "PlayerSession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_session: Option<PlayerSession>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreatePlayerSessionsInput {
    /// <p>A unique identifier for the game session to add players to.</p>
    #[serde(rename = "GameSessionId")]
    pub game_session_id: String,
    /// <p>Map of string pairs, each specifying a player ID and a set of developer-defined information related to the player. Amazon GameLift does not use this data, so it can be formatted as needed for use in the game. Player data strings for player IDs not included in the <code>PlayerIds</code> parameter are ignored. </p>
    #[serde(rename = "PlayerDataMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_data_map: Option<::std::collections::HashMap<String, String>>,
    /// <p>List of unique identifiers for the players to be added.</p>
    #[serde(rename = "PlayerIds")]
    pub player_ids: Vec<String>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreatePlayerSessionsOutput {
    /// <p>A collection of player session objects created for the added players.</p>
    #[serde(rename = "PlayerSessions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_sessions: Option<Vec<PlayerSession>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateScriptInput {
    /// <p>A descriptive label that is associated with a script. Script names do not need to be unique. You can use <a>UpdateScript</a> to change this value later. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon S3 location of your Realtime scripts. The storage location must specify the S3 bucket name, the zip file name (the "key"), and an IAM role ARN that allows Amazon GameLift to access the S3 storage location. The S3 bucket must be in the same Region where you are creating a new script. By default, Amazon GameLift uploads the latest version of the zip file; if you have S3 object versioning turned on, you can use the <code>ObjectVersion</code> parameter to specify an earlier version. To call this operation with a storage location, you must have IAM PassRole permission. For more details on IAM roles and PassRole permissions, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/setting-up-role.html"> Set up a role for GameLift access</a>. </p>
    #[serde(rename = "StorageLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_location: Option<S3Location>,
    /// <p>A list of labels to assign to the new script resource. Tags are developer-defined key-value pairs. Tagging AWS resources are useful for resource management, access management and cost allocation. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html"> Tagging AWS Resources</a> in the <i>AWS General Reference</i>. Once the resource is created, you can use <a>TagResource</a>, <a>UntagResource</a>, and <a>ListTagsForResource</a> to add, remove, and view tags. The maximum tag limit may be lower than stated. See the AWS General Reference for actual tagging limits.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The version that is associated with a build or script. Version strings do not need to be unique. You can use <a>UpdateScript</a> to change this value later. </p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>A data object containing your Realtime scripts and dependencies as a zip file. The zip file can have one or multiple files. Maximum size of a zip file is 5 MB.</p> <p>When using the AWS CLI tool to create a script, this parameter is set to the zip file name. It must be prepended with the string "fileb://" to indicate that the file data is a binary object. For example: <code>--zip-file fileb://myRealtimeScript.zip</code>.</p>
    #[serde(rename = "ZipFile")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_file: Option<bytes::Bytes>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateScriptOutput {
    /// <p>The newly created script record with a unique script ID and ARN. The new script's storage location reflects an Amazon S3 location: (1) If the script was uploaded from an S3 bucket under your account, the storage location reflects the information that was provided in the <i>CreateScript</i> request; (2) If the script file was uploaded from a local zip file, the storage location reflects an S3 location controls by the Amazon GameLift service.</p>
    #[serde(rename = "Script")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<Script>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateVpcPeeringAuthorizationInput {
    /// <p>A unique identifier for the AWS account that you use to manage your Amazon GameLift fleet. You can find your Account ID in the AWS Management Console under account settings.</p>
    #[serde(rename = "GameLiftAwsAccountId")]
    pub game_lift_aws_account_id: String,
    /// <p>A unique identifier for a VPC with resources to be accessed by your Amazon GameLift fleet. The VPC must be in the same Region where your fleet is deployed. Look up a VPC ID using the <a href="https://console.aws.amazon.com/vpc/">VPC Dashboard</a> in the AWS Management Console. Learn more about VPC peering in <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/vpc-peering.html">VPC Peering with Amazon GameLift Fleets</a>.</p>
    #[serde(rename = "PeerVpcId")]
    pub peer_vpc_id: String,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateVpcPeeringAuthorizationOutput {
    /// <p>Details on the requested VPC peering authorization, including expiration.</p>
    #[serde(rename = "VpcPeeringAuthorization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_peering_authorization: Option<VpcPeeringAuthorization>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateVpcPeeringConnectionInput {
    /// <p>A unique identifier for a fleet. You can use either the fleet ID or ARN value. This tells Amazon GameLift which GameLift VPC to peer with. </p>
    #[serde(rename = "FleetId")]
    pub fleet_id: String,
    /// <p>A unique identifier for the AWS account with the VPC that you want to peer your Amazon GameLift fleet with. You can find your Account ID in the AWS Management Console under account settings.</p>
    #[serde(rename = "PeerVpcAwsAccountId")]
    pub peer_vpc_aws_account_id: String,
    /// <p>A unique identifier for a VPC with resources to be accessed by your Amazon GameLift fleet. The VPC must be in the same Region where your fleet is deployed. Look up a VPC ID using the <a href="https://console.aws.amazon.com/vpc/">VPC Dashboard</a> in the AWS Management Console. Learn more about VPC peering in <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/vpc-peering.html">VPC Peering with Amazon GameLift Fleets</a>.</p>
    #[serde(rename = "PeerVpcId")]
    pub peer_vpc_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateVpcPeeringConnectionOutput {}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAliasInput {
    /// <p>A unique identifier of the alias that you want to delete. You can use either the alias ID or ARN value.</p>
    #[serde(rename = "AliasId")]
    pub alias_id: String,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteBuildInput {
    /// <p>A unique identifier for a build to delete. You can use either the build ID or ARN value. </p>
    #[serde(rename = "BuildId")]
    pub build_id: String,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFleetInput {
    /// <p>A unique identifier for a fleet to be deleted. You can use either the fleet ID or ARN value.</p>
    #[serde(rename = "FleetId")]
    pub fleet_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteGameServerGroupInput {
    /// <p><p>The type of delete to perform. Options include the following:</p> <ul> <li> <p> <code>SAFE<em>DELETE</code> – (default) Terminates the game server group and EC2 Auto Scaling group only when it has no game servers that are in <code>UTILIZED</code> status.</p> </li> <li> <p> <code>FORCE</em>DELETE</code> – Terminates the game server group, including all active game servers regardless of their utilization status, and the EC2 Auto Scaling group. </p> </li> <li> <p> <code>RETAIN</code> – Does a safe delete of the game server group but retains the EC2 Auto Scaling group as is.</p> </li> </ul></p>
    #[serde(rename = "DeleteOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_option: Option<String>,
    /// <p>A unique identifier for the game server group. Use either the <a>GameServerGroup</a> name or ARN value.</p>
    #[serde(rename = "GameServerGroupName")]
    pub game_server_group_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteGameServerGroupOutput {
    /// <p>An object that describes the deleted game server group resource, with status updated to <code>DELETE_SCHEDULED</code>. </p>
    #[serde(rename = "GameServerGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_server_group: Option<GameServerGroup>,
}

/// <p>Represents the input for a request operation. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteGameSessionQueueInput {
    /// <p>A descriptive label that is associated with game session queue. Queue names must be unique within each Region. You can use either the queue ID or ARN value. </p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteGameSessionQueueOutput {}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteMatchmakingConfigurationInput {
    /// <p>A unique identifier for a matchmaking configuration. You can use either the configuration name or ARN value.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteMatchmakingConfigurationOutput {}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteMatchmakingRuleSetInput {
    /// <p>A unique identifier for a matchmaking rule set to be deleted. (Note: The rule set name is different from the optional "name" field in the rule set body.) You can use either the rule set name or ARN value.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteMatchmakingRuleSetOutput {}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteScalingPolicyInput {
    /// <p>A unique identifier for a fleet to be deleted. You can use either the fleet ID or ARN value.</p>
    #[serde(rename = "FleetId")]
    pub fleet_id: String,
    /// <p>A descriptive label that is associated with a scaling policy. Policy names do not need to be unique.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteScriptInput {
    /// <p>A unique identifier for a Realtime script to delete. You can use either the script ID or ARN value.</p>
    #[serde(rename = "ScriptId")]
    pub script_id: String,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVpcPeeringAuthorizationInput {
    /// <p>A unique identifier for the AWS account that you use to manage your Amazon GameLift fleet. You can find your Account ID in the AWS Management Console under account settings.</p>
    #[serde(rename = "GameLiftAwsAccountId")]
    pub game_lift_aws_account_id: String,
    /// <p>A unique identifier for a VPC with resources to be accessed by your Amazon GameLift fleet. The VPC must be in the same Region where your fleet is deployed. Look up a VPC ID using the <a href="https://console.aws.amazon.com/vpc/">VPC Dashboard</a> in the AWS Management Console. Learn more about VPC peering in <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/vpc-peering.html">VPC Peering with Amazon GameLift Fleets</a>.</p>
    #[serde(rename = "PeerVpcId")]
    pub peer_vpc_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteVpcPeeringAuthorizationOutput {}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVpcPeeringConnectionInput {
    /// <p>A unique identifier for a fleet. This fleet specified must match the fleet referenced in the VPC peering connection record. You can use either the fleet ID or ARN value.</p>
    #[serde(rename = "FleetId")]
    pub fleet_id: String,
    /// <p>A unique identifier for a VPC peering connection. This value is included in the <a>VpcPeeringConnection</a> object, which can be retrieved by calling <a>DescribeVpcPeeringConnections</a>.</p>
    #[serde(rename = "VpcPeeringConnectionId")]
    pub vpc_peering_connection_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteVpcPeeringConnectionOutput {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeregisterGameServerInput {
    /// <p>A unique identifier for the game server group where the game server is running. Use either the <a>GameServerGroup</a> name or ARN value.</p>
    #[serde(rename = "GameServerGroupName")]
    pub game_server_group_name: String,
    /// <p>A custom string that uniquely identifies the game server to deregister.</p>
    #[serde(rename = "GameServerId")]
    pub game_server_id: String,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAliasInput {
    /// <p>The unique identifier for the fleet alias that you want to retrieve. You can use either the alias ID or ARN value. </p>
    #[serde(rename = "AliasId")]
    pub alias_id: String,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAliasOutput {
    /// <p>The requested alias resource.</p>
    #[serde(rename = "Alias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Alias>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeBuildInput {
    /// <p>A unique identifier for a build to retrieve properties for. You can use either the build ID or ARN value. </p>
    #[serde(rename = "BuildId")]
    pub build_id: String,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeBuildOutput {
    /// <p>Set of properties describing the requested build.</p>
    #[serde(rename = "Build")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<Build>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEC2InstanceLimitsInput {
    /// <p>Name of an EC2 instance type that is supported in Amazon GameLift. A fleet instance type determines the computing resources of each instance in the fleet, including CPU, memory, storage, and networking capacity. Amazon GameLift supports the following EC2 instance types. See <a href="http://aws.amazon.com/ec2/instance-types/">Amazon EC2 Instance Types</a> for detailed descriptions. Leave this parameter blank to retrieve limits for all types.</p>
    #[serde(rename = "EC2InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_type: Option<String>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEC2InstanceLimitsOutput {
    /// <p>The maximum number of instances for the specified instance type.</p>
    #[serde(rename = "EC2InstanceLimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_limits: Option<Vec<EC2InstanceLimit>>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeFleetAttributesInput {
    /// <p>A list of unique fleet identifiers to retrieve attributes for. You can use either the fleet ID or ARN value. To retrieve attributes for all current fleets, do not include this parameter. If the list of fleet identifiers includes fleets that don't currently exist, the request succeeds but no attributes for that fleet are returned.</p>
    #[serde(rename = "FleetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_ids: Option<Vec<String>>,
    /// <p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages. This parameter is ignored when the request specifies one or a list of fleet IDs.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value. This parameter is ignored when the request specifies one or a list of fleet IDs.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeFleetAttributesOutput {
    /// <p>A collection of objects containing attribute metadata for each requested fleet ID. Attribute objects are returned only for fleets that currently exist.</p>
    #[serde(rename = "FleetAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_attributes: Option<Vec<FleetAttributes>>,
    /// <p>Token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeFleetCapacityInput {
    /// <p>A unique identifier for a fleet(s) to retrieve capacity information for. You can use either the fleet ID or ARN value.</p>
    #[serde(rename = "FleetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_ids: Option<Vec<String>>,
    /// <p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages. This parameter is ignored when the request specifies one or a list of fleet IDs.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value. This parameter is ignored when the request specifies one or a list of fleet IDs.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeFleetCapacityOutput {
    /// <p>A collection of objects containing capacity information for each requested fleet ID. Leave this parameter empty to retrieve capacity information for all fleets.</p>
    #[serde(rename = "FleetCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_capacity: Option<Vec<FleetCapacity>>,
    /// <p>Token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeFleetEventsInput {
    /// <p>Most recent date to retrieve event logs for. If no end time is specified, this call returns entries from the specified start time up to the present. Format is a number expressed in Unix time as milliseconds (ex: "1469498468.057").</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>A unique identifier for a fleet to get event logs for. You can use either the fleet ID or ARN value.</p>
    #[serde(rename = "FleetId")]
    pub fleet_id: String,
    /// <p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Earliest date to retrieve event logs for. If no start time is specified, this call returns entries starting from when the fleet was created to the specified end time. Format is a number expressed in Unix time as milliseconds (ex: "1469498468.057").</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeFleetEventsOutput {
    /// <p>A collection of objects containing event log entries for the specified fleet.</p>
    #[serde(rename = "Events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
    /// <p>Token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeFleetPortSettingsInput {
    /// <p>A unique identifier for a fleet to retrieve port settings for. You can use either the fleet ID or ARN value.</p>
    #[serde(rename = "FleetId")]
    pub fleet_id: String,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeFleetPortSettingsOutput {
    /// <p>The port settings for the requested fleet ID.</p>
    #[serde(rename = "InboundPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_permissions: Option<Vec<IpPermission>>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeFleetUtilizationInput {
    /// <p>A unique identifier for a fleet(s) to retrieve utilization data for. You can use either the fleet ID or ARN value. To retrieve attributes for all current fleets, do not include this parameter. If the list of fleet identifiers includes fleets that don't currently exist, the request succeeds but no attributes for that fleet are returned.</p>
    #[serde(rename = "FleetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_ids: Option<Vec<String>>,
    /// <p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages. This parameter is ignored when the request specifies one or a list of fleet IDs.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value. This parameter is ignored when the request specifies one or a list of fleet IDs.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeFleetUtilizationOutput {
    /// <p>A collection of objects containing utilization information for each requested fleet ID.</p>
    #[serde(rename = "FleetUtilization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_utilization: Option<Vec<FleetUtilization>>,
    /// <p>Token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeGameServerGroupInput {
    /// <p>A unique identifier for the game server group. Use either the <a>GameServerGroup</a> name or ARN value.</p>
    #[serde(rename = "GameServerGroupName")]
    pub game_server_group_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeGameServerGroupOutput {
    /// <p>An object with the property settings for the requested game server group resource. </p>
    #[serde(rename = "GameServerGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_server_group: Option<GameServerGroup>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeGameServerInput {
    /// <p>A unique identifier for the game server group where the game server is running. Use either the <a>GameServerGroup</a> name or ARN value.</p>
    #[serde(rename = "GameServerGroupName")]
    pub game_server_group_name: String,
    /// <p>A custom string that uniquely identifies the game server information to be retrieved.</p>
    #[serde(rename = "GameServerId")]
    pub game_server_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeGameServerInstancesInput {
    /// <p>A unique identifier for the game server group. Use either the <a>GameServerGroup</a> name or ARN value.</p>
    #[serde(rename = "GameServerGroupName")]
    pub game_server_group_name: String,
    /// <p>The EC2 instance IDs that you want to retrieve status on. EC2 instance IDs use a 17-character format, for example: <code>i-1234567890abcdef0</code>. To retrieve all instances in the game server group, leave this parameter empty. </p>
    #[serde(rename = "InstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
    /// <p> The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential segments. </p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p> A token that indicates the start of the next sequential segment of results. Use the token returned with the previous call to this operation. To start at the beginning of the result set, do not specify a value. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeGameServerInstancesOutput {
    /// <p> The collection of requested game server instances. </p>
    #[serde(rename = "GameServerInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_server_instances: Option<Vec<GameServerInstance>>,
    /// <p> A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeGameServerOutput {
    /// <p>Object that describes the requested game server.</p>
    #[serde(rename = "GameServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_server: Option<GameServer>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeGameSessionDetailsInput {
    /// <p>A unique identifier for an alias associated with the fleet to retrieve all game sessions for. You can use either the alias ID or ARN value.</p>
    #[serde(rename = "AliasId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_id: Option<String>,
    /// <p>A unique identifier for a fleet to retrieve all game sessions active on the fleet. You can use either the fleet ID or ARN value.</p>
    #[serde(rename = "FleetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_id: Option<String>,
    /// <p>A unique identifier for the game session to retrieve. </p>
    #[serde(rename = "GameSessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_id: Option<String>,
    /// <p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Game session status to filter results on. Possible game session statuses include <code>ACTIVE</code>, <code>TERMINATED</code>, <code>ACTIVATING</code> and <code>TERMINATING</code> (the last two are transitory). </p>
    #[serde(rename = "StatusFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_filter: Option<String>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeGameSessionDetailsOutput {
    /// <p>A collection of objects containing game session properties and the protection policy currently in force for each session matching the request.</p>
    #[serde(rename = "GameSessionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_details: Option<Vec<GameSessionDetail>>,
    /// <p>Token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeGameSessionPlacementInput {
    /// <p>A unique identifier for a game session placement to retrieve.</p>
    #[serde(rename = "PlacementId")]
    pub placement_id: String,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeGameSessionPlacementOutput {
    /// <p>Object that describes the requested game session placement.</p>
    #[serde(rename = "GameSessionPlacement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_placement: Option<GameSessionPlacement>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeGameSessionQueuesInput {
    /// <p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages. You can request up to 50 results.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>A list of queue names to retrieve information for. You can use either the queue ID or ARN value. To request settings for all queues, leave this parameter empty. </p>
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// <p>A token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeGameSessionQueuesOutput {
    /// <p>A collection of objects that describe the requested game session queues.</p>
    #[serde(rename = "GameSessionQueues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_queues: Option<Vec<GameSessionQueue>>,
    /// <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeGameSessionsInput {
    /// <p>A unique identifier for an alias associated with the fleet to retrieve all game sessions for. You can use either the alias ID or ARN value.</p>
    #[serde(rename = "AliasId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_id: Option<String>,
    /// <p>A unique identifier for a fleet to retrieve all game sessions for. You can use either the fleet ID or ARN value. </p>
    #[serde(rename = "FleetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_id: Option<String>,
    /// <p>A unique identifier for the game session to retrieve. </p>
    #[serde(rename = "GameSessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_id: Option<String>,
    /// <p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Game session status to filter results on. Possible game session statuses include <code>ACTIVE</code>, <code>TERMINATED</code>, <code>ACTIVATING</code>, and <code>TERMINATING</code> (the last two are transitory). </p>
    #[serde(rename = "StatusFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_filter: Option<String>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeGameSessionsOutput {
    /// <p>A collection of objects containing game session properties for each session matching the request.</p>
    #[serde(rename = "GameSessions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_sessions: Option<Vec<GameSession>>,
    /// <p>Token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeInstancesInput {
    /// <p>A unique identifier for a fleet to retrieve instance information for. You can use either the fleet ID or ARN value.</p>
    #[serde(rename = "FleetId")]
    pub fleet_id: String,
    /// <p>A unique identifier for an instance to retrieve. Specify an instance ID or leave blank to retrieve all instances in the fleet.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeInstancesOutput {
    /// <p>A collection of objects containing properties for each instance returned.</p>
    #[serde(rename = "Instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<Instance>>,
    /// <p>Token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeMatchmakingConfigurationsInput {
    /// <p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages. This parameter is limited to 10.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>A unique identifier for a matchmaking configuration(s) to retrieve. You can use either the configuration name or ARN value. To request all existing configurations, leave this parameter empty.</p>
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// <p>A token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A unique identifier for a matchmaking rule set. You can use either the rule set name or ARN value. Use this parameter to retrieve all matchmaking configurations that use this rule set.</p>
    #[serde(rename = "RuleSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_set_name: Option<String>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeMatchmakingConfigurationsOutput {
    /// <p>A collection of requested matchmaking configurations.</p>
    #[serde(rename = "Configurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<MatchmakingConfiguration>>,
    /// <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeMatchmakingInput {
    /// <p>A unique identifier for a matchmaking ticket. You can include up to 10 ID values. </p>
    #[serde(rename = "TicketIds")]
    pub ticket_ids: Vec<String>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeMatchmakingOutput {
    /// <p>A collection of existing matchmaking ticket objects matching the request.</p>
    #[serde(rename = "TicketList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket_list: Option<Vec<MatchmakingTicket>>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeMatchmakingRuleSetsInput {
    /// <p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>A list of one or more matchmaking rule set names to retrieve details for. (Note: The rule set name is different from the optional "name" field in the rule set body.) You can use either the rule set name or ARN value. </p>
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// <p>A token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeMatchmakingRuleSetsOutput {
    /// <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A collection of requested matchmaking rule set objects. </p>
    #[serde(rename = "RuleSets")]
    pub rule_sets: Vec<MatchmakingRuleSet>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribePlayerSessionsInput {
    /// <p>A unique identifier for the game session to retrieve player sessions for.</p>
    #[serde(rename = "GameSessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_id: Option<String>,
    /// <p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages. If a player session ID is specified, this parameter is ignored.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value. If a player session ID is specified, this parameter is ignored.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A unique identifier for a player to retrieve player sessions for.</p>
    #[serde(rename = "PlayerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_id: Option<String>,
    /// <p>A unique identifier for a player session to retrieve.</p>
    #[serde(rename = "PlayerSessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_session_id: Option<String>,
    /// <p><p>Player session status to filter results on.</p> <p>Possible player session statuses include the following:</p> <ul> <li> <p> <b>RESERVED</b> -- The player session request has been received, but the player has not yet connected to the server process and/or been validated. </p> </li> <li> <p> <b>ACTIVE</b> -- The player has been validated by the server process and is currently connected.</p> </li> <li> <p> <b>COMPLETED</b> -- The player connection has been dropped.</p> </li> <li> <p> <b>TIMEDOUT</b> -- A player session request was received, but the player did not connect and/or was not validated within the timeout limit (60 seconds).</p> </li> </ul></p>
    #[serde(rename = "PlayerSessionStatusFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_session_status_filter: Option<String>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribePlayerSessionsOutput {
    /// <p>Token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A collection of objects containing properties for each player session that matches the request.</p>
    #[serde(rename = "PlayerSessions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_sessions: Option<Vec<PlayerSession>>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRuntimeConfigurationInput {
    /// <p>A unique identifier for a fleet to get the runtime configuration for. You can use either the fleet ID or ARN value.</p>
    #[serde(rename = "FleetId")]
    pub fleet_id: String,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeRuntimeConfigurationOutput {
    /// <p>Instructions describing how server processes should be launched and maintained on each instance in the fleet.</p>
    #[serde(rename = "RuntimeConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_configuration: Option<RuntimeConfiguration>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeScalingPoliciesInput {
    /// <p>A unique identifier for a fleet to retrieve scaling policies for. You can use either the fleet ID or ARN value.</p>
    #[serde(rename = "FleetId")]
    pub fleet_id: String,
    /// <p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>Scaling policy status to filter results on. A scaling policy is only in force when in an <code>ACTIVE</code> status.</p> <ul> <li> <p> <b>ACTIVE</b> -- The scaling policy is currently in force.</p> </li> <li> <p> <b>UPDATEREQUESTED</b> -- A request to update the scaling policy has been received.</p> </li> <li> <p> <b>UPDATING</b> -- A change is being made to the scaling policy.</p> </li> <li> <p> <b>DELETEREQUESTED</b> -- A request to delete the scaling policy has been received.</p> </li> <li> <p> <b>DELETING</b> -- The scaling policy is being deleted.</p> </li> <li> <p> <b>DELETED</b> -- The scaling policy has been deleted.</p> </li> <li> <p> <b>ERROR</b> -- An error occurred in creating the policy. It should be removed and recreated.</p> </li> </ul></p>
    #[serde(rename = "StatusFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_filter: Option<String>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeScalingPoliciesOutput {
    /// <p>Token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A collection of objects containing the scaling policies matching the request.</p>
    #[serde(rename = "ScalingPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_policies: Option<Vec<ScalingPolicy>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeScriptInput {
    /// <p>A unique identifier for a Realtime script to retrieve properties for. You can use either the script ID or ARN value.</p>
    #[serde(rename = "ScriptId")]
    pub script_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeScriptOutput {
    /// <p>A set of properties describing the requested script.</p>
    #[serde(rename = "Script")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<Script>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeVpcPeeringAuthorizationsInput {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeVpcPeeringAuthorizationsOutput {
    /// <p>A collection of objects that describe all valid VPC peering operations for the current AWS account.</p>
    #[serde(rename = "VpcPeeringAuthorizations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_peering_authorizations: Option<Vec<VpcPeeringAuthorization>>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeVpcPeeringConnectionsInput {
    /// <p>A unique identifier for a fleet. You can use either the fleet ID or ARN value.</p>
    #[serde(rename = "FleetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_id: Option<String>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeVpcPeeringConnectionsOutput {
    /// <p>A collection of VPC peering connection records that match the request.</p>
    #[serde(rename = "VpcPeeringConnections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_peering_connections: Option<Vec<VpcPeeringConnection>>,
}

/// <p>Player information for use when creating player sessions using a game session placement request with <a>StartGameSessionPlacement</a>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DesiredPlayerSession {
    /// <p>Developer-defined information related to a player. Amazon GameLift does not use this data, so it can be formatted as needed for use in the game.</p>
    #[serde(rename = "PlayerData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_data: Option<String>,
    /// <p>A unique identifier for a player to associate with the player session.</p>
    #[serde(rename = "PlayerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_id: Option<String>,
}

/// <p><p>Current status of fleet capacity. The number of active instances should match or be in the process of matching the number of desired instances. Pending and terminating counts are non-zero only if fleet capacity is adjusting to an <a>UpdateFleetCapacity</a> request, or if access to resources is temporarily affected.</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EC2InstanceCounts {
    /// <p>Actual number of active instances in the fleet.</p>
    #[serde(rename = "ACTIVE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<i64>,
    /// <p>Ideal number of active instances in the fleet.</p>
    #[serde(rename = "DESIRED")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired: Option<i64>,
    /// <p>Number of active instances in the fleet that are not currently hosting a game session.</p>
    #[serde(rename = "IDLE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle: Option<i64>,
    /// <p>The maximum value allowed for the fleet's instance count.</p>
    #[serde(rename = "MAXIMUM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    /// <p>The minimum value allowed for the fleet's instance count.</p>
    #[serde(rename = "MINIMUM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
    /// <p>Number of instances in the fleet that are starting but not yet active.</p>
    #[serde(rename = "PENDING")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<i64>,
    /// <p>Number of instances in the fleet that are no longer active but haven't yet been terminated.</p>
    #[serde(rename = "TERMINATING")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminating: Option<i64>,
}

/// <p>The maximum number of instances allowed based on the Amazon Elastic Compute Cloud (Amazon EC2) instance type. Instance limits can be retrieved by calling <a>DescribeEC2InstanceLimits</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EC2InstanceLimit {
    /// <p>Number of instances of the specified type that are currently in use by this AWS account.</p>
    #[serde(rename = "CurrentInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_instances: Option<i64>,
    /// <p>Name of an EC2 instance type that is supported in Amazon GameLift. A fleet instance type determines the computing resources of each instance in the fleet, including CPU, memory, storage, and networking capacity. Amazon GameLift supports the following EC2 instance types. See <a href="http://aws.amazon.com/ec2/instance-types/">Amazon EC2 Instance Types</a> for detailed descriptions.</p>
    #[serde(rename = "EC2InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_type: Option<String>,
    /// <p>Number of instances allowed.</p>
    #[serde(rename = "InstanceLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_limit: Option<i64>,
}

/// <p>Log entry describing an event that involves Amazon GameLift resources (such as a fleet). In addition to tracking activity, event codes and messages can provide additional information for troubleshooting and debugging problems.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Event {
    /// <p><p>The type of event being logged. </p> <p> <b>Fleet creation events (ordered by fleet creation activity):</b> </p> <ul> <li> <p>FLEET<em>CREATED -- A fleet resource was successfully created with a status of <code>NEW</code>. Event messaging includes the fleet ID.</p> </li> <li> <p>FLEET</em>STATE<em>DOWNLOADING -- Fleet status changed from <code>NEW</code> to <code>DOWNLOADING</code>. The compressed build has started downloading to a fleet instance for installation.</p> </li> <li> <p> FLEET</em>BINARY<em>DOWNLOAD</em>FAILED -- The build failed to download to the fleet instance.</p> </li> <li> <p>FLEET<em>CREATION</em>EXTRACTING<em>BUILD – The game server build was successfully downloaded to an instance, and the build files are now being extracted from the uploaded build and saved to an instance. Failure at this stage prevents a fleet from moving to <code>ACTIVE</code> status. Logs for this stage display a list of the files that are extracted and saved on the instance. Access the logs by using the URL in <i>PreSignedLogUrl</i>.</p> </li> <li> <p>FLEET</em>CREATION<em>RUNNING</em>INSTALLER – The game server build files were successfully extracted, and the Amazon GameLift is now running the build&#39;s install script (if one is included). Failure in this stage prevents a fleet from moving to <code>ACTIVE</code> status. Logs for this stage list the installation steps and whether or not the install completed successfully. Access the logs by using the URL in <i>PreSignedLogUrl</i>. </p> </li> <li> <p>FLEET<em>CREATION</em>VALIDATING<em>RUNTIME</em>CONFIG -- The build process was successful, and the Amazon GameLift is now verifying that the game server launch paths, which are specified in the fleet&#39;s runtime configuration, exist. If any listed launch path exists, Amazon GameLift tries to launch a game server process and waits for the process to report ready. Failures in this stage prevent a fleet from moving to <code>ACTIVE</code> status. Logs for this stage list the launch paths in the runtime configuration and indicate whether each is found. Access the logs by using the URL in <i>PreSignedLogUrl</i>. </p> </li> <li> <p>FLEET<em>STATE</em>VALIDATING -- Fleet status changed from <code>DOWNLOADING</code> to <code>VALIDATING</code>.</p> </li> <li> <p> FLEET<em>VALIDATION</em>LAUNCH<em>PATH</em>NOT<em>FOUND -- Validation of the runtime configuration failed because the executable specified in a launch path does not exist on the instance.</p> </li> <li> <p>FLEET</em>STATE<em>BUILDING -- Fleet status changed from <code>VALIDATING</code> to <code>BUILDING</code>.</p> </li> <li> <p>FLEET</em>VALIDATION<em>EXECUTABLE</em>RUNTIME<em>FAILURE -- Validation of the runtime configuration failed because the executable specified in a launch path failed to run on the fleet instance.</p> </li> <li> <p>FLEET</em>STATE<em>ACTIVATING -- Fleet status changed from <code>BUILDING</code> to <code>ACTIVATING</code>. </p> </li> <li> <p> FLEET</em>ACTIVATION<em>FAILED - The fleet failed to successfully complete one of the steps in the fleet activation process. This event code indicates that the game build was successfully downloaded to a fleet instance, built, and validated, but was not able to start a server process. Learn more at <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-creating-debug.html#fleets-creating-debug-creation"> Debug Fleet Creation Issues</a> </p> </li> <li> <p>FLEET</em>STATE<em>ACTIVE -- The fleet&#39;s status changed from <code>ACTIVATING</code> to <code>ACTIVE</code>. The fleet is now ready to host game sessions.</p> </li> </ul> <p> <b>VPC peering events:</b> </p> <ul> <li> <p>FLEET</em>VPC<em>PEERING</em>SUCCEEDED -- A VPC peering connection has been established between the VPC for an Amazon GameLift fleet and a VPC in your AWS account.</p> </li> <li> <p>FLEET<em>VPC</em>PEERING<em>FAILED -- A requested VPC peering connection has failed. Event details and status information (see <a>DescribeVpcPeeringConnections</a>) provide additional detail. A common reason for peering failure is that the two VPCs have overlapping CIDR blocks of IPv4 addresses. To resolve this, change the CIDR block for the VPC in your AWS account. For more information on VPC peering failures, see <a href="https://docs.aws.amazon.com/AmazonVPC/latest/PeeringGuide/invalid-peering-configurations.html">https://docs.aws.amazon.com/AmazonVPC/latest/PeeringGuide/invalid-peering-configurations.html</a> </p> </li> <li> <p>FLEET</em>VPC<em>PEERING</em>DELETED -- A VPC peering connection has been successfully deleted.</p> </li> </ul> <p> <b>Spot instance events:</b> </p> <ul> <li> <p> INSTANCE<em>INTERRUPTED -- A spot instance was interrupted by EC2 with a two-minute notification.</p> </li> </ul> <p> <b>Other fleet events:</b> </p> <ul> <li> <p>FLEET</em>SCALING<em>EVENT -- A change was made to the fleet&#39;s capacity settings (desired instances, minimum/maximum scaling limits). Event messaging includes the new capacity settings.</p> </li> <li> <p>FLEET</em>NEW<em>GAME</em>SESSION<em>PROTECTION</em>POLICY<em>UPDATED -- A change was made to the fleet&#39;s game session protection policy setting. Event messaging includes both the old and new policy setting. </p> </li> <li> <p>FLEET</em>DELETED -- A request to delete a fleet was initiated.</p> </li> <li> <p> GENERIC_EVENT -- An unspecified event has occurred.</p> </li> </ul></p>
    #[serde(rename = "EventCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_code: Option<String>,
    /// <p>A unique identifier for a fleet event.</p>
    #[serde(rename = "EventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// <p>Time stamp indicating when this event occurred. Format is a number expressed in Unix time as milliseconds (for example "1469498468.057").</p>
    #[serde(rename = "EventTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<f64>,
    /// <p>Additional information related to the event.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>Location of stored logs with additional detail that is related to the event. This is useful for debugging issues. The URL is valid for 15 minutes. You can also access fleet creation logs through the Amazon GameLift console.</p>
    #[serde(rename = "PreSignedLogUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_signed_log_url: Option<String>,
    /// <p>A unique identifier for an event resource, such as a fleet ID.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

/// <p><p>General properties describing a fleet.</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FleetAttributes {
    /// <p> The Amazon Resource Name (<a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a>) associated with the GameLift build resource that is deployed on instances in this fleet. In a GameLift build ARN, the resource ID matches the <i>BuildId</i> value.</p>
    #[serde(rename = "BuildArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_arn: Option<String>,
    /// <p>A unique identifier for a build.</p>
    #[serde(rename = "BuildId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_id: Option<String>,
    /// <p>Indicates whether a TLS/SSL certificate was generated for the fleet. </p>
    #[serde(rename = "CertificateConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_configuration: Option<CertificateConfiguration>,
    /// <p>Time stamp indicating when this data object was created. Format is a number expressed in Unix time as milliseconds (for example "1469498468.057").</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>Human-readable description of the fleet.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a>) that is assigned to a GameLift fleet resource and uniquely identifies it. ARNs are unique across all Regions. In a GameLift fleet ARN, the resource ID matches the <i>FleetId</i> value.</p>
    #[serde(rename = "FleetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_arn: Option<String>,
    /// <p>A unique identifier for a fleet.</p>
    #[serde(rename = "FleetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_id: Option<String>,
    /// <p>Indicates whether the fleet uses on-demand or spot instances. A spot instance in use may be interrupted with a two-minute notification.</p>
    #[serde(rename = "FleetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_type: Option<String>,
    /// <p>A unique identifier for an AWS IAM role that manages access to your AWS services.</p>
    #[serde(rename = "InstanceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_role_arn: Option<String>,
    /// <p>EC2 instance type indicating the computing resources of each instance in the fleet, including CPU, memory, storage, and networking capacity. See <a href="http://aws.amazon.com/ec2/instance-types/">Amazon EC2 Instance Types</a> for detailed descriptions.</p>
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>Location of default log files. When a server process is shut down, Amazon GameLift captures and stores any log files in this location. These logs are in addition to game session logs; see more on game session logs in the <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-api-server-code">Amazon GameLift Developer Guide</a>. If no default log path for a fleet is specified, Amazon GameLift automatically uploads logs that are stored on each instance at <code>C:\game\logs</code> (for Windows) or <code>/local/game/logs</code> (for Linux). Use the Amazon GameLift console to access stored logs. </p>
    #[serde(rename = "LogPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_paths: Option<Vec<String>>,
    /// <p>Names of metric groups that this fleet is included in. In Amazon CloudWatch, you can view metrics for an individual fleet or aggregated metrics for fleets that are in a fleet metric group. A fleet can be included in only one metric group at a time.</p>
    #[serde(rename = "MetricGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_groups: Option<Vec<String>>,
    /// <p>A descriptive label that is associated with a fleet. Fleet names do not need to be unique.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The type of game session protection to set for all new instances started in the fleet.</p> <ul> <li> <p> <b>NoProtection</b> -- The game session can be terminated during a scale-down event.</p> </li> <li> <p> <b>FullProtection</b> -- If the game session is in an <code>ACTIVE</code> status, it cannot be terminated during a scale-down event.</p> </li> </ul></p>
    #[serde(rename = "NewGameSessionProtectionPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_game_session_protection_policy: Option<String>,
    /// <p>Operating system of the fleet's computing resources. A fleet's operating system depends on the OS specified for the build that is deployed on this fleet.</p>
    #[serde(rename = "OperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    /// <p>Fleet policy to limit the number of game sessions an individual player can create over a span of time.</p>
    #[serde(rename = "ResourceCreationLimitPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_creation_limit_policy: Option<ResourceCreationLimitPolicy>,
    /// <p> The Amazon Resource Name (<a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a>) associated with the GameLift script resource that is deployed on instances in this fleet. In a GameLift script ARN, the resource ID matches the <i>ScriptId</i> value.</p>
    #[serde(rename = "ScriptArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_arn: Option<String>,
    /// <p>A unique identifier for a Realtime script.</p>
    #[serde(rename = "ScriptId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_id: Option<String>,
    /// <p>Game server launch parameters specified for fleets created before 2016-08-04 (or AWS SDK v. 0.12.16). Server launch parameters for fleets created after this date are specified in the fleet's <a>RuntimeConfiguration</a>.</p>
    #[serde(rename = "ServerLaunchParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_launch_parameters: Option<String>,
    /// <p>Path to a game server executable in the fleet's build, specified for fleets created before 2016-08-04 (or AWS SDK v. 0.12.16). Server launch paths for fleets created after this date are specified in the fleet's <a>RuntimeConfiguration</a>.</p>
    #[serde(rename = "ServerLaunchPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_launch_path: Option<String>,
    /// <p><p>Current status of the fleet.</p> <p>Possible fleet statuses include the following:</p> <ul> <li> <p> <b>NEW</b> -- A new fleet has been defined and desired instances is set to 1. </p> </li> <li> <p> <b>DOWNLOADING/VALIDATING/BUILDING/ACTIVATING</b> -- Amazon GameLift is setting up the new fleet, creating new instances with the game build or Realtime script and starting server processes.</p> </li> <li> <p> <b>ACTIVE</b> -- Hosts can now accept game sessions.</p> </li> <li> <p> <b>ERROR</b> -- An error occurred when downloading, validating, building, or activating the fleet.</p> </li> <li> <p> <b>DELETING</b> -- Hosts are responding to a delete fleet request.</p> </li> <li> <p> <b>TERMINATED</b> -- The fleet no longer exists.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>List of fleet activity that have been suspended using <a>StopFleetActions</a>. This includes auto-scaling.</p>
    #[serde(rename = "StoppedActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_actions: Option<Vec<String>>,
    /// <p>Time stamp indicating when this data object was terminated. Format is a number expressed in Unix time as milliseconds (for example "1469498468.057").</p>
    #[serde(rename = "TerminationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_time: Option<f64>,
}

/// <p><p>Information about the fleet&#39;s capacity. Fleet capacity is measured in EC2 instances. By default, new fleets have a capacity of one instance, but can be updated as needed. The maximum number of instances for a fleet is determined by the fleet&#39;s instance type.</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FleetCapacity {
    /// <p>A unique identifier for a fleet.</p>
    #[serde(rename = "FleetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_id: Option<String>,
    /// <p>Current status of fleet capacity.</p>
    #[serde(rename = "InstanceCounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_counts: Option<EC2InstanceCounts>,
    /// <p>Name of an EC2 instance type that is supported in Amazon GameLift. A fleet instance type determines the computing resources of each instance in the fleet, including CPU, memory, storage, and networking capacity. Amazon GameLift supports the following EC2 instance types. See <a href="http://aws.amazon.com/ec2/instance-types/">Amazon EC2 Instance Types</a> for detailed descriptions.</p>
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
}

/// <p><p>Current status of fleet utilization, including the number of game and player sessions being hosted.</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FleetUtilization {
    /// <p>Number of active game sessions currently being hosted on all instances in the fleet.</p>
    #[serde(rename = "ActiveGameSessionCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_game_session_count: Option<i64>,
    /// <p>Number of server processes in an <code>ACTIVE</code> status currently running across all instances in the fleet</p>
    #[serde(rename = "ActiveServerProcessCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_server_process_count: Option<i64>,
    /// <p>Number of active player sessions currently being hosted on all instances in the fleet.</p>
    #[serde(rename = "CurrentPlayerSessionCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_player_session_count: Option<i64>,
    /// <p>A unique identifier for a fleet.</p>
    #[serde(rename = "FleetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_id: Option<String>,
    /// <p>The maximum number of players allowed across all game sessions currently being hosted on all instances in the fleet.</p>
    #[serde(rename = "MaximumPlayerSessionCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_player_session_count: Option<i64>,
}

/// <p>Set of key-value pairs that contain information about a game session. When included in a game session request, these properties communicate details to be used when setting up the new game session. For example, a game property might specify a game mode, level, or map. Game properties are passed to the game server process when initiating a new game session. For more information, see the <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-client-api.html#gamelift-sdk-client-api-create"> Amazon GameLift Developer Guide</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GameProperty {
    /// <p>The game property identifier.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The game property value.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p><p> <b>This data type is used with the Amazon GameLift FleetIQ and game server groups.</b> </p> <p>Properties describing a game server that is running on an instance in a <a>GameServerGroup</a>. </p> <p>A game server is created by a successful call to <code>RegisterGameServer</code> and deleted by calling <code>DeregisterGameServer</code>. A game server is claimed to host a game session by calling <code>ClaimGameServer</code>. </p> <ul> <li> <p> <a>RegisterGameServer</a> </p> </li> <li> <p> <a>ListGameServers</a> </p> </li> <li> <p> <a>ClaimGameServer</a> </p> </li> <li> <p> <a>DescribeGameServer</a> </p> </li> <li> <p> <a>UpdateGameServer</a> </p> </li> <li> <p> <a>DeregisterGameServer</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GameServer {
    /// <p>Indicates when an available game server has been reserved for gameplay but has not yet started hosting a game. Once it is claimed, the game server remains in <code>CLAIMED</code> status for a maximum of one minute. During this time, game clients connect to the game server to start the game and trigger the game server to update its utilization status. After one minute, the game server claim status reverts to null.</p>
    #[serde(rename = "ClaimStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim_status: Option<String>,
    /// <p>The port and IP address that must be used to establish a client connection to the game server.</p>
    #[serde(rename = "ConnectionInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_info: Option<String>,
    /// <p>A set of custom game server properties, formatted as a single string value. This data is passed to a game client or service when it requests information on game servers using <a>ListGameServers</a> or <a>ClaimGameServer</a>.</p>
    #[serde(rename = "GameServerData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_server_data: Option<String>,
    /// <p>The ARN identifier for the game server group where the game server is located.</p>
    #[serde(rename = "GameServerGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_server_group_arn: Option<String>,
    /// <p>A unique identifier for the game server group where the game server is running. Use either the <a>GameServerGroup</a> name or ARN value.</p>
    #[serde(rename = "GameServerGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_server_group_name: Option<String>,
    /// <p>A custom string that uniquely identifies the game server. Game server IDs are developer-defined and are unique across all game server groups in an AWS account.</p>
    #[serde(rename = "GameServerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_server_id: Option<String>,
    /// <p>The unique identifier for the instance where the game server is running. This ID is available in the instance metadata. EC2 instance IDs use a 17-character format, for example: <code>i-1234567890abcdef0</code>.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>Timestamp that indicates the last time the game server was claimed with a <a>ClaimGameServer</a> request. The format is a number expressed in Unix time as milliseconds (for example <code>"1469498468.057"</code>). This value is used to calculate when a claimed game server's status should revert to null.</p>
    #[serde(rename = "LastClaimTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_claim_time: Option<f64>,
    /// <p>Timestamp that indicates the last time the game server was updated with health status using an <a>UpdateGameServer</a> request. The format is a number expressed in Unix time as milliseconds (for example <code>"1469498468.057"</code>). After game server registration, this property is only changed when a game server update specifies a health check value.</p>
    #[serde(rename = "LastHealthCheckTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_health_check_time: Option<f64>,
    /// <p>Timestamp that indicates when the game server was created with a <a>RegisterGameServer</a> request. The format is a number expressed in Unix time as milliseconds (for example <code>"1469498468.057"</code>).</p>
    #[serde(rename = "RegistrationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_time: Option<f64>,
    /// <p><p>Indicates whether the game server is currently available for new games or is busy. Possible statuses include:</p> <ul> <li> <p> <code>AVAILABLE</code> - The game server is available to be claimed. A game server that has been claimed remains in this status until it reports game hosting activity. </p> </li> <li> <p> <code>UTILIZED</code> - The game server is currently hosting a game session with players. </p> </li> </ul></p>
    #[serde(rename = "UtilizationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilization_status: Option<String>,
}

/// <p><p> <b>This data type is used with the Amazon GameLift FleetIQ and game server groups.</b> </p> <p>Properties that describe a game server group resource. A game server group manages certain properties related to a corresponding EC2 Auto Scaling group. </p> <p>A game server group is created by a successful call to <code>CreateGameServerGroup</code> and deleted by calling <code>DeleteGameServerGroup</code>. Game server group activity can be temporarily suspended and resumed by calling <code>SuspendGameServerGroup</code> and <code>ResumeGameServerGroup</code>, respectively. </p> <ul> <li> <p> <a>CreateGameServerGroup</a> </p> </li> <li> <p> <a>ListGameServerGroups</a> </p> </li> <li> <p> <a>DescribeGameServerGroup</a> </p> </li> <li> <p> <a>UpdateGameServerGroup</a> </p> </li> <li> <p> <a>DeleteGameServerGroup</a> </p> </li> <li> <p> <a>ResumeGameServerGroup</a> </p> </li> <li> <p> <a>SuspendGameServerGroup</a> </p> </li> <li> <p> <a>DescribeGameServerInstances</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GameServerGroup {
    /// <p>A generated unique ID for the EC2 Auto Scaling group that is associated with this game server group.</p>
    #[serde(rename = "AutoScalingGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group_arn: Option<String>,
    /// <p><p>Indicates how GameLift FleetIQ balances the use of Spot Instances and On-Demand Instances in the game server group. Method options include the following:</p> <ul> <li> <p> <code>SPOT<em>ONLY</code> - Only Spot Instances are used in the game server group. If Spot Instances are unavailable or not viable for game hosting, the game server group provides no hosting capacity until Spot Instances can again be used. Until then, no new instances are started, and the existing nonviable Spot Instances are terminated (after current gameplay ends) and are not replaced.</p> </li> <li> <p> <code>SPOT</em>PREFERRED</code> - (default value) Spot Instances are used whenever available in the game server group. If Spot Instances are unavailable, the game server group continues to provide hosting capacity by falling back to On-Demand Instances. Existing nonviable Spot Instances are terminated (after current gameplay ends) and are replaced with new On-Demand Instances.</p> </li> <li> <p> <code>ON<em>DEMAND</em>ONLY</code> - Only On-Demand Instances are used in the game server group. No Spot Instances are used, even when available, while this balancing strategy is in force.</p> </li> </ul></p>
    #[serde(rename = "BalancingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balancing_strategy: Option<String>,
    /// <p>A timestamp that indicates when this data object was created. Format is a number expressed in Unix time as milliseconds (for example <code>"1469498468.057"</code>).</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>A generated unique ID for the game server group.</p>
    #[serde(rename = "GameServerGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_server_group_arn: Option<String>,
    /// <p>A developer-defined identifier for the game server group. The name is unique for each Region in each AWS account.</p>
    #[serde(rename = "GameServerGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_server_group_name: Option<String>,
    /// <p>A flag that indicates whether instances in the game server group are protected from early termination. Unprotected instances that have active game servers running might be terminated during a scale-down event, causing players to be dropped from the game. Protected instances cannot be terminated while there are active game servers running except in the event of a forced game server group deletion (see ). An exception to this is with Spot Instances, which can be terminated by AWS regardless of protection status. </p>
    #[serde(rename = "GameServerProtectionPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_server_protection_policy: Option<String>,
    /// <p>The set of EC2 instance types that GameLift FleetIQ can use when balancing and automatically scaling instances in the corresponding Auto Scaling group. </p>
    #[serde(rename = "InstanceDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_definitions: Option<Vec<InstanceDefinition>>,
    /// <p>A timestamp that indicates when this game server group was last updated.</p>
    #[serde(rename = "LastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) for an IAM role that allows Amazon GameLift to access your EC2 Auto Scaling groups.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p><p>The current status of the game server group. Possible statuses include:</p> <ul> <li> <p> <code>NEW</code> - GameLift FleetIQ has validated the <code>CreateGameServerGroup()</code> request. </p> </li> <li> <p> <code>ACTIVATING</code> - GameLift FleetIQ is setting up a game server group, which includes creating an Auto Scaling group in your AWS account. </p> </li> <li> <p> <code>ACTIVE</code> - The game server group has been successfully created. </p> </li> <li> <p> <code>DELETE_SCHEDULED</code> - A request to delete the game server group has been received. </p> </li> <li> <p> <code>DELETING</code> - GameLift FleetIQ has received a valid <code>DeleteGameServerGroup()</code> request and is processing it. GameLift FleetIQ must first complete and release hosts before it deletes the Auto Scaling group and the game server group. </p> </li> <li> <p> <code>DELETED</code> - The game server group has been successfully deleted. </p> </li> <li> <p> <code>ERROR</code> - The asynchronous processes of activating or deleting a game server group has failed, resulting in an error state.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Additional information about the current game server group status. This information might provide additional insight on groups that are in <code>ERROR</code> status.</p>
    #[serde(rename = "StatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    /// <p>A list of activities that are currently suspended for this game server group. If this property is empty, all activities are occurring.</p>
    #[serde(rename = "SuspendedActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspended_actions: Option<Vec<String>>,
}

/// <p> <b>This data type is used with the Amazon GameLift FleetIQ and game server groups.</b> </p> <p>Configuration settings for intelligent automatic scaling that uses target tracking. These settings are used to add an Auto Scaling policy when creating the corresponding Auto Scaling group with <a>CreateGameServerGroup</a>. After the Auto Scaling group is created, all updates to Auto Scaling policies, including changing this policy and adding or removing other policies, is done directly on the Auto Scaling group. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GameServerGroupAutoScalingPolicy {
    /// <p>Length of time, in seconds, it takes for a new instance to start new game server processes and register with GameLift FleetIQ. Specifying a warm-up time can be useful, particularly with game servers that take a long time to start up, because it avoids prematurely starting new instances. </p>
    #[serde(rename = "EstimatedInstanceWarmup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_instance_warmup: Option<i64>,
    /// <p>Settings for a target-based scaling policy applied to Auto Scaling group. These settings are used to create a target-based policy that tracks the GameLift FleetIQ metric <code>"PercentUtilizedGameServers"</code> and specifies a target value for the metric. As player usage changes, the policy triggers to adjust the game server group capacity so that the metric returns to the target value. </p>
    #[serde(rename = "TargetTrackingConfiguration")]
    pub target_tracking_configuration: TargetTrackingConfiguration,
}

/// <p><p> <b>This data type is used with the Amazon GameLift FleetIQ and game server groups.</b> </p> <p> Additional properties, including status, that describe an EC2 instance in a game server group. Instance configurations are set with game server group properties (see <code>DescribeGameServerGroup</code> and with the EC2 launch template that was used when creating the game server group. </p> <p>Retrieve game server instances for a game server group by calling <code>DescribeGameServerInstances</code>. </p> <ul> <li> <p> <a>CreateGameServerGroup</a> </p> </li> <li> <p> <a>ListGameServerGroups</a> </p> </li> <li> <p> <a>DescribeGameServerGroup</a> </p> </li> <li> <p> <a>UpdateGameServerGroup</a> </p> </li> <li> <p> <a>DeleteGameServerGroup</a> </p> </li> <li> <p> <a>ResumeGameServerGroup</a> </p> </li> <li> <p> <a>SuspendGameServerGroup</a> </p> </li> <li> <p> <a>DescribeGameServerInstances</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GameServerInstance {
    /// <p>A generated unique identifier for the game server group that includes the game server instance. </p>
    #[serde(rename = "GameServerGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_server_group_arn: Option<String>,
    /// <p>A developer-defined identifier for the game server group that includes the game server instance. The name is unique for each Region in each AWS account.</p>
    #[serde(rename = "GameServerGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_server_group_name: Option<String>,
    /// <p>The unique identifier for the instance where the game server is running. This ID is available in the instance metadata. EC2 instance IDs use a 17-character format, for example: <code>i-1234567890abcdef0</code>.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p><p> Current status of the game server instance. </p> <ul> <li> <p> <b>ACTIVE</b> -- The instance is viable for hosting game servers. </p> </li> <li> <p> <b>DRAINING</b> -- The instance is not viable for hosting game servers. Existing game servers are in the process of ending, and new game servers are not started on this instance unless no other resources are available. When the instance is put in DRAINING, a new instance is started up to replace it. Once the instance has no UTILIZED game servers, it will be terminated in favor of the new instance.</p> </li> <li> <p> <b>SPOT_TERMINATING</b> -- The instance is in the process of shutting down due to a Spot instance interruption. No new game servers are started on this instance.</p> </li> </ul></p>
    #[serde(rename = "InstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_status: Option<String>,
}

/// <p><p>Properties describing a game session.</p> <p>A game session in ACTIVE status can host players. When a game session ends, its status is set to <code>TERMINATED</code>. </p> <p>Once the session ends, the game session object is retained for 30 days. This means you can reuse idempotency token values after this time. Game session logs are retained for 14 days.</p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GameSession {
    /// <p>Time stamp indicating when this data object was created. Format is a number expressed in Unix time as milliseconds (for example "1469498468.057").</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>A unique identifier for a player. This ID is used to enforce a resource protection policy (if one exists), that limits the number of game sessions a player can create.</p>
    #[serde(rename = "CreatorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    /// <p>Number of players currently in the game session.</p>
    #[serde(rename = "CurrentPlayerSessionCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_player_session_count: Option<i64>,
    /// <p>DNS identifier assigned to the instance that is running the game session. Values have the following format:</p> <ul> <li> <p>TLS-enabled fleets: <code>&lt;unique identifier&gt;.&lt;region identifier&gt;.amazongamelift.com</code>.</p> </li> <li> <p>Non-TLS-enabled fleets: <code>ec2-&lt;unique identifier&gt;.compute.amazonaws.com</code>. (See <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-instance-addressing.html#concepts-public-addresses">Amazon EC2 Instance IP Addressing</a>.)</p> </li> </ul> <p>When connecting to a game session that is running on a TLS-enabled fleet, you must use the DNS name, not the IP address.</p>
    #[serde(rename = "DnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    /// <p> The Amazon Resource Name (<a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a>) associated with the GameLift fleet that this game session is running on. </p>
    #[serde(rename = "FleetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_arn: Option<String>,
    /// <p>A unique identifier for a fleet that the game session is running on.</p>
    #[serde(rename = "FleetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_id: Option<String>,
    /// <p>Set of custom properties for a game session, formatted as key:value pairs. These properties are passed to a game server process in the <a>GameSession</a> object with a request to start a new game session (see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession">Start a Game Session</a>). You can search for active game sessions based on this custom data with <a>SearchGameSessions</a>.</p>
    #[serde(rename = "GameProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_properties: Option<Vec<GameProperty>>,
    /// <p>Set of custom game session properties, formatted as a single string value. This data is passed to a game server process in the <a>GameSession</a> object with a request to start a new game session (see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession">Start a Game Session</a>).</p>
    #[serde(rename = "GameSessionData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_data: Option<String>,
    /// <p>A unique identifier for the game session. A game session ARN has the following format: <code>arn:aws:gamelift:&lt;region&gt;::gamesession/&lt;fleet ID&gt;/&lt;custom ID string or idempotency token&gt;</code>.</p>
    #[serde(rename = "GameSessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_id: Option<String>,
    /// <p>IP address of the instance that is running the game session. When connecting to a Amazon GameLift game server, a client needs to reference an IP address (or DNS name) and port number.</p>
    #[serde(rename = "IpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// <p>Information about the matchmaking process that was used to create the game session. It is in JSON syntax, formatted as a string. In addition the matchmaking configuration used, it contains data on all players assigned to the match, including player attributes and team assignments. For more details on matchmaker data, see <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-server.html#match-server-data">Match Data</a>. Matchmaker data is useful when requesting match backfills, and is updated whenever new players are added during a successful backfill (see <a>StartMatchBackfill</a>). </p>
    #[serde(rename = "MatchmakerData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matchmaker_data: Option<String>,
    /// <p>The maximum number of players that can be connected simultaneously to the game session.</p>
    #[serde(rename = "MaximumPlayerSessionCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_player_session_count: Option<i64>,
    /// <p>A descriptive label that is associated with a game session. Session names do not need to be unique.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Indicates whether or not the game session is accepting new players.</p>
    #[serde(rename = "PlayerSessionCreationPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_session_creation_policy: Option<String>,
    /// <p>Port number for the game session. To connect to a Amazon GameLift game server, an app needs both the IP address and port number.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>Current status of the game session. A game session must have an <code>ACTIVE</code> status to have player sessions.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Provides additional information about game session status. <code>INTERRUPTED</code> indicates that the game session was hosted on a spot instance that was reclaimed, causing the active game session to be terminated.</p>
    #[serde(rename = "StatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    /// <p>Time stamp indicating when this data object was terminated. Format is a number expressed in Unix time as milliseconds (for example "1469498468.057").</p>
    #[serde(rename = "TerminationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_time: Option<f64>,
}

/// <p>Connection information for a new game session that is created in response to a <a>StartMatchmaking</a> request. Once a match is made, the FlexMatch engine creates a new game session for it. This information, including the game session endpoint and player sessions for each player in the original matchmaking request, is added to the <a>MatchmakingTicket</a>, which can be retrieved by calling <a>DescribeMatchmaking</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GameSessionConnectionInfo {
    /// <p>DNS identifier assigned to the instance that is running the game session. Values have the following format:</p> <ul> <li> <p>TLS-enabled fleets: <code>&lt;unique identifier&gt;.&lt;region identifier&gt;.amazongamelift.com</code>.</p> </li> <li> <p>Non-TLS-enabled fleets: <code>ec2-&lt;unique identifier&gt;.compute.amazonaws.com</code>. (See <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-instance-addressing.html#concepts-public-addresses">Amazon EC2 Instance IP Addressing</a>.)</p> </li> </ul> <p>When connecting to a game session that is running on a TLS-enabled fleet, you must use the DNS name, not the IP address.</p>
    #[serde(rename = "DnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    /// <p>Amazon Resource Name (<a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a>) that is assigned to a game session and uniquely identifies it.</p>
    #[serde(rename = "GameSessionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_arn: Option<String>,
    /// <p>IP address of the instance that is running the game session. When connecting to a Amazon GameLift game server, a client needs to reference an IP address (or DNS name) and port number.</p>
    #[serde(rename = "IpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// <p>A collection of player session IDs, one for each player ID that was included in the original matchmaking request. </p>
    #[serde(rename = "MatchedPlayerSessions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_player_sessions: Option<Vec<MatchedPlayerSession>>,
    /// <p>Port number for the game session. To connect to a Amazon GameLift game server, an app needs both the IP address and port number.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}

/// <p>A game session's properties plus the protection policy currently in force.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GameSessionDetail {
    /// <p>Object that describes a game session.</p>
    #[serde(rename = "GameSession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session: Option<GameSession>,
    /// <p><p>Current status of protection for the game session.</p> <ul> <li> <p> <b>NoProtection</b> -- The game session can be terminated during a scale-down event.</p> </li> <li> <p> <b>FullProtection</b> -- If the game session is in an <code>ACTIVE</code> status, it cannot be terminated during a scale-down event.</p> </li> </ul></p>
    #[serde(rename = "ProtectionPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_policy: Option<String>,
}

/// <p><p>Object that describes a <a>StartGameSessionPlacement</a> request. This object includes the full details of the original request plus the current status and start/end time stamps.</p> <p>Game session placement-related operations include:</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GameSessionPlacement {
    /// <p>DNS identifier assigned to the instance that is running the game session. Values have the following format:</p> <ul> <li> <p>TLS-enabled fleets: <code>&lt;unique identifier&gt;.&lt;region identifier&gt;.amazongamelift.com</code>.</p> </li> <li> <p>Non-TLS-enabled fleets: <code>ec2-&lt;unique identifier&gt;.compute.amazonaws.com</code>. (See <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-instance-addressing.html#concepts-public-addresses">Amazon EC2 Instance IP Addressing</a>.)</p> </li> </ul> <p>When connecting to a game session that is running on a TLS-enabled fleet, you must use the DNS name, not the IP address.</p>
    #[serde(rename = "DnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    /// <p>Time stamp indicating when this request was completed, canceled, or timed out.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>Set of custom properties for a game session, formatted as key:value pairs. These properties are passed to a game server process in the <a>GameSession</a> object with a request to start a new game session (see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession">Start a Game Session</a>).</p>
    #[serde(rename = "GameProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_properties: Option<Vec<GameProperty>>,
    /// <p>Identifier for the game session created by this placement request. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>). This identifier is unique across all Regions. You can use this value as a <code>GameSessionId</code> value as needed.</p>
    #[serde(rename = "GameSessionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_arn: Option<String>,
    /// <p>Set of custom game session properties, formatted as a single string value. This data is passed to a game server process in the <a>GameSession</a> object with a request to start a new game session (see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession">Start a Game Session</a>).</p>
    #[serde(rename = "GameSessionData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_data: Option<String>,
    /// <p>A unique identifier for the game session. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>).</p>
    #[serde(rename = "GameSessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_id: Option<String>,
    /// <p>A descriptive label that is associated with a game session. Session names do not need to be unique.</p>
    #[serde(rename = "GameSessionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_name: Option<String>,
    /// <p>A descriptive label that is associated with game session queue. Queue names must be unique within each Region.</p>
    #[serde(rename = "GameSessionQueueName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_queue_name: Option<String>,
    /// <p>Name of the Region where the game session created by this placement request is running. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>).</p>
    #[serde(rename = "GameSessionRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_region: Option<String>,
    /// <p>IP address of the instance that is running the game session. When connecting to a Amazon GameLift game server, a client needs to reference an IP address (or DNS name) and port number. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>). </p>
    #[serde(rename = "IpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// <p>Information on the matchmaking process for this game. Data is in JSON syntax, formatted as a string. It identifies the matchmaking configuration used to create the match, and contains data on all players assigned to the match, including player attributes and team assignments. For more details on matchmaker data, see <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-server.html#match-server-data">Match Data</a>.</p>
    #[serde(rename = "MatchmakerData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matchmaker_data: Option<String>,
    /// <p>The maximum number of players that can be connected simultaneously to the game session.</p>
    #[serde(rename = "MaximumPlayerSessionCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_player_session_count: Option<i64>,
    /// <p>A collection of information on player sessions created in response to the game session placement request. These player sessions are created only once a new game session is successfully placed (placement status is <code>FULFILLED</code>). This information includes the player ID (as provided in the placement request) and the corresponding player session ID. Retrieve full player sessions by calling <a>DescribePlayerSessions</a> with the player session ID.</p>
    #[serde(rename = "PlacedPlayerSessions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placed_player_sessions: Option<Vec<PlacedPlayerSession>>,
    /// <p>A unique identifier for a game session placement.</p>
    #[serde(rename = "PlacementId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_id: Option<String>,
    /// <p>Set of values, expressed in milliseconds, indicating the amount of latency that a player experiences when connected to AWS Regions.</p>
    #[serde(rename = "PlayerLatencies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_latencies: Option<Vec<PlayerLatency>>,
    /// <p>Port number for the game session. To connect to a Amazon GameLift game server, an app needs both the IP address and port number. This value is set once the new game session is placed (placement status is <code>FULFILLED</code>).</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>Time stamp indicating when this request was placed in the queue. Format is a number expressed in Unix time as milliseconds (for example "1469498468.057").</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p><p>Current status of the game session placement request.</p> <ul> <li> <p> <b>PENDING</b> -- The placement request is currently in the queue waiting to be processed.</p> </li> <li> <p> <b>FULFILLED</b> -- A new game session and player sessions (if requested) have been successfully created. Values for <i>GameSessionArn</i> and <i>GameSessionRegion</i> are available. </p> </li> <li> <p> <b>CANCELLED</b> -- The placement request was canceled with a call to <a>StopGameSessionPlacement</a>.</p> </li> <li> <p> <b>TIMED_OUT</b> -- A new game session was not successfully created before the time limit expired. You can resubmit the placement request as needed.</p> </li> <li> <p> <b>FAILED</b> -- GameLift is not able to complete the process of placing the game session. Common reasons are the game session terminated before the placement process was completed, or an unexpected internal error.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p><p>Configuration of a queue that is used to process game session placement requests. The queue configuration identifies several game features:</p> <ul> <li> <p>The destinations where a new game session can potentially be hosted. Amazon GameLift tries these destinations in an order based on either the queue&#39;s default order or player latency information, if provided in a placement request. With latency information, Amazon GameLift can place game sessions where the majority of players are reporting the lowest possible latency. </p> </li> <li> <p>The length of time that placement requests can wait in the queue before timing out. </p> </li> <li> <p>A set of optional latency policies that protect individual players from high latencies, preventing game sessions from being placed where any individual player is reporting latency higher than a policy&#39;s maximum.</p> </li> </ul> <ul> <li> <p> <a>CreateGameSessionQueue</a> </p> </li> <li> <p> <a>DescribeGameSessionQueues</a> </p> </li> <li> <p> <a>UpdateGameSessionQueue</a> </p> </li> <li> <p> <a>DeleteGameSessionQueue</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GameSessionQueue {
    /// <p>A list of fleets that can be used to fulfill game session placement requests in the queue. Fleets are identified by either a fleet ARN or a fleet alias ARN. Destinations are listed in default preference order.</p>
    #[serde(rename = "Destinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<GameSessionQueueDestination>>,
    /// <p>Amazon Resource Name (<a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a>) that is assigned to a GameLift game session queue resource and uniquely identifies it. ARNs are unique across all Regions. In a GameLift game session queue ARN, the resource ID matches the <i>Name</i> value.</p>
    #[serde(rename = "GameSessionQueueArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_queue_arn: Option<String>,
    /// <p>A descriptive label that is associated with game session queue. Queue names must be unique within each Region.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A collection of latency policies to apply when processing game sessions placement requests with player latency information. Multiple policies are evaluated in order of the maximum latency value, starting with the lowest latency values. With just one policy, the policy is enforced at the start of the game session placement for the duration period. With multiple policies, each policy is enforced consecutively for its duration period. For example, a queue might enforce a 60-second policy followed by a 120-second policy, and then no policy for the remainder of the placement. </p>
    #[serde(rename = "PlayerLatencyPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_latency_policies: Option<Vec<PlayerLatencyPolicy>>,
    /// <p>The maximum time, in seconds, that a new game session placement request remains in the queue. When a request exceeds this time, the game session placement changes to a <code>TIMED_OUT</code> status.</p>
    #[serde(rename = "TimeoutInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_seconds: Option<i64>,
}

/// <p><p>Fleet designated in a game session queue. Requests for new game sessions in the queue are fulfilled by starting a new game session on any destination that is configured for a queue. </p> <ul> <li> <p> <a>CreateGameSessionQueue</a> </p> </li> <li> <p> <a>DescribeGameSessionQueues</a> </p> </li> <li> <p> <a>UpdateGameSessionQueue</a> </p> </li> <li> <p> <a>DeleteGameSessionQueue</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GameSessionQueueDestination {
    /// <p>The Amazon Resource Name (ARN) that is assigned to fleet or fleet alias. ARNs, which include a fleet ID or alias ID and a Region name, provide a unique identifier across all Regions. </p>
    #[serde(rename = "DestinationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_arn: Option<String>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetGameSessionLogUrlInput {
    /// <p>A unique identifier for the game session to get logs for. </p>
    #[serde(rename = "GameSessionId")]
    pub game_session_id: String,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetGameSessionLogUrlOutput {
    /// <p>Location of the requested game session logs, available for download. This URL is valid for 15 minutes, after which S3 will reject any download request using this URL. You can request a new URL any time within the 14-day period that the logs are retained.</p>
    #[serde(rename = "PreSignedUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_signed_url: Option<String>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetInstanceAccessInput {
    /// <p>A unique identifier for a fleet that contains the instance you want access to. You can use either the fleet ID or ARN value. The fleet can be in any of the following statuses: <code>ACTIVATING</code>, <code>ACTIVE</code>, or <code>ERROR</code>. Fleets with an <code>ERROR</code> status may be accessible for a short time before they are deleted.</p>
    #[serde(rename = "FleetId")]
    pub fleet_id: String,
    /// <p>A unique identifier for an instance you want to get access to. You can access an instance in any status.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetInstanceAccessOutput {
    /// <p>The connection information for a fleet instance, including IP address and access credentials.</p>
    #[serde(rename = "InstanceAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_access: Option<InstanceAccess>,
}

/// <p>Properties that describe an instance of a virtual computing resource that hosts one or more game servers. A fleet may contain zero or more instances.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Instance {
    /// <p>Time stamp indicating when this data object was created. Format is a number expressed in Unix time as milliseconds (for example "1469498468.057").</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>DNS identifier assigned to the instance that is running the game session. Values have the following format:</p> <ul> <li> <p>TLS-enabled fleets: <code>&lt;unique identifier&gt;.&lt;region identifier&gt;.amazongamelift.com</code>.</p> </li> <li> <p>Non-TLS-enabled fleets: <code>ec2-&lt;unique identifier&gt;.compute.amazonaws.com</code>. (See <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-instance-addressing.html#concepts-public-addresses">Amazon EC2 Instance IP Addressing</a>.)</p> </li> </ul> <p>When connecting to a game session that is running on a TLS-enabled fleet, you must use the DNS name, not the IP address.</p>
    #[serde(rename = "DnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    /// <p>A unique identifier for a fleet that the instance is in.</p>
    #[serde(rename = "FleetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_id: Option<String>,
    /// <p>A unique identifier for an instance.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>IP address that is assigned to the instance.</p>
    #[serde(rename = "IpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// <p>Operating system that is running on this instance. </p>
    #[serde(rename = "OperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    /// <p><p>Current status of the instance. Possible statuses include the following:</p> <ul> <li> <p> <b>PENDING</b> -- The instance is in the process of being created and launching server processes as defined in the fleet&#39;s run-time configuration. </p> </li> <li> <p> <b>ACTIVE</b> -- The instance has been successfully created and at least one server process has successfully launched and reported back to Amazon GameLift that it is ready to host a game session. The instance is now considered ready to host game sessions. </p> </li> <li> <p> <b>TERMINATING</b> -- The instance is in the process of shutting down. This may happen to reduce capacity during a scaling down event or to recycle resources in the event of a problem.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>EC2 instance type that defines the computing resources of this instance. </p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Information required to remotely connect to a fleet instance. Access is requested by calling <a>GetInstanceAccess</a>. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceAccess {
    /// <p>Credentials required to access the instance.</p>
    #[serde(rename = "Credentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<InstanceCredentials>,
    /// <p>A unique identifier for a fleet containing the instance being accessed.</p>
    #[serde(rename = "FleetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_id: Option<String>,
    /// <p>A unique identifier for an instance being accessed.</p>
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p>IP address that is assigned to the instance.</p>
    #[serde(rename = "IpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// <p>Operating system that is running on the instance.</p>
    #[serde(rename = "OperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
}

/// <p>Set of credentials required to remotely access a fleet instance. Access credentials are requested by calling <a>GetInstanceAccess</a> and returned in an <a>InstanceAccess</a> object.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceCredentials {
    /// <p>Secret string. For Windows instances, the secret is a password for use with Windows Remote Desktop. For Linux instances, it is a private key (which must be saved as a <code>.pem</code> file) for use with SSH.</p>
    #[serde(rename = "Secret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// <p>User login string.</p>
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

/// <p> <b>This data type is used with the Amazon GameLift FleetIQ and game server groups.</b> </p> <p>An allowed instance type for a <a>GameServerGroup</a>. All game server groups must have at least two instance types defined for it. GameLift FleetIQ periodically evaluates each defined instance type for viability. It then updates the Auto Scaling group with the list of viable instance types.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InstanceDefinition {
    /// <p>An EC2 instance type designation.</p>
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// <p>Instance weighting that indicates how much this instance type contributes to the total capacity of a game server group. Instance weights are used by GameLift FleetIQ to calculate the instance type's cost per unit hour and better identify the most cost-effective options. For detailed information on weighting instance capacity, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-instance-weighting.html">Instance Weighting</a> in the <i>Amazon EC2 Auto Scaling User Guide</i>. Default value is "1".</p>
    #[serde(rename = "WeightedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weighted_capacity: Option<String>,
}

/// <p>A range of IP addresses and port settings that allow inbound traffic to connect to server processes on an Amazon GameLift hosting resource. New game sessions that are started on the fleet are assigned an IP address/port number combination, which must fall into the fleet's allowed ranges. For fleets created with a custom game server, the ranges reflect the server's game session assignments. For Realtime Servers fleets, Amazon GameLift automatically opens two port ranges, one for TCP messaging and one for UDP for use by the Realtime servers.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct IpPermission {
    /// <p>A starting value for a range of allowed port numbers.</p>
    #[serde(rename = "FromPort")]
    pub from_port: i64,
    /// <p>A range of allowed IP addresses. This value must be expressed in CIDR notation. Example: "<code>000.000.000.000/[subnet mask]</code>" or optionally the shortened version "<code>0.0.0.0/[subnet mask]</code>".</p>
    #[serde(rename = "IpRange")]
    pub ip_range: String,
    /// <p>The network communication protocol used by the fleet.</p>
    #[serde(rename = "Protocol")]
    pub protocol: String,
    /// <p>An ending value for a range of allowed port numbers. Port numbers are end-inclusive. This value must be higher than <code>FromPort</code>.</p>
    #[serde(rename = "ToPort")]
    pub to_port: i64,
}

/// <p> <b>This data type is used with the Amazon GameLift FleetIQ and game server groups.</b> </p> <p>An EC2 launch template that contains configuration settings and game server code to be deployed to all instances in a game server group. The launch template is specified when creating a new game server group with <a>CreateGameServerGroup</a>. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct LaunchTemplateSpecification {
    /// <p>A unique identifier for an existing EC2 launch template.</p>
    #[serde(rename = "LaunchTemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_id: Option<String>,
    /// <p>A readable identifier for an existing EC2 launch template. </p>
    #[serde(rename = "LaunchTemplateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_name: Option<String>,
    /// <p>The version of the EC2 launch template to use. If no version is specified, the default version will be used. With Amazon EC2, you can specify a default version for a launch template. If none is set, the default is the first version created.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAliasesInput {
    /// <p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>A descriptive label that is associated with an alias. Alias names do not need to be unique.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The routing type to filter results on. Use this parameter to retrieve only aliases with a certain routing type. To retrieve all aliases, leave this parameter empty.</p> <p>Possible routing types include the following:</p> <ul> <li> <p> <b>SIMPLE</b> -- The alias resolves to one specific fleet. Use this type when routing to active fleets.</p> </li> <li> <p> <b>TERMINAL</b> -- The alias does not resolve to a fleet but instead can be used to display a message to the user. A terminal alias throws a TerminalRoutingStrategyException with the <a>RoutingStrategy</a> message embedded.</p> </li> </ul></p>
    #[serde(rename = "RoutingStrategyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_strategy_type: Option<String>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAliasesOutput {
    /// <p>A collection of alias resources that match the request parameters.</p>
    #[serde(rename = "Aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<Alias>>,
    /// <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListBuildsInput {
    /// <p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>Build status to filter results by. To retrieve all builds, leave this parameter empty.</p> <p>Possible build statuses include the following:</p> <ul> <li> <p> <b>INITIALIZED</b> -- A new build has been defined, but no files have been uploaded. You cannot create fleets for builds that are in this status. When a build is successfully created, the build status is set to this value. </p> </li> <li> <p> <b>READY</b> -- The game build has been successfully uploaded. You can now create new fleets for this build.</p> </li> <li> <p> <b>FAILED</b> -- The game build upload failed. You cannot create new fleets for this build. </p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListBuildsOutput {
    /// <p>A collection of build resources that match the request.</p>
    #[serde(rename = "Builds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builds: Option<Vec<Build>>,
    /// <p>Token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListFleetsInput {
    /// <p>A unique identifier for a build to return fleets for. Use this parameter to return only fleets using a specified build. Use either the build ID or ARN value. To retrieve all fleets, do not include either a BuildId and ScriptID parameter.</p>
    #[serde(rename = "BuildId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_id: Option<String>,
    /// <p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A unique identifier for a Realtime script to return fleets for. Use this parameter to return only fleets using a specified script. Use either the script ID or ARN value. To retrieve all fleets, leave this parameter empty.</p>
    #[serde(rename = "ScriptId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_id: Option<String>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListFleetsOutput {
    /// <p>Set of fleet IDs matching the list request. You can retrieve additional information about all returned fleets by passing this result set to a call to <a>DescribeFleetAttributes</a>, <a>DescribeFleetCapacity</a>, or <a>DescribeFleetUtilization</a>.</p>
    #[serde(rename = "FleetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_ids: Option<Vec<String>>,
    /// <p>Token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListGameServerGroupsInput {
    /// <p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential segments.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>A token that indicates the start of the next sequential segment of results. Use the token returned with the previous call to this operation. To start at the beginning of the result set, do not specify a value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListGameServerGroupsOutput {
    /// <p>A collection of game server group objects that match the request.</p>
    #[serde(rename = "GameServerGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_server_groups: Option<Vec<GameServerGroup>>,
    /// <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListGameServersInput {
    /// <p>An identifier for the game server group to retrieve a list of game servers from. Use either the <a>GameServerGroup</a> name or ARN value.</p>
    #[serde(rename = "GameServerGroupName")]
    pub game_server_group_name: String,
    /// <p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential segments.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>A token that indicates the start of the next sequential segment of results. Use the token returned with the previous call to this operation. To start at the beginning of the result set, do not specify a value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Indicates how to sort the returned data based on game server registration timestamp. Use ASCENDING to retrieve oldest game servers first, or use DESCENDING to retrieve newest game servers first. If this parameter is left empty, game servers are returned in no particular order.</p>
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListGameServersOutput {
    /// <p>A collection of game server objects that match the request.</p>
    #[serde(rename = "GameServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_servers: Option<Vec<GameServer>>,
    /// <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListScriptsInput {
    /// <p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>A token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListScriptsOutput {
    /// <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A set of properties describing the requested script.</p>
    #[serde(rename = "Scripts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<Vec<Script>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p> The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) that is assigned to and uniquely identifies the GameLift resource that you want to retrieve tags for. GameLift resource ARNs are included in the data object for the resource, which can be retrieved by calling a List or Describe operation for the resource type. </p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p> The collection of tags that have been assigned to the specified resource. </p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Represents a new player session that is created as a result of a successful FlexMatch match. A successful match automatically creates new player sessions for every player ID in the original matchmaking request. </p> <p>When players connect to the match's game session, they must include both player ID and player session ID in order to claim their assigned player slot.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MatchedPlayerSession {
    /// <p>A unique identifier for a player </p>
    #[serde(rename = "PlayerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_id: Option<String>,
    /// <p>A unique identifier for a player session</p>
    #[serde(rename = "PlayerSessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_session_id: Option<String>,
}

/// <p>Guidelines for use with FlexMatch to match players into games. All matchmaking requests must specify a matchmaking configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MatchmakingConfiguration {
    /// <p>A flag that indicates whether a match that was created with this configuration must be accepted by the matched players. To require acceptance, set to TRUE. When this option is enabled, matchmaking tickets use the status <code>REQUIRES_ACCEPTANCE</code> to indicate when a completed potential match is waiting for player acceptance.</p>
    #[serde(rename = "AcceptanceRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance_required: Option<bool>,
    /// <p>The length of time (in seconds) to wait for players to accept a proposed match, if acceptance is required. If any player rejects the match or fails to accept before the timeout, the tickets are returned to the ticket pool and continue to be evaluated for an acceptable match.</p>
    #[serde(rename = "AcceptanceTimeoutSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance_timeout_seconds: Option<i64>,
    /// <p>The number of player slots in a match to keep open for future players. For example, assume that the configuration's rule set specifies a match for a single 12-person team. If the additional player count is set to 2, only 10 players are initially selected for the match. This parameter is not used when <code>FlexMatchMode</code> is set to <code>STANDALONE</code>.</p>
    #[serde(rename = "AdditionalPlayerCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_player_count: Option<i64>,
    /// <p>The method used to backfill game sessions created with this matchmaking configuration. MANUAL indicates that the game makes backfill requests or does not use the match backfill feature. AUTOMATIC indicates that GameLift creates <a>StartMatchBackfill</a> requests whenever a game session has one or more open slots. Learn more about manual and automatic backfill in <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-backfill.html">Backfill Existing Games with FlexMatch</a>. Automatic backfill is not available when <code>FlexMatchMode</code> is set to <code>STANDALONE</code>.</p>
    #[serde(rename = "BackfillMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backfill_mode: Option<String>,
    /// <p>Amazon Resource Name (<a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a>) that is assigned to a GameLift matchmaking configuration resource and uniquely identifies it. ARNs are unique across all Regions. In a GameLift configuration ARN, the resource ID matches the <i>Name</i> value. </p>
    #[serde(rename = "ConfigurationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_arn: Option<String>,
    /// <p>The time stamp indicating when this data object was created. The format is a number expressed in Unix time as milliseconds (for example "1469498468.057").</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>Information to attach to all events related to the matchmaking configuration. </p>
    #[serde(rename = "CustomEventData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_event_data: Option<String>,
    /// <p>A descriptive label that is associated with matchmaking configuration.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p><p>Indicates whether this matchmaking configuration is being used with GameLift hosting or as a standalone matchmaking solution. </p> <ul> <li> <p> <b>STANDALONE</b> - FlexMatch forms matches and returns match information, including players and team assignments, in a <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-events.html#match-events-matchmakingsucceeded"> MatchmakingSucceeded</a> event.</p> </li> <li> <p> <b>WITH_QUEUE</b> - FlexMatch forms matches and uses the specified GameLift queue to start a game session for the match. </p> </li> </ul></p>
    #[serde(rename = "FlexMatchMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex_match_mode: Option<String>,
    /// <p>A set of custom properties for a game session, formatted as key-value pairs. These properties are passed to a game server process in the <a>GameSession</a> object with a request to start a new game session (see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession">Start a Game Session</a>). This information is added to the new <a>GameSession</a> object that is created for a successful match. This parameter is not used when <code>FlexMatchMode</code> is set to <code>STANDALONE</code>.</p>
    #[serde(rename = "GameProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_properties: Option<Vec<GameProperty>>,
    /// <p>A set of custom game session properties, formatted as a single string value. This data is passed to a game server process in the <a>GameSession</a> object with a request to start a new game session (see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession">Start a Game Session</a>). This information is added to the new <a>GameSession</a> object that is created for a successful match. This parameter is not used when <code>FlexMatchMode</code> is set to <code>STANDALONE</code>.</p>
    #[serde(rename = "GameSessionData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_data: Option<String>,
    /// <p>Amazon Resource Name (<a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a>) that is assigned to a GameLift game session queue resource and uniquely identifies it. ARNs are unique across all Regions. Queues can be located in any Region. Queues are used to start new GameLift-hosted game sessions for matches that are created with this matchmaking configuration. Thais property is not set when <code>FlexMatchMode</code> is set to <code>STANDALONE</code>.</p>
    #[serde(rename = "GameSessionQueueArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_queue_arns: Option<Vec<String>>,
    /// <p>A unique identifier for a matchmaking configuration. This name is used to identify the configuration associated with a matchmaking request or ticket.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An SNS topic ARN that is set up to receive matchmaking notifications.</p>
    #[serde(rename = "NotificationTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_target: Option<String>,
    /// <p>The maximum duration, in seconds, that a matchmaking ticket can remain in process before timing out. Requests that fail due to timing out can be resubmitted as needed.</p>
    #[serde(rename = "RequestTimeoutSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_timeout_seconds: Option<i64>,
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a>) associated with the GameLift matchmaking rule set resource that this configuration uses.</p>
    #[serde(rename = "RuleSetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_set_arn: Option<String>,
    /// <p>A unique identifier for a matchmaking rule set to use with this configuration. A matchmaking configuration can only use rule sets that are defined in the same Region.</p>
    #[serde(rename = "RuleSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_set_name: Option<String>,
}

/// <p><p>Set of rule statements, used with FlexMatch, that determine how to build your player matches. Each rule set describes a type of group to be created and defines the parameters for acceptable player matches. Rule sets are used in <a>MatchmakingConfiguration</a> objects.</p> <p>A rule set may define the following elements for a match. For detailed information and examples showing how to construct a rule set, see <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-rulesets.html">Build a FlexMatch Rule Set</a>. </p> <ul> <li> <p>Teams -- Required. A rule set must define one or multiple teams for the match and set minimum and maximum team sizes. For example, a rule set might describe a 4x4 match that requires all eight slots to be filled. </p> </li> <li> <p>Player attributes -- Optional. These attributes specify a set of player characteristics to evaluate when looking for a match. Matchmaking requests that use a rule set with player attributes must provide the corresponding attribute values. For example, an attribute might specify a player&#39;s skill or level.</p> </li> <li> <p>Rules -- Optional. Rules define how to evaluate potential players for a match based on player attributes. A rule might specify minimum requirements for individual players, teams, or entire matches. For example, a rule might require each player to meet a certain skill level, each team to have at least one player in a certain role, or the match to have a minimum average skill level. or may describe an entire group--such as all teams must be evenly matched or have at least one player in a certain role. </p> </li> <li> <p>Expansions -- Optional. Expansions allow you to relax the rules after a period of time when no acceptable matches are found. This feature lets you balance getting players into games in a reasonable amount of time instead of making them wait indefinitely for the best possible match. For example, you might use an expansion to increase the maximum skill variance between players after 30 seconds.</p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MatchmakingRuleSet {
    /// <p>The time stamp indicating when this data object was created. The format is a number expressed in Unix time as milliseconds (for example "1469498468.057").</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>Amazon Resource Name (<a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a>) that is assigned to a GameLift matchmaking rule set resource and uniquely identifies it. ARNs are unique across all Regions. In a GameLift rule set ARN, the resource ID matches the <i>RuleSetName</i> value.</p>
    #[serde(rename = "RuleSetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_set_arn: Option<String>,
    /// <p>A collection of matchmaking rules, formatted as a JSON string. Comments are not allowed in JSON, but most elements support a description field.</p>
    #[serde(rename = "RuleSetBody")]
    pub rule_set_body: String,
    /// <p>A unique identifier for a matchmaking rule set</p>
    #[serde(rename = "RuleSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_set_name: Option<String>,
}

/// <p>Ticket generated to track the progress of a matchmaking request. Each ticket is uniquely identified by a ticket ID, supplied by the requester, when creating a matchmaking request with <a>StartMatchmaking</a>. Tickets can be retrieved by calling <a>DescribeMatchmaking</a> with the ticket ID.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MatchmakingTicket {
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a>) associated with the GameLift matchmaking configuration resource that is used with this ticket.</p>
    #[serde(rename = "ConfigurationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_arn: Option<String>,
    /// <p>Name of the <a>MatchmakingConfiguration</a> that is used with this ticket. Matchmaking configurations determine how players are grouped into a match and how a new game session is created for the match.</p>
    #[serde(rename = "ConfigurationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_name: Option<String>,
    /// <p>Time stamp indicating when this matchmaking request stopped being processed due to success, failure, or cancellation. Format is a number expressed in Unix time as milliseconds (for example "1469498468.057").</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>Average amount of time (in seconds) that players are currently waiting for a match. If there is not enough recent data, this property may be empty.</p>
    #[serde(rename = "EstimatedWaitTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_wait_time: Option<i64>,
    /// <p>Identifier and connection information of the game session created for the match. This information is added to the ticket only after the matchmaking request has been successfully completed. This parameter is not set when FlexMatch is being used without GameLift hosting.</p>
    #[serde(rename = "GameSessionConnectionInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_connection_info: Option<GameSessionConnectionInfo>,
    /// <p>A set of <code>Player</code> objects, each representing a player to find matches for. Players are identified by a unique player ID and may include latency data for use during matchmaking. If the ticket is in status <code>COMPLETED</code>, the <code>Player</code> objects include the team the players were assigned to in the resulting match.</p>
    #[serde(rename = "Players")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub players: Option<Vec<Player>>,
    /// <p>Time stamp indicating when this matchmaking request was received. Format is a number expressed in Unix time as milliseconds (for example "1469498468.057").</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p><p>Current status of the matchmaking request.</p> <ul> <li> <p> <b>QUEUED</b> -- The matchmaking request has been received and is currently waiting to be processed.</p> </li> <li> <p> <b>SEARCHING</b> -- The matchmaking request is currently being processed. </p> </li> <li> <p> <b>REQUIRES<em>ACCEPTANCE</b> -- A match has been proposed and the players must accept the match (see <a>AcceptMatch</a>). This status is used only with requests that use a matchmaking configuration with a player acceptance requirement.</p> </li> <li> <p> <b>PLACING</b> -- The FlexMatch engine has matched players and is in the process of placing a new game session for the match.</p> </li> <li> <p> <b>COMPLETED</b> -- Players have been matched and a game session is ready to host the players. A ticket in this state contains the necessary connection information for players.</p> </li> <li> <p> <b>FAILED</b> -- The matchmaking request was not completed.</p> </li> <li> <p> <b>CANCELLED</b> -- The matchmaking request was canceled. This may be the result of a call to <a>StopMatchmaking</a> or a proposed match that one or more players failed to accept.</p> </li> <li> <p> <b>TIMED</em>OUT</b> -- The matchmaking request was not successful within the duration specified in the matchmaking configuration. </p> </li> </ul> <note> <p>Matchmaking requests that fail to successfully complete (statuses FAILED, CANCELLED, TIMED_OUT) can be resubmitted as new requests with new ticket IDs.</p> </note></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Additional information about the current status.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>Code to explain the current status. For example, a status reason may indicate when a ticket has returned to <code>SEARCHING</code> status after a proposed match fails to receive player acceptances.</p>
    #[serde(rename = "StatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    /// <p>A unique identifier for a matchmaking ticket.</p>
    #[serde(rename = "TicketId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket_id: Option<String>,
}

/// <p><p>Information about a player session that was created as part of a <a>StartGameSessionPlacement</a> request. This object contains only the player ID and player session ID. To retrieve full details on a player session, call <a>DescribePlayerSessions</a> with the player session ID.</p> <ul> <li> <p> <a>CreatePlayerSession</a> </p> </li> <li> <p> <a>CreatePlayerSessions</a> </p> </li> <li> <p> <a>DescribePlayerSessions</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PlacedPlayerSession {
    /// <p>A unique identifier for a player that is associated with this player session.</p>
    #[serde(rename = "PlayerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_id: Option<String>,
    /// <p>A unique identifier for a player session.</p>
    #[serde(rename = "PlayerSessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_session_id: Option<String>,
}

/// <p>Represents a player in matchmaking. When starting a matchmaking request, a player has a player ID, attributes, and may have latency data. Team information is added after a match has been successfully completed.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Player {
    /// <p>Set of values, expressed in milliseconds, indicating the amount of latency that a player experiences when connected to AWS Regions. If this property is present, FlexMatch considers placing the match only in Regions for which latency is reported. </p> <p>If a matchmaker has a rule that evaluates player latency, players must report latency in order to be matched. If no latency is reported in this scenario, FlexMatch assumes that no Regions are available to the player and the ticket is not matchable. </p>
    #[serde(rename = "LatencyInMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_in_ms: Option<::std::collections::HashMap<String, i64>>,
    /// <p>A collection of key:value pairs containing player information for use in matchmaking. Player attribute keys must match the <i>playerAttributes</i> used in a matchmaking rule set. Example: <code>"PlayerAttributes": {"skill": {"N": "23"}, "gameMode": {"S": "deathmatch"}}</code>.</p>
    #[serde(rename = "PlayerAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_attributes: Option<::std::collections::HashMap<String, AttributeValue>>,
    /// <p>A unique identifier for a player</p>
    #[serde(rename = "PlayerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_id: Option<String>,
    /// <p>Name of the team that the player is assigned to in a match. Team names are defined in a matchmaking rule set.</p>
    #[serde(rename = "Team")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
}

/// <p>Regional latency information for a player, used when requesting a new game session with <a>StartGameSessionPlacement</a>. This value indicates the amount of time lag that exists when the player is connected to a fleet in the specified Region. The relative difference between a player's latency values for multiple Regions are used to determine which fleets are best suited to place a new game session for the player. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PlayerLatency {
    /// <p>Amount of time that represents the time lag experienced by the player when connected to the specified Region.</p>
    #[serde(rename = "LatencyInMilliseconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_in_milliseconds: Option<f32>,
    /// <p>A unique identifier for a player associated with the latency data.</p>
    #[serde(rename = "PlayerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_id: Option<String>,
    /// <p>Name of the Region that is associated with the latency value.</p>
    #[serde(rename = "RegionIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_identifier: Option<String>,
}

/// <p><p>Queue setting that determines the highest latency allowed for individual players when placing a game session. When a latency policy is in force, a game session cannot be placed with any fleet in a Region where a player reports latency higher than the cap. Latency policies are only enforced when the placement request contains player latency information.</p> <ul> <li> <p> <a>CreateGameSessionQueue</a> </p> </li> <li> <p> <a>DescribeGameSessionQueues</a> </p> </li> <li> <p> <a>UpdateGameSessionQueue</a> </p> </li> <li> <p> <a>DeleteGameSessionQueue</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PlayerLatencyPolicy {
    /// <p>The maximum latency value that is allowed for any player, in milliseconds. All policies must have a value set for this property.</p>
    #[serde(rename = "MaximumIndividualPlayerLatencyMilliseconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_individual_player_latency_milliseconds: Option<i64>,
    /// <p>The length of time, in seconds, that the policy is enforced while placing a new game session. A null value for this property means that the policy is enforced until the queue times out.</p>
    #[serde(rename = "PolicyDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_duration_seconds: Option<i64>,
}

/// <p><p>Properties describing a player session. Player session objects are created either by creating a player session for a specific game session, or as part of a game session placement. A player session represents either a player reservation for a game session (status <code>RESERVED</code>) or actual player activity in a game session (status <code>ACTIVE</code>). A player session object (including player data) is automatically passed to a game session when the player connects to the game session and is validated.</p> <p>When a player disconnects, the player session status changes to <code>COMPLETED</code>. Once the session ends, the player session object is retained for 30 days and then removed.</p> <ul> <li> <p> <a>CreatePlayerSession</a> </p> </li> <li> <p> <a>CreatePlayerSessions</a> </p> </li> <li> <p> <a>DescribePlayerSessions</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PlayerSession {
    /// <p>Time stamp indicating when this data object was created. Format is a number expressed in Unix time as milliseconds (for example "1469498468.057").</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>DNS identifier assigned to the instance that is running the game session. Values have the following format:</p> <ul> <li> <p>TLS-enabled fleets: <code>&lt;unique identifier&gt;.&lt;region identifier&gt;.amazongamelift.com</code>.</p> </li> <li> <p>Non-TLS-enabled fleets: <code>ec2-&lt;unique identifier&gt;.compute.amazonaws.com</code>. (See <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-instance-addressing.html#concepts-public-addresses">Amazon EC2 Instance IP Addressing</a>.)</p> </li> </ul> <p>When connecting to a game session that is running on a TLS-enabled fleet, you must use the DNS name, not the IP address.</p>
    #[serde(rename = "DnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    /// <p> The Amazon Resource Name (<a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a>) associated with the GameLift fleet that the player's game session is running on. </p>
    #[serde(rename = "FleetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_arn: Option<String>,
    /// <p>A unique identifier for a fleet that the player's game session is running on.</p>
    #[serde(rename = "FleetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_id: Option<String>,
    /// <p>A unique identifier for the game session that the player session is connected to.</p>
    #[serde(rename = "GameSessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_id: Option<String>,
    /// <p>IP address of the instance that is running the game session. When connecting to a Amazon GameLift game server, a client needs to reference an IP address (or DNS name) and port number.</p>
    #[serde(rename = "IpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// <p>Developer-defined information related to a player. Amazon GameLift does not use this data, so it can be formatted as needed for use in the game. </p>
    #[serde(rename = "PlayerData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_data: Option<String>,
    /// <p>A unique identifier for a player that is associated with this player session.</p>
    #[serde(rename = "PlayerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_id: Option<String>,
    /// <p>A unique identifier for a player session.</p>
    #[serde(rename = "PlayerSessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_session_id: Option<String>,
    /// <p>Port number for the game session. To connect to a Amazon GameLift server process, an app needs both the IP address and port number.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p><p>Current status of the player session.</p> <p>Possible player session statuses include the following:</p> <ul> <li> <p> <b>RESERVED</b> -- The player session request has been received, but the player has not yet connected to the server process and/or been validated. </p> </li> <li> <p> <b>ACTIVE</b> -- The player has been validated by the server process and is currently connected.</p> </li> <li> <p> <b>COMPLETED</b> -- The player connection has been dropped.</p> </li> <li> <p> <b>TIMEDOUT</b> -- A player session request was received, but the player did not connect and/or was not validated within the timeout limit (60 seconds).</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Time stamp indicating when this data object was terminated. Format is a number expressed in Unix time as milliseconds (for example "1469498468.057").</p>
    #[serde(rename = "TerminationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_time: Option<f64>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutScalingPolicyInput {
    /// <p>Comparison operator to use when measuring the metric against the threshold value.</p>
    #[serde(rename = "ComparisonOperator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<String>,
    /// <p>Length of time (in minutes) the metric must be at or beyond the threshold before a scaling event is triggered.</p>
    #[serde(rename = "EvaluationPeriods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_periods: Option<i64>,
    /// <p>A unique identifier for a fleet to apply this policy to. You can use either the fleet ID or ARN value. The fleet cannot be in any of the following statuses: ERROR or DELETING.</p>
    #[serde(rename = "FleetId")]
    pub fleet_id: String,
    /// <p><p>Name of the Amazon GameLift-defined metric that is used to trigger a scaling adjustment. For detailed descriptions of fleet metrics, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/monitoring-cloudwatch.html">Monitor Amazon GameLift with Amazon CloudWatch</a>. </p> <ul> <li> <p> <b>ActivatingGameSessions</b> -- Game sessions in the process of being created.</p> </li> <li> <p> <b>ActiveGameSessions</b> -- Game sessions that are currently running.</p> </li> <li> <p> <b>ActiveInstances</b> -- Fleet instances that are currently running at least one game session.</p> </li> <li> <p> <b>AvailableGameSessions</b> -- Additional game sessions that fleet could host simultaneously, given current capacity.</p> </li> <li> <p> <b>AvailablePlayerSessions</b> -- Empty player slots in currently active game sessions. This includes game sessions that are not currently accepting players. Reserved player slots are not included.</p> </li> <li> <p> <b>CurrentPlayerSessions</b> -- Player slots in active game sessions that are being used by a player or are reserved for a player. </p> </li> <li> <p> <b>IdleInstances</b> -- Active instances that are currently hosting zero game sessions. </p> </li> <li> <p> <b>PercentAvailableGameSessions</b> -- Unused percentage of the total number of game sessions that a fleet could host simultaneously, given current capacity. Use this metric for a target-based scaling policy.</p> </li> <li> <p> <b>PercentIdleInstances</b> -- Percentage of the total number of active instances that are hosting zero game sessions.</p> </li> <li> <p> <b>QueueDepth</b> -- Pending game session placement requests, in any queue, where the current fleet is the top-priority destination.</p> </li> <li> <p> <b>WaitTime</b> -- Current wait time for pending game session placement requests, in any queue, where the current fleet is the top-priority destination. </p> </li> </ul></p>
    #[serde(rename = "MetricName")]
    pub metric_name: String,
    /// <p>A descriptive label that is associated with a scaling policy. Policy names do not need to be unique. A fleet can have only one scaling policy with the same name.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The type of scaling policy to create. For a target-based policy, set the parameter <i>MetricName</i> to 'PercentAvailableGameSessions' and specify a <i>TargetConfiguration</i>. For a rule-based policy set the following parameters: <i>MetricName</i>, <i>ComparisonOperator</i>, <i>Threshold</i>, <i>EvaluationPeriods</i>, <i>ScalingAdjustmentType</i>, and <i>ScalingAdjustment</i>.</p>
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    /// <p>Amount of adjustment to make, based on the scaling adjustment type.</p>
    #[serde(rename = "ScalingAdjustment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_adjustment: Option<i64>,
    /// <p><p>The type of adjustment to make to a fleet&#39;s instance count (see <a>FleetCapacity</a>):</p> <ul> <li> <p> <b>ChangeInCapacity</b> -- add (or subtract) the scaling adjustment value from the current instance count. Positive values scale up while negative values scale down.</p> </li> <li> <p> <b>ExactCapacity</b> -- set the instance count to the scaling adjustment value.</p> </li> <li> <p> <b>PercentChangeInCapacity</b> -- increase or reduce the current instance count by the scaling adjustment, read as a percentage. Positive values scale up while negative values scale down; for example, a value of &quot;-10&quot; scales the fleet down by 10%.</p> </li> </ul></p>
    #[serde(rename = "ScalingAdjustmentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_adjustment_type: Option<String>,
    /// <p>The settings for a target-based scaling policy.</p>
    #[serde(rename = "TargetConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_configuration: Option<TargetConfiguration>,
    /// <p>Metric value used to trigger a scaling event.</p>
    #[serde(rename = "Threshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutScalingPolicyOutput {
    /// <p>A descriptive label that is associated with a scaling policy. Policy names do not need to be unique.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterGameServerInput {
    /// <p>Information that is needed to make inbound client connections to the game server. This might include the IP address and port, DNS name, and other information.</p>
    #[serde(rename = "ConnectionInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_info: Option<String>,
    /// <p>A set of custom game server properties, formatted as a single string value. This data is passed to a game client or service when it requests information on game servers using <a>ListGameServers</a> or <a>ClaimGameServer</a>. </p>
    #[serde(rename = "GameServerData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_server_data: Option<String>,
    /// <p>A unique identifier for the game server group where the game server is running. Use either the <a>GameServerGroup</a> name or ARN value.</p>
    #[serde(rename = "GameServerGroupName")]
    pub game_server_group_name: String,
    /// <p>A custom string that uniquely identifies the game server to register. Game server IDs are developer-defined and must be unique across all game server groups in your AWS account.</p>
    #[serde(rename = "GameServerId")]
    pub game_server_id: String,
    /// <p>The unique identifier for the instance where the game server is running. This ID is available in the instance metadata. EC2 instance IDs use a 17-character format, for example: <code>i-1234567890abcdef0</code>.</p>
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterGameServerOutput {
    /// <p>Object that describes the newly registered game server.</p>
    #[serde(rename = "GameServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_server: Option<GameServer>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RequestUploadCredentialsInput {
    /// <p>A unique identifier for a build to get credentials for. You can use either the build ID or ARN value. </p>
    #[serde(rename = "BuildId")]
    pub build_id: String,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RequestUploadCredentialsOutput {
    /// <p>Amazon S3 path and key, identifying where the game build files are stored.</p>
    #[serde(rename = "StorageLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_location: Option<S3Location>,
    /// <p>AWS credentials required when uploading a game build to the storage location. These credentials have a limited lifespan and are valid only for the build they were issued for.</p>
    #[serde(rename = "UploadCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_credentials: Option<AwsCredentials>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ResolveAliasInput {
    /// <p>The unique identifier of the alias that you want to retrieve a fleet ID for. You can use either the alias ID or ARN value.</p>
    #[serde(rename = "AliasId")]
    pub alias_id: String,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResolveAliasOutput {
    /// <p> The Amazon Resource Name (<a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a>) associated with the GameLift fleet resource that this alias points to. </p>
    #[serde(rename = "FleetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_arn: Option<String>,
    /// <p>The fleet identifier that the alias is pointing to.</p>
    #[serde(rename = "FleetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_id: Option<String>,
}

/// <p>A policy that limits the number of game sessions a player can create on the same fleet. This optional policy gives game owners control over how players can consume available game server resources. A resource creation policy makes the following statement: "An individual player can create a maximum number of new game sessions within a specified time period".</p> <p>The policy is evaluated when a player tries to create a new game session. For example: Assume you have a policy of 10 new game sessions and a time period of 60 minutes. On receiving a <code>CreateGameSession</code> request, Amazon GameLift checks that the player (identified by <code>CreatorId</code>) has created fewer than 10 game sessions in the past 60 minutes.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ResourceCreationLimitPolicy {
    /// <p>The maximum number of game sessions that an individual can create during the policy period. </p>
    #[serde(rename = "NewGameSessionsPerCreator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_game_sessions_per_creator: Option<i64>,
    /// <p>The time span used in evaluating the resource creation limit policy. </p>
    #[serde(rename = "PolicyPeriodInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_period_in_minutes: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ResumeGameServerGroupInput {
    /// <p>A unique identifier for the game server group. Use either the <a>GameServerGroup</a> name or ARN value.</p>
    #[serde(rename = "GameServerGroupName")]
    pub game_server_group_name: String,
    /// <p>The activity to resume for this game server group.</p>
    #[serde(rename = "ResumeActions")]
    pub resume_actions: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResumeGameServerGroupOutput {
    /// <p>An object that describes the game server group resource, with the <code>SuspendedActions</code> property updated to reflect the resumed activity.</p>
    #[serde(rename = "GameServerGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_server_group: Option<GameServerGroup>,
}

/// <p><p>The routing configuration for a fleet alias.</p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>DescribeAlias</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ResolveAlias</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RoutingStrategy {
    /// <p>The unique identifier for a fleet that the alias points to. This value is the fleet ID, not the fleet ARN.</p>
    #[serde(rename = "FleetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_id: Option<String>,
    /// <p>The message text to be used with a terminal routing strategy.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p><p>The type of routing strategy for the alias.</p> <p>Possible routing types include the following:</p> <ul> <li> <p> <b>SIMPLE</b> - The alias resolves to one specific fleet. Use this type when routing to active fleets.</p> </li> <li> <p> <b>TERMINAL</b> - The alias does not resolve to a fleet but instead can be used to display a message to the user. A terminal alias throws a TerminalRoutingStrategyException with the <a>RoutingStrategy</a> message embedded.</p> </li> </ul></p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p><p>A collection of server process configurations that describe what processes to run on each instance in a fleet. Server processes run either a custom game build executable or a Realtime Servers script. Each instance in the fleet starts the specified server processes and continues to start new processes as existing processes end. Each instance regularly checks for an updated runtime configuration. </p> <p>The runtime configuration enables the instances in a fleet to run multiple processes simultaneously. Learn more about <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-multiprocess.html"> Running Multiple Processes on a Fleet </a>.</p> <p>A Amazon GameLift instance is limited to 50 processes running simultaneously. To calculate the total number of processes in a runtime configuration, add the values of the <code>ConcurrentExecutions</code> parameter for each <a>ServerProcess</a> object.</p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RuntimeConfiguration {
    /// <p>The maximum amount of time (in seconds) that a game session can remain in status <code>ACTIVATING</code>. If the game session is not active before the timeout, activation is terminated and the game session status is changed to <code>TERMINATED</code>.</p>
    #[serde(rename = "GameSessionActivationTimeoutSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_activation_timeout_seconds: Option<i64>,
    /// <p>The maximum number of game sessions with status <code>ACTIVATING</code> to allow on an instance simultaneously. This setting limits the amount of instance resources that can be used for new game activations at any one time.</p>
    #[serde(rename = "MaxConcurrentGameSessionActivations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_game_session_activations: Option<i64>,
    /// <p>A collection of server process configurations that describe which server processes to run on each instance in a fleet.</p>
    #[serde(rename = "ServerProcesses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_processes: Option<Vec<ServerProcess>>,
}

/// <p>The location in S3 where build or script files are stored for access by Amazon GameLift. This location is specified in <a>CreateBuild</a>, <a>CreateScript</a>, and <a>UpdateScript</a> requests. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct S3Location {
    /// <p><p>An S3 bucket identifier. This is the name of the S3 bucket.</p> <note> <p>GameLift currently does not support uploading from S3 buckets with names that contain a dot (.).</p> </note></p>
    #[serde(rename = "Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// <p>The name of the zip file that contains the build files or script files. </p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The version of the file, if object versioning is turned on for the bucket. Amazon GameLift uses this information when retrieving files from an S3 bucket that you own. Use this parameter to specify a specific version of the file. If not set, the latest version of the file is retrieved. </p>
    #[serde(rename = "ObjectVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_version: Option<String>,
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) for an IAM role that allows Amazon GameLift to access the S3 bucket.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p><p>Rule that controls how a fleet is scaled. Scaling policies are uniquely identified by the combination of name and fleet ID.</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> <li> <p>Manage scaling policies:</p> <ul> <li> <p> <a>PutScalingPolicy</a> (auto-scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (auto-scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (auto-scaling)</p> </li> </ul> </li> <li> <p>Manage fleet actions:</p> <ul> <li> <p> <a>StartFleetActions</a> </p> </li> <li> <p> <a>StopFleetActions</a> </p> </li> </ul> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ScalingPolicy {
    /// <p>Comparison operator to use when measuring a metric against the threshold value.</p>
    #[serde(rename = "ComparisonOperator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<String>,
    /// <p>Length of time (in minutes) the metric must be at or beyond the threshold before a scaling event is triggered.</p>
    #[serde(rename = "EvaluationPeriods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_periods: Option<i64>,
    /// <p>A unique identifier for a fleet that is associated with this scaling policy.</p>
    #[serde(rename = "FleetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_id: Option<String>,
    /// <p><p>Name of the Amazon GameLift-defined metric that is used to trigger a scaling adjustment. For detailed descriptions of fleet metrics, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/monitoring-cloudwatch.html">Monitor Amazon GameLift with Amazon CloudWatch</a>. </p> <ul> <li> <p> <b>ActivatingGameSessions</b> -- Game sessions in the process of being created.</p> </li> <li> <p> <b>ActiveGameSessions</b> -- Game sessions that are currently running.</p> </li> <li> <p> <b>ActiveInstances</b> -- Fleet instances that are currently running at least one game session.</p> </li> <li> <p> <b>AvailableGameSessions</b> -- Additional game sessions that fleet could host simultaneously, given current capacity.</p> </li> <li> <p> <b>AvailablePlayerSessions</b> -- Empty player slots in currently active game sessions. This includes game sessions that are not currently accepting players. Reserved player slots are not included.</p> </li> <li> <p> <b>CurrentPlayerSessions</b> -- Player slots in active game sessions that are being used by a player or are reserved for a player. </p> </li> <li> <p> <b>IdleInstances</b> -- Active instances that are currently hosting zero game sessions. </p> </li> <li> <p> <b>PercentAvailableGameSessions</b> -- Unused percentage of the total number of game sessions that a fleet could host simultaneously, given current capacity. Use this metric for a target-based scaling policy.</p> </li> <li> <p> <b>PercentIdleInstances</b> -- Percentage of the total number of active instances that are hosting zero game sessions.</p> </li> <li> <p> <b>QueueDepth</b> -- Pending game session placement requests, in any queue, where the current fleet is the top-priority destination.</p> </li> <li> <p> <b>WaitTime</b> -- Current wait time for pending game session placement requests, in any queue, where the current fleet is the top-priority destination. </p> </li> </ul></p>
    #[serde(rename = "MetricName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    /// <p>A descriptive label that is associated with a scaling policy. Policy names do not need to be unique.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type of scaling policy to create. For a target-based policy, set the parameter <i>MetricName</i> to 'PercentAvailableGameSessions' and specify a <i>TargetConfiguration</i>. For a rule-based policy set the following parameters: <i>MetricName</i>, <i>ComparisonOperator</i>, <i>Threshold</i>, <i>EvaluationPeriods</i>, <i>ScalingAdjustmentType</i>, and <i>ScalingAdjustment</i>.</p>
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    /// <p>Amount of adjustment to make, based on the scaling adjustment type.</p>
    #[serde(rename = "ScalingAdjustment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_adjustment: Option<i64>,
    /// <p><p>The type of adjustment to make to a fleet&#39;s instance count (see <a>FleetCapacity</a>):</p> <ul> <li> <p> <b>ChangeInCapacity</b> -- add (or subtract) the scaling adjustment value from the current instance count. Positive values scale up while negative values scale down.</p> </li> <li> <p> <b>ExactCapacity</b> -- set the instance count to the scaling adjustment value.</p> </li> <li> <p> <b>PercentChangeInCapacity</b> -- increase or reduce the current instance count by the scaling adjustment, read as a percentage. Positive values scale up while negative values scale down.</p> </li> </ul></p>
    #[serde(rename = "ScalingAdjustmentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_adjustment_type: Option<String>,
    /// <p><p>Current status of the scaling policy. The scaling policy can be in force only when in an <code>ACTIVE</code> status. Scaling policies can be suspended for individual fleets (see <a>StopFleetActions</a>; if suspended for a fleet, the policy status does not change. View a fleet&#39;s stopped actions by calling <a>DescribeFleetCapacity</a>.</p> <ul> <li> <p> <b>ACTIVE</b> -- The scaling policy can be used for auto-scaling a fleet.</p> </li> <li> <p> <b>UPDATE<em>REQUESTED</b> -- A request to update the scaling policy has been received.</p> </li> <li> <p> <b>UPDATING</b> -- A change is being made to the scaling policy.</p> </li> <li> <p> <b>DELETE</em>REQUESTED</b> -- A request to delete the scaling policy has been received.</p> </li> <li> <p> <b>DELETING</b> -- The scaling policy is being deleted.</p> </li> <li> <p> <b>DELETED</b> -- The scaling policy has been deleted.</p> </li> <li> <p> <b>ERROR</b> -- An error occurred in creating the policy. It should be removed and recreated.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The settings for a target-based scaling policy.</p>
    #[serde(rename = "TargetConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_configuration: Option<TargetConfiguration>,
    /// <p>Metric value used to trigger a scaling event.</p>
    #[serde(rename = "Threshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
}

/// <p><p>Properties describing a Realtime script.</p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateScript</a> </p> </li> <li> <p> <a>ListScripts</a> </p> </li> <li> <p> <a>DescribeScript</a> </p> </li> <li> <p> <a>UpdateScript</a> </p> </li> <li> <p> <a>DeleteScript</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Script {
    /// <p>A time stamp indicating when this data object was created. The format is a number expressed in Unix time as milliseconds (for example "1469498468.057").</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>A descriptive label that is associated with a script. Script names do not need to be unique.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Amazon Resource Name (<a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a>) that is assigned to a GameLift script resource and uniquely identifies it. ARNs are unique across all Regions. In a GameLift script ARN, the resource ID matches the <i>ScriptId</i> value.</p>
    #[serde(rename = "ScriptArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_arn: Option<String>,
    /// <p>A unique identifier for a Realtime script</p>
    #[serde(rename = "ScriptId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_id: Option<String>,
    /// <p>The file size of the uploaded Realtime script, expressed in bytes. When files are uploaded from an S3 location, this value remains at "0".</p>
    #[serde(rename = "SizeOnDisk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_on_disk: Option<i64>,
    #[serde(rename = "StorageLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_location: Option<S3Location>,
    /// <p>The version that is associated with a build or script. Version strings do not need to be unique.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SearchGameSessionsInput {
    /// <p>A unique identifier for an alias associated with the fleet to search for active game sessions. You can use either the alias ID or ARN value. Each request must reference either a fleet ID or alias ID, but not both.</p>
    #[serde(rename = "AliasId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_id: Option<String>,
    /// <p>String containing the search criteria for the session search. If no filter expression is included, the request returns results for all game sessions in the fleet that are in <code>ACTIVE</code> status.</p> <p>A filter expression can contain one or multiple conditions. Each condition consists of the following:</p> <ul> <li> <p> <b>Operand</b> -- Name of a game session attribute. Valid values are <code>gameSessionName</code>, <code>gameSessionId</code>, <code>gameSessionProperties</code>, <code>maximumSessions</code>, <code>creationTimeMillis</code>, <code>playerSessionCount</code>, <code>hasAvailablePlayerSessions</code>.</p> </li> <li> <p> <b>Comparator</b> -- Valid comparators are: <code>=</code>, <code>&lt;&gt;</code>, <code>&lt;</code>, <code>&gt;</code>, <code>&lt;=</code>, <code>&gt;=</code>. </p> </li> <li> <p> <b>Value</b> -- Value to be searched for. Values may be numbers, boolean values (true/false) or strings depending on the operand. String values are case sensitive and must be enclosed in single quotes. Special characters must be escaped. Boolean and string values can only be used with the comparators <code>=</code> and <code>&lt;&gt;</code>. For example, the following filter expression searches on <code>gameSessionName</code>: "<code>FilterExpression": "gameSessionName = 'Matt\\'s Awesome Game 1'"</code>. </p> </li> </ul> <p>To chain multiple conditions in a single expression, use the logical keywords <code>AND</code>, <code>OR</code>, and <code>NOT</code> and parentheses as needed. For example: <code>x AND y AND NOT z</code>, <code>NOT (x OR y)</code>.</p> <p>Session search evaluates conditions from left to right using the following precedence rules:</p> <ol> <li> <p> <code>=</code>, <code>&lt;&gt;</code>, <code>&lt;</code>, <code>&gt;</code>, <code>&lt;=</code>, <code>&gt;=</code> </p> </li> <li> <p>Parentheses</p> </li> <li> <p>NOT</p> </li> <li> <p>AND</p> </li> <li> <p>OR</p> </li> </ol> <p>For example, this filter expression retrieves game sessions hosting at least ten players that have an open player slot: <code>"maximumSessions&gt;=10 AND hasAvailablePlayerSessions=true"</code>. </p>
    #[serde(rename = "FilterExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    /// <p>A unique identifier for a fleet to search for active game sessions. You can use either the fleet ID or ARN value. Each request must reference either a fleet ID or alias ID, but not both.</p>
    #[serde(rename = "FleetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_id: Option<String>,
    /// <p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages. The maximum number of results returned is 20, even if this value is not set or is set higher than 20. </p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Instructions on how to sort the search results. If no sort expression is included, the request returns results in random order. A sort expression consists of the following elements:</p> <ul> <li> <p> <b>Operand</b> -- Name of a game session attribute. Valid values are <code>gameSessionName</code>, <code>gameSessionId</code>, <code>gameSessionProperties</code>, <code>maximumSessions</code>, <code>creationTimeMillis</code>, <code>playerSessionCount</code>, <code>hasAvailablePlayerSessions</code>.</p> </li> <li> <p> <b>Order</b> -- Valid sort orders are <code>ASC</code> (ascending) and <code>DESC</code> (descending).</p> </li> </ul> <p>For example, this sort expression returns the oldest active sessions first: <code>"SortExpression": "creationTimeMillis ASC"</code>. Results with a null value for the sort operand are returned at the end of the list.</p>
    #[serde(rename = "SortExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_expression: Option<String>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SearchGameSessionsOutput {
    /// <p>A collection of objects containing game session properties for each session matching the request.</p>
    #[serde(rename = "GameSessions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_sessions: Option<Vec<GameSession>>,
    /// <p>Token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>A set of instructions for launching server processes on each instance in a fleet. Server processes run either a custom game build executable or a Realtime Servers script. Each instruction set identifies the location of the custom game build executable or Realtime launch script, optional launch parameters, and the number of server processes with this configuration to maintain concurrently on the instance. Server process configurations make up a fleet's <code> <a>RuntimeConfiguration</a> </code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ServerProcess {
    /// <p>The number of server processes that use this configuration to run concurrently on an instance.</p>
    #[serde(rename = "ConcurrentExecutions")]
    pub concurrent_executions: i64,
    /// <p><p>The location of the server executable in a custom game build or the name of the Realtime script file that contains the <code>Init()</code> function. Game builds and Realtime scripts are installed on instances at the root: </p> <ul> <li> <p>Windows (for custom game builds only): <code>C:\game</code>. Example: &quot;<code>C:\game\MyGame\server.exe</code>&quot; </p> </li> <li> <p>Linux: <code>/local/game</code>. Examples: &quot;<code>/local/game/MyGame/server.exe</code>&quot; or &quot;<code>/local/game/MyRealtimeScript.js</code>&quot;</p> </li> </ul></p>
    #[serde(rename = "LaunchPath")]
    pub launch_path: String,
    /// <p>An optional list of parameters to pass to the server executable or Realtime script on launch.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartFleetActionsInput {
    /// <p>List of actions to restart on the fleet.</p>
    #[serde(rename = "Actions")]
    pub actions: Vec<String>,
    /// <p>A unique identifier for a fleet to start actions on. You can use either the fleet ID or ARN value.</p>
    #[serde(rename = "FleetId")]
    pub fleet_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartFleetActionsOutput {}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartGameSessionPlacementInput {
    /// <p>Set of information on each player to create a player session for.</p>
    #[serde(rename = "DesiredPlayerSessions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_player_sessions: Option<Vec<DesiredPlayerSession>>,
    /// <p>Set of custom properties for a game session, formatted as key:value pairs. These properties are passed to a game server process in the <a>GameSession</a> object with a request to start a new game session (see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession">Start a Game Session</a>).</p>
    #[serde(rename = "GameProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_properties: Option<Vec<GameProperty>>,
    /// <p>Set of custom game session properties, formatted as a single string value. This data is passed to a game server process in the <a>GameSession</a> object with a request to start a new game session (see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession">Start a Game Session</a>).</p>
    #[serde(rename = "GameSessionData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_data: Option<String>,
    /// <p>A descriptive label that is associated with a game session. Session names do not need to be unique.</p>
    #[serde(rename = "GameSessionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_name: Option<String>,
    /// <p>Name of the queue to use to place the new game session. You can use either the queue name or ARN value. </p>
    #[serde(rename = "GameSessionQueueName")]
    pub game_session_queue_name: String,
    /// <p>The maximum number of players that can be connected simultaneously to the game session.</p>
    #[serde(rename = "MaximumPlayerSessionCount")]
    pub maximum_player_session_count: i64,
    /// <p>A unique identifier to assign to the new game session placement. This value is developer-defined. The value must be unique across all Regions and cannot be reused unless you are resubmitting a canceled or timed-out placement request.</p>
    #[serde(rename = "PlacementId")]
    pub placement_id: String,
    /// <p>Set of values, expressed in milliseconds, indicating the amount of latency that a player experiences when connected to AWS Regions. This information is used to try to place the new game session where it can offer the best possible gameplay experience for the players. </p>
    #[serde(rename = "PlayerLatencies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_latencies: Option<Vec<PlayerLatency>>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartGameSessionPlacementOutput {
    /// <p>Object that describes the newly created game session placement. This object includes all the information provided in the request, as well as start/end time stamps and placement status. </p>
    #[serde(rename = "GameSessionPlacement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_placement: Option<GameSessionPlacement>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartMatchBackfillInput {
    /// <p>Name of the matchmaker to use for this request. You can use either the configuration name or ARN value. The ARN of the matchmaker that was used with the original game session is listed in the <a>GameSession</a> object, <code>MatchmakerData</code> property.</p>
    #[serde(rename = "ConfigurationName")]
    pub configuration_name: String,
    /// <p>Amazon Resource Name (<a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a>) that is assigned to a game session and uniquely identifies it. This is the same as the game session ID.</p>
    #[serde(rename = "GameSessionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_arn: Option<String>,
    /// <p><p>Match information on all players that are currently assigned to the game session. This information is used by the matchmaker to find new players and add them to the existing game.</p> <ul> <li> <p>PlayerID, PlayerAttributes, Team -\- This information is maintained in the <a>GameSession</a> object, <code>MatchmakerData</code> property, for all players who are currently assigned to the game session. The matchmaker data is in JSON syntax, formatted as a string. For more details, see <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-server.html#match-server-data"> Match Data</a>. </p> </li> <li> <p>LatencyInMs -\- If the matchmaker uses player latency, include a latency value, in milliseconds, for the Region that the game session is currently in. Do not include latency values for any other Region.</p> </li> </ul></p>
    #[serde(rename = "Players")]
    pub players: Vec<Player>,
    /// <p>A unique identifier for a matchmaking ticket. If no ticket ID is specified here, Amazon GameLift will generate one in the form of a UUID. Use this identifier to track the match backfill ticket status and retrieve match results.</p>
    #[serde(rename = "TicketId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket_id: Option<String>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartMatchBackfillOutput {
    /// <p>Ticket representing the backfill matchmaking request. This object includes the information in the request, ticket status, and match results as generated during the matchmaking process.</p>
    #[serde(rename = "MatchmakingTicket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matchmaking_ticket: Option<MatchmakingTicket>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartMatchmakingInput {
    /// <p>Name of the matchmaking configuration to use for this request. Matchmaking configurations must exist in the same Region as this request. You can use either the configuration name or ARN value.</p>
    #[serde(rename = "ConfigurationName")]
    pub configuration_name: String,
    /// <p>Information on each player to be matched. This information must include a player ID, and may contain player attributes and latency data to be used in the matchmaking process. After a successful match, <code>Player</code> objects contain the name of the team the player is assigned to.</p>
    #[serde(rename = "Players")]
    pub players: Vec<Player>,
    /// <p>A unique identifier for a matchmaking ticket. If no ticket ID is specified here, Amazon GameLift will generate one in the form of a UUID. Use this identifier to track the matchmaking ticket status and retrieve match results.</p>
    #[serde(rename = "TicketId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket_id: Option<String>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartMatchmakingOutput {
    /// <p>Ticket representing the matchmaking request. This object include the information included in the request, ticket status, and match results as generated during the matchmaking process.</p>
    #[serde(rename = "MatchmakingTicket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matchmaking_ticket: Option<MatchmakingTicket>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopFleetActionsInput {
    /// <p>List of actions to suspend on the fleet. </p>
    #[serde(rename = "Actions")]
    pub actions: Vec<String>,
    /// <p>A unique identifier for a fleet to stop actions on. You can use either the fleet ID or ARN value.</p>
    #[serde(rename = "FleetId")]
    pub fleet_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopFleetActionsOutput {}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopGameSessionPlacementInput {
    /// <p>A unique identifier for a game session placement to cancel.</p>
    #[serde(rename = "PlacementId")]
    pub placement_id: String,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopGameSessionPlacementOutput {
    /// <p>Object that describes the canceled game session placement, with <code>CANCELLED</code> status and an end time stamp. </p>
    #[serde(rename = "GameSessionPlacement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_placement: Option<GameSessionPlacement>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopMatchmakingInput {
    /// <p>A unique identifier for a matchmaking ticket.</p>
    #[serde(rename = "TicketId")]
    pub ticket_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopMatchmakingOutput {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SuspendGameServerGroupInput {
    /// <p>A unique identifier for the game server group. Use either the <a>GameServerGroup</a> name or ARN value.</p>
    #[serde(rename = "GameServerGroupName")]
    pub game_server_group_name: String,
    /// <p>The activity to suspend for this game server group.</p>
    #[serde(rename = "SuspendActions")]
    pub suspend_actions: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SuspendGameServerGroupOutput {
    /// <p>An object that describes the game server group resource, with the <code>SuspendedActions</code> property updated to reflect the suspended activity.</p>
    #[serde(rename = "GameServerGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_server_group: Option<GameServerGroup>,
}

/// <p><p> A label that can be assigned to a GameLift resource. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS Resources</a> in the <i>AWS General Reference</i> </p> <p> <a href="http://aws.amazon.com/answers/account-management/aws-tagging-strategies/"> AWS Tagging Strategies</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>TagResource</a> </p> </li> <li> <p> <a>UntagResource</a> </p> </li> <li> <p> <a>ListTagsForResource</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p> The key for a developer-defined key:value pair for tagging an AWS resource. </p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p> The value for a developer-defined key:value pair for tagging an AWS resource. </p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p> The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) that is assigned to and uniquely identifies the GameLift resource that you want to assign tags to. GameLift resource ARNs are included in the data object for the resource, which can be retrieved by calling a List or Describe operation for the resource type. </p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>A list of one or more tags to assign to the specified GameLift resource. Tags are developer-defined and structured as key-value pairs. The maximum tag limit may be lower than stated. See <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html"> Tagging AWS Resources</a> for actual tagging limits.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p><p>Settings for a target-based scaling policy (see <a>ScalingPolicy</a>. A target-based policy tracks a particular fleet metric specifies a target value for the metric. As player usage changes, the policy triggers Amazon GameLift to adjust capacity so that the metric returns to the target value. The target configuration specifies settings as needed for the target based policy, including the target value. </p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> <li> <p>Manage scaling policies:</p> <ul> <li> <p> <a>PutScalingPolicy</a> (auto-scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (auto-scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (auto-scaling)</p> </li> </ul> </li> <li> <p>Manage fleet actions:</p> <ul> <li> <p> <a>StartFleetActions</a> </p> </li> <li> <p> <a>StopFleetActions</a> </p> </li> </ul> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TargetConfiguration {
    /// <p>Desired value to use with a target-based scaling policy. The value must be relevant for whatever metric the scaling policy is using. For example, in a policy using the metric PercentAvailableGameSessions, the target value should be the preferred size of the fleet's buffer (the percent of capacity that should be idle and ready for new game sessions).</p>
    #[serde(rename = "TargetValue")]
    pub target_value: f64,
}

/// <p> <b>This data type is used with the Amazon GameLift FleetIQ and game server groups.</b> </p> <p>Settings for a target-based scaling policy as part of a <a>GameServerGroupAutoScalingPolicy</a>. These settings are used to create a target-based policy that tracks the GameLift FleetIQ metric <code>"PercentUtilizedGameServers"</code> and specifies a target value for the metric. As player usage changes, the policy triggers to adjust the game server group capacity so that the metric returns to the target value. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TargetTrackingConfiguration {
    /// <p>Desired value to use with a game server group target-based scaling policy. </p>
    #[serde(rename = "TargetValue")]
    pub target_value: f64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) that is assigned to and uniquely identifies the GameLift resource that you want to remove tags from. GameLift resource ARNs are included in the data object for the resource, which can be retrieved by calling a List or Describe operation for the resource type. </p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>A list of one or more tag keys to remove from the specified GameLift resource. An AWS resource can have only one tag with a specific tag key, so specifying the tag key identifies which tag to remove. </p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateAliasInput {
    /// <p>A unique identifier for the alias that you want to update. You can use either the alias ID or ARN value.</p>
    #[serde(rename = "AliasId")]
    pub alias_id: String,
    /// <p>A human-readable description of the alias.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A descriptive label that is associated with an alias. Alias names do not need to be unique.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The routing configuration, including routing type and fleet target, for the alias.</p>
    #[serde(rename = "RoutingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_strategy: Option<RoutingStrategy>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateAliasOutput {
    /// <p>The updated alias resource.</p>
    #[serde(rename = "Alias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<Alias>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateBuildInput {
    /// <p>A unique identifier for a build to update. You can use either the build ID or ARN value. </p>
    #[serde(rename = "BuildId")]
    pub build_id: String,
    /// <p>A descriptive label that is associated with a build. Build names do not need to be unique. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Version information that is associated with a build or script. Version strings do not need to be unique.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateBuildOutput {
    /// <p>The updated build resource.</p>
    #[serde(rename = "Build")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<Build>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFleetAttributesInput {
    /// <p>Human-readable description of a fleet.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A unique identifier for a fleet to update attribute metadata for. You can use either the fleet ID or ARN value.</p>
    #[serde(rename = "FleetId")]
    pub fleet_id: String,
    /// <p>Names of metric groups to include this fleet in. Amazon CloudWatch uses a fleet metric group is to aggregate metrics from multiple fleets. Use an existing metric group name to add this fleet to the group. Or use a new name to create a new metric group. A fleet can only be included in one metric group at a time.</p>
    #[serde(rename = "MetricGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_groups: Option<Vec<String>>,
    /// <p>A descriptive label that is associated with a fleet. Fleet names do not need to be unique.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>Game session protection policy to apply to all new instances created in this fleet. Instances that already exist are not affected. You can set protection for individual instances using <a>UpdateGameSession</a>.</p> <ul> <li> <p> <b>NoProtection</b> -- The game session can be terminated during a scale-down event.</p> </li> <li> <p> <b>FullProtection</b> -- If the game session is in an <code>ACTIVE</code> status, it cannot be terminated during a scale-down event.</p> </li> </ul></p>
    #[serde(rename = "NewGameSessionProtectionPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_game_session_protection_policy: Option<String>,
    /// <p>Policy that limits the number of game sessions an individual player can create over a span of time. </p>
    #[serde(rename = "ResourceCreationLimitPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_creation_limit_policy: Option<ResourceCreationLimitPolicy>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateFleetAttributesOutput {
    /// <p>A unique identifier for a fleet that was updated. Use either the fleet ID or ARN value.</p>
    #[serde(rename = "FleetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_id: Option<String>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFleetCapacityInput {
    /// <p>Number of EC2 instances you want this fleet to host.</p>
    #[serde(rename = "DesiredInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_instances: Option<i64>,
    /// <p>A unique identifier for a fleet to update capacity for. You can use either the fleet ID or ARN value.</p>
    #[serde(rename = "FleetId")]
    pub fleet_id: String,
    /// <p>The maximum value allowed for the fleet's instance count. Default if not set is 1.</p>
    #[serde(rename = "MaxSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<i64>,
    /// <p>The minimum value allowed for the fleet's instance count. Default if not set is 0.</p>
    #[serde(rename = "MinSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<i64>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateFleetCapacityOutput {
    /// <p>A unique identifier for a fleet that was updated.</p>
    #[serde(rename = "FleetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_id: Option<String>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFleetPortSettingsInput {
    /// <p>A unique identifier for a fleet to update port settings for. You can use either the fleet ID or ARN value.</p>
    #[serde(rename = "FleetId")]
    pub fleet_id: String,
    /// <p>A collection of port settings to be added to the fleet resource.</p>
    #[serde(rename = "InboundPermissionAuthorizations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_permission_authorizations: Option<Vec<IpPermission>>,
    /// <p>A collection of port settings to be removed from the fleet resource.</p>
    #[serde(rename = "InboundPermissionRevocations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_permission_revocations: Option<Vec<IpPermission>>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateFleetPortSettingsOutput {
    /// <p>A unique identifier for a fleet that was updated.</p>
    #[serde(rename = "FleetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateGameServerGroupInput {
    /// <p><p>Indicates how GameLift FleetIQ balances the use of Spot Instances and On-Demand Instances in the game server group. Method options include the following:</p> <ul> <li> <p> <code>SPOT<em>ONLY</code> - Only Spot Instances are used in the game server group. If Spot Instances are unavailable or not viable for game hosting, the game server group provides no hosting capacity until Spot Instances can again be used. Until then, no new instances are started, and the existing nonviable Spot Instances are terminated (after current gameplay ends) and are not replaced.</p> </li> <li> <p> <code>SPOT</em>PREFERRED</code> - (default value) Spot Instances are used whenever available in the game server group. If Spot Instances are unavailable, the game server group continues to provide hosting capacity by falling back to On-Demand Instances. Existing nonviable Spot Instances are terminated (after current gameplay ends) and are replaced with new On-Demand Instances.</p> </li> <li> <p> <code>ON<em>DEMAND</em>ONLY</code> - Only On-Demand Instances are used in the game server group. No Spot Instances are used, even when available, while this balancing strategy is in force.</p> </li> </ul></p>
    #[serde(rename = "BalancingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balancing_strategy: Option<String>,
    /// <p>A unique identifier for the game server group. Use either the <a>GameServerGroup</a> name or ARN value.</p>
    #[serde(rename = "GameServerGroupName")]
    pub game_server_group_name: String,
    /// <p>A flag that indicates whether instances in the game server group are protected from early termination. Unprotected instances that have active game servers running might be terminated during a scale-down event, causing players to be dropped from the game. Protected instances cannot be terminated while there are active game servers running except in the event of a forced game server group deletion (see ). An exception to this is with Spot Instances, which can be terminated by AWS regardless of protection status. This property is set to <code>NO_PROTECTION</code> by default.</p>
    #[serde(rename = "GameServerProtectionPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_server_protection_policy: Option<String>,
    /// <p>An updated list of EC2 instance types to use in the Auto Scaling group. The instance definitions must specify at least two different instance types that are supported by GameLift FleetIQ. This updated list replaces the entire current list of instance definitions for the game server group. For more information on instance types, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html">EC2 Instance Types</a> in the <i>Amazon EC2 User Guide</i>. You can optionally specify capacity weighting for each instance type. If no weight value is specified for an instance type, it is set to the default value "1". For more information about capacity weighting, see <a href="https://docs.aws.amazon.com/autoscaling/ec2/userguide/asg-instance-weighting.html"> Instance Weighting for Amazon EC2 Auto Scaling</a> in the Amazon EC2 Auto Scaling User Guide.</p>
    #[serde(rename = "InstanceDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_definitions: Option<Vec<InstanceDefinition>>,
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) for an IAM role that allows Amazon GameLift to access your EC2 Auto Scaling groups.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateGameServerGroupOutput {
    /// <p>An object that describes the game server group resource with updated properties. </p>
    #[serde(rename = "GameServerGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_server_group: Option<GameServerGroup>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateGameServerInput {
    /// <p>A set of custom game server properties, formatted as a single string value. This data is passed to a game client or service when it requests information on game servers using <a>ListGameServers</a> or <a>ClaimGameServer</a>. </p>
    #[serde(rename = "GameServerData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_server_data: Option<String>,
    /// <p>A unique identifier for the game server group where the game server is running. Use either the <a>GameServerGroup</a> name or ARN value.</p>
    #[serde(rename = "GameServerGroupName")]
    pub game_server_group_name: String,
    /// <p>A custom string that uniquely identifies the game server to update.</p>
    #[serde(rename = "GameServerId")]
    pub game_server_id: String,
    /// <p>Indicates health status of the game server. A request that includes this parameter updates the game server's <i>LastHealthCheckTime</i> timestamp. </p>
    #[serde(rename = "HealthCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<String>,
    /// <p>Indicates whether the game server is available or is currently hosting gameplay.</p>
    #[serde(rename = "UtilizationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilization_status: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateGameServerOutput {
    /// <p>Object that describes the newly updated game server.</p>
    #[serde(rename = "GameServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_server: Option<GameServer>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateGameSessionInput {
    /// <p>A unique identifier for the game session to update. </p>
    #[serde(rename = "GameSessionId")]
    pub game_session_id: String,
    /// <p>The maximum number of players that can be connected simultaneously to the game session.</p>
    #[serde(rename = "MaximumPlayerSessionCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_player_session_count: Option<i64>,
    /// <p>A descriptive label that is associated with a game session. Session names do not need to be unique.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Policy determining whether or not the game session accepts new players.</p>
    #[serde(rename = "PlayerSessionCreationPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_session_creation_policy: Option<String>,
    /// <p><p>Game session protection policy to apply to this game session only.</p> <ul> <li> <p> <b>NoProtection</b> -- The game session can be terminated during a scale-down event.</p> </li> <li> <p> <b>FullProtection</b> -- If the game session is in an <code>ACTIVE</code> status, it cannot be terminated during a scale-down event.</p> </li> </ul></p>
    #[serde(rename = "ProtectionPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_policy: Option<String>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateGameSessionOutput {
    /// <p>The updated game session metadata.</p>
    #[serde(rename = "GameSession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session: Option<GameSession>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateGameSessionQueueInput {
    /// <p>A list of fleets that can be used to fulfill game session placement requests in the queue. Fleets are identified by either a fleet ARN or a fleet alias ARN. Destinations are listed in default preference order. When updating this list, provide a complete list of destinations.</p>
    #[serde(rename = "Destinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<GameSessionQueueDestination>>,
    /// <p>A descriptive label that is associated with game session queue. Queue names must be unique within each Region. You can use either the queue ID or ARN value. </p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A collection of latency policies to apply when processing game sessions placement requests with player latency information. Multiple policies are evaluated in order of the maximum latency value, starting with the lowest latency values. With just one policy, the policy is enforced at the start of the game session placement for the duration period. With multiple policies, each policy is enforced consecutively for its duration period. For example, a queue might enforce a 60-second policy followed by a 120-second policy, and then no policy for the remainder of the placement. When updating policies, provide a complete collection of policies.</p>
    #[serde(rename = "PlayerLatencyPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_latency_policies: Option<Vec<PlayerLatencyPolicy>>,
    /// <p>The maximum time, in seconds, that a new game session placement request remains in the queue. When a request exceeds this time, the game session placement changes to a <code>TIMED_OUT</code> status.</p>
    #[serde(rename = "TimeoutInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_seconds: Option<i64>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateGameSessionQueueOutput {
    /// <p>An object that describes the newly updated game session queue.</p>
    #[serde(rename = "GameSessionQueue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_queue: Option<GameSessionQueue>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateMatchmakingConfigurationInput {
    /// <p>A flag that indicates whether a match that was created with this configuration must be accepted by the matched players. To require acceptance, set to TRUE. With this option enabled, matchmaking tickets use the status <code>REQUIRES_ACCEPTANCE</code> to indicate when a completed potential match is waiting for player acceptance. </p>
    #[serde(rename = "AcceptanceRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance_required: Option<bool>,
    /// <p>The length of time (in seconds) to wait for players to accept a proposed match, if acceptance is required. If any player rejects the match or fails to accept before the timeout, the tickets are returned to the ticket pool and continue to be evaluated for an acceptable match.</p>
    #[serde(rename = "AcceptanceTimeoutSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance_timeout_seconds: Option<i64>,
    /// <p>The number of player slots in a match to keep open for future players. For example, assume that the configuration's rule set specifies a match for a single 12-person team. If the additional player count is set to 2, only 10 players are initially selected for the match. This parameter is not used if <code>FlexMatchMode</code> is set to <code>STANDALONE</code>.</p>
    #[serde(rename = "AdditionalPlayerCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_player_count: Option<i64>,
    /// <p>The method that is used to backfill game sessions created with this matchmaking configuration. Specify MANUAL when your game manages backfill requests manually or does not use the match backfill feature. Specify AUTOMATIC to have GameLift create a <a>StartMatchBackfill</a> request whenever a game session has one or more open slots. Learn more about manual and automatic backfill in <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-backfill.html">Backfill Existing Games with FlexMatch</a>. Automatic backfill is not available when <code>FlexMatchMode</code> is set to <code>STANDALONE</code>.</p>
    #[serde(rename = "BackfillMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backfill_mode: Option<String>,
    /// <p>Information to add to all events related to the matchmaking configuration. </p>
    #[serde(rename = "CustomEventData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_event_data: Option<String>,
    /// <p>A descriptive label that is associated with matchmaking configuration.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p><p>Indicates whether this matchmaking configuration is being used with GameLift hosting or as a standalone matchmaking solution. </p> <ul> <li> <p> <b>STANDALONE</b> - FlexMatch forms matches and returns match information, including players and team assignments, in a <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-events.html#match-events-matchmakingsucceeded"> MatchmakingSucceeded</a> event.</p> </li> <li> <p> <b>WITH_QUEUE</b> - FlexMatch forms matches and uses the specified GameLift queue to start a game session for the match. </p> </li> </ul></p>
    #[serde(rename = "FlexMatchMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex_match_mode: Option<String>,
    /// <p>A set of custom properties for a game session, formatted as key-value pairs. These properties are passed to a game server process in the <a>GameSession</a> object with a request to start a new game session (see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession">Start a Game Session</a>). This information is added to the new <a>GameSession</a> object that is created for a successful match. This parameter is not used if <code>FlexMatchMode</code> is set to <code>STANDALONE</code>.</p>
    #[serde(rename = "GameProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_properties: Option<Vec<GameProperty>>,
    /// <p>A set of custom game session properties, formatted as a single string value. This data is passed to a game server process in the <a>GameSession</a> object with a request to start a new game session (see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-sdk-server-api.html#gamelift-sdk-server-startsession">Start a Game Session</a>). This information is added to the new <a>GameSession</a> object that is created for a successful match. This parameter is not used if <code>FlexMatchMode</code> is set to <code>STANDALONE</code>.</p>
    #[serde(rename = "GameSessionData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_data: Option<String>,
    /// <p>Amazon Resource Name (<a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a>) that is assigned to a GameLift game session queue resource and uniquely identifies it. ARNs are unique across all Regions. Queues can be located in any Region. Queues are used to start new GameLift-hosted game sessions for matches that are created with this matchmaking configuration. If <code>FlexMatchMode</code> is set to <code>STANDALONE</code>, do not set this parameter.</p>
    #[serde(rename = "GameSessionQueueArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_session_queue_arns: Option<Vec<String>>,
    /// <p>A unique identifier for a matchmaking configuration to update. You can use either the configuration name or ARN value. </p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>An SNS topic ARN that is set up to receive matchmaking notifications. See <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-notification.html"> Setting up Notifications for Matchmaking</a> for more information.</p>
    #[serde(rename = "NotificationTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_target: Option<String>,
    /// <p>The maximum duration, in seconds, that a matchmaking ticket can remain in process before timing out. Requests that fail due to timing out can be resubmitted as needed.</p>
    #[serde(rename = "RequestTimeoutSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_timeout_seconds: Option<i64>,
    /// <p>A unique identifier for a matchmaking rule set to use with this configuration. You can use either the rule set name or ARN value. A matchmaking configuration can only use rule sets that are defined in the same Region.</p>
    #[serde(rename = "RuleSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_set_name: Option<String>,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateMatchmakingConfigurationOutput {
    /// <p>The updated matchmaking configuration.</p>
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<MatchmakingConfiguration>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRuntimeConfigurationInput {
    /// <p>A unique identifier for a fleet to update runtime configuration for. You can use either the fleet ID or ARN value.</p>
    #[serde(rename = "FleetId")]
    pub fleet_id: String,
    /// <p>Instructions for launching server processes on each instance in the fleet. Server processes run either a custom game build executable or a Realtime Servers script. The runtime configuration lists the types of server processes to run on an instance and includes the following configuration settings: the server executable or launch script file, launch parameters, and the number of processes to run concurrently on each instance. A CreateFleet request must include a runtime configuration with at least one server process configuration.</p>
    #[serde(rename = "RuntimeConfiguration")]
    pub runtime_configuration: RuntimeConfiguration,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateRuntimeConfigurationOutput {
    /// <p>The runtime configuration currently in force. If the update was successful, this object matches the one in the request.</p>
    #[serde(rename = "RuntimeConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_configuration: Option<RuntimeConfiguration>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateScriptInput {
    /// <p>A descriptive label that is associated with a script. Script names do not need to be unique.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A unique identifier for a Realtime script to update. You can use either the script ID or ARN value.</p>
    #[serde(rename = "ScriptId")]
    pub script_id: String,
    /// <p>The Amazon S3 location of your Realtime scripts. The storage location must specify the S3 bucket name, the zip file name (the "key"), and an IAM role ARN that allows Amazon GameLift to access the S3 storage location. The S3 bucket must be in the same Region as the script you're updating. By default, Amazon GameLift uploads the latest version of the zip file; if you have S3 object versioning turned on, you can use the <code>ObjectVersion</code> parameter to specify an earlier version. To call this operation with a storage location, you must have IAM PassRole permission. For more details on IAM roles and PassRole permissions, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/setting-up-role.html"> Set up a role for GameLift access</a>. </p>
    #[serde(rename = "StorageLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_location: Option<S3Location>,
    /// <p>The version that is associated with a build or script. Version strings do not need to be unique.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>A data object containing your Realtime scripts and dependencies as a zip file. The zip file can have one or multiple files. Maximum size of a zip file is 5 MB.</p> <p>When using the AWS CLI tool to create a script, this parameter is set to the zip file name. It must be prepended with the string "fileb://" to indicate that the file data is a binary object. For example: <code>--zip-file fileb://myRealtimeScript.zip</code>.</p>
    #[serde(rename = "ZipFile")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_file: Option<bytes::Bytes>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateScriptOutput {
    /// <p>The newly created script record with a unique script ID. The new script's storage location reflects an Amazon S3 location: (1) If the script was uploaded from an S3 bucket under your account, the storage location reflects the information that was provided in the <i>CreateScript</i> request; (2) If the script file was uploaded from a local zip file, the storage location reflects an S3 location controls by the Amazon GameLift service.</p>
    #[serde(rename = "Script")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<Script>,
}

/// <p>Represents the input for a request operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ValidateMatchmakingRuleSetInput {
    /// <p>A collection of matchmaking rules to validate, formatted as a JSON string.</p>
    #[serde(rename = "RuleSetBody")]
    pub rule_set_body: String,
}

/// <p>Represents the returned data in response to a request operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ValidateMatchmakingRuleSetOutput {
    /// <p>A response indicating whether the rule set is valid.</p>
    #[serde(rename = "Valid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
}

/// <p><p>Represents an authorization for a VPC peering connection between the VPC for an Amazon GameLift fleet and another VPC on an account you have access to. This authorization must exist and be valid for the peering connection to be established. Authorizations are valid for 24 hours after they are issued.</p> <ul> <li> <p> <a>CreateVpcPeeringAuthorization</a> </p> </li> <li> <p> <a>DescribeVpcPeeringAuthorizations</a> </p> </li> <li> <p> <a>DeleteVpcPeeringAuthorization</a> </p> </li> <li> <p> <a>CreateVpcPeeringConnection</a> </p> </li> <li> <p> <a>DescribeVpcPeeringConnections</a> </p> </li> <li> <p> <a>DeleteVpcPeeringConnection</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VpcPeeringAuthorization {
    /// <p>Time stamp indicating when this authorization was issued. Format is a number expressed in Unix time as milliseconds (for example "1469498468.057").</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>Time stamp indicating when this authorization expires (24 hours after issuance). Format is a number expressed in Unix time as milliseconds (for example "1469498468.057").</p>
    #[serde(rename = "ExpirationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<f64>,
    /// <p>A unique identifier for the AWS account that you use to manage your Amazon GameLift fleet. You can find your Account ID in the AWS Management Console under account settings.</p>
    #[serde(rename = "GameLiftAwsAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_lift_aws_account_id: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "PeerVpcAwsAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_vpc_aws_account_id: Option<String>,
    /// <p>A unique identifier for a VPC with resources to be accessed by your Amazon GameLift fleet. The VPC must be in the same Region where your fleet is deployed. Look up a VPC ID using the <a href="https://console.aws.amazon.com/vpc/">VPC Dashboard</a> in the AWS Management Console. Learn more about VPC peering in <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/vpc-peering.html">VPC Peering with Amazon GameLift Fleets</a>.</p>
    #[serde(rename = "PeerVpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_vpc_id: Option<String>,
}

/// <p><p>Represents a peering connection between a VPC on one of your AWS accounts and the VPC for your Amazon GameLift fleets. This record may be for an active peering connection or a pending connection that has not yet been established.</p> <ul> <li> <p> <a>CreateVpcPeeringAuthorization</a> </p> </li> <li> <p> <a>DescribeVpcPeeringAuthorizations</a> </p> </li> <li> <p> <a>DeleteVpcPeeringAuthorization</a> </p> </li> <li> <p> <a>CreateVpcPeeringConnection</a> </p> </li> <li> <p> <a>DescribeVpcPeeringConnections</a> </p> </li> <li> <p> <a>DeleteVpcPeeringConnection</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VpcPeeringConnection {
    /// <p> The Amazon Resource Name (<a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a>) associated with the GameLift fleet resource for this connection. </p>
    #[serde(rename = "FleetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_arn: Option<String>,
    /// <p>A unique identifier for a fleet. This ID determines the ID of the Amazon GameLift VPC for your fleet.</p>
    #[serde(rename = "FleetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet_id: Option<String>,
    /// <p>A unique identifier for the VPC that contains the Amazon GameLift fleet for this connection. This VPC is managed by Amazon GameLift and does not appear in your AWS account. </p>
    #[serde(rename = "GameLiftVpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_lift_vpc_id: Option<String>,
    /// <p>CIDR block of IPv4 addresses assigned to the VPC peering connection for the GameLift VPC. The peered VPC also has an IPv4 CIDR block associated with it; these blocks cannot overlap or the peering connection cannot be created. </p>
    #[serde(rename = "IpV4CidrBlock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_v4_cidr_block: Option<String>,
    /// <p>A unique identifier for a VPC with resources to be accessed by your Amazon GameLift fleet. The VPC must be in the same Region where your fleet is deployed. Look up a VPC ID using the <a href="https://console.aws.amazon.com/vpc/">VPC Dashboard</a> in the AWS Management Console. Learn more about VPC peering in <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/vpc-peering.html">VPC Peering with Amazon GameLift Fleets</a>.</p>
    #[serde(rename = "PeerVpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_vpc_id: Option<String>,
    /// <p>The status information about the connection. Status indicates if a connection is pending, successful, or failed.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<VpcPeeringConnectionStatus>,
    /// <p>A unique identifier that is automatically assigned to the connection record. This ID is referenced in VPC peering connection events, and is used when deleting a connection with <a>DeleteVpcPeeringConnection</a>. </p>
    #[serde(rename = "VpcPeeringConnectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_peering_connection_id: Option<String>,
}

/// <p>Represents status information for a VPC peering connection. Status is associated with a <a>VpcPeeringConnection</a> object. Status codes and messages are provided from EC2 (see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_VpcPeeringConnectionStateReason.html">VpcPeeringConnectionStateReason</a>). Connection status information is also communicated as a fleet <a>Event</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VpcPeeringConnectionStatus {
    /// <p>Code indicating the status of a VPC peering connection.</p>
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>Additional messaging associated with the connection status. </p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Errors returned by AcceptMatch
#[derive(Debug, PartialEq)]
pub enum AcceptMatchError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The requested operation is not supported in the Region specified.</p>
    UnsupportedRegion(String),
}

impl AcceptMatchError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AcceptMatchError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(AcceptMatchError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(AcceptMatchError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(AcceptMatchError::NotFound(err.msg))
                }
                "UnsupportedRegionException" => {
                    return RusotoError::Service(AcceptMatchError::UnsupportedRegion(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AcceptMatchError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AcceptMatchError::InternalService(ref cause) => write!(f, "{}", cause),
            AcceptMatchError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            AcceptMatchError::NotFound(ref cause) => write!(f, "{}", cause),
            AcceptMatchError::UnsupportedRegion(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AcceptMatchError {}
/// Errors returned by ClaimGameServer
#[derive(Debug, PartialEq)]
pub enum ClaimGameServerError {
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request.</p>
    Conflict(String),
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The specified game server group has no available game servers to fulfill a <code>ClaimGameServer</code> request. Clients can retry such requests immediately or after a waiting period. </p>
    OutOfCapacity(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl ClaimGameServerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ClaimGameServerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(ClaimGameServerError::Conflict(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(ClaimGameServerError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ClaimGameServerError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ClaimGameServerError::NotFound(err.msg))
                }
                "OutOfCapacityException" => {
                    return RusotoError::Service(ClaimGameServerError::OutOfCapacity(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ClaimGameServerError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ClaimGameServerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ClaimGameServerError::Conflict(ref cause) => write!(f, "{}", cause),
            ClaimGameServerError::InternalService(ref cause) => write!(f, "{}", cause),
            ClaimGameServerError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ClaimGameServerError::NotFound(ref cause) => write!(f, "{}", cause),
            ClaimGameServerError::OutOfCapacity(ref cause) => write!(f, "{}", cause),
            ClaimGameServerError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ClaimGameServerError {}
/// Errors returned by CreateAlias
#[derive(Debug, PartialEq)]
pub enum CreateAliasError {
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request.</p>
    Conflict(String),
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>The requested operation would cause the resource to exceed the allowed service limit. Resolve the issue before retrying.</p>
    LimitExceeded(String),
    /// <p> The requested tagging operation did not succeed. This may be due to invalid tag format or the maximum tag limit may have been exceeded. Resolve the issue before retrying. </p>
    TaggingFailed(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl CreateAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAliasError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(CreateAliasError::Conflict(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateAliasError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateAliasError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateAliasError::LimitExceeded(err.msg))
                }
                "TaggingFailedException" => {
                    return RusotoError::Service(CreateAliasError::TaggingFailed(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateAliasError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateAliasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAliasError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateAliasError::InternalService(ref cause) => write!(f, "{}", cause),
            CreateAliasError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateAliasError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateAliasError::TaggingFailed(ref cause) => write!(f, "{}", cause),
            CreateAliasError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateAliasError {}
/// Errors returned by CreateBuild
#[derive(Debug, PartialEq)]
pub enum CreateBuildError {
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request.</p>
    Conflict(String),
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p> The requested tagging operation did not succeed. This may be due to invalid tag format or the maximum tag limit may have been exceeded. Resolve the issue before retrying. </p>
    TaggingFailed(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl CreateBuildError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateBuildError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(CreateBuildError::Conflict(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateBuildError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateBuildError::InvalidRequest(err.msg))
                }
                "TaggingFailedException" => {
                    return RusotoError::Service(CreateBuildError::TaggingFailed(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateBuildError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateBuildError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateBuildError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateBuildError::InternalService(ref cause) => write!(f, "{}", cause),
            CreateBuildError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateBuildError::TaggingFailed(ref cause) => write!(f, "{}", cause),
            CreateBuildError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateBuildError {}
/// Errors returned by CreateFleet
#[derive(Debug, PartialEq)]
pub enum CreateFleetError {
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request.</p>
    Conflict(String),
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>The requested operation would cause the resource to exceed the allowed service limit. Resolve the issue before retrying.</p>
    LimitExceeded(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p> The requested tagging operation did not succeed. This may be due to invalid tag format or the maximum tag limit may have been exceeded. Resolve the issue before retrying. </p>
    TaggingFailed(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl CreateFleetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateFleetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(CreateFleetError::Conflict(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateFleetError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateFleetError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateFleetError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateFleetError::NotFound(err.msg))
                }
                "TaggingFailedException" => {
                    return RusotoError::Service(CreateFleetError::TaggingFailed(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateFleetError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateFleetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateFleetError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateFleetError::InternalService(ref cause) => write!(f, "{}", cause),
            CreateFleetError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateFleetError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateFleetError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateFleetError::TaggingFailed(ref cause) => write!(f, "{}", cause),
            CreateFleetError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateFleetError {}
/// Errors returned by CreateGameServerGroup
#[derive(Debug, PartialEq)]
pub enum CreateGameServerGroupError {
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request.</p>
    Conflict(String),
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>The requested operation would cause the resource to exceed the allowed service limit. Resolve the issue before retrying.</p>
    LimitExceeded(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl CreateGameServerGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateGameServerGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(CreateGameServerGroupError::Conflict(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateGameServerGroupError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateGameServerGroupError::InvalidRequest(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateGameServerGroupError::LimitExceeded(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateGameServerGroupError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateGameServerGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateGameServerGroupError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateGameServerGroupError::InternalService(ref cause) => write!(f, "{}", cause),
            CreateGameServerGroupError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateGameServerGroupError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateGameServerGroupError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateGameServerGroupError {}
/// Errors returned by CreateGameSession
#[derive(Debug, PartialEq)]
pub enum CreateGameSessionError {
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request.</p>
    Conflict(String),
    /// <p>The specified fleet has no available instances to fulfill a <code>CreateGameSession</code> request. Clients can retry such requests immediately or after a waiting period.</p>
    FleetCapacityExceeded(String),
    /// <p>A game session with this custom ID string already exists in this fleet. Resolve this conflict before retrying this request.</p>
    IdempotentParameterMismatch(String),
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>The requested operation would cause a conflict with the current state of a resource associated with the request and/or the fleet. Resolve the conflict before retrying.</p>
    InvalidFleetStatus(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>The requested operation would cause the resource to exceed the allowed service limit. Resolve the issue before retrying.</p>
    LimitExceeded(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The service is unable to resolve the routing for a particular alias because it has a terminal <a>RoutingStrategy</a> associated with it. The message returned in this exception is the message defined in the routing strategy itself. Such requests should only be retried if the routing strategy for the specified alias is modified. </p>
    TerminalRoutingStrategy(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl CreateGameSessionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateGameSessionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(CreateGameSessionError::Conflict(err.msg))
                }
                "FleetCapacityExceededException" => {
                    return RusotoError::Service(CreateGameSessionError::FleetCapacityExceeded(
                        err.msg,
                    ))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        CreateGameSessionError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateGameSessionError::InternalService(err.msg))
                }
                "InvalidFleetStatusException" => {
                    return RusotoError::Service(CreateGameSessionError::InvalidFleetStatus(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateGameSessionError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateGameSessionError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateGameSessionError::NotFound(err.msg))
                }
                "TerminalRoutingStrategyException" => {
                    return RusotoError::Service(CreateGameSessionError::TerminalRoutingStrategy(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateGameSessionError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateGameSessionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateGameSessionError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateGameSessionError::FleetCapacityExceeded(ref cause) => write!(f, "{}", cause),
            CreateGameSessionError::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateGameSessionError::InternalService(ref cause) => write!(f, "{}", cause),
            CreateGameSessionError::InvalidFleetStatus(ref cause) => write!(f, "{}", cause),
            CreateGameSessionError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateGameSessionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateGameSessionError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateGameSessionError::TerminalRoutingStrategy(ref cause) => write!(f, "{}", cause),
            CreateGameSessionError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateGameSessionError {}
/// Errors returned by CreateGameSessionQueue
#[derive(Debug, PartialEq)]
pub enum CreateGameSessionQueueError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>The requested operation would cause the resource to exceed the allowed service limit. Resolve the issue before retrying.</p>
    LimitExceeded(String),
    /// <p> The requested tagging operation did not succeed. This may be due to invalid tag format or the maximum tag limit may have been exceeded. Resolve the issue before retrying. </p>
    TaggingFailed(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl CreateGameSessionQueueError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateGameSessionQueueError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(CreateGameSessionQueueError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateGameSessionQueueError::InvalidRequest(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateGameSessionQueueError::LimitExceeded(
                        err.msg,
                    ))
                }
                "TaggingFailedException" => {
                    return RusotoError::Service(CreateGameSessionQueueError::TaggingFailed(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateGameSessionQueueError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateGameSessionQueueError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateGameSessionQueueError::InternalService(ref cause) => write!(f, "{}", cause),
            CreateGameSessionQueueError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateGameSessionQueueError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateGameSessionQueueError::TaggingFailed(ref cause) => write!(f, "{}", cause),
            CreateGameSessionQueueError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateGameSessionQueueError {}
/// Errors returned by CreateMatchmakingConfiguration
#[derive(Debug, PartialEq)]
pub enum CreateMatchmakingConfigurationError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>The requested operation would cause the resource to exceed the allowed service limit. Resolve the issue before retrying.</p>
    LimitExceeded(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p> The requested tagging operation did not succeed. This may be due to invalid tag format or the maximum tag limit may have been exceeded. Resolve the issue before retrying. </p>
    TaggingFailed(String),
    /// <p>The requested operation is not supported in the Region specified.</p>
    UnsupportedRegion(String),
}

impl CreateMatchmakingConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateMatchmakingConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(
                        CreateMatchmakingConfigurationError::InternalService(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        CreateMatchmakingConfigurationError::InvalidRequest(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        CreateMatchmakingConfigurationError::LimitExceeded(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateMatchmakingConfigurationError::NotFound(
                        err.msg,
                    ))
                }
                "TaggingFailedException" => {
                    return RusotoError::Service(
                        CreateMatchmakingConfigurationError::TaggingFailed(err.msg),
                    )
                }
                "UnsupportedRegionException" => {
                    return RusotoError::Service(
                        CreateMatchmakingConfigurationError::UnsupportedRegion(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateMatchmakingConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateMatchmakingConfigurationError::InternalService(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateMatchmakingConfigurationError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateMatchmakingConfigurationError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateMatchmakingConfigurationError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateMatchmakingConfigurationError::TaggingFailed(ref cause) => write!(f, "{}", cause),
            CreateMatchmakingConfigurationError::UnsupportedRegion(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateMatchmakingConfigurationError {}
/// Errors returned by CreateMatchmakingRuleSet
#[derive(Debug, PartialEq)]
pub enum CreateMatchmakingRuleSetError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p> The requested tagging operation did not succeed. This may be due to invalid tag format or the maximum tag limit may have been exceeded. Resolve the issue before retrying. </p>
    TaggingFailed(String),
    /// <p>The requested operation is not supported in the Region specified.</p>
    UnsupportedRegion(String),
}

impl CreateMatchmakingRuleSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateMatchmakingRuleSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(CreateMatchmakingRuleSetError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateMatchmakingRuleSetError::InvalidRequest(
                        err.msg,
                    ))
                }
                "TaggingFailedException" => {
                    return RusotoError::Service(CreateMatchmakingRuleSetError::TaggingFailed(
                        err.msg,
                    ))
                }
                "UnsupportedRegionException" => {
                    return RusotoError::Service(CreateMatchmakingRuleSetError::UnsupportedRegion(
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
impl fmt::Display for CreateMatchmakingRuleSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateMatchmakingRuleSetError::InternalService(ref cause) => write!(f, "{}", cause),
            CreateMatchmakingRuleSetError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateMatchmakingRuleSetError::TaggingFailed(ref cause) => write!(f, "{}", cause),
            CreateMatchmakingRuleSetError::UnsupportedRegion(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateMatchmakingRuleSetError {}
/// Errors returned by CreatePlayerSession
#[derive(Debug, PartialEq)]
pub enum CreatePlayerSessionError {
    /// <p>The game instance is currently full and cannot allow the requested player(s) to join. Clients can retry such requests immediately or after a waiting period.</p>
    GameSessionFull(String),
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>The requested operation would cause a conflict with the current state of a resource associated with the request and/or the game instance. Resolve the conflict before retrying.</p>
    InvalidGameSessionStatus(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The service is unable to resolve the routing for a particular alias because it has a terminal <a>RoutingStrategy</a> associated with it. The message returned in this exception is the message defined in the routing strategy itself. Such requests should only be retried if the routing strategy for the specified alias is modified. </p>
    TerminalRoutingStrategy(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl CreatePlayerSessionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePlayerSessionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "GameSessionFullException" => {
                    return RusotoError::Service(CreatePlayerSessionError::GameSessionFull(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreatePlayerSessionError::InternalService(err.msg))
                }
                "InvalidGameSessionStatusException" => {
                    return RusotoError::Service(
                        CreatePlayerSessionError::InvalidGameSessionStatus(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreatePlayerSessionError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreatePlayerSessionError::NotFound(err.msg))
                }
                "TerminalRoutingStrategyException" => {
                    return RusotoError::Service(CreatePlayerSessionError::TerminalRoutingStrategy(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreatePlayerSessionError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreatePlayerSessionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreatePlayerSessionError::GameSessionFull(ref cause) => write!(f, "{}", cause),
            CreatePlayerSessionError::InternalService(ref cause) => write!(f, "{}", cause),
            CreatePlayerSessionError::InvalidGameSessionStatus(ref cause) => write!(f, "{}", cause),
            CreatePlayerSessionError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreatePlayerSessionError::NotFound(ref cause) => write!(f, "{}", cause),
            CreatePlayerSessionError::TerminalRoutingStrategy(ref cause) => write!(f, "{}", cause),
            CreatePlayerSessionError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreatePlayerSessionError {}
/// Errors returned by CreatePlayerSessions
#[derive(Debug, PartialEq)]
pub enum CreatePlayerSessionsError {
    /// <p>The game instance is currently full and cannot allow the requested player(s) to join. Clients can retry such requests immediately or after a waiting period.</p>
    GameSessionFull(String),
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>The requested operation would cause a conflict with the current state of a resource associated with the request and/or the game instance. Resolve the conflict before retrying.</p>
    InvalidGameSessionStatus(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The service is unable to resolve the routing for a particular alias because it has a terminal <a>RoutingStrategy</a> associated with it. The message returned in this exception is the message defined in the routing strategy itself. Such requests should only be retried if the routing strategy for the specified alias is modified. </p>
    TerminalRoutingStrategy(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl CreatePlayerSessionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePlayerSessionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "GameSessionFullException" => {
                    return RusotoError::Service(CreatePlayerSessionsError::GameSessionFull(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreatePlayerSessionsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidGameSessionStatusException" => {
                    return RusotoError::Service(
                        CreatePlayerSessionsError::InvalidGameSessionStatus(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreatePlayerSessionsError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreatePlayerSessionsError::NotFound(err.msg))
                }
                "TerminalRoutingStrategyException" => {
                    return RusotoError::Service(
                        CreatePlayerSessionsError::TerminalRoutingStrategy(err.msg),
                    )
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreatePlayerSessionsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreatePlayerSessionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreatePlayerSessionsError::GameSessionFull(ref cause) => write!(f, "{}", cause),
            CreatePlayerSessionsError::InternalService(ref cause) => write!(f, "{}", cause),
            CreatePlayerSessionsError::InvalidGameSessionStatus(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePlayerSessionsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreatePlayerSessionsError::NotFound(ref cause) => write!(f, "{}", cause),
            CreatePlayerSessionsError::TerminalRoutingStrategy(ref cause) => write!(f, "{}", cause),
            CreatePlayerSessionsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreatePlayerSessionsError {}
/// Errors returned by CreateScript
#[derive(Debug, PartialEq)]
pub enum CreateScriptError {
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request.</p>
    Conflict(String),
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p> The requested tagging operation did not succeed. This may be due to invalid tag format or the maximum tag limit may have been exceeded. Resolve the issue before retrying. </p>
    TaggingFailed(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl CreateScriptError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateScriptError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(CreateScriptError::Conflict(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(CreateScriptError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateScriptError::InvalidRequest(err.msg))
                }
                "TaggingFailedException" => {
                    return RusotoError::Service(CreateScriptError::TaggingFailed(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateScriptError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateScriptError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateScriptError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateScriptError::InternalService(ref cause) => write!(f, "{}", cause),
            CreateScriptError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateScriptError::TaggingFailed(ref cause) => write!(f, "{}", cause),
            CreateScriptError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateScriptError {}
/// Errors returned by CreateVpcPeeringAuthorization
#[derive(Debug, PartialEq)]
pub enum CreateVpcPeeringAuthorizationError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl CreateVpcPeeringAuthorizationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateVpcPeeringAuthorizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(
                        CreateVpcPeeringAuthorizationError::InternalService(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        CreateVpcPeeringAuthorizationError::InvalidRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateVpcPeeringAuthorizationError::NotFound(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateVpcPeeringAuthorizationError::Unauthorized(
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
impl fmt::Display for CreateVpcPeeringAuthorizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateVpcPeeringAuthorizationError::InternalService(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateVpcPeeringAuthorizationError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateVpcPeeringAuthorizationError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateVpcPeeringAuthorizationError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateVpcPeeringAuthorizationError {}
/// Errors returned by CreateVpcPeeringConnection
#[derive(Debug, PartialEq)]
pub enum CreateVpcPeeringConnectionError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl CreateVpcPeeringConnectionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateVpcPeeringConnectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(CreateVpcPeeringConnectionError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateVpcPeeringConnectionError::InvalidRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateVpcPeeringConnectionError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateVpcPeeringConnectionError::Unauthorized(
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
impl fmt::Display for CreateVpcPeeringConnectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateVpcPeeringConnectionError::InternalService(ref cause) => write!(f, "{}", cause),
            CreateVpcPeeringConnectionError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateVpcPeeringConnectionError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateVpcPeeringConnectionError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateVpcPeeringConnectionError {}
/// Errors returned by DeleteAlias
#[derive(Debug, PartialEq)]
pub enum DeleteAliasError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p> The requested tagging operation did not succeed. This may be due to invalid tag format or the maximum tag limit may have been exceeded. Resolve the issue before retrying. </p>
    TaggingFailed(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DeleteAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAliasError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteAliasError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteAliasError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteAliasError::NotFound(err.msg))
                }
                "TaggingFailedException" => {
                    return RusotoError::Service(DeleteAliasError::TaggingFailed(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteAliasError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAliasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAliasError::InternalService(ref cause) => write!(f, "{}", cause),
            DeleteAliasError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteAliasError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteAliasError::TaggingFailed(ref cause) => write!(f, "{}", cause),
            DeleteAliasError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAliasError {}
/// Errors returned by DeleteBuild
#[derive(Debug, PartialEq)]
pub enum DeleteBuildError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p> The requested tagging operation did not succeed. This may be due to invalid tag format or the maximum tag limit may have been exceeded. Resolve the issue before retrying. </p>
    TaggingFailed(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DeleteBuildError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBuildError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteBuildError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteBuildError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteBuildError::NotFound(err.msg))
                }
                "TaggingFailedException" => {
                    return RusotoError::Service(DeleteBuildError::TaggingFailed(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteBuildError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteBuildError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteBuildError::InternalService(ref cause) => write!(f, "{}", cause),
            DeleteBuildError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteBuildError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteBuildError::TaggingFailed(ref cause) => write!(f, "{}", cause),
            DeleteBuildError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteBuildError {}
/// Errors returned by DeleteFleet
#[derive(Debug, PartialEq)]
pub enum DeleteFleetError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>The requested operation would cause a conflict with the current state of a resource associated with the request and/or the fleet. Resolve the conflict before retrying.</p>
    InvalidFleetStatus(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p> The requested tagging operation did not succeed. This may be due to invalid tag format or the maximum tag limit may have been exceeded. Resolve the issue before retrying. </p>
    TaggingFailed(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DeleteFleetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteFleetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteFleetError::InternalService(err.msg))
                }
                "InvalidFleetStatusException" => {
                    return RusotoError::Service(DeleteFleetError::InvalidFleetStatus(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteFleetError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteFleetError::NotFound(err.msg))
                }
                "TaggingFailedException" => {
                    return RusotoError::Service(DeleteFleetError::TaggingFailed(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteFleetError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteFleetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteFleetError::InternalService(ref cause) => write!(f, "{}", cause),
            DeleteFleetError::InvalidFleetStatus(ref cause) => write!(f, "{}", cause),
            DeleteFleetError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteFleetError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteFleetError::TaggingFailed(ref cause) => write!(f, "{}", cause),
            DeleteFleetError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteFleetError {}
/// Errors returned by DeleteGameServerGroup
#[derive(Debug, PartialEq)]
pub enum DeleteGameServerGroupError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DeleteGameServerGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteGameServerGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteGameServerGroupError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteGameServerGroupError::InvalidRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteGameServerGroupError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteGameServerGroupError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteGameServerGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteGameServerGroupError::InternalService(ref cause) => write!(f, "{}", cause),
            DeleteGameServerGroupError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteGameServerGroupError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteGameServerGroupError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteGameServerGroupError {}
/// Errors returned by DeleteGameSessionQueue
#[derive(Debug, PartialEq)]
pub enum DeleteGameSessionQueueError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p> The requested tagging operation did not succeed. This may be due to invalid tag format or the maximum tag limit may have been exceeded. Resolve the issue before retrying. </p>
    TaggingFailed(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DeleteGameSessionQueueError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteGameSessionQueueError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteGameSessionQueueError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteGameSessionQueueError::InvalidRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteGameSessionQueueError::NotFound(err.msg))
                }
                "TaggingFailedException" => {
                    return RusotoError::Service(DeleteGameSessionQueueError::TaggingFailed(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteGameSessionQueueError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteGameSessionQueueError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteGameSessionQueueError::InternalService(ref cause) => write!(f, "{}", cause),
            DeleteGameSessionQueueError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteGameSessionQueueError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteGameSessionQueueError::TaggingFailed(ref cause) => write!(f, "{}", cause),
            DeleteGameSessionQueueError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteGameSessionQueueError {}
/// Errors returned by DeleteMatchmakingConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteMatchmakingConfigurationError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p> The requested tagging operation did not succeed. This may be due to invalid tag format or the maximum tag limit may have been exceeded. Resolve the issue before retrying. </p>
    TaggingFailed(String),
    /// <p>The requested operation is not supported in the Region specified.</p>
    UnsupportedRegion(String),
}

impl DeleteMatchmakingConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteMatchmakingConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(
                        DeleteMatchmakingConfigurationError::InternalService(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DeleteMatchmakingConfigurationError::InvalidRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteMatchmakingConfigurationError::NotFound(
                        err.msg,
                    ))
                }
                "TaggingFailedException" => {
                    return RusotoError::Service(
                        DeleteMatchmakingConfigurationError::TaggingFailed(err.msg),
                    )
                }
                "UnsupportedRegionException" => {
                    return RusotoError::Service(
                        DeleteMatchmakingConfigurationError::UnsupportedRegion(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteMatchmakingConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteMatchmakingConfigurationError::InternalService(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteMatchmakingConfigurationError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteMatchmakingConfigurationError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteMatchmakingConfigurationError::TaggingFailed(ref cause) => write!(f, "{}", cause),
            DeleteMatchmakingConfigurationError::UnsupportedRegion(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteMatchmakingConfigurationError {}
/// Errors returned by DeleteMatchmakingRuleSet
#[derive(Debug, PartialEq)]
pub enum DeleteMatchmakingRuleSetError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p> The requested tagging operation did not succeed. This may be due to invalid tag format or the maximum tag limit may have been exceeded. Resolve the issue before retrying. </p>
    TaggingFailed(String),
    /// <p>The requested operation is not supported in the Region specified.</p>
    UnsupportedRegion(String),
}

impl DeleteMatchmakingRuleSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteMatchmakingRuleSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteMatchmakingRuleSetError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteMatchmakingRuleSetError::InvalidRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteMatchmakingRuleSetError::NotFound(err.msg))
                }
                "TaggingFailedException" => {
                    return RusotoError::Service(DeleteMatchmakingRuleSetError::TaggingFailed(
                        err.msg,
                    ))
                }
                "UnsupportedRegionException" => {
                    return RusotoError::Service(DeleteMatchmakingRuleSetError::UnsupportedRegion(
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
impl fmt::Display for DeleteMatchmakingRuleSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteMatchmakingRuleSetError::InternalService(ref cause) => write!(f, "{}", cause),
            DeleteMatchmakingRuleSetError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteMatchmakingRuleSetError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteMatchmakingRuleSetError::TaggingFailed(ref cause) => write!(f, "{}", cause),
            DeleteMatchmakingRuleSetError::UnsupportedRegion(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteMatchmakingRuleSetError {}
/// Errors returned by DeleteScalingPolicy
#[derive(Debug, PartialEq)]
pub enum DeleteScalingPolicyError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DeleteScalingPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteScalingPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteScalingPolicyError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteScalingPolicyError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteScalingPolicyError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteScalingPolicyError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteScalingPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteScalingPolicyError::InternalService(ref cause) => write!(f, "{}", cause),
            DeleteScalingPolicyError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteScalingPolicyError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteScalingPolicyError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteScalingPolicyError {}
/// Errors returned by DeleteScript
#[derive(Debug, PartialEq)]
pub enum DeleteScriptError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p> The requested tagging operation did not succeed. This may be due to invalid tag format or the maximum tag limit may have been exceeded. Resolve the issue before retrying. </p>
    TaggingFailed(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DeleteScriptError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteScriptError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteScriptError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteScriptError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteScriptError::NotFound(err.msg))
                }
                "TaggingFailedException" => {
                    return RusotoError::Service(DeleteScriptError::TaggingFailed(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteScriptError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteScriptError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteScriptError::InternalService(ref cause) => write!(f, "{}", cause),
            DeleteScriptError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteScriptError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteScriptError::TaggingFailed(ref cause) => write!(f, "{}", cause),
            DeleteScriptError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteScriptError {}
/// Errors returned by DeleteVpcPeeringAuthorization
#[derive(Debug, PartialEq)]
pub enum DeleteVpcPeeringAuthorizationError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DeleteVpcPeeringAuthorizationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteVpcPeeringAuthorizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(
                        DeleteVpcPeeringAuthorizationError::InternalService(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DeleteVpcPeeringAuthorizationError::InvalidRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteVpcPeeringAuthorizationError::NotFound(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteVpcPeeringAuthorizationError::Unauthorized(
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
impl fmt::Display for DeleteVpcPeeringAuthorizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteVpcPeeringAuthorizationError::InternalService(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteVpcPeeringAuthorizationError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteVpcPeeringAuthorizationError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteVpcPeeringAuthorizationError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteVpcPeeringAuthorizationError {}
/// Errors returned by DeleteVpcPeeringConnection
#[derive(Debug, PartialEq)]
pub enum DeleteVpcPeeringConnectionError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DeleteVpcPeeringConnectionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteVpcPeeringConnectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DeleteVpcPeeringConnectionError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteVpcPeeringConnectionError::InvalidRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteVpcPeeringConnectionError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteVpcPeeringConnectionError::Unauthorized(
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
impl fmt::Display for DeleteVpcPeeringConnectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteVpcPeeringConnectionError::InternalService(ref cause) => write!(f, "{}", cause),
            DeleteVpcPeeringConnectionError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteVpcPeeringConnectionError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteVpcPeeringConnectionError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteVpcPeeringConnectionError {}
/// Errors returned by DeregisterGameServer
#[derive(Debug, PartialEq)]
pub enum DeregisterGameServerError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DeregisterGameServerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeregisterGameServerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DeregisterGameServerError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeregisterGameServerError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeregisterGameServerError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeregisterGameServerError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeregisterGameServerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeregisterGameServerError::InternalService(ref cause) => write!(f, "{}", cause),
            DeregisterGameServerError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeregisterGameServerError::NotFound(ref cause) => write!(f, "{}", cause),
            DeregisterGameServerError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeregisterGameServerError {}
/// Errors returned by DescribeAlias
#[derive(Debug, PartialEq)]
pub enum DescribeAliasError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DescribeAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAliasError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeAliasError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeAliasError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeAliasError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeAliasError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeAliasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAliasError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeAliasError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeAliasError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeAliasError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAliasError {}
/// Errors returned by DescribeBuild
#[derive(Debug, PartialEq)]
pub enum DescribeBuildError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DescribeBuildError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeBuildError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeBuildError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeBuildError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeBuildError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeBuildError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeBuildError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeBuildError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeBuildError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeBuildError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeBuildError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeBuildError {}
/// Errors returned by DescribeEC2InstanceLimits
#[derive(Debug, PartialEq)]
pub enum DescribeEC2InstanceLimitsError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DescribeEC2InstanceLimitsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEC2InstanceLimitsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeEC2InstanceLimitsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeEC2InstanceLimitsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeEC2InstanceLimitsError::Unauthorized(
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
impl fmt::Display for DescribeEC2InstanceLimitsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEC2InstanceLimitsError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeEC2InstanceLimitsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeEC2InstanceLimitsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeEC2InstanceLimitsError {}
/// Errors returned by DescribeFleetAttributes
#[derive(Debug, PartialEq)]
pub enum DescribeFleetAttributesError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DescribeFleetAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeFleetAttributesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeFleetAttributesError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeFleetAttributesError::InvalidRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeFleetAttributesError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeFleetAttributesError::Unauthorized(
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
impl fmt::Display for DescribeFleetAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeFleetAttributesError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeFleetAttributesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeFleetAttributesError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeFleetAttributesError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeFleetAttributesError {}
/// Errors returned by DescribeFleetCapacity
#[derive(Debug, PartialEq)]
pub enum DescribeFleetCapacityError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DescribeFleetCapacityError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeFleetCapacityError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeFleetCapacityError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeFleetCapacityError::InvalidRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeFleetCapacityError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeFleetCapacityError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeFleetCapacityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeFleetCapacityError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeFleetCapacityError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeFleetCapacityError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeFleetCapacityError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeFleetCapacityError {}
/// Errors returned by DescribeFleetEvents
#[derive(Debug, PartialEq)]
pub enum DescribeFleetEventsError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DescribeFleetEventsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeFleetEventsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeFleetEventsError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeFleetEventsError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeFleetEventsError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeFleetEventsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeFleetEventsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeFleetEventsError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeFleetEventsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeFleetEventsError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeFleetEventsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeFleetEventsError {}
/// Errors returned by DescribeFleetPortSettings
#[derive(Debug, PartialEq)]
pub enum DescribeFleetPortSettingsError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DescribeFleetPortSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeFleetPortSettingsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeFleetPortSettingsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeFleetPortSettingsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeFleetPortSettingsError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeFleetPortSettingsError::Unauthorized(
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
impl fmt::Display for DescribeFleetPortSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeFleetPortSettingsError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeFleetPortSettingsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeFleetPortSettingsError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeFleetPortSettingsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeFleetPortSettingsError {}
/// Errors returned by DescribeFleetUtilization
#[derive(Debug, PartialEq)]
pub enum DescribeFleetUtilizationError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DescribeFleetUtilizationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeFleetUtilizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeFleetUtilizationError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeFleetUtilizationError::InvalidRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeFleetUtilizationError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeFleetUtilizationError::Unauthorized(
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
impl fmt::Display for DescribeFleetUtilizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeFleetUtilizationError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeFleetUtilizationError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeFleetUtilizationError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeFleetUtilizationError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeFleetUtilizationError {}
/// Errors returned by DescribeGameServer
#[derive(Debug, PartialEq)]
pub enum DescribeGameServerError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DescribeGameServerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeGameServerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeGameServerError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeGameServerError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeGameServerError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeGameServerError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeGameServerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeGameServerError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeGameServerError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeGameServerError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeGameServerError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeGameServerError {}
/// Errors returned by DescribeGameServerGroup
#[derive(Debug, PartialEq)]
pub enum DescribeGameServerGroupError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DescribeGameServerGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeGameServerGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeGameServerGroupError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeGameServerGroupError::InvalidRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeGameServerGroupError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeGameServerGroupError::Unauthorized(
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
impl fmt::Display for DescribeGameServerGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeGameServerGroupError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeGameServerGroupError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeGameServerGroupError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeGameServerGroupError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeGameServerGroupError {}
/// Errors returned by DescribeGameServerInstances
#[derive(Debug, PartialEq)]
pub enum DescribeGameServerInstancesError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DescribeGameServerInstancesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeGameServerInstancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeGameServerInstancesError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeGameServerInstancesError::InvalidRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeGameServerInstancesError::NotFound(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeGameServerInstancesError::Unauthorized(
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
impl fmt::Display for DescribeGameServerInstancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeGameServerInstancesError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeGameServerInstancesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeGameServerInstancesError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeGameServerInstancesError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeGameServerInstancesError {}
/// Errors returned by DescribeGameSessionDetails
#[derive(Debug, PartialEq)]
pub enum DescribeGameSessionDetailsError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The service is unable to resolve the routing for a particular alias because it has a terminal <a>RoutingStrategy</a> associated with it. The message returned in this exception is the message defined in the routing strategy itself. Such requests should only be retried if the routing strategy for the specified alias is modified. </p>
    TerminalRoutingStrategy(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DescribeGameSessionDetailsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeGameSessionDetailsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeGameSessionDetailsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeGameSessionDetailsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeGameSessionDetailsError::NotFound(err.msg))
                }
                "TerminalRoutingStrategyException" => {
                    return RusotoError::Service(
                        DescribeGameSessionDetailsError::TerminalRoutingStrategy(err.msg),
                    )
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeGameSessionDetailsError::Unauthorized(
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
impl fmt::Display for DescribeGameSessionDetailsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeGameSessionDetailsError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeGameSessionDetailsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeGameSessionDetailsError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeGameSessionDetailsError::TerminalRoutingStrategy(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeGameSessionDetailsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeGameSessionDetailsError {}
/// Errors returned by DescribeGameSessionPlacement
#[derive(Debug, PartialEq)]
pub enum DescribeGameSessionPlacementError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DescribeGameSessionPlacementError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeGameSessionPlacementError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(
                        DescribeGameSessionPlacementError::InternalService(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeGameSessionPlacementError::InvalidRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeGameSessionPlacementError::NotFound(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeGameSessionPlacementError::Unauthorized(
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
impl fmt::Display for DescribeGameSessionPlacementError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeGameSessionPlacementError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeGameSessionPlacementError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeGameSessionPlacementError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeGameSessionPlacementError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeGameSessionPlacementError {}
/// Errors returned by DescribeGameSessionQueues
#[derive(Debug, PartialEq)]
pub enum DescribeGameSessionQueuesError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DescribeGameSessionQueuesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeGameSessionQueuesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeGameSessionQueuesError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeGameSessionQueuesError::InvalidRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeGameSessionQueuesError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeGameSessionQueuesError::Unauthorized(
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
impl fmt::Display for DescribeGameSessionQueuesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeGameSessionQueuesError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeGameSessionQueuesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeGameSessionQueuesError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeGameSessionQueuesError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeGameSessionQueuesError {}
/// Errors returned by DescribeGameSessions
#[derive(Debug, PartialEq)]
pub enum DescribeGameSessionsError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The service is unable to resolve the routing for a particular alias because it has a terminal <a>RoutingStrategy</a> associated with it. The message returned in this exception is the message defined in the routing strategy itself. Such requests should only be retried if the routing strategy for the specified alias is modified. </p>
    TerminalRoutingStrategy(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DescribeGameSessionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeGameSessionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeGameSessionsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeGameSessionsError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeGameSessionsError::NotFound(err.msg))
                }
                "TerminalRoutingStrategyException" => {
                    return RusotoError::Service(
                        DescribeGameSessionsError::TerminalRoutingStrategy(err.msg),
                    )
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeGameSessionsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeGameSessionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeGameSessionsError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeGameSessionsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeGameSessionsError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeGameSessionsError::TerminalRoutingStrategy(ref cause) => write!(f, "{}", cause),
            DescribeGameSessionsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeGameSessionsError {}
/// Errors returned by DescribeInstances
#[derive(Debug, PartialEq)]
pub enum DescribeInstancesError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DescribeInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeInstancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeInstancesError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeInstancesError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeInstancesError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeInstancesError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeInstancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeInstancesError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeInstancesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeInstancesError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeInstancesError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeInstancesError {}
/// Errors returned by DescribeMatchmaking
#[derive(Debug, PartialEq)]
pub enum DescribeMatchmakingError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>The requested operation is not supported in the Region specified.</p>
    UnsupportedRegion(String),
}

impl DescribeMatchmakingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeMatchmakingError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeMatchmakingError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeMatchmakingError::InvalidRequest(err.msg))
                }
                "UnsupportedRegionException" => {
                    return RusotoError::Service(DescribeMatchmakingError::UnsupportedRegion(
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
impl fmt::Display for DescribeMatchmakingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeMatchmakingError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeMatchmakingError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeMatchmakingError::UnsupportedRegion(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeMatchmakingError {}
/// Errors returned by DescribeMatchmakingConfigurations
#[derive(Debug, PartialEq)]
pub enum DescribeMatchmakingConfigurationsError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>The requested operation is not supported in the Region specified.</p>
    UnsupportedRegion(String),
}

impl DescribeMatchmakingConfigurationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeMatchmakingConfigurationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(
                        DescribeMatchmakingConfigurationsError::InternalService(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DescribeMatchmakingConfigurationsError::InvalidRequest(err.msg),
                    )
                }
                "UnsupportedRegionException" => {
                    return RusotoError::Service(
                        DescribeMatchmakingConfigurationsError::UnsupportedRegion(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeMatchmakingConfigurationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeMatchmakingConfigurationsError::InternalService(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeMatchmakingConfigurationsError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeMatchmakingConfigurationsError::UnsupportedRegion(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeMatchmakingConfigurationsError {}
/// Errors returned by DescribeMatchmakingRuleSets
#[derive(Debug, PartialEq)]
pub enum DescribeMatchmakingRuleSetsError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The requested operation is not supported in the Region specified.</p>
    UnsupportedRegion(String),
}

impl DescribeMatchmakingRuleSetsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeMatchmakingRuleSetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeMatchmakingRuleSetsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeMatchmakingRuleSetsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeMatchmakingRuleSetsError::NotFound(
                        err.msg,
                    ))
                }
                "UnsupportedRegionException" => {
                    return RusotoError::Service(
                        DescribeMatchmakingRuleSetsError::UnsupportedRegion(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeMatchmakingRuleSetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeMatchmakingRuleSetsError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeMatchmakingRuleSetsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeMatchmakingRuleSetsError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeMatchmakingRuleSetsError::UnsupportedRegion(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeMatchmakingRuleSetsError {}
/// Errors returned by DescribePlayerSessions
#[derive(Debug, PartialEq)]
pub enum DescribePlayerSessionsError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DescribePlayerSessionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribePlayerSessionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DescribePlayerSessionsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribePlayerSessionsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribePlayerSessionsError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribePlayerSessionsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribePlayerSessionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribePlayerSessionsError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribePlayerSessionsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribePlayerSessionsError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribePlayerSessionsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribePlayerSessionsError {}
/// Errors returned by DescribeRuntimeConfiguration
#[derive(Debug, PartialEq)]
pub enum DescribeRuntimeConfigurationError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DescribeRuntimeConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeRuntimeConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(
                        DescribeRuntimeConfigurationError::InternalService(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeRuntimeConfigurationError::InvalidRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeRuntimeConfigurationError::NotFound(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeRuntimeConfigurationError::Unauthorized(
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
impl fmt::Display for DescribeRuntimeConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeRuntimeConfigurationError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeRuntimeConfigurationError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeRuntimeConfigurationError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeRuntimeConfigurationError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeRuntimeConfigurationError {}
/// Errors returned by DescribeScalingPolicies
#[derive(Debug, PartialEq)]
pub enum DescribeScalingPoliciesError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DescribeScalingPoliciesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeScalingPoliciesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeScalingPoliciesError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeScalingPoliciesError::InvalidRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeScalingPoliciesError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeScalingPoliciesError::Unauthorized(
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
impl fmt::Display for DescribeScalingPoliciesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeScalingPoliciesError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeScalingPoliciesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeScalingPoliciesError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeScalingPoliciesError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeScalingPoliciesError {}
/// Errors returned by DescribeScript
#[derive(Debug, PartialEq)]
pub enum DescribeScriptError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DescribeScriptError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeScriptError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeScriptError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeScriptError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeScriptError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeScriptError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeScriptError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeScriptError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeScriptError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeScriptError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeScriptError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeScriptError {}
/// Errors returned by DescribeVpcPeeringAuthorizations
#[derive(Debug, PartialEq)]
pub enum DescribeVpcPeeringAuthorizationsError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DescribeVpcPeeringAuthorizationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeVpcPeeringAuthorizationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(
                        DescribeVpcPeeringAuthorizationsError::InternalService(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DescribeVpcPeeringAuthorizationsError::InvalidRequest(err.msg),
                    )
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(
                        DescribeVpcPeeringAuthorizationsError::Unauthorized(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeVpcPeeringAuthorizationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeVpcPeeringAuthorizationsError::InternalService(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeVpcPeeringAuthorizationsError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeVpcPeeringAuthorizationsError::Unauthorized(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeVpcPeeringAuthorizationsError {}
/// Errors returned by DescribeVpcPeeringConnections
#[derive(Debug, PartialEq)]
pub enum DescribeVpcPeeringConnectionsError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl DescribeVpcPeeringConnectionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeVpcPeeringConnectionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(
                        DescribeVpcPeeringConnectionsError::InternalService(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DescribeVpcPeeringConnectionsError::InvalidRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeVpcPeeringConnectionsError::NotFound(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DescribeVpcPeeringConnectionsError::Unauthorized(
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
impl fmt::Display for DescribeVpcPeeringConnectionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeVpcPeeringConnectionsError::InternalService(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeVpcPeeringConnectionsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeVpcPeeringConnectionsError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeVpcPeeringConnectionsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeVpcPeeringConnectionsError {}
/// Errors returned by GetGameSessionLogUrl
#[derive(Debug, PartialEq)]
pub enum GetGameSessionLogUrlError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl GetGameSessionLogUrlError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetGameSessionLogUrlError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(GetGameSessionLogUrlError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetGameSessionLogUrlError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetGameSessionLogUrlError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetGameSessionLogUrlError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetGameSessionLogUrlError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetGameSessionLogUrlError::InternalService(ref cause) => write!(f, "{}", cause),
            GetGameSessionLogUrlError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetGameSessionLogUrlError::NotFound(ref cause) => write!(f, "{}", cause),
            GetGameSessionLogUrlError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetGameSessionLogUrlError {}
/// Errors returned by GetInstanceAccess
#[derive(Debug, PartialEq)]
pub enum GetInstanceAccessError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl GetInstanceAccessError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInstanceAccessError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(GetInstanceAccessError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetInstanceAccessError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetInstanceAccessError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetInstanceAccessError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetInstanceAccessError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetInstanceAccessError::InternalService(ref cause) => write!(f, "{}", cause),
            GetInstanceAccessError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            GetInstanceAccessError::NotFound(ref cause) => write!(f, "{}", cause),
            GetInstanceAccessError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetInstanceAccessError {}
/// Errors returned by ListAliases
#[derive(Debug, PartialEq)]
pub enum ListAliasesError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl ListAliasesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAliasesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListAliasesError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListAliasesError::InvalidRequest(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListAliasesError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAliasesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAliasesError::InternalService(ref cause) => write!(f, "{}", cause),
            ListAliasesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListAliasesError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAliasesError {}
/// Errors returned by ListBuilds
#[derive(Debug, PartialEq)]
pub enum ListBuildsError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl ListBuildsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListBuildsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListBuildsError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListBuildsError::InvalidRequest(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListBuildsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListBuildsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListBuildsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListBuildsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListBuildsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListBuildsError {}
/// Errors returned by ListFleets
#[derive(Debug, PartialEq)]
pub enum ListFleetsError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl ListFleetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListFleetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListFleetsError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListFleetsError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListFleetsError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListFleetsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListFleetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListFleetsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListFleetsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListFleetsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListFleetsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListFleetsError {}
/// Errors returned by ListGameServerGroups
#[derive(Debug, PartialEq)]
pub enum ListGameServerGroupsError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl ListGameServerGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListGameServerGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListGameServerGroupsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListGameServerGroupsError::InvalidRequest(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListGameServerGroupsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListGameServerGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListGameServerGroupsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListGameServerGroupsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListGameServerGroupsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListGameServerGroupsError {}
/// Errors returned by ListGameServers
#[derive(Debug, PartialEq)]
pub enum ListGameServersError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl ListGameServersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListGameServersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListGameServersError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListGameServersError::InvalidRequest(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListGameServersError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListGameServersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListGameServersError::InternalService(ref cause) => write!(f, "{}", cause),
            ListGameServersError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListGameServersError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListGameServersError {}
/// Errors returned by ListScripts
#[derive(Debug, PartialEq)]
pub enum ListScriptsError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl ListScriptsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListScriptsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListScriptsError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListScriptsError::InvalidRequest(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListScriptsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListScriptsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListScriptsError::InternalService(ref cause) => write!(f, "{}", cause),
            ListScriptsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListScriptsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListScriptsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p> The requested tagging operation did not succeed. This may be due to invalid tag format or the maximum tag limit may have been exceeded. Resolve the issue before retrying. </p>
    TaggingFailed(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::NotFound(err.msg))
                }
                "TaggingFailedException" => {
                    return RusotoError::Service(ListTagsForResourceError::TaggingFailed(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::InternalService(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::TaggingFailed(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by PutScalingPolicy
#[derive(Debug, PartialEq)]
pub enum PutScalingPolicyError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl PutScalingPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutScalingPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(PutScalingPolicyError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(PutScalingPolicyError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutScalingPolicyError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(PutScalingPolicyError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutScalingPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutScalingPolicyError::InternalService(ref cause) => write!(f, "{}", cause),
            PutScalingPolicyError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            PutScalingPolicyError::NotFound(ref cause) => write!(f, "{}", cause),
            PutScalingPolicyError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutScalingPolicyError {}
/// Errors returned by RegisterGameServer
#[derive(Debug, PartialEq)]
pub enum RegisterGameServerError {
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request.</p>
    Conflict(String),
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>The requested operation would cause the resource to exceed the allowed service limit. Resolve the issue before retrying.</p>
    LimitExceeded(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl RegisterGameServerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterGameServerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(RegisterGameServerError::Conflict(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(RegisterGameServerError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(RegisterGameServerError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(RegisterGameServerError::LimitExceeded(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(RegisterGameServerError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RegisterGameServerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterGameServerError::Conflict(ref cause) => write!(f, "{}", cause),
            RegisterGameServerError::InternalService(ref cause) => write!(f, "{}", cause),
            RegisterGameServerError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            RegisterGameServerError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            RegisterGameServerError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterGameServerError {}
/// Errors returned by RequestUploadCredentials
#[derive(Debug, PartialEq)]
pub enum RequestUploadCredentialsError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl RequestUploadCredentialsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RequestUploadCredentialsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(RequestUploadCredentialsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(RequestUploadCredentialsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(RequestUploadCredentialsError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(RequestUploadCredentialsError::Unauthorized(
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
impl fmt::Display for RequestUploadCredentialsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RequestUploadCredentialsError::InternalService(ref cause) => write!(f, "{}", cause),
            RequestUploadCredentialsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            RequestUploadCredentialsError::NotFound(ref cause) => write!(f, "{}", cause),
            RequestUploadCredentialsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RequestUploadCredentialsError {}
/// Errors returned by ResolveAlias
#[derive(Debug, PartialEq)]
pub enum ResolveAliasError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The service is unable to resolve the routing for a particular alias because it has a terminal <a>RoutingStrategy</a> associated with it. The message returned in this exception is the message defined in the routing strategy itself. Such requests should only be retried if the routing strategy for the specified alias is modified. </p>
    TerminalRoutingStrategy(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl ResolveAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ResolveAliasError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ResolveAliasError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ResolveAliasError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ResolveAliasError::NotFound(err.msg))
                }
                "TerminalRoutingStrategyException" => {
                    return RusotoError::Service(ResolveAliasError::TerminalRoutingStrategy(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ResolveAliasError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ResolveAliasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResolveAliasError::InternalService(ref cause) => write!(f, "{}", cause),
            ResolveAliasError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ResolveAliasError::NotFound(ref cause) => write!(f, "{}", cause),
            ResolveAliasError::TerminalRoutingStrategy(ref cause) => write!(f, "{}", cause),
            ResolveAliasError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ResolveAliasError {}
/// Errors returned by ResumeGameServerGroup
#[derive(Debug, PartialEq)]
pub enum ResumeGameServerGroupError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl ResumeGameServerGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ResumeGameServerGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ResumeGameServerGroupError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ResumeGameServerGroupError::InvalidRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ResumeGameServerGroupError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ResumeGameServerGroupError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ResumeGameServerGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResumeGameServerGroupError::InternalService(ref cause) => write!(f, "{}", cause),
            ResumeGameServerGroupError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ResumeGameServerGroupError::NotFound(ref cause) => write!(f, "{}", cause),
            ResumeGameServerGroupError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ResumeGameServerGroupError {}
/// Errors returned by SearchGameSessions
#[derive(Debug, PartialEq)]
pub enum SearchGameSessionsError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The service is unable to resolve the routing for a particular alias because it has a terminal <a>RoutingStrategy</a> associated with it. The message returned in this exception is the message defined in the routing strategy itself. Such requests should only be retried if the routing strategy for the specified alias is modified. </p>
    TerminalRoutingStrategy(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl SearchGameSessionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchGameSessionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(SearchGameSessionsError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(SearchGameSessionsError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(SearchGameSessionsError::NotFound(err.msg))
                }
                "TerminalRoutingStrategyException" => {
                    return RusotoError::Service(SearchGameSessionsError::TerminalRoutingStrategy(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(SearchGameSessionsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SearchGameSessionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SearchGameSessionsError::InternalService(ref cause) => write!(f, "{}", cause),
            SearchGameSessionsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            SearchGameSessionsError::NotFound(ref cause) => write!(f, "{}", cause),
            SearchGameSessionsError::TerminalRoutingStrategy(ref cause) => write!(f, "{}", cause),
            SearchGameSessionsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SearchGameSessionsError {}
/// Errors returned by StartFleetActions
#[derive(Debug, PartialEq)]
pub enum StartFleetActionsError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl StartFleetActionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartFleetActionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(StartFleetActionsError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartFleetActionsError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StartFleetActionsError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(StartFleetActionsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartFleetActionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartFleetActionsError::InternalService(ref cause) => write!(f, "{}", cause),
            StartFleetActionsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StartFleetActionsError::NotFound(ref cause) => write!(f, "{}", cause),
            StartFleetActionsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartFleetActionsError {}
/// Errors returned by StartGameSessionPlacement
#[derive(Debug, PartialEq)]
pub enum StartGameSessionPlacementError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl StartGameSessionPlacementError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartGameSessionPlacementError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(StartGameSessionPlacementError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartGameSessionPlacementError::InvalidRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StartGameSessionPlacementError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(StartGameSessionPlacementError::Unauthorized(
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
impl fmt::Display for StartGameSessionPlacementError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartGameSessionPlacementError::InternalService(ref cause) => write!(f, "{}", cause),
            StartGameSessionPlacementError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StartGameSessionPlacementError::NotFound(ref cause) => write!(f, "{}", cause),
            StartGameSessionPlacementError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartGameSessionPlacementError {}
/// Errors returned by StartMatchBackfill
#[derive(Debug, PartialEq)]
pub enum StartMatchBackfillError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The requested operation is not supported in the Region specified.</p>
    UnsupportedRegion(String),
}

impl StartMatchBackfillError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartMatchBackfillError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(StartMatchBackfillError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartMatchBackfillError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StartMatchBackfillError::NotFound(err.msg))
                }
                "UnsupportedRegionException" => {
                    return RusotoError::Service(StartMatchBackfillError::UnsupportedRegion(
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
impl fmt::Display for StartMatchBackfillError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartMatchBackfillError::InternalService(ref cause) => write!(f, "{}", cause),
            StartMatchBackfillError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StartMatchBackfillError::NotFound(ref cause) => write!(f, "{}", cause),
            StartMatchBackfillError::UnsupportedRegion(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartMatchBackfillError {}
/// Errors returned by StartMatchmaking
#[derive(Debug, PartialEq)]
pub enum StartMatchmakingError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The requested operation is not supported in the Region specified.</p>
    UnsupportedRegion(String),
}

impl StartMatchmakingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartMatchmakingError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(StartMatchmakingError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartMatchmakingError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StartMatchmakingError::NotFound(err.msg))
                }
                "UnsupportedRegionException" => {
                    return RusotoError::Service(StartMatchmakingError::UnsupportedRegion(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartMatchmakingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartMatchmakingError::InternalService(ref cause) => write!(f, "{}", cause),
            StartMatchmakingError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StartMatchmakingError::NotFound(ref cause) => write!(f, "{}", cause),
            StartMatchmakingError::UnsupportedRegion(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartMatchmakingError {}
/// Errors returned by StopFleetActions
#[derive(Debug, PartialEq)]
pub enum StopFleetActionsError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl StopFleetActionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopFleetActionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(StopFleetActionsError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StopFleetActionsError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StopFleetActionsError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(StopFleetActionsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopFleetActionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopFleetActionsError::InternalService(ref cause) => write!(f, "{}", cause),
            StopFleetActionsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StopFleetActionsError::NotFound(ref cause) => write!(f, "{}", cause),
            StopFleetActionsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopFleetActionsError {}
/// Errors returned by StopGameSessionPlacement
#[derive(Debug, PartialEq)]
pub enum StopGameSessionPlacementError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl StopGameSessionPlacementError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopGameSessionPlacementError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(StopGameSessionPlacementError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StopGameSessionPlacementError::InvalidRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StopGameSessionPlacementError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(StopGameSessionPlacementError::Unauthorized(
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
impl fmt::Display for StopGameSessionPlacementError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopGameSessionPlacementError::InternalService(ref cause) => write!(f, "{}", cause),
            StopGameSessionPlacementError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StopGameSessionPlacementError::NotFound(ref cause) => write!(f, "{}", cause),
            StopGameSessionPlacementError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopGameSessionPlacementError {}
/// Errors returned by StopMatchmaking
#[derive(Debug, PartialEq)]
pub enum StopMatchmakingError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The requested operation is not supported in the Region specified.</p>
    UnsupportedRegion(String),
}

impl StopMatchmakingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopMatchmakingError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(StopMatchmakingError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StopMatchmakingError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StopMatchmakingError::NotFound(err.msg))
                }
                "UnsupportedRegionException" => {
                    return RusotoError::Service(StopMatchmakingError::UnsupportedRegion(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopMatchmakingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopMatchmakingError::InternalService(ref cause) => write!(f, "{}", cause),
            StopMatchmakingError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StopMatchmakingError::NotFound(ref cause) => write!(f, "{}", cause),
            StopMatchmakingError::UnsupportedRegion(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopMatchmakingError {}
/// Errors returned by SuspendGameServerGroup
#[derive(Debug, PartialEq)]
pub enum SuspendGameServerGroupError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl SuspendGameServerGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SuspendGameServerGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(SuspendGameServerGroupError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(SuspendGameServerGroupError::InvalidRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(SuspendGameServerGroupError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(SuspendGameServerGroupError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SuspendGameServerGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SuspendGameServerGroupError::InternalService(ref cause) => write!(f, "{}", cause),
            SuspendGameServerGroupError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            SuspendGameServerGroupError::NotFound(ref cause) => write!(f, "{}", cause),
            SuspendGameServerGroupError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SuspendGameServerGroupError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p> The requested tagging operation did not succeed. This may be due to invalid tag format or the maximum tag limit may have been exceeded. Resolve the issue before retrying. </p>
    TaggingFailed(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(TagResourceError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(TagResourceError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(TagResourceError::NotFound(err.msg))
                }
                "TaggingFailedException" => {
                    return RusotoError::Service(TagResourceError::TaggingFailed(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::InternalService(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            TagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::TaggingFailed(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p> The requested tagging operation did not succeed. This may be due to invalid tag format or the maximum tag limit may have been exceeded. Resolve the issue before retrying. </p>
    TaggingFailed(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UntagResourceError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UntagResourceError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UntagResourceError::NotFound(err.msg))
                }
                "TaggingFailedException" => {
                    return RusotoError::Service(UntagResourceError::TaggingFailed(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UntagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::InternalService(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UntagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::TaggingFailed(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateAlias
#[derive(Debug, PartialEq)]
pub enum UpdateAliasError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl UpdateAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAliasError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateAliasError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateAliasError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateAliasError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateAliasError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateAliasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAliasError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateAliasError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateAliasError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateAliasError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateAliasError {}
/// Errors returned by UpdateBuild
#[derive(Debug, PartialEq)]
pub enum UpdateBuildError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl UpdateBuildError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateBuildError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateBuildError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateBuildError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateBuildError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateBuildError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateBuildError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateBuildError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateBuildError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateBuildError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateBuildError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateBuildError {}
/// Errors returned by UpdateFleetAttributes
#[derive(Debug, PartialEq)]
pub enum UpdateFleetAttributesError {
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request.</p>
    Conflict(String),
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>The requested operation would cause a conflict with the current state of a resource associated with the request and/or the fleet. Resolve the conflict before retrying.</p>
    InvalidFleetStatus(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>The requested operation would cause the resource to exceed the allowed service limit. Resolve the issue before retrying.</p>
    LimitExceeded(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl UpdateFleetAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateFleetAttributesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(UpdateFleetAttributesError::Conflict(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateFleetAttributesError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidFleetStatusException" => {
                    return RusotoError::Service(UpdateFleetAttributesError::InvalidFleetStatus(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateFleetAttributesError::InvalidRequest(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateFleetAttributesError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateFleetAttributesError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateFleetAttributesError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateFleetAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateFleetAttributesError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateFleetAttributesError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateFleetAttributesError::InvalidFleetStatus(ref cause) => write!(f, "{}", cause),
            UpdateFleetAttributesError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateFleetAttributesError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateFleetAttributesError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateFleetAttributesError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateFleetAttributesError {}
/// Errors returned by UpdateFleetCapacity
#[derive(Debug, PartialEq)]
pub enum UpdateFleetCapacityError {
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request.</p>
    Conflict(String),
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>The requested operation would cause a conflict with the current state of a resource associated with the request and/or the fleet. Resolve the conflict before retrying.</p>
    InvalidFleetStatus(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>The requested operation would cause the resource to exceed the allowed service limit. Resolve the issue before retrying.</p>
    LimitExceeded(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl UpdateFleetCapacityError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateFleetCapacityError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(UpdateFleetCapacityError::Conflict(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateFleetCapacityError::InternalService(err.msg))
                }
                "InvalidFleetStatusException" => {
                    return RusotoError::Service(UpdateFleetCapacityError::InvalidFleetStatus(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateFleetCapacityError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateFleetCapacityError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateFleetCapacityError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateFleetCapacityError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateFleetCapacityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateFleetCapacityError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateFleetCapacityError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateFleetCapacityError::InvalidFleetStatus(ref cause) => write!(f, "{}", cause),
            UpdateFleetCapacityError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateFleetCapacityError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateFleetCapacityError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateFleetCapacityError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateFleetCapacityError {}
/// Errors returned by UpdateFleetPortSettings
#[derive(Debug, PartialEq)]
pub enum UpdateFleetPortSettingsError {
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request.</p>
    Conflict(String),
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>The requested operation would cause a conflict with the current state of a resource associated with the request and/or the fleet. Resolve the conflict before retrying.</p>
    InvalidFleetStatus(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>The requested operation would cause the resource to exceed the allowed service limit. Resolve the issue before retrying.</p>
    LimitExceeded(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl UpdateFleetPortSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateFleetPortSettingsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(UpdateFleetPortSettingsError::Conflict(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateFleetPortSettingsError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidFleetStatusException" => {
                    return RusotoError::Service(UpdateFleetPortSettingsError::InvalidFleetStatus(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateFleetPortSettingsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateFleetPortSettingsError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateFleetPortSettingsError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateFleetPortSettingsError::Unauthorized(
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
impl fmt::Display for UpdateFleetPortSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateFleetPortSettingsError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateFleetPortSettingsError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateFleetPortSettingsError::InvalidFleetStatus(ref cause) => write!(f, "{}", cause),
            UpdateFleetPortSettingsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateFleetPortSettingsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateFleetPortSettingsError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateFleetPortSettingsError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateFleetPortSettingsError {}
/// Errors returned by UpdateGameServer
#[derive(Debug, PartialEq)]
pub enum UpdateGameServerError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl UpdateGameServerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateGameServerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateGameServerError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateGameServerError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateGameServerError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateGameServerError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateGameServerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateGameServerError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateGameServerError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateGameServerError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateGameServerError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateGameServerError {}
/// Errors returned by UpdateGameServerGroup
#[derive(Debug, PartialEq)]
pub enum UpdateGameServerGroupError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl UpdateGameServerGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateGameServerGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateGameServerGroupError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateGameServerGroupError::InvalidRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateGameServerGroupError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateGameServerGroupError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateGameServerGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateGameServerGroupError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateGameServerGroupError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateGameServerGroupError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateGameServerGroupError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateGameServerGroupError {}
/// Errors returned by UpdateGameSession
#[derive(Debug, PartialEq)]
pub enum UpdateGameSessionError {
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request.</p>
    Conflict(String),
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>The requested operation would cause a conflict with the current state of a resource associated with the request and/or the game instance. Resolve the conflict before retrying.</p>
    InvalidGameSessionStatus(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl UpdateGameSessionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateGameSessionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(UpdateGameSessionError::Conflict(err.msg))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateGameSessionError::InternalService(err.msg))
                }
                "InvalidGameSessionStatusException" => {
                    return RusotoError::Service(UpdateGameSessionError::InvalidGameSessionStatus(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateGameSessionError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateGameSessionError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateGameSessionError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateGameSessionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateGameSessionError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateGameSessionError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateGameSessionError::InvalidGameSessionStatus(ref cause) => write!(f, "{}", cause),
            UpdateGameSessionError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateGameSessionError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateGameSessionError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateGameSessionError {}
/// Errors returned by UpdateGameSessionQueue
#[derive(Debug, PartialEq)]
pub enum UpdateGameSessionQueueError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl UpdateGameSessionQueueError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateGameSessionQueueError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateGameSessionQueueError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateGameSessionQueueError::InvalidRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateGameSessionQueueError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateGameSessionQueueError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateGameSessionQueueError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateGameSessionQueueError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateGameSessionQueueError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateGameSessionQueueError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateGameSessionQueueError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateGameSessionQueueError {}
/// Errors returned by UpdateMatchmakingConfiguration
#[derive(Debug, PartialEq)]
pub enum UpdateMatchmakingConfigurationError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The requested operation is not supported in the Region specified.</p>
    UnsupportedRegion(String),
}

impl UpdateMatchmakingConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateMatchmakingConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(
                        UpdateMatchmakingConfigurationError::InternalService(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        UpdateMatchmakingConfigurationError::InvalidRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateMatchmakingConfigurationError::NotFound(
                        err.msg,
                    ))
                }
                "UnsupportedRegionException" => {
                    return RusotoError::Service(
                        UpdateMatchmakingConfigurationError::UnsupportedRegion(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateMatchmakingConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateMatchmakingConfigurationError::InternalService(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateMatchmakingConfigurationError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateMatchmakingConfigurationError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateMatchmakingConfigurationError::UnsupportedRegion(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateMatchmakingConfigurationError {}
/// Errors returned by UpdateRuntimeConfiguration
#[derive(Debug, PartialEq)]
pub enum UpdateRuntimeConfigurationError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>The requested operation would cause a conflict with the current state of a resource associated with the request and/or the fleet. Resolve the conflict before retrying.</p>
    InvalidFleetStatus(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl UpdateRuntimeConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateRuntimeConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateRuntimeConfigurationError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidFleetStatusException" => {
                    return RusotoError::Service(
                        UpdateRuntimeConfigurationError::InvalidFleetStatus(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateRuntimeConfigurationError::InvalidRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateRuntimeConfigurationError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateRuntimeConfigurationError::Unauthorized(
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
impl fmt::Display for UpdateRuntimeConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRuntimeConfigurationError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateRuntimeConfigurationError::InvalidFleetStatus(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateRuntimeConfigurationError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateRuntimeConfigurationError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateRuntimeConfigurationError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateRuntimeConfigurationError {}
/// Errors returned by UpdateScript
#[derive(Debug, PartialEq)]
pub enum UpdateScriptError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>A service resource associated with the request could not be found. Clients should not retry such requests.</p>
    NotFound(String),
    /// <p>The client failed authentication. Clients should not retry such requests.</p>
    Unauthorized(String),
}

impl UpdateScriptError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateScriptError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UpdateScriptError::InternalService(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateScriptError::InvalidRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateScriptError::NotFound(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateScriptError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateScriptError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateScriptError::InternalService(ref cause) => write!(f, "{}", cause),
            UpdateScriptError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateScriptError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateScriptError::Unauthorized(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateScriptError {}
/// Errors returned by ValidateMatchmakingRuleSet
#[derive(Debug, PartialEq)]
pub enum ValidateMatchmakingRuleSetError {
    /// <p>The service encountered an unrecoverable internal failure while processing the request. Clients can retry such requests immediately or after a waiting period.</p>
    InternalService(String),
    /// <p>One or more parameter values in the request are invalid. Correct the invalid parameter values before retrying.</p>
    InvalidRequest(String),
    /// <p>The requested operation is not supported in the Region specified.</p>
    UnsupportedRegion(String),
}

impl ValidateMatchmakingRuleSetError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ValidateMatchmakingRuleSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(ValidateMatchmakingRuleSetError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ValidateMatchmakingRuleSetError::InvalidRequest(
                        err.msg,
                    ))
                }
                "UnsupportedRegionException" => {
                    return RusotoError::Service(
                        ValidateMatchmakingRuleSetError::UnsupportedRegion(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ValidateMatchmakingRuleSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ValidateMatchmakingRuleSetError::InternalService(ref cause) => write!(f, "{}", cause),
            ValidateMatchmakingRuleSetError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            ValidateMatchmakingRuleSetError::UnsupportedRegion(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ValidateMatchmakingRuleSetError {}
/// Trait representing the capabilities of the Amazon GameLift API. Amazon GameLift clients implement this trait.
#[async_trait]
pub trait GameLift {
    /// <p><p>Registers a player&#39;s acceptance or rejection of a proposed FlexMatch match. A matchmaking configuration may require player acceptance; if so, then matches built with that configuration cannot be completed unless all players accept the proposed match within a specified time limit. </p> <p>When FlexMatch builds a match, all the matchmaking tickets involved in the proposed match are placed into status <code>REQUIRES_ACCEPTANCE</code>. This is a trigger for your game to get acceptance from all players in the ticket. Acceptances are only valid for tickets when they are in this status; all other acceptances result in an error.</p> <p>To register acceptance, specify the ticket ID, a response, and one or more players. Once all players have registered acceptance, the matchmaking tickets advance to status <code>PLACING</code>, where a new game session is created for the match. </p> <p>If any player rejects the match, or if acceptances are not received before a specified timeout, the proposed match is dropped. The matchmaking tickets are then handled in one of two ways: For tickets where one or more players rejected the match, the ticket status is returned to <code>SEARCHING</code> to find a new match. For tickets where one or more players failed to respond, the ticket status is set to <code>CANCELLED</code>, and processing is terminated. A new matchmaking request for these players can be submitted as needed. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-client.html"> Add FlexMatch to a Game Client</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-events.html"> FlexMatch Events Reference</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>StartMatchmaking</a> </p> </li> <li> <p> <a>DescribeMatchmaking</a> </p> </li> <li> <p> <a>StopMatchmaking</a> </p> </li> <li> <p> <a>AcceptMatch</a> </p> </li> <li> <p> <a>StartMatchBackfill</a> </p> </li> </ul></p>
    async fn accept_match(
        &self,
        input: AcceptMatchInput,
    ) -> Result<AcceptMatchOutput, RusotoError<AcceptMatchError>>;

    /// <p><p> <b>This operation is used with the Amazon GameLift FleetIQ solution and game server groups.</b> </p> <p>Locates an available game server and temporarily reserves it to host gameplay and players. This operation is called from a game client or client service (such as a matchmaker) to request hosting resources for a new game session. In response, GameLift FleetIQ locates an available game server, places it in <code>CLAIMED</code> status for 60 seconds, and returns connection information that players can use to connect to the game server. </p> <p>To claim a game server, identify a game server group. You can also specify a game server ID, although this approach bypasses GameLift FleetIQ placement optimization. Optionally, include game data to pass to the game server at the start of a game session, such as a game map or player information. </p> <p>When a game server is successfully claimed, connection information is returned. A claimed game server&#39;s utilization status remains <code>AVAILABLE</code> while the claim status is set to <code>CLAIMED</code> for up to 60 seconds. This time period gives the game server time to update its status to <code>UTILIZED</code> (using <a>UpdateGameServer</a>) once players join. If the game server&#39;s status is not updated within 60 seconds, the game server reverts to unclaimed status and is available to be claimed by another request. The claim time period is a fixed value and is not configurable.</p> <p>If you try to claim a specific game server, this request will fail in the following cases:</p> <ul> <li> <p>If the game server utilization status is <code>UTILIZED</code>.</p> </li> <li> <p>If the game server claim status is <code>CLAIMED</code>.</p> </li> </ul> <note> <p>When claiming a specific game server, this request will succeed even if the game server is running on an instance in <code>DRAINING</code> status. To avoid this, first check the instance status by calling <a>DescribeGameServerInstances</a>.</p> </note> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">GameLift FleetIQ Guide</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>RegisterGameServer</a> </p> </li> <li> <p> <a>ListGameServers</a> </p> </li> <li> <p> <a>ClaimGameServer</a> </p> </li> <li> <p> <a>DescribeGameServer</a> </p> </li> <li> <p> <a>UpdateGameServer</a> </p> </li> <li> <p> <a>DeregisterGameServer</a> </p> </li> </ul></p>
    async fn claim_game_server(
        &self,
        input: ClaimGameServerInput,
    ) -> Result<ClaimGameServerOutput, RusotoError<ClaimGameServerError>>;

    /// <p><p>Creates an alias for a fleet. In most situations, you can use an alias ID in place of a fleet ID. An alias provides a level of abstraction for a fleet that is useful when redirecting player traffic from one fleet to another, such as when updating your game build. </p> <p>Amazon GameLift supports two types of routing strategies for aliases: simple and terminal. A simple alias points to an active fleet. A terminal alias is used to display messaging or link to a URL instead of routing players to an active fleet. For example, you might use a terminal alias when a game version is no longer supported and you want to direct players to an upgrade site. </p> <p>To create a fleet alias, specify an alias name, routing strategy, and optional description. Each simple alias can point to only one fleet, but a fleet can have multiple aliases. If successful, a new alias record is returned, including an alias ID and an ARN. You can reassign an alias to another fleet by calling <code>UpdateAlias</code>.</p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>DescribeAlias</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ResolveAlias</a> </p> </li> </ul></p>
    async fn create_alias(
        &self,
        input: CreateAliasInput,
    ) -> Result<CreateAliasOutput, RusotoError<CreateAliasError>>;

    /// <p><p>Creates a new Amazon GameLift build resource for your game server binary files. Game server binaries must be combined into a zip file for use with Amazon GameLift. </p> <important> <p>When setting up a new game build for GameLift, we recommend using the AWS CLI command <b> <a href="https://docs.aws.amazon.com/cli/latest/reference/gamelift/upload-build.html">upload-build</a> </b>. This helper command combines two tasks: (1) it uploads your build files from a file directory to a GameLift Amazon S3 location, and (2) it creates a new build resource. </p> </important> <p>The <code>CreateBuild</code> operation can used in the following scenarios:</p> <ul> <li> <p>To create a new game build with build files that are in an S3 location under an AWS account that you control. To use this option, you must first give Amazon GameLift access to the S3 bucket. With permissions in place, call <code>CreateBuild</code> and specify a build name, operating system, and the S3 storage location of your game build.</p> </li> <li> <p>To directly upload your build files to a GameLift S3 location. To use this option, first call <code>CreateBuild</code> and specify a build name and operating system. This operation creates a new build resource and also returns an S3 location with temporary access credentials. Use the credentials to manually upload your build files to the specified S3 location. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UploadingObjects.html">Uploading Objects</a> in the <i>Amazon S3 Developer Guide</i>. Build files can be uploaded to the GameLift S3 location once only; that can&#39;t be updated. </p> </li> </ul> <p>If successful, this operation creates a new build resource with a unique build ID and places it in <code>INITIALIZED</code> status. A build must be in <code>READY</code> status before you can create fleets with it.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-build-intro.html">Uploading Your Game</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-build-cli-uploading.html#gamelift-build-cli-uploading-create-build"> Create a Build with Files in Amazon S3</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateBuild</a> </p> </li> <li> <p> <a>ListBuilds</a> </p> </li> <li> <p> <a>DescribeBuild</a> </p> </li> <li> <p> <a>UpdateBuild</a> </p> </li> <li> <p> <a>DeleteBuild</a> </p> </li> </ul></p>
    async fn create_build(
        &self,
        input: CreateBuildInput,
    ) -> Result<CreateBuildOutput, RusotoError<CreateBuildError>>;

    /// <p><p>Creates a new fleet to run your game servers. whether they are custom game builds or Realtime Servers with game-specific script. A fleet is a set of Amazon Elastic Compute Cloud (Amazon EC2) instances, each of which can host multiple game sessions. When creating a fleet, you choose the hardware specifications, set some configuration options, and specify the game server to deploy on the new fleet. </p> <p>To create a new fleet, provide the following: (1) a fleet name, (2) an EC2 instance type and fleet type (spot or on-demand), (3) the build ID for your game build or script ID if using Realtime Servers, and (4) a runtime configuration, which determines how game servers will run on each instance in the fleet. </p> <p>If the <code>CreateFleet</code> call is successful, Amazon GameLift performs the following tasks. You can track the process of a fleet by checking the fleet status or by monitoring fleet creation events:</p> <ul> <li> <p>Creates a fleet resource. Status: <code>NEW</code>.</p> </li> <li> <p>Begins writing events to the fleet event log, which can be accessed in the Amazon GameLift console.</p> </li> <li> <p>Sets the fleet&#39;s target capacity to 1 (desired instances), which triggers Amazon GameLift to start one new EC2 instance.</p> </li> <li> <p>Downloads the game build or Realtime script to the new instance and installs it. Statuses: <code>DOWNLOADING</code>, <code>VALIDATING</code>, <code>BUILDING</code>. </p> </li> <li> <p>Starts launching server processes on the instance. If the fleet is configured to run multiple server processes per instance, Amazon GameLift staggers each process launch by a few seconds. Status: <code>ACTIVATING</code>.</p> </li> <li> <p>Sets the fleet&#39;s status to <code>ACTIVE</code> as soon as one server process is ready to host a game session.</p> </li> </ul> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting Up Fleets</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-creating-debug.html#fleets-creating-debug-creation">Debug Fleet Creation Issues</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn create_fleet(
        &self,
        input: CreateFleetInput,
    ) -> Result<CreateFleetOutput, RusotoError<CreateFleetError>>;

    /// <p><p> <b>This operation is used with the Amazon GameLift FleetIQ solution and game server groups.</b> </p> <p>Creates a GameLift FleetIQ game server group for managing game hosting on a collection of Amazon EC2 instances for game hosting. This operation creates the game server group, creates an Auto Scaling group in your AWS account, and establishes a link between the two groups. You can view the status of your game server groups in the GameLift console. Game server group metrics and events are emitted to Amazon CloudWatch.</p> <p>Before creating a new game server group, you must have the following: </p> <ul> <li> <p>An Amazon EC2 launch template that specifies how to launch Amazon EC2 instances with your game server build. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-launch-templates.html"> Launching an Instance from a Launch Template</a> in the <i>Amazon EC2 User Guide</i>. </p> </li> <li> <p>An IAM role that extends limited access to your AWS account to allow GameLift FleetIQ to create and interact with the Auto Scaling group. For more information, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gsg-iam-permissions-roles.html">Create IAM roles for cross-service interaction</a> in the <i>GameLift FleetIQ Developer Guide</i>.</p> </li> </ul> <p>To create a new game server group, specify a unique group name, IAM role and Amazon EC2 launch template, and provide a list of instance types that can be used in the group. You must also set initial maximum and minimum limits on the group&#39;s instance count. You can optionally set an Auto Scaling policy with target tracking based on a GameLift FleetIQ metric.</p> <p>Once the game server group and corresponding Auto Scaling group are created, you have full access to change the Auto Scaling group&#39;s configuration as needed. Several properties that are set when creating a game server group, including maximum/minimum size and auto-scaling policy settings, must be updated directly in the Auto Scaling group. Keep in mind that some Auto Scaling group properties are periodically updated by GameLift FleetIQ as part of its balancing activities to optimize for availability and cost.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">GameLift FleetIQ Guide</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateGameServerGroup</a> </p> </li> <li> <p> <a>ListGameServerGroups</a> </p> </li> <li> <p> <a>DescribeGameServerGroup</a> </p> </li> <li> <p> <a>UpdateGameServerGroup</a> </p> </li> <li> <p> <a>DeleteGameServerGroup</a> </p> </li> <li> <p> <a>ResumeGameServerGroup</a> </p> </li> <li> <p> <a>SuspendGameServerGroup</a> </p> </li> <li> <p> <a>DescribeGameServerInstances</a> </p> </li> </ul></p>
    async fn create_game_server_group(
        &self,
        input: CreateGameServerGroupInput,
    ) -> Result<CreateGameServerGroupOutput, RusotoError<CreateGameServerGroupError>>;

    /// <p><p>Creates a multiplayer game session for players. This operation creates a game session record and assigns an available server process in the specified fleet to host the game session. A fleet must have an <code>ACTIVE</code> status before a game session can be created in it.</p> <p>To create a game session, specify either fleet ID or alias ID and indicate a maximum number of players to allow in the game session. You can also provide a name and game-specific properties for this game session. If successful, a <a>GameSession</a> object is returned containing the game session properties and other settings you specified.</p> <p> <b>Idempotency tokens.</b> You can add a token that uniquely identifies game session requests. This is useful for ensuring that game session requests are idempotent. Multiple requests with the same idempotency token are processed only once; subsequent requests return the original result. All response values are the same with the exception of game session status, which may change.</p> <p> <b>Resource creation limits.</b> If you are creating a game session on a fleet with a resource creation limit policy in force, then you must specify a creator ID. Without this ID, Amazon GameLift has no way to evaluate the policy for this new game session request.</p> <p> <b>Player acceptance policy.</b> By default, newly created game sessions are open to new players. You can restrict new player access by using <a>UpdateGameSession</a> to change the game session&#39;s player session creation policy.</p> <p> <b>Game session logs.</b> Logs are retained for all active game sessions for 14 days. To access the logs, call <a>GetGameSessionLogUrl</a> to download the log files.</p> <p> <i>Available in Amazon GameLift Local.</i> </p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul></p>
    async fn create_game_session(
        &self,
        input: CreateGameSessionInput,
    ) -> Result<CreateGameSessionOutput, RusotoError<CreateGameSessionError>>;

    /// <p><p>Establishes a new queue for processing requests to place new game sessions. A queue identifies where new game sessions can be hosted -- by specifying a list of destinations (fleets or aliases) -- and how long requests can wait in the queue before timing out. You can set up a queue to try to place game sessions on fleets in multiple Regions. To add placement requests to a queue, call <a>StartGameSessionPlacement</a> and reference the queue name.</p> <p> <b>Destination order.</b> When processing a request for a game session, Amazon GameLift tries each destination in order until it finds one with available resources to host the new game session. A queue&#39;s default order is determined by how destinations are listed. The default order is overridden when a game session placement request provides player latency information. Player latency information enables Amazon GameLift to prioritize destinations where players report the lowest average latency, as a result placing the new game session where the majority of players will have the best possible gameplay experience.</p> <p> <b>Player latency policies.</b> For placement requests containing player latency information, use player latency policies to protect individual players from very high latencies. With a latency cap, even when a destination can deliver a low latency for most players, the game is not placed where any individual player is reporting latency higher than a policy&#39;s maximum. A queue can have multiple latency policies, which are enforced consecutively starting with the policy with the lowest latency cap. Use multiple policies to gradually relax latency controls; for example, you might set a policy with a low latency cap for the first 60 seconds, a second policy with a higher cap for the next 60 seconds, etc. </p> <p>To create a new queue, provide a name, timeout value, a list of destinations and, if desired, a set of latency policies. If successful, a new queue object is returned.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/queues-design.html"> Design a Game Session Queue</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/queues-creating.html"> Create a Game Session Queue</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateGameSessionQueue</a> </p> </li> <li> <p> <a>DescribeGameSessionQueues</a> </p> </li> <li> <p> <a>UpdateGameSessionQueue</a> </p> </li> <li> <p> <a>DeleteGameSessionQueue</a> </p> </li> </ul></p>
    async fn create_game_session_queue(
        &self,
        input: CreateGameSessionQueueInput,
    ) -> Result<CreateGameSessionQueueOutput, RusotoError<CreateGameSessionQueueError>>;

    /// <p><p>Defines a new matchmaking configuration for use with FlexMatch. Whether your are using FlexMatch with GameLift hosting or as a standalone matchmaking service, the matchmaking configuration sets out rules for matching players and forming teams. If you&#39;re also using GameLift hosting, it defines how to start game sessions for each match. Your matchmaking system can use multiple configurations to handle different game scenarios. All matchmaking requests (<a>StartMatchmaking</a> or <a>StartMatchBackfill</a>) identify the matchmaking configuration to use and provide player attributes consistent with that configuration. </p> <p>To create a matchmaking configuration, you must provide the following: configuration name and FlexMatch mode (with or without GameLift hosting); a rule set that specifies how to evaluate players and find acceptable matches; whether player acceptance is required; and the maximum time allowed for a matchmaking attempt. When using FlexMatch with GameLift hosting, you also need to identify the game session queue to use when starting a game session for the match.</p> <p>In addition, you must set up an Amazon Simple Notification Service (SNS) to receive matchmaking notifications, and provide the topic ARN in the matchmaking configuration. An alternative method, continuously polling ticket status with <a>DescribeMatchmaking</a>, is only suitable for games in development with low matchmaking usage.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/gamelift-match.html"> FlexMatch Developer Guide</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-configuration.html"> Design a FlexMatch Matchmaker</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-notification.html"> Set Up FlexMatch Event Notification</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DeleteMatchmakingRuleSet</a> </p> </li> </ul></p>
    async fn create_matchmaking_configuration(
        &self,
        input: CreateMatchmakingConfigurationInput,
    ) -> Result<
        CreateMatchmakingConfigurationOutput,
        RusotoError<CreateMatchmakingConfigurationError>,
    >;

    /// <p><p>Creates a new rule set for FlexMatch matchmaking. A rule set describes the type of match to create, such as the number and size of teams. It also sets the parameters for acceptable player matches, such as minimum skill level or character type. A rule set is used by a <a>MatchmakingConfiguration</a>. </p> <p>To create a matchmaking rule set, provide unique rule set name and the rule set body in JSON format. Rule sets must be defined in the same Region as the matchmaking configuration they are used with.</p> <p>Since matchmaking rule sets cannot be edited, it is a good idea to check the rule set syntax using <a>ValidateMatchmakingRuleSet</a> before creating a new rule set.</p> <p> <b>Learn more</b> </p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-rulesets.html">Build a Rule Set</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-configuration.html">Design a Matchmaker</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-intro.html">Matchmaking with FlexMatch</a> </p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DeleteMatchmakingRuleSet</a> </p> </li> </ul></p>
    async fn create_matchmaking_rule_set(
        &self,
        input: CreateMatchmakingRuleSetInput,
    ) -> Result<CreateMatchmakingRuleSetOutput, RusotoError<CreateMatchmakingRuleSetError>>;

    /// <p><p>Reserves an open player slot in an active game session. Before a player can be added, a game session must have an <code>ACTIVE</code> status, have a creation policy of <code>ALLOW_ALL</code>, and have an open player slot. To add a group of players to a game session, use <a>CreatePlayerSessions</a>. When the player connects to the game server and references a player session ID, the game server contacts the Amazon GameLift service to validate the player reservation and accept the player.</p> <p>To create a player session, specify a game session ID, player ID, and optionally a string of player data. If successful, a slot is reserved in the game session for the player and a new <a>PlayerSession</a> object is returned. Player sessions cannot be updated. </p> <p> <i>Available in Amazon GameLift Local.</i> </p> <ul> <li> <p> <a>CreatePlayerSession</a> </p> </li> <li> <p> <a>CreatePlayerSessions</a> </p> </li> <li> <p> <a>DescribePlayerSessions</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul></p>
    async fn create_player_session(
        &self,
        input: CreatePlayerSessionInput,
    ) -> Result<CreatePlayerSessionOutput, RusotoError<CreatePlayerSessionError>>;

    /// <p><p>Reserves open slots in a game session for a group of players. Before players can be added, a game session must have an <code>ACTIVE</code> status, have a creation policy of <code>ALLOW_ALL</code>, and have an open player slot. To add a single player to a game session, use <a>CreatePlayerSession</a>. When a player connects to the game server and references a player session ID, the game server contacts the Amazon GameLift service to validate the player reservation and accept the player.</p> <p>To create player sessions, specify a game session ID, a list of player IDs, and optionally a set of player data strings. If successful, a slot is reserved in the game session for each player and a set of new <a>PlayerSession</a> objects is returned. Player sessions cannot be updated.</p> <p> <i>Available in Amazon GameLift Local.</i> </p> <ul> <li> <p> <a>CreatePlayerSession</a> </p> </li> <li> <p> <a>CreatePlayerSessions</a> </p> </li> <li> <p> <a>DescribePlayerSessions</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul></p>
    async fn create_player_sessions(
        &self,
        input: CreatePlayerSessionsInput,
    ) -> Result<CreatePlayerSessionsOutput, RusotoError<CreatePlayerSessionsError>>;

    /// <p><p>Creates a new script record for your Realtime Servers script. Realtime scripts are JavaScript that provide configuration settings and optional custom game logic for your game. The script is deployed when you create a Realtime Servers fleet to host your game sessions. Script logic is executed during an active game session. </p> <p>To create a new script record, specify a script name and provide the script file(s). The script files and all dependencies must be zipped into a single file. You can pull the zip file from either of these locations: </p> <ul> <li> <p>A locally available directory. Use the <i>ZipFile</i> parameter for this option.</p> </li> <li> <p>An Amazon Simple Storage Service (Amazon S3) bucket under your AWS account. Use the <i>StorageLocation</i> parameter for this option. You&#39;ll need to have an Identity Access Management (IAM) role that allows the Amazon GameLift service to access your S3 bucket. </p> </li> </ul> <p>If the call is successful, a new script record is created with a unique script ID. If the script file is provided as a local file, the file is uploaded to an Amazon GameLift-owned S3 bucket and the script record&#39;s storage location reflects this location. If the script file is provided as an S3 bucket, Amazon GameLift accesses the file at this storage location as needed for deployment.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/realtime-intro.html">Amazon GameLift Realtime Servers</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/setting-up-role.html">Set Up a Role for Amazon GameLift Access</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateScript</a> </p> </li> <li> <p> <a>ListScripts</a> </p> </li> <li> <p> <a>DescribeScript</a> </p> </li> <li> <p> <a>UpdateScript</a> </p> </li> <li> <p> <a>DeleteScript</a> </p> </li> </ul></p>
    async fn create_script(
        &self,
        input: CreateScriptInput,
    ) -> Result<CreateScriptOutput, RusotoError<CreateScriptError>>;

    /// <p><p>Requests authorization to create or delete a peer connection between the VPC for your Amazon GameLift fleet and a virtual private cloud (VPC) in your AWS account. VPC peering enables the game servers on your fleet to communicate directly with other AWS resources. Once you&#39;ve received authorization, call <a>CreateVpcPeeringConnection</a> to establish the peering connection. For more information, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/vpc-peering.html">VPC Peering with Amazon GameLift Fleets</a>.</p> <p>You can peer with VPCs that are owned by any AWS account you have access to, including the account that you use to manage your Amazon GameLift fleets. You cannot peer with VPCs that are in different Regions.</p> <p>To request authorization to create a connection, call this operation from the AWS account with the VPC that you want to peer to your Amazon GameLift fleet. For example, to enable your game servers to retrieve data from a DynamoDB table, use the account that manages that DynamoDB resource. Identify the following values: (1) The ID of the VPC that you want to peer with, and (2) the ID of the AWS account that you use to manage Amazon GameLift. If successful, VPC peering is authorized for the specified VPC. </p> <p>To request authorization to delete a connection, call this operation from the AWS account with the VPC that is peered with your Amazon GameLift fleet. Identify the following values: (1) VPC ID that you want to delete the peering connection for, and (2) ID of the AWS account that you use to manage Amazon GameLift. </p> <p>The authorization remains valid for 24 hours unless it is canceled by a call to <a>DeleteVpcPeeringAuthorization</a>. You must create or delete the peering connection while the authorization is valid. </p> <ul> <li> <p> <a>CreateVpcPeeringAuthorization</a> </p> </li> <li> <p> <a>DescribeVpcPeeringAuthorizations</a> </p> </li> <li> <p> <a>DeleteVpcPeeringAuthorization</a> </p> </li> <li> <p> <a>CreateVpcPeeringConnection</a> </p> </li> <li> <p> <a>DescribeVpcPeeringConnections</a> </p> </li> <li> <p> <a>DeleteVpcPeeringConnection</a> </p> </li> </ul></p>
    async fn create_vpc_peering_authorization(
        &self,
        input: CreateVpcPeeringAuthorizationInput,
    ) -> Result<CreateVpcPeeringAuthorizationOutput, RusotoError<CreateVpcPeeringAuthorizationError>>;

    /// <p><p>Establishes a VPC peering connection between a virtual private cloud (VPC) in an AWS account with the VPC for your Amazon GameLift fleet. VPC peering enables the game servers on your fleet to communicate directly with other AWS resources. You can peer with VPCs in any AWS account that you have access to, including the account that you use to manage your Amazon GameLift fleets. You cannot peer with VPCs that are in different Regions. For more information, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/vpc-peering.html">VPC Peering with Amazon GameLift Fleets</a>.</p> <p>Before calling this operation to establish the peering connection, you first need to call <a>CreateVpcPeeringAuthorization</a> and identify the VPC you want to peer with. Once the authorization for the specified VPC is issued, you have 24 hours to establish the connection. These two operations handle all tasks necessary to peer the two VPCs, including acceptance, updating routing tables, etc. </p> <p>To establish the connection, call this operation from the AWS account that is used to manage the Amazon GameLift fleets. Identify the following values: (1) The ID of the fleet you want to be enable a VPC peering connection for; (2) The AWS account with the VPC that you want to peer with; and (3) The ID of the VPC you want to peer with. This operation is asynchronous. If successful, a <a>VpcPeeringConnection</a> request is created. You can use continuous polling to track the request&#39;s status using <a>DescribeVpcPeeringConnections</a>, or by monitoring fleet events for success or failure using <a>DescribeFleetEvents</a>. </p> <ul> <li> <p> <a>CreateVpcPeeringAuthorization</a> </p> </li> <li> <p> <a>DescribeVpcPeeringAuthorizations</a> </p> </li> <li> <p> <a>DeleteVpcPeeringAuthorization</a> </p> </li> <li> <p> <a>CreateVpcPeeringConnection</a> </p> </li> <li> <p> <a>DescribeVpcPeeringConnections</a> </p> </li> <li> <p> <a>DeleteVpcPeeringConnection</a> </p> </li> </ul></p>
    async fn create_vpc_peering_connection(
        &self,
        input: CreateVpcPeeringConnectionInput,
    ) -> Result<CreateVpcPeeringConnectionOutput, RusotoError<CreateVpcPeeringConnectionError>>;

    /// <p><p>Deletes an alias. This operation removes all record of the alias. Game clients attempting to access a server process using the deleted alias receive an error. To delete an alias, specify the alias ID to be deleted.</p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>DescribeAlias</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ResolveAlias</a> </p> </li> </ul></p>
    async fn delete_alias(
        &self,
        input: DeleteAliasInput,
    ) -> Result<(), RusotoError<DeleteAliasError>>;

    /// <p><p>Deletes a build. This operation permanently deletes the build resource and any uploaded build files. Deleting a build does not affect the status of any active fleets using the build, but you can no longer create new fleets with the deleted build.</p> <p>To delete a build, specify the build ID. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-build-intro.html"> Upload a Custom Server Build</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateBuild</a> </p> </li> <li> <p> <a>ListBuilds</a> </p> </li> <li> <p> <a>DescribeBuild</a> </p> </li> <li> <p> <a>UpdateBuild</a> </p> </li> <li> <p> <a>DeleteBuild</a> </p> </li> </ul></p>
    async fn delete_build(
        &self,
        input: DeleteBuildInput,
    ) -> Result<(), RusotoError<DeleteBuildError>>;

    /// <p><p>Deletes everything related to a fleet. Before deleting a fleet, you must set the fleet&#39;s desired capacity to zero. See <a>UpdateFleetCapacity</a>.</p> <p>If the fleet being deleted has a VPC peering connection, you first need to get a valid authorization (good for 24 hours) by calling <a>CreateVpcPeeringAuthorization</a>. You do not need to explicitly delete the VPC peering connection--this is done as part of the delete fleet process.</p> <p>This operation removes the fleet and its resources. Once a fleet is deleted, you can no longer use any of the resource in that fleet.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn delete_fleet(
        &self,
        input: DeleteFleetInput,
    ) -> Result<(), RusotoError<DeleteFleetError>>;

    /// <p><p> <b>This operation is used with the Amazon GameLift FleetIQ solution and game server groups.</b> </p> <p>Terminates a game server group and permanently deletes the game server group record. You have several options for how these resources are impacted when deleting the game server group. Depending on the type of delete operation selected, this operation might affect these resources:</p> <ul> <li> <p>The game server group</p> </li> <li> <p>The corresponding Auto Scaling group</p> </li> <li> <p>All game servers that are currently running in the group</p> </li> </ul> <p>To delete a game server group, identify the game server group to delete and specify the type of delete operation to initiate. Game server groups can only be deleted if they are in <code>ACTIVE</code> or <code>ERROR</code> status.</p> <p>If the delete request is successful, a series of operations are kicked off. The game server group status is changed to <code>DELETE_SCHEDULED</code>, which prevents new game servers from being registered and stops automatic scaling activity. Once all game servers in the game server group are deregistered, GameLift FleetIQ can begin deleting resources. If any of the delete operations fail, the game server group is placed in <code>ERROR</code> status.</p> <p>GameLift FleetIQ emits delete events to Amazon CloudWatch.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">GameLift FleetIQ Guide</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateGameServerGroup</a> </p> </li> <li> <p> <a>ListGameServerGroups</a> </p> </li> <li> <p> <a>DescribeGameServerGroup</a> </p> </li> <li> <p> <a>UpdateGameServerGroup</a> </p> </li> <li> <p> <a>DeleteGameServerGroup</a> </p> </li> <li> <p> <a>ResumeGameServerGroup</a> </p> </li> <li> <p> <a>SuspendGameServerGroup</a> </p> </li> <li> <p> <a>DescribeGameServerInstances</a> </p> </li> </ul></p>
    async fn delete_game_server_group(
        &self,
        input: DeleteGameServerGroupInput,
    ) -> Result<DeleteGameServerGroupOutput, RusotoError<DeleteGameServerGroupError>>;

    /// <p><p>Deletes a game session queue. Once a queue is successfully deleted, unfulfilled <a>StartGameSessionPlacement</a> requests that reference the queue will fail. To delete a queue, specify the queue name.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/queues-intro.html"> Using Multi-Region Queues</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateGameSessionQueue</a> </p> </li> <li> <p> <a>DescribeGameSessionQueues</a> </p> </li> <li> <p> <a>UpdateGameSessionQueue</a> </p> </li> <li> <p> <a>DeleteGameSessionQueue</a> </p> </li> </ul></p>
    async fn delete_game_session_queue(
        &self,
        input: DeleteGameSessionQueueInput,
    ) -> Result<DeleteGameSessionQueueOutput, RusotoError<DeleteGameSessionQueueError>>;

    /// <p><p>Permanently removes a FlexMatch matchmaking configuration. To delete, specify the configuration name. A matchmaking configuration cannot be deleted if it is being used in any active matchmaking tickets.</p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DeleteMatchmakingRuleSet</a> </p> </li> </ul></p>
    async fn delete_matchmaking_configuration(
        &self,
        input: DeleteMatchmakingConfigurationInput,
    ) -> Result<
        DeleteMatchmakingConfigurationOutput,
        RusotoError<DeleteMatchmakingConfigurationError>,
    >;

    /// <p><p>Deletes an existing matchmaking rule set. To delete the rule set, provide the rule set name. Rule sets cannot be deleted if they are currently being used by a matchmaking configuration. </p> <p> <b>Learn more</b> </p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-rulesets.html">Build a Rule Set</a> </p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DeleteMatchmakingRuleSet</a> </p> </li> </ul></p>
    async fn delete_matchmaking_rule_set(
        &self,
        input: DeleteMatchmakingRuleSetInput,
    ) -> Result<DeleteMatchmakingRuleSetOutput, RusotoError<DeleteMatchmakingRuleSetError>>;

    /// <p><p>Deletes a fleet scaling policy. Once deleted, the policy is no longer in force and GameLift removes all record of it. To delete a scaling policy, specify both the scaling policy name and the fleet ID it is associated with.</p> <p>To temporarily suspend scaling policies, call <a>StopFleetActions</a>. This operation suspends all policies for the fleet.</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> <li> <p>Manage scaling policies:</p> <ul> <li> <p> <a>PutScalingPolicy</a> (auto-scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (auto-scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (auto-scaling)</p> </li> </ul> </li> <li> <p>Manage fleet actions:</p> <ul> <li> <p> <a>StartFleetActions</a> </p> </li> <li> <p> <a>StopFleetActions</a> </p> </li> </ul> </li> </ul></p>
    async fn delete_scaling_policy(
        &self,
        input: DeleteScalingPolicyInput,
    ) -> Result<(), RusotoError<DeleteScalingPolicyError>>;

    /// <p><p>Deletes a Realtime script. This operation permanently deletes the script record. If script files were uploaded, they are also deleted (files stored in an S3 bucket are not deleted). </p> <p>To delete a script, specify the script ID. Before deleting a script, be sure to terminate all fleets that are deployed with the script being deleted. Fleet instances periodically check for script updates, and if the script record no longer exists, the instance will go into an error state and be unable to host game sessions.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/realtime-intro.html">Amazon GameLift Realtime Servers</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateScript</a> </p> </li> <li> <p> <a>ListScripts</a> </p> </li> <li> <p> <a>DescribeScript</a> </p> </li> <li> <p> <a>UpdateScript</a> </p> </li> <li> <p> <a>DeleteScript</a> </p> </li> </ul></p>
    async fn delete_script(
        &self,
        input: DeleteScriptInput,
    ) -> Result<(), RusotoError<DeleteScriptError>>;

    /// <p><p>Cancels a pending VPC peering authorization for the specified VPC. If you need to delete an existing VPC peering connection, call <a>DeleteVpcPeeringConnection</a>. </p> <ul> <li> <p> <a>CreateVpcPeeringAuthorization</a> </p> </li> <li> <p> <a>DescribeVpcPeeringAuthorizations</a> </p> </li> <li> <p> <a>DeleteVpcPeeringAuthorization</a> </p> </li> <li> <p> <a>CreateVpcPeeringConnection</a> </p> </li> <li> <p> <a>DescribeVpcPeeringConnections</a> </p> </li> <li> <p> <a>DeleteVpcPeeringConnection</a> </p> </li> </ul></p>
    async fn delete_vpc_peering_authorization(
        &self,
        input: DeleteVpcPeeringAuthorizationInput,
    ) -> Result<DeleteVpcPeeringAuthorizationOutput, RusotoError<DeleteVpcPeeringAuthorizationError>>;

    /// <p><p>Removes a VPC peering connection. To delete the connection, you must have a valid authorization for the VPC peering connection that you want to delete. You can check for an authorization by calling <a>DescribeVpcPeeringAuthorizations</a> or request a new one using <a>CreateVpcPeeringAuthorization</a>. </p> <p>Once a valid authorization exists, call this operation from the AWS account that is used to manage the Amazon GameLift fleets. Identify the connection to delete by the connection ID and fleet ID. If successful, the connection is removed. </p> <ul> <li> <p> <a>CreateVpcPeeringAuthorization</a> </p> </li> <li> <p> <a>DescribeVpcPeeringAuthorizations</a> </p> </li> <li> <p> <a>DeleteVpcPeeringAuthorization</a> </p> </li> <li> <p> <a>CreateVpcPeeringConnection</a> </p> </li> <li> <p> <a>DescribeVpcPeeringConnections</a> </p> </li> <li> <p> <a>DeleteVpcPeeringConnection</a> </p> </li> </ul></p>
    async fn delete_vpc_peering_connection(
        &self,
        input: DeleteVpcPeeringConnectionInput,
    ) -> Result<DeleteVpcPeeringConnectionOutput, RusotoError<DeleteVpcPeeringConnectionError>>;

    /// <p><p> <b>This operation is used with the Amazon GameLift FleetIQ solution and game server groups.</b> </p> <p>Removes the game server from a game server group. As a result of this operation, the deregistered game server can no longer be claimed and will not be returned in a list of active game servers. </p> <p>To deregister a game server, specify the game server group and game server ID. If successful, this operation emits a CloudWatch event with termination timestamp and reason.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">GameLift FleetIQ Guide</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>RegisterGameServer</a> </p> </li> <li> <p> <a>ListGameServers</a> </p> </li> <li> <p> <a>ClaimGameServer</a> </p> </li> <li> <p> <a>DescribeGameServer</a> </p> </li> <li> <p> <a>UpdateGameServer</a> </p> </li> <li> <p> <a>DeregisterGameServer</a> </p> </li> </ul></p>
    async fn deregister_game_server(
        &self,
        input: DeregisterGameServerInput,
    ) -> Result<(), RusotoError<DeregisterGameServerError>>;

    /// <p><p>Retrieves properties for an alias. This operation returns all alias metadata and settings. To get an alias&#39;s target fleet ID only, use <code>ResolveAlias</code>. </p> <p>To get alias properties, specify the alias ID. If successful, the requested alias record is returned.</p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>DescribeAlias</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ResolveAlias</a> </p> </li> </ul></p>
    async fn describe_alias(
        &self,
        input: DescribeAliasInput,
    ) -> Result<DescribeAliasOutput, RusotoError<DescribeAliasError>>;

    /// <p><p>Retrieves properties for a custom game build. To request a build resource, specify a build ID. If successful, an object containing the build properties is returned.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-build-intro.html"> Upload a Custom Server Build</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateBuild</a> </p> </li> <li> <p> <a>ListBuilds</a> </p> </li> <li> <p> <a>DescribeBuild</a> </p> </li> <li> <p> <a>UpdateBuild</a> </p> </li> <li> <p> <a>DeleteBuild</a> </p> </li> </ul></p>
    async fn describe_build(
        &self,
        input: DescribeBuildInput,
    ) -> Result<DescribeBuildOutput, RusotoError<DescribeBuildError>>;

    /// <p><p>Retrieves the following information for the specified EC2 instance type:</p> <ul> <li> <p>Maximum number of instances allowed per AWS account (service limit).</p> </li> <li> <p>Current usage for the AWS account.</p> </li> </ul> <p>To learn more about the capabilities of each instance type, see <a href="http://aws.amazon.com/ec2/instance-types/">Amazon EC2 Instance Types</a>. Note that the instance types offered may vary depending on the region.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn describe_ec2_instance_limits(
        &self,
        input: DescribeEC2InstanceLimitsInput,
    ) -> Result<DescribeEC2InstanceLimitsOutput, RusotoError<DescribeEC2InstanceLimitsError>>;

    /// <p><p>Retrieves core properties, including configuration, status, and metadata, for a fleet. </p> <p>To get attributes for one or more fleets, provide a list of fleet IDs or fleet ARNs. To get attributes for all fleets, do not specify a fleet identifier. When requesting attributes for multiple fleets, use the pagination parameters to retrieve results as a set of sequential pages. If successful, a <a>FleetAttributes</a> object is returned for each fleet requested, unless the fleet identifier is not found.</p> <note> <p>Some API operations may limit the number of fleet IDs allowed in one request. If a request exceeds this limit, the request fails and the error message includes the maximum allowed number.</p> </note> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn describe_fleet_attributes(
        &self,
        input: DescribeFleetAttributesInput,
    ) -> Result<DescribeFleetAttributesOutput, RusotoError<DescribeFleetAttributesError>>;

    /// <p><p>Retrieves the current capacity statistics for one or more fleets. These statistics present a snapshot of the fleet&#39;s instances and provide insight on current or imminent scaling activity. To get statistics on game hosting activity in the fleet, see <a>DescribeFleetUtilization</a>.</p> <p>You can request capacity for all fleets or specify a list of one or more fleet identifiers. When requesting multiple fleets, use the pagination parameters to retrieve results as a set of sequential pages. If successful, a <a>FleetCapacity</a> object is returned for each requested fleet ID. When a list of fleet IDs is provided, attribute objects are returned only for fleets that currently exist.</p> <note> <p>Some API operations may limit the number of fleet IDs allowed in one request. If a request exceeds this limit, the request fails and the error message includes the maximum allowed.</p> </note> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/monitoring-cloudwatch.html#gamelift-metrics-fleet">GameLift Metrics for Fleets</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn describe_fleet_capacity(
        &self,
        input: DescribeFleetCapacityInput,
    ) -> Result<DescribeFleetCapacityOutput, RusotoError<DescribeFleetCapacityError>>;

    /// <p><p>Retrieves entries from the specified fleet&#39;s event log. You can specify a time range to limit the result set. Use the pagination parameters to retrieve results as a set of sequential pages. If successful, a collection of event log entries matching the request are returned.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn describe_fleet_events(
        &self,
        input: DescribeFleetEventsInput,
    ) -> Result<DescribeFleetEventsOutput, RusotoError<DescribeFleetEventsError>>;

    /// <p><p>Retrieves a fleet&#39;s inbound connection permissions. Connection permissions specify the range of IP addresses and port settings that incoming traffic can use to access server processes in the fleet. Game sessions that are running on instances in the fleet use connections that fall in this range. </p> <p>To get a fleet&#39;s inbound connection permissions, specify the fleet&#39;s unique identifier. If successful, a collection of <a>IpPermission</a> objects is returned for the requested fleet ID. If the requested fleet has been deleted, the result set is empty.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn describe_fleet_port_settings(
        &self,
        input: DescribeFleetPortSettingsInput,
    ) -> Result<DescribeFleetPortSettingsOutput, RusotoError<DescribeFleetPortSettingsError>>;

    /// <p><p>Retrieves utilization statistics for one or more fleets. These statistics provide insight into how available hosting resources are currently being used. To get statistics on available hosting resources, see <a>DescribeFleetCapacity</a>.</p> <p>You can request utilization data for all fleets, or specify a list of one or more fleet IDs. When requesting multiple fleets, use the pagination parameters to retrieve results as a set of sequential pages. If successful, a <a>FleetUtilization</a> object is returned for each requested fleet ID, unless the fleet identifier is not found. </p> <note> <p>Some API operations may limit the number of fleet IDs allowed in one request. If a request exceeds this limit, the request fails and the error message includes the maximum allowed.</p> </note> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/monitoring-cloudwatch.html#gamelift-metrics-fleet">GameLift Metrics for Fleets</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn describe_fleet_utilization(
        &self,
        input: DescribeFleetUtilizationInput,
    ) -> Result<DescribeFleetUtilizationOutput, RusotoError<DescribeFleetUtilizationError>>;

    /// <p><p> <b>This operation is used with the Amazon GameLift FleetIQ solution and game server groups.</b> </p> <p>Retrieves information for a registered game server. Information includes game server status, health check info, and the instance that the game server is running on. </p> <p>To retrieve game server information, specify the game server ID. If successful, the requested game server object is returned. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">GameLift FleetIQ Guide</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>RegisterGameServer</a> </p> </li> <li> <p> <a>ListGameServers</a> </p> </li> <li> <p> <a>ClaimGameServer</a> </p> </li> <li> <p> <a>DescribeGameServer</a> </p> </li> <li> <p> <a>UpdateGameServer</a> </p> </li> <li> <p> <a>DeregisterGameServer</a> </p> </li> </ul></p>
    async fn describe_game_server(
        &self,
        input: DescribeGameServerInput,
    ) -> Result<DescribeGameServerOutput, RusotoError<DescribeGameServerError>>;

    /// <p><p> <b>This operation is used with the Amazon GameLift FleetIQ solution and game server groups.</b> </p> <p>Retrieves information on a game server group. This operation returns only properties related to GameLift FleetIQ. To view or update properties for the corresponding Auto Scaling group, such as launch template, auto scaling policies, and maximum/minimum group size, access the Auto Scaling group directly.</p> <p>To get attributes for a game server group, provide a group name or ARN value. If successful, a <a>GameServerGroup</a> object is returned.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">GameLift FleetIQ Guide</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateGameServerGroup</a> </p> </li> <li> <p> <a>ListGameServerGroups</a> </p> </li> <li> <p> <a>DescribeGameServerGroup</a> </p> </li> <li> <p> <a>UpdateGameServerGroup</a> </p> </li> <li> <p> <a>DeleteGameServerGroup</a> </p> </li> <li> <p> <a>ResumeGameServerGroup</a> </p> </li> <li> <p> <a>SuspendGameServerGroup</a> </p> </li> <li> <p> <a>DescribeGameServerInstances</a> </p> </li> </ul></p>
    async fn describe_game_server_group(
        &self,
        input: DescribeGameServerGroupInput,
    ) -> Result<DescribeGameServerGroupOutput, RusotoError<DescribeGameServerGroupError>>;

    /// <p><p> <b>This operation is used with the Amazon GameLift FleetIQ solution and game server groups.</b> </p> <p>Retrieves status information about the Amazon EC2 instances associated with a GameLift FleetIQ game server group. Use this operation to detect when instances are active or not available to host new game servers. If you are looking for instance configuration information, call <a>DescribeGameServerGroup</a> or access the corresponding Auto Scaling group properties.</p> <p>To request status for all instances in the game server group, provide a game server group ID only. To request status for specific instances, provide the game server group ID and one or more instance IDs. Use the pagination parameters to retrieve results in sequential segments. If successful, a collection of <code>GameServerInstance</code> objects is returned. </p> <p>This operation is not designed to be called with every game server claim request; this practice can cause you to exceed your API limit, which results in errors. Instead, as a best practice, cache the results and refresh your cache no more than once every 10 seconds.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">GameLift FleetIQ Guide</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateGameServerGroup</a> </p> </li> <li> <p> <a>ListGameServerGroups</a> </p> </li> <li> <p> <a>DescribeGameServerGroup</a> </p> </li> <li> <p> <a>UpdateGameServerGroup</a> </p> </li> <li> <p> <a>DeleteGameServerGroup</a> </p> </li> <li> <p> <a>ResumeGameServerGroup</a> </p> </li> <li> <p> <a>SuspendGameServerGroup</a> </p> </li> <li> <p> <a>DescribeGameServerInstances</a> </p> </li> </ul></p>
    async fn describe_game_server_instances(
        &self,
        input: DescribeGameServerInstancesInput,
    ) -> Result<DescribeGameServerInstancesOutput, RusotoError<DescribeGameServerInstancesError>>;

    /// <p><p>Retrieves properties, including the protection policy in force, for one or more game sessions. This operation can be used in several ways: (1) provide a <code>GameSessionId</code> or <code>GameSessionArn</code> to request details for a specific game session; (2) provide either a <code>FleetId</code> or an <code>AliasId</code> to request properties for all game sessions running on a fleet. </p> <p>To get game session record(s), specify just one of the following: game session ID, fleet ID, or alias ID. You can filter this request by game session status. Use the pagination parameters to retrieve results as a set of sequential pages. If successful, a <a>GameSessionDetail</a> object is returned for each session matching the request.</p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul></p>
    async fn describe_game_session_details(
        &self,
        input: DescribeGameSessionDetailsInput,
    ) -> Result<DescribeGameSessionDetailsOutput, RusotoError<DescribeGameSessionDetailsError>>;

    /// <p><p>Retrieves properties and current status of a game session placement request. To get game session placement details, specify the placement ID. If successful, a <a>GameSessionPlacement</a> object is returned.</p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul></p>
    async fn describe_game_session_placement(
        &self,
        input: DescribeGameSessionPlacementInput,
    ) -> Result<DescribeGameSessionPlacementOutput, RusotoError<DescribeGameSessionPlacementError>>;

    /// <p><p>Retrieves the properties for one or more game session queues. When requesting multiple queues, use the pagination parameters to retrieve results as a set of sequential pages. If successful, a <a>GameSessionQueue</a> object is returned for each requested queue. When specifying a list of queues, objects are returned only for queues that currently exist in the Region.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/queues-console.html"> View Your Queues</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateGameSessionQueue</a> </p> </li> <li> <p> <a>DescribeGameSessionQueues</a> </p> </li> <li> <p> <a>UpdateGameSessionQueue</a> </p> </li> <li> <p> <a>DeleteGameSessionQueue</a> </p> </li> </ul></p>
    async fn describe_game_session_queues(
        &self,
        input: DescribeGameSessionQueuesInput,
    ) -> Result<DescribeGameSessionQueuesOutput, RusotoError<DescribeGameSessionQueuesError>>;

    /// <p><p>Retrieves a set of one or more game sessions. Request a specific game session or request all game sessions on a fleet. Alternatively, use <a>SearchGameSessions</a> to request a set of active game sessions that are filtered by certain criteria. To retrieve protection policy settings for game sessions, use <a>DescribeGameSessionDetails</a>.</p> <p>To get game sessions, specify one of the following: game session ID, fleet ID, or alias ID. You can filter this request by game session status. Use the pagination parameters to retrieve results as a set of sequential pages. If successful, a <a>GameSession</a> object is returned for each game session matching the request.</p> <p> <i>Available in Amazon GameLift Local.</i> </p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul></p>
    async fn describe_game_sessions(
        &self,
        input: DescribeGameSessionsInput,
    ) -> Result<DescribeGameSessionsOutput, RusotoError<DescribeGameSessionsError>>;

    /// <p><p>Retrieves information about a fleet&#39;s instances, including instance IDs. Use this operation to get details on all instances in the fleet or get details on one specific instance.</p> <p>To get a specific instance, specify fleet ID and instance ID. To get all instances in a fleet, specify a fleet ID only. Use the pagination parameters to retrieve results as a set of sequential pages. If successful, an <a>Instance</a> object is returned for each result.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-remote-access.html">Remotely Access Fleet Instances</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-creating-debug.html">Debug Fleet Issues</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>DescribeInstances</a> </p> </li> <li> <p> <a>GetInstanceAccess</a> </p> </li> </ul></p>
    async fn describe_instances(
        &self,
        input: DescribeInstancesInput,
    ) -> Result<DescribeInstancesOutput, RusotoError<DescribeInstancesError>>;

    /// <p><p>Retrieves one or more matchmaking tickets. Use this operation to retrieve ticket information, including--after a successful match is made--connection information for the resulting new game session. </p> <p>To request matchmaking tickets, provide a list of up to 10 ticket IDs. If the request is successful, a ticket object is returned for each requested ID that currently exists.</p> <p>This operation is not designed to be continually called to track matchmaking ticket status. This practice can cause you to exceed your API limit, which results in errors. Instead, as a best practice, set up an Amazon Simple Notification Service (SNS) to receive notifications, and provide the topic ARN in the matchmaking configuration. Continuously poling ticket status with <a>DescribeMatchmaking</a> should only be used for games in development with low matchmaking usage.</p> <p/> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-client.html"> Add FlexMatch to a Game Client</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-notification.html"> Set Up FlexMatch Event Notification</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>StartMatchmaking</a> </p> </li> <li> <p> <a>DescribeMatchmaking</a> </p> </li> <li> <p> <a>StopMatchmaking</a> </p> </li> <li> <p> <a>AcceptMatch</a> </p> </li> <li> <p> <a>StartMatchBackfill</a> </p> </li> </ul></p>
    async fn describe_matchmaking(
        &self,
        input: DescribeMatchmakingInput,
    ) -> Result<DescribeMatchmakingOutput, RusotoError<DescribeMatchmakingError>>;

    /// <p><p>Retrieves the details of FlexMatch matchmaking configurations. </p> <p>This operation offers the following options: (1) retrieve all matchmaking configurations, (2) retrieve configurations for a specified list, or (3) retrieve all configurations that use a specified rule set name. When requesting multiple items, use the pagination parameters to retrieve results as a set of sequential pages. </p> <p>If successful, a configuration is returned for each requested name. When specifying a list of names, only configurations that currently exist are returned. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/matchmaker-build.html"> Setting Up FlexMatch Matchmakers</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DeleteMatchmakingRuleSet</a> </p> </li> </ul></p>
    async fn describe_matchmaking_configurations(
        &self,
        input: DescribeMatchmakingConfigurationsInput,
    ) -> Result<
        DescribeMatchmakingConfigurationsOutput,
        RusotoError<DescribeMatchmakingConfigurationsError>,
    >;

    /// <p><p>Retrieves the details for FlexMatch matchmaking rule sets. You can request all existing rule sets for the Region, or provide a list of one or more rule set names. When requesting multiple items, use the pagination parameters to retrieve results as a set of sequential pages. If successful, a rule set is returned for each requested name. </p> <p> <b>Learn more</b> </p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-rulesets.html">Build a Rule Set</a> </p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DeleteMatchmakingRuleSet</a> </p> </li> </ul></p>
    async fn describe_matchmaking_rule_sets(
        &self,
        input: DescribeMatchmakingRuleSetsInput,
    ) -> Result<DescribeMatchmakingRuleSetsOutput, RusotoError<DescribeMatchmakingRuleSetsError>>;

    /// <p><p>Retrieves properties for one or more player sessions. This operation can be used in several ways: (1) provide a <code>PlayerSessionId</code> to request properties for a specific player session; (2) provide a <code>GameSessionId</code> to request properties for all player sessions in the specified game session; (3) provide a <code>PlayerId</code> to request properties for all player sessions of a specified player. </p> <p>To get game session record(s), specify only one of the following: a player session ID, a game session ID, or a player ID. You can filter this request by player session status. Use the pagination parameters to retrieve results as a set of sequential pages. If successful, a <a>PlayerSession</a> object is returned for each session matching the request.</p> <p> <i>Available in Amazon GameLift Local.</i> </p> <ul> <li> <p> <a>CreatePlayerSession</a> </p> </li> <li> <p> <a>CreatePlayerSessions</a> </p> </li> <li> <p> <a>DescribePlayerSessions</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul></p>
    async fn describe_player_sessions(
        &self,
        input: DescribePlayerSessionsInput,
    ) -> Result<DescribePlayerSessionsOutput, RusotoError<DescribePlayerSessionsError>>;

    /// <p><p>Retrieves a fleet&#39;s runtime configuration settings. The runtime configuration tells Amazon GameLift which server processes to run (and how) on each instance in the fleet.</p> <p>To get a runtime configuration, specify the fleet&#39;s unique identifier. If successful, a <a>RuntimeConfiguration</a> object is returned for the requested fleet. If the requested fleet has been deleted, the result set is empty.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-multiprocess.html">Running Multiple Processes on a Fleet</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn describe_runtime_configuration(
        &self,
        input: DescribeRuntimeConfigurationInput,
    ) -> Result<DescribeRuntimeConfigurationOutput, RusotoError<DescribeRuntimeConfigurationError>>;

    /// <p><p>Retrieves all scaling policies applied to a fleet.</p> <p>To get a fleet&#39;s scaling policies, specify the fleet ID. You can filter this request by policy status, such as to retrieve only active scaling policies. Use the pagination parameters to retrieve results as a set of sequential pages. If successful, set of <a>ScalingPolicy</a> objects is returned for the fleet.</p> <p>A fleet may have all of its scaling policies suspended (<a>StopFleetActions</a>). This operation does not affect the status of the scaling policies, which remains ACTIVE. To see whether a fleet&#39;s scaling policies are in force or suspended, call <a>DescribeFleetAttributes</a> and check the stopped actions.</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> <li> <p>Manage scaling policies:</p> <ul> <li> <p> <a>PutScalingPolicy</a> (auto-scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (auto-scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (auto-scaling)</p> </li> </ul> </li> <li> <p>Manage fleet actions:</p> <ul> <li> <p> <a>StartFleetActions</a> </p> </li> <li> <p> <a>StopFleetActions</a> </p> </li> </ul> </li> </ul></p>
    async fn describe_scaling_policies(
        &self,
        input: DescribeScalingPoliciesInput,
    ) -> Result<DescribeScalingPoliciesOutput, RusotoError<DescribeScalingPoliciesError>>;

    /// <p><p>Retrieves properties for a Realtime script. </p> <p>To request a script record, specify the script ID. If successful, an object containing the script properties is returned.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/realtime-intro.html">Amazon GameLift Realtime Servers</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateScript</a> </p> </li> <li> <p> <a>ListScripts</a> </p> </li> <li> <p> <a>DescribeScript</a> </p> </li> <li> <p> <a>UpdateScript</a> </p> </li> <li> <p> <a>DeleteScript</a> </p> </li> </ul></p>
    async fn describe_script(
        &self,
        input: DescribeScriptInput,
    ) -> Result<DescribeScriptOutput, RusotoError<DescribeScriptError>>;

    /// <p><p>Retrieves valid VPC peering authorizations that are pending for the AWS account. This operation returns all VPC peering authorizations and requests for peering. This includes those initiated and received by this account. </p> <ul> <li> <p> <a>CreateVpcPeeringAuthorization</a> </p> </li> <li> <p> <a>DescribeVpcPeeringAuthorizations</a> </p> </li> <li> <p> <a>DeleteVpcPeeringAuthorization</a> </p> </li> <li> <p> <a>CreateVpcPeeringConnection</a> </p> </li> <li> <p> <a>DescribeVpcPeeringConnections</a> </p> </li> <li> <p> <a>DeleteVpcPeeringConnection</a> </p> </li> </ul></p>
    async fn describe_vpc_peering_authorizations(
        &self,
    ) -> Result<
        DescribeVpcPeeringAuthorizationsOutput,
        RusotoError<DescribeVpcPeeringAuthorizationsError>,
    >;

    /// <p><p>Retrieves information on VPC peering connections. Use this operation to get peering information for all fleets or for one specific fleet ID. </p> <p>To retrieve connection information, call this operation from the AWS account that is used to manage the Amazon GameLift fleets. Specify a fleet ID or leave the parameter empty to retrieve all connection records. If successful, the retrieved information includes both active and pending connections. Active connections identify the IpV4 CIDR block that the VPC uses to connect. </p> <ul> <li> <p> <a>CreateVpcPeeringAuthorization</a> </p> </li> <li> <p> <a>DescribeVpcPeeringAuthorizations</a> </p> </li> <li> <p> <a>DeleteVpcPeeringAuthorization</a> </p> </li> <li> <p> <a>CreateVpcPeeringConnection</a> </p> </li> <li> <p> <a>DescribeVpcPeeringConnections</a> </p> </li> <li> <p> <a>DeleteVpcPeeringConnection</a> </p> </li> </ul></p>
    async fn describe_vpc_peering_connections(
        &self,
        input: DescribeVpcPeeringConnectionsInput,
    ) -> Result<DescribeVpcPeeringConnectionsOutput, RusotoError<DescribeVpcPeeringConnectionsError>>;

    /// <p><p>Retrieves the location of stored game session logs for a specified game session. When a game session is terminated, Amazon GameLift automatically stores the logs in Amazon S3 and retains them for 14 days. Use this URL to download the logs.</p> <note> <p>See the <a href="https://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_gamelift">AWS Service Limits</a> page for maximum log file sizes. Log files that exceed this limit are not saved.</p> </note> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul></p>
    async fn get_game_session_log_url(
        &self,
        input: GetGameSessionLogUrlInput,
    ) -> Result<GetGameSessionLogUrlOutput, RusotoError<GetGameSessionLogUrlError>>;

    /// <p><p>Requests remote access to a fleet instance. Remote access is useful for debugging, gathering benchmarking data, or observing activity in real time. </p> <p>To remotely access an instance, you need credentials that match the operating system of the instance. For a Windows instance, Amazon GameLift returns a user name and password as strings for use with a Windows Remote Desktop client. For a Linux instance, Amazon GameLift returns a user name and RSA private key, also as strings, for use with an SSH client. The private key must be saved in the proper format to a <code>.pem</code> file before using. If you&#39;re making this request using the AWS CLI, saving the secret can be handled as part of the GetInstanceAccess request, as shown in one of the examples for this operation. </p> <p>To request access to a specific instance, specify the IDs of both the instance and the fleet it belongs to. You can retrieve a fleet&#39;s instance IDs by calling <a>DescribeInstances</a>. If successful, an <a>InstanceAccess</a> object is returned that contains the instance&#39;s IP address and a set of credentials.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-remote-access.html">Remotely Access Fleet Instances</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-creating-debug.html">Debug Fleet Issues</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>DescribeInstances</a> </p> </li> <li> <p> <a>GetInstanceAccess</a> </p> </li> </ul></p>
    async fn get_instance_access(
        &self,
        input: GetInstanceAccessInput,
    ) -> Result<GetInstanceAccessOutput, RusotoError<GetInstanceAccessError>>;

    /// <p><p>Retrieves all aliases for this AWS account. You can filter the result set by alias name and/or routing strategy type. Use the pagination parameters to retrieve results in sequential pages.</p> <note> <p>Returned aliases are not listed in any particular order.</p> </note> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>DescribeAlias</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ResolveAlias</a> </p> </li> </ul></p>
    async fn list_aliases(
        &self,
        input: ListAliasesInput,
    ) -> Result<ListAliasesOutput, RusotoError<ListAliasesError>>;

    /// <p><p>Retrieves build resources for all builds associated with the AWS account in use. You can limit results to builds that are in a specific status by using the <code>Status</code> parameter. Use the pagination parameters to retrieve results in a set of sequential pages. </p> <note> <p>Build resources are not listed in any particular order.</p> </note> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-build-intro.html"> Upload a Custom Server Build</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateBuild</a> </p> </li> <li> <p> <a>ListBuilds</a> </p> </li> <li> <p> <a>DescribeBuild</a> </p> </li> <li> <p> <a>UpdateBuild</a> </p> </li> <li> <p> <a>DeleteBuild</a> </p> </li> </ul></p>
    async fn list_builds(
        &self,
        input: ListBuildsInput,
    ) -> Result<ListBuildsOutput, RusotoError<ListBuildsError>>;

    /// <p><p>Retrieves a collection of fleet resources for this AWS account. You can filter the result set to find only those fleets that are deployed with a specific build or script. Use the pagination parameters to retrieve results in sequential pages.</p> <note> <p>Fleet resources are not listed in a particular order.</p> </note> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn list_fleets(
        &self,
        input: ListFleetsInput,
    ) -> Result<ListFleetsOutput, RusotoError<ListFleetsError>>;

    /// <p><p> <b>This operation is used with the Amazon GameLift FleetIQ solution and game server groups.</b> </p> <p>Retrieves information on all game servers groups that exist in the current AWS account for the selected Region. Use the pagination parameters to retrieve results in a set of sequential segments. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">GameLift FleetIQ Guide</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateGameServerGroup</a> </p> </li> <li> <p> <a>ListGameServerGroups</a> </p> </li> <li> <p> <a>DescribeGameServerGroup</a> </p> </li> <li> <p> <a>UpdateGameServerGroup</a> </p> </li> <li> <p> <a>DeleteGameServerGroup</a> </p> </li> <li> <p> <a>ResumeGameServerGroup</a> </p> </li> <li> <p> <a>SuspendGameServerGroup</a> </p> </li> <li> <p> <a>DescribeGameServerInstances</a> </p> </li> </ul></p>
    async fn list_game_server_groups(
        &self,
        input: ListGameServerGroupsInput,
    ) -> Result<ListGameServerGroupsOutput, RusotoError<ListGameServerGroupsError>>;

    /// <p><p> <b>This operation is used with the Amazon GameLift FleetIQ solution and game server groups.</b> </p> <p>Retrieves information on all game servers that are currently active in a specified game server group. You can opt to sort the list by game server age. Use the pagination parameters to retrieve results in a set of sequential segments. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">GameLift FleetIQ Guide</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>RegisterGameServer</a> </p> </li> <li> <p> <a>ListGameServers</a> </p> </li> <li> <p> <a>ClaimGameServer</a> </p> </li> <li> <p> <a>DescribeGameServer</a> </p> </li> <li> <p> <a>UpdateGameServer</a> </p> </li> <li> <p> <a>DeregisterGameServer</a> </p> </li> </ul></p>
    async fn list_game_servers(
        &self,
        input: ListGameServersInput,
    ) -> Result<ListGameServersOutput, RusotoError<ListGameServersError>>;

    /// <p><p>Retrieves script records for all Realtime scripts that are associated with the AWS account in use. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/realtime-intro.html">Amazon GameLift Realtime Servers</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateScript</a> </p> </li> <li> <p> <a>ListScripts</a> </p> </li> <li> <p> <a>DescribeScript</a> </p> </li> <li> <p> <a>UpdateScript</a> </p> </li> <li> <p> <a>DeleteScript</a> </p> </li> </ul></p>
    async fn list_scripts(
        &self,
        input: ListScriptsInput,
    ) -> Result<ListScriptsOutput, RusotoError<ListScriptsError>>;

    /// <p><p> Retrieves all tags that are assigned to a GameLift resource. Resource tags are used to organize AWS resources for a range of purposes. This operation handles the permissions necessary to manage tags for the following GameLift resource types:</p> <ul> <li> <p>Build</p> </li> <li> <p>Script</p> </li> <li> <p>Fleet</p> </li> <li> <p>Alias</p> </li> <li> <p>GameSessionQueue</p> </li> <li> <p>MatchmakingConfiguration</p> </li> <li> <p>MatchmakingRuleSet</p> </li> </ul> <p>To list tags for a resource, specify the unique ARN value for the resource.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS Resources</a> in the <i>AWS General Reference</i> </p> <p> <a href="http://aws.amazon.com/answers/account-management/aws-tagging-strategies/"> AWS Tagging Strategies</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>TagResource</a> </p> </li> <li> <p> <a>UntagResource</a> </p> </li> <li> <p> <a>ListTagsForResource</a> </p> </li> </ul></p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p><p>Creates or updates a scaling policy for a fleet. Scaling policies are used to automatically scale a fleet&#39;s hosting capacity to meet player demand. An active scaling policy instructs Amazon GameLift to track a fleet metric and automatically change the fleet&#39;s capacity when a certain threshold is reached. There are two types of scaling policies: target-based and rule-based. Use a target-based policy to quickly and efficiently manage fleet scaling; this option is the most commonly used. Use rule-based policies when you need to exert fine-grained control over auto-scaling. </p> <p>Fleets can have multiple scaling policies of each type in force at the same time; you can have one target-based policy, one or multiple rule-based scaling policies, or both. We recommend caution, however, because multiple auto-scaling policies can have unintended consequences.</p> <p>You can temporarily suspend all scaling policies for a fleet by calling <a>StopFleetActions</a> with the fleet action AUTO_SCALING. To resume scaling policies, call <a>StartFleetActions</a> with the same fleet action. To stop just one scaling policy--or to permanently remove it, you must delete the policy with <a>DeleteScalingPolicy</a>.</p> <p>Learn more about how to work with auto-scaling in <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-autoscaling.html">Set Up Fleet Automatic Scaling</a>.</p> <p> <b>Target-based policy</b> </p> <p>A target-based policy tracks a single metric: PercentAvailableGameSessions. This metric tells us how much of a fleet&#39;s hosting capacity is ready to host game sessions but is not currently in use. This is the fleet&#39;s buffer; it measures the additional player demand that the fleet could handle at current capacity. With a target-based policy, you set your ideal buffer size and leave it to Amazon GameLift to take whatever action is needed to maintain that target. </p> <p>For example, you might choose to maintain a 10% buffer for a fleet that has the capacity to host 100 simultaneous game sessions. This policy tells Amazon GameLift to take action whenever the fleet&#39;s available capacity falls below or rises above 10 game sessions. Amazon GameLift will start new instances or stop unused instances in order to return to the 10% buffer. </p> <p>To create or update a target-based policy, specify a fleet ID and name, and set the policy type to &quot;TargetBased&quot;. Specify the metric to track (PercentAvailableGameSessions) and reference a <a>TargetConfiguration</a> object with your desired buffer value. Exclude all other parameters. On a successful request, the policy name is returned. The scaling policy is automatically in force as soon as it&#39;s successfully created. If the fleet&#39;s auto-scaling actions are temporarily suspended, the new policy will be in force once the fleet actions are restarted.</p> <p> <b>Rule-based policy</b> </p> <p>A rule-based policy tracks specified fleet metric, sets a threshold value, and specifies the type of action to initiate when triggered. With a rule-based policy, you can select from several available fleet metrics. Each policy specifies whether to scale up or scale down (and by how much), so you need one policy for each type of action. </p> <p>For example, a policy may make the following statement: &quot;If the percentage of idle instances is greater than 20% for more than 15 minutes, then reduce the fleet capacity by 10%.&quot;</p> <p>A policy&#39;s rule statement has the following structure:</p> <p>If <code>[MetricName]</code> is <code>[ComparisonOperator]</code> <code>[Threshold]</code> for <code>[EvaluationPeriods]</code> minutes, then <code>[ScalingAdjustmentType]</code> to/by <code>[ScalingAdjustment]</code>.</p> <p>To implement the example, the rule statement would look like this:</p> <p>If <code>[PercentIdleInstances]</code> is <code>[GreaterThanThreshold]</code> <code>[20]</code> for <code>[15]</code> minutes, then <code>[PercentChangeInCapacity]</code> to/by <code>[10]</code>.</p> <p>To create or update a scaling policy, specify a unique combination of name and fleet ID, and set the policy type to &quot;RuleBased&quot;. Specify the parameter values for a policy rule statement. On a successful request, the policy name is returned. Scaling policies are automatically in force as soon as they&#39;re successfully created. If the fleet&#39;s auto-scaling actions are temporarily suspended, the new policy will be in force once the fleet actions are restarted.</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> <li> <p>Manage scaling policies:</p> <ul> <li> <p> <a>PutScalingPolicy</a> (auto-scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (auto-scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (auto-scaling)</p> </li> </ul> </li> <li> <p>Manage fleet actions:</p> <ul> <li> <p> <a>StartFleetActions</a> </p> </li> <li> <p> <a>StopFleetActions</a> </p> </li> </ul> </li> </ul></p>
    async fn put_scaling_policy(
        &self,
        input: PutScalingPolicyInput,
    ) -> Result<PutScalingPolicyOutput, RusotoError<PutScalingPolicyError>>;

    /// <p><p> <b>This operation is used with the Amazon GameLift FleetIQ solution and game server groups.</b> </p> <p>Creates a new game server resource and notifies GameLift FleetIQ that the game server is ready to host gameplay and players. This operation is called by a game server process that is running on an instance in a game server group. Registering game servers enables GameLift FleetIQ to track available game servers and enables game clients and services to claim a game server for a new game session. </p> <p>To register a game server, identify the game server group and instance where the game server is running, and provide a unique identifier for the game server. You can also include connection and game server data. When a game client or service requests a game server by calling <a>ClaimGameServer</a>, this information is returned in the response.</p> <p>Once a game server is successfully registered, it is put in status <code>AVAILABLE</code>. A request to register a game server may fail if the instance it is running on is in the process of shutting down as part of instance balancing or scale-down activity. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">GameLift FleetIQ Guide</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>RegisterGameServer</a> </p> </li> <li> <p> <a>ListGameServers</a> </p> </li> <li> <p> <a>ClaimGameServer</a> </p> </li> <li> <p> <a>DescribeGameServer</a> </p> </li> <li> <p> <a>UpdateGameServer</a> </p> </li> <li> <p> <a>DeregisterGameServer</a> </p> </li> </ul></p>
    async fn register_game_server(
        &self,
        input: RegisterGameServerInput,
    ) -> Result<RegisterGameServerOutput, RusotoError<RegisterGameServerError>>;

    /// <p><p>Retrieves a fresh set of credentials for use when uploading a new set of game build files to Amazon GameLift&#39;s Amazon S3. This is done as part of the build creation process; see <a>CreateBuild</a>.</p> <p>To request new credentials, specify the build ID as returned with an initial <code>CreateBuild</code> request. If successful, a new set of credentials are returned, along with the S3 storage location associated with the build ID.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-build-cli-uploading.html#gamelift-build-cli-uploading-create-build"> Create a Build with Files in S3</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateBuild</a> </p> </li> <li> <p> <a>ListBuilds</a> </p> </li> <li> <p> <a>DescribeBuild</a> </p> </li> <li> <p> <a>UpdateBuild</a> </p> </li> <li> <p> <a>DeleteBuild</a> </p> </li> </ul></p>
    async fn request_upload_credentials(
        &self,
        input: RequestUploadCredentialsInput,
    ) -> Result<RequestUploadCredentialsOutput, RusotoError<RequestUploadCredentialsError>>;

    /// <p><p>Retrieves the fleet ID that an alias is currently pointing to.</p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>DescribeAlias</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ResolveAlias</a> </p> </li> </ul></p>
    async fn resolve_alias(
        &self,
        input: ResolveAliasInput,
    ) -> Result<ResolveAliasOutput, RusotoError<ResolveAliasError>>;

    /// <p><p> <b>This operation is used with the Amazon GameLift FleetIQ solution and game server groups.</b> </p> <p>Reinstates activity on a game server group after it has been suspended. A game server group might be suspended by the<a>SuspendGameServerGroup</a> operation, or it might be suspended involuntarily due to a configuration problem. In the second case, you can manually resume activity on the group once the configuration problem has been resolved. Refer to the game server group status and status reason for more information on why group activity is suspended.</p> <p>To resume activity, specify a game server group ARN and the type of activity to be resumed. If successful, a <a>GameServerGroup</a> object is returned showing that the resumed activity is no longer listed in <code>SuspendedActions</code>. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">GameLift FleetIQ Guide</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateGameServerGroup</a> </p> </li> <li> <p> <a>ListGameServerGroups</a> </p> </li> <li> <p> <a>DescribeGameServerGroup</a> </p> </li> <li> <p> <a>UpdateGameServerGroup</a> </p> </li> <li> <p> <a>DeleteGameServerGroup</a> </p> </li> <li> <p> <a>ResumeGameServerGroup</a> </p> </li> <li> <p> <a>SuspendGameServerGroup</a> </p> </li> <li> <p> <a>DescribeGameServerInstances</a> </p> </li> </ul></p>
    async fn resume_game_server_group(
        &self,
        input: ResumeGameServerGroupInput,
    ) -> Result<ResumeGameServerGroupOutput, RusotoError<ResumeGameServerGroupError>>;

    /// <p><p>Retrieves all active game sessions that match a set of search criteria and sorts them in a specified order. You can search or sort by the following game session attributes:</p> <ul> <li> <p> <b>gameSessionId</b> -- A unique identifier for the game session. You can use either a <code>GameSessionId</code> or <code>GameSessionArn</code> value. </p> </li> <li> <p> <b>gameSessionName</b> -- Name assigned to a game session. This value is set when requesting a new game session with <a>CreateGameSession</a> or updating with <a>UpdateGameSession</a>. Game session names do not need to be unique to a game session.</p> </li> <li> <p> <b>gameSessionProperties</b> -- Custom data defined in a game session&#39;s <code>GameProperty</code> parameter. <code>GameProperty</code> values are stored as key:value pairs; the filter expression must indicate the key and a string to search the data values for. For example, to search for game sessions with custom data containing the key:value pair &quot;gameMode:brawl&quot;, specify the following: <code>gameSessionProperties.gameMode = &quot;brawl&quot;</code>. All custom data values are searched as strings.</p> </li> <li> <p> <b>maximumSessions</b> -- Maximum number of player sessions allowed for a game session. This value is set when requesting a new game session with <a>CreateGameSession</a> or updating with <a>UpdateGameSession</a>.</p> </li> <li> <p> <b>creationTimeMillis</b> -- Value indicating when a game session was created. It is expressed in Unix time as milliseconds.</p> </li> <li> <p> <b>playerSessionCount</b> -- Number of players currently connected to a game session. This value changes rapidly as players join the session or drop out.</p> </li> <li> <p> <b>hasAvailablePlayerSessions</b> -- Boolean value indicating whether a game session has reached its maximum number of players. It is highly recommended that all search requests include this filter attribute to optimize search performance and return only sessions that players can join. </p> </li> </ul> <note> <p>Returned values for <code>playerSessionCount</code> and <code>hasAvailablePlayerSessions</code> change quickly as players join sessions and others drop out. Results should be considered a snapshot in time. Be sure to refresh search results often, and handle sessions that fill up before a player can join. </p> </note> <p>To search or sort, specify either a fleet ID or an alias ID, and provide a search filter expression, a sort expression, or both. If successful, a collection of <a>GameSession</a> objects matching the request is returned. Use the pagination parameters to retrieve results as a set of sequential pages. </p> <p>You can search for game sessions one fleet at a time only. To find game sessions across multiple fleets, you must search each fleet separately and combine the results. This search feature finds only game sessions that are in <code>ACTIVE</code> status. To locate games in statuses other than active, use <a>DescribeGameSessionDetails</a>.</p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul></p>
    async fn search_game_sessions(
        &self,
        input: SearchGameSessionsInput,
    ) -> Result<SearchGameSessionsOutput, RusotoError<SearchGameSessionsError>>;

    /// <p><p>Resumes activity on a fleet that was suspended with <a>StopFleetActions</a>. Currently, this operation is used to restart a fleet&#39;s auto-scaling activity. </p> <p>To start fleet actions, specify the fleet ID and the type of actions to restart. When auto-scaling fleet actions are restarted, Amazon GameLift once again initiates scaling events as triggered by the fleet&#39;s scaling policies. If actions on the fleet were never stopped, this operation will have no effect. You can view a fleet&#39;s stopped actions using <a>DescribeFleetAttributes</a>.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn start_fleet_actions(
        &self,
        input: StartFleetActionsInput,
    ) -> Result<StartFleetActionsOutput, RusotoError<StartFleetActionsError>>;

    /// <p><p>Places a request for a new game session in a queue (see <a>CreateGameSessionQueue</a>). When processing a placement request, Amazon GameLift searches for available resources on the queue&#39;s destinations, scanning each until it finds resources or the placement request times out.</p> <p>A game session placement request can also request player sessions. When a new game session is successfully created, Amazon GameLift creates a player session for each player included in the request.</p> <p>When placing a game session, by default Amazon GameLift tries each fleet in the order they are listed in the queue configuration. Ideally, a queue&#39;s destinations are listed in preference order.</p> <p>Alternatively, when requesting a game session with players, you can also provide latency data for each player in relevant Regions. Latency data indicates the performance lag a player experiences when connected to a fleet in the Region. Amazon GameLift uses latency data to reorder the list of destinations to place the game session in a Region with minimal lag. If latency data is provided for multiple players, Amazon GameLift calculates each Region&#39;s average lag for all players and reorders to get the best game play across all players. </p> <p>To place a new game session request, specify the following:</p> <ul> <li> <p>The queue name and a set of game session properties and settings</p> </li> <li> <p>A unique ID (such as a UUID) for the placement. You use this ID to track the status of the placement request</p> </li> <li> <p>(Optional) A set of player data and a unique player ID for each player that you are joining to the new game session (player data is optional, but if you include it, you must also provide a unique ID for each player)</p> </li> <li> <p>Latency data for all players (if you want to optimize game play for the players)</p> </li> </ul> <p>If successful, a new game session placement is created.</p> <p>To track the status of a placement request, call <a>DescribeGameSessionPlacement</a> and check the request&#39;s status. If the status is <code>FULFILLED</code>, a new game session has been created and a game session ARN and Region are referenced. If the placement request times out, you can resubmit the request or retry it with a different queue. </p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul></p>
    async fn start_game_session_placement(
        &self,
        input: StartGameSessionPlacementInput,
    ) -> Result<StartGameSessionPlacementOutput, RusotoError<StartGameSessionPlacementError>>;

    /// <p><p>Finds new players to fill open slots in an existing game session. This operation can be used to add players to matched games that start with fewer than the maximum number of players or to replace players when they drop out. By backfilling with the same matchmaker used to create the original match, you ensure that new players meet the match criteria and maintain a consistent experience throughout the game session. You can backfill a match anytime after a game session has been created. </p> <p>To request a match backfill, specify a unique ticket ID, the existing game session&#39;s ARN, a matchmaking configuration, and a set of data that describes all current players in the game session. If successful, a match backfill ticket is created and returned with status set to QUEUED. The ticket is placed in the matchmaker&#39;s ticket pool and processed. Track the status of the ticket to respond as needed. </p> <p>The process of finding backfill matches is essentially identical to the initial matchmaking process. The matchmaker searches the pool and groups tickets together to form potential matches, allowing only one backfill ticket per potential match. Once the a match is formed, the matchmaker creates player sessions for the new players. All tickets in the match are updated with the game session&#39;s connection information, and the <a>GameSession</a> object is updated to include matchmaker data on the new players. For more detail on how match backfill requests are processed, see <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/gamelift-match.html"> How Amazon GameLift FlexMatch Works</a>. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-backfill.html"> Backfill Existing Games with FlexMatch</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/gamelift-match.html"> How GameLift FlexMatch Works</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>StartMatchmaking</a> </p> </li> <li> <p> <a>DescribeMatchmaking</a> </p> </li> <li> <p> <a>StopMatchmaking</a> </p> </li> <li> <p> <a>AcceptMatch</a> </p> </li> <li> <p> <a>StartMatchBackfill</a> </p> </li> </ul></p>
    async fn start_match_backfill(
        &self,
        input: StartMatchBackfillInput,
    ) -> Result<StartMatchBackfillOutput, RusotoError<StartMatchBackfillError>>;

    /// <p><p>Uses FlexMatch to create a game match for a group of players based on custom matchmaking rules. If you&#39;re also using GameLift hosting, a new game session is started for the matched players. Each matchmaking request identifies one or more players to find a match for, and specifies the type of match to build, including the team configuration and the rules for an acceptable match. When a matchmaking request identifies a group of players who want to play together, FlexMatch finds additional players to fill the match. Match type, rules, and other features are defined in a <code>MatchmakingConfiguration</code>. </p> <p>To start matchmaking, provide a unique ticket ID, specify a matchmaking configuration, and include the players to be matched. For each player, you must also include the player attribute values that are required by the matchmaking configuration (in the rule set). If successful, a matchmaking ticket is returned with status set to <code>QUEUED</code>. </p> <p>Track the status of the ticket to respond as needed. If you&#39;re also using GameLift hosting, a successfully completed ticket contains game session connection information. Ticket status updates are tracked using event notification through Amazon Simple Notification Service (SNS), which is defined in the matchmaking configuration.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-client.html"> Add FlexMatch to a Game Client</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-notification.html"> Set Up FlexMatch Event Notification</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-tasks.html"> FlexMatch Integration Roadmap</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/gamelift-match.html"> How GameLift FlexMatch Works</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>StartMatchmaking</a> </p> </li> <li> <p> <a>DescribeMatchmaking</a> </p> </li> <li> <p> <a>StopMatchmaking</a> </p> </li> <li> <p> <a>AcceptMatch</a> </p> </li> <li> <p> <a>StartMatchBackfill</a> </p> </li> </ul></p>
    async fn start_matchmaking(
        &self,
        input: StartMatchmakingInput,
    ) -> Result<StartMatchmakingOutput, RusotoError<StartMatchmakingError>>;

    /// <p><p>Suspends activity on a fleet. Currently, this operation is used to stop a fleet&#39;s auto-scaling activity. It is used to temporarily stop triggering scaling events. The policies can be retained and auto-scaling activity can be restarted using <a>StartFleetActions</a>. You can view a fleet&#39;s stopped actions using <a>DescribeFleetAttributes</a>.</p> <p>To stop fleet actions, specify the fleet ID and the type of actions to suspend. When auto-scaling fleet actions are stopped, Amazon GameLift no longer initiates scaling events except in response to manual changes using <a>UpdateFleetCapacity</a>. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn stop_fleet_actions(
        &self,
        input: StopFleetActionsInput,
    ) -> Result<StopFleetActionsOutput, RusotoError<StopFleetActionsError>>;

    /// <p><p>Cancels a game session placement that is in <code>PENDING</code> status. To stop a placement, provide the placement ID values. If successful, the placement is moved to <code>CANCELLED</code> status.</p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul></p>
    async fn stop_game_session_placement(
        &self,
        input: StopGameSessionPlacementInput,
    ) -> Result<StopGameSessionPlacementOutput, RusotoError<StopGameSessionPlacementError>>;

    /// <p><p>Cancels a matchmaking ticket or match backfill ticket that is currently being processed. To stop the matchmaking operation, specify the ticket ID. If successful, work on the ticket is stopped, and the ticket status is changed to <code>CANCELLED</code>.</p> <p>This call is also used to turn off automatic backfill for an individual game session. This is for game sessions that are created with a matchmaking configuration that has automatic backfill enabled. The ticket ID is included in the <code>MatchmakerData</code> of an updated game session object, which is provided to the game server.</p> <note> <p>If the operation is successful, the service sends back an empty JSON struct with the HTTP 200 response (not an empty HTTP body).</p> </note> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-client.html"> Add FlexMatch to a Game Client</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>StartMatchmaking</a> </p> </li> <li> <p> <a>DescribeMatchmaking</a> </p> </li> <li> <p> <a>StopMatchmaking</a> </p> </li> <li> <p> <a>AcceptMatch</a> </p> </li> <li> <p> <a>StartMatchBackfill</a> </p> </li> </ul></p>
    async fn stop_matchmaking(
        &self,
        input: StopMatchmakingInput,
    ) -> Result<StopMatchmakingOutput, RusotoError<StopMatchmakingError>>;

    /// <p><p> <b>This operation is used with the Amazon GameLift FleetIQ solution and game server groups.</b> </p> <p>Temporarily stops activity on a game server group without terminating instances or the game server group. You can restart activity by calling <a>ResumeGameServerGroup</a>. You can suspend the following activity:</p> <ul> <li> <p> <b>Instance type replacement</b> - This activity evaluates the current game hosting viability of all Spot instance types that are defined for the game server group. It updates the Auto Scaling group to remove nonviable Spot Instance types, which have a higher chance of game server interruptions. It then balances capacity across the remaining viable Spot Instance types. When this activity is suspended, the Auto Scaling group continues with its current balance, regardless of viability. Instance protection, utilization metrics, and capacity scaling activities continue to be active. </p> </li> </ul> <p>To suspend activity, specify a game server group ARN and the type of activity to be suspended. If successful, a <a>GameServerGroup</a> object is returned showing that the activity is listed in <code>SuspendedActions</code>.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">GameLift FleetIQ Guide</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateGameServerGroup</a> </p> </li> <li> <p> <a>ListGameServerGroups</a> </p> </li> <li> <p> <a>DescribeGameServerGroup</a> </p> </li> <li> <p> <a>UpdateGameServerGroup</a> </p> </li> <li> <p> <a>DeleteGameServerGroup</a> </p> </li> <li> <p> <a>ResumeGameServerGroup</a> </p> </li> <li> <p> <a>SuspendGameServerGroup</a> </p> </li> <li> <p> <a>DescribeGameServerInstances</a> </p> </li> </ul></p>
    async fn suspend_game_server_group(
        &self,
        input: SuspendGameServerGroupInput,
    ) -> Result<SuspendGameServerGroupOutput, RusotoError<SuspendGameServerGroupError>>;

    /// <p><p> Assigns a tag to a GameLift resource. AWS resource tags provide an additional management tool set. You can use tags to organize resources, create IAM permissions policies to manage access to groups of resources, customize AWS cost breakdowns, etc. This operation handles the permissions necessary to manage tags for the following GameLift resource types:</p> <ul> <li> <p>Build</p> </li> <li> <p>Script</p> </li> <li> <p>Fleet</p> </li> <li> <p>Alias</p> </li> <li> <p>GameSessionQueue</p> </li> <li> <p>MatchmakingConfiguration</p> </li> <li> <p>MatchmakingRuleSet</p> </li> </ul> <p>To add a tag to a resource, specify the unique ARN value for the resource and provide a tag list containing one or more tags. The operation succeeds even if the list includes tags that are already assigned to the specified resource. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS Resources</a> in the <i>AWS General Reference</i> </p> <p> <a href="http://aws.amazon.com/answers/account-management/aws-tagging-strategies/"> AWS Tagging Strategies</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>TagResource</a> </p> </li> <li> <p> <a>UntagResource</a> </p> </li> <li> <p> <a>ListTagsForResource</a> </p> </li> </ul></p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p><p>Removes a tag that is assigned to a GameLift resource. Resource tags are used to organize AWS resources for a range of purposes. This operation handles the permissions necessary to manage tags for the following GameLift resource types:</p> <ul> <li> <p>Build</p> </li> <li> <p>Script</p> </li> <li> <p>Fleet</p> </li> <li> <p>Alias</p> </li> <li> <p>GameSessionQueue</p> </li> <li> <p>MatchmakingConfiguration</p> </li> <li> <p>MatchmakingRuleSet</p> </li> </ul> <p>To remove a tag from a resource, specify the unique ARN value for the resource and provide a string list containing one or more tags to be removed. This operation succeeds even if the list includes tags that are not currently assigned to the specified resource.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS Resources</a> in the <i>AWS General Reference</i> </p> <p> <a href="http://aws.amazon.com/answers/account-management/aws-tagging-strategies/"> AWS Tagging Strategies</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>TagResource</a> </p> </li> <li> <p> <a>UntagResource</a> </p> </li> <li> <p> <a>ListTagsForResource</a> </p> </li> </ul></p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p><p>Updates properties for an alias. To update properties, specify the alias ID to be updated and provide the information to be changed. To reassign an alias to another fleet, provide an updated routing strategy. If successful, the updated alias record is returned.</p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>DescribeAlias</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ResolveAlias</a> </p> </li> </ul></p>
    async fn update_alias(
        &self,
        input: UpdateAliasInput,
    ) -> Result<UpdateAliasOutput, RusotoError<UpdateAliasError>>;

    /// <p><p>Updates metadata in a build resource, including the build name and version. To update the metadata, specify the build ID to update and provide the new values. If successful, a build object containing the updated metadata is returned.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-build-intro.html"> Upload a Custom Server Build</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateBuild</a> </p> </li> <li> <p> <a>ListBuilds</a> </p> </li> <li> <p> <a>DescribeBuild</a> </p> </li> <li> <p> <a>UpdateBuild</a> </p> </li> <li> <p> <a>DeleteBuild</a> </p> </li> </ul></p>
    async fn update_build(
        &self,
        input: UpdateBuildInput,
    ) -> Result<UpdateBuildOutput, RusotoError<UpdateBuildError>>;

    /// <p><p>Updates fleet properties, including name and description, for a fleet. To update metadata, specify the fleet ID and the property values that you want to change. If successful, the fleet ID for the updated fleet is returned.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn update_fleet_attributes(
        &self,
        input: UpdateFleetAttributesInput,
    ) -> Result<UpdateFleetAttributesOutput, RusotoError<UpdateFleetAttributesError>>;

    /// <p><p>Updates capacity settings for a fleet. Use this operation to specify the number of EC2 instances (hosts) that you want this fleet to contain. Before calling this operation, you may want to call <a>DescribeEC2InstanceLimits</a> to get the maximum capacity based on the fleet&#39;s EC2 instance type.</p> <p>Specify minimum and maximum number of instances. Amazon GameLift will not change fleet capacity to values fall outside of this range. This is particularly important when using auto-scaling (see <a>PutScalingPolicy</a>) to allow capacity to adjust based on player demand while imposing limits on automatic adjustments.</p> <p>To update fleet capacity, specify the fleet ID and the number of instances you want the fleet to host. If successful, Amazon GameLift starts or terminates instances so that the fleet&#39;s active instance count matches the desired instance count. You can view a fleet&#39;s current capacity information by calling <a>DescribeFleetCapacity</a>. If the desired instance count is higher than the instance type&#39;s limit, the &quot;Limit Exceeded&quot; exception occurs.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn update_fleet_capacity(
        &self,
        input: UpdateFleetCapacityInput,
    ) -> Result<UpdateFleetCapacityOutput, RusotoError<UpdateFleetCapacityError>>;

    /// <p><p>Updates port settings for a fleet. To update settings, specify the fleet ID to be updated and list the permissions you want to update. List the permissions you want to add in <code>InboundPermissionAuthorizations</code>, and permissions you want to remove in <code>InboundPermissionRevocations</code>. Permissions to be removed must match existing fleet permissions. If successful, the fleet ID for the updated fleet is returned.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn update_fleet_port_settings(
        &self,
        input: UpdateFleetPortSettingsInput,
    ) -> Result<UpdateFleetPortSettingsOutput, RusotoError<UpdateFleetPortSettingsError>>;

    /// <p><p> <b>This operation is used with the Amazon GameLift FleetIQ solution and game server groups.</b> </p> <p>Updates information about a registered game server to help GameLift FleetIQ to track game server availability. This operation is called by a game server process that is running on an instance in a game server group. </p> <p>Use this operation to update the following types of game server information. You can make all three types of updates in the same request:</p> <ul> <li> <p>To update the game server&#39;s utilization status, identify the game server and game server group and specify the current utilization status. Use this status to identify when game servers are currently hosting games and when they are available to be claimed.</p> </li> <li> <p>To report health status, identify the game server and game server group and set health check to <code>HEALTHY</code>. If a game server does not report health status for a certain length of time, the game server is no longer considered healthy. As a result, it will be eventually deregistered from the game server group to avoid affecting utilization metrics. The best practice is to report health every 60 seconds.</p> </li> <li> <p>To change game server metadata, provide updated game server data.</p> </li> </ul> <p>Once a game server is successfully updated, the relevant statuses and timestamps are updated.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">GameLift FleetIQ Guide</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>RegisterGameServer</a> </p> </li> <li> <p> <a>ListGameServers</a> </p> </li> <li> <p> <a>ClaimGameServer</a> </p> </li> <li> <p> <a>DescribeGameServer</a> </p> </li> <li> <p> <a>UpdateGameServer</a> </p> </li> <li> <p> <a>DeregisterGameServer</a> </p> </li> </ul></p>
    async fn update_game_server(
        &self,
        input: UpdateGameServerInput,
    ) -> Result<UpdateGameServerOutput, RusotoError<UpdateGameServerError>>;

    /// <p><p> <b>This operation is used with the Amazon GameLift FleetIQ solution and game server groups.</b> </p> <p>Updates GameLift FleetIQ-specific properties for a game server group. Many Auto Scaling group properties are updated on the Auto Scaling group directly, including the launch template, Auto Scaling policies, and maximum/minimum/desired instance counts.</p> <p>To update the game server group, specify the game server group ID and provide the updated values. Before applying the updates, the new values are validated to ensure that GameLift FleetIQ can continue to perform instance balancing activity. If successful, a <a>GameServerGroup</a> object is returned.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">GameLift FleetIQ Guide</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateGameServerGroup</a> </p> </li> <li> <p> <a>ListGameServerGroups</a> </p> </li> <li> <p> <a>DescribeGameServerGroup</a> </p> </li> <li> <p> <a>UpdateGameServerGroup</a> </p> </li> <li> <p> <a>DeleteGameServerGroup</a> </p> </li> <li> <p> <a>ResumeGameServerGroup</a> </p> </li> <li> <p> <a>SuspendGameServerGroup</a> </p> </li> <li> <p> <a>DescribeGameServerInstances</a> </p> </li> </ul></p>
    async fn update_game_server_group(
        &self,
        input: UpdateGameServerGroupInput,
    ) -> Result<UpdateGameServerGroupOutput, RusotoError<UpdateGameServerGroupError>>;

    /// <p><p>Updates game session properties. This includes the session name, maximum player count, protection policy, which controls whether or not an active game session can be terminated during a scale-down event, and the player session creation policy, which controls whether or not new players can join the session. To update a game session, specify the game session ID and the values you want to change. If successful, an updated <a>GameSession</a> object is returned. </p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul></p>
    async fn update_game_session(
        &self,
        input: UpdateGameSessionInput,
    ) -> Result<UpdateGameSessionOutput, RusotoError<UpdateGameSessionError>>;

    /// <p><p>Updates settings for a game session queue, which determines how new game session requests in the queue are processed. To update settings, specify the queue name to be updated and provide the new settings. When updating destinations, provide a complete list of destinations. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/queues-intro.html"> Using Multi-Region Queues</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateGameSessionQueue</a> </p> </li> <li> <p> <a>DescribeGameSessionQueues</a> </p> </li> <li> <p> <a>UpdateGameSessionQueue</a> </p> </li> <li> <p> <a>DeleteGameSessionQueue</a> </p> </li> </ul></p>
    async fn update_game_session_queue(
        &self,
        input: UpdateGameSessionQueueInput,
    ) -> Result<UpdateGameSessionQueueOutput, RusotoError<UpdateGameSessionQueueError>>;

    /// <p><p>Updates settings for a FlexMatch matchmaking configuration. These changes affect all matches and game sessions that are created after the update. To update settings, specify the configuration name to be updated and provide the new settings. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-configuration.html"> Design a FlexMatch Matchmaker</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DeleteMatchmakingRuleSet</a> </p> </li> </ul></p>
    async fn update_matchmaking_configuration(
        &self,
        input: UpdateMatchmakingConfigurationInput,
    ) -> Result<
        UpdateMatchmakingConfigurationOutput,
        RusotoError<UpdateMatchmakingConfigurationError>,
    >;

    /// <p><p>Updates the current runtime configuration for the specified fleet, which tells Amazon GameLift how to launch server processes on instances in the fleet. You can update a fleet&#39;s runtime configuration at any time after the fleet is created; it does not need to be in an <code>ACTIVE</code> status.</p> <p>To update runtime configuration, specify the fleet ID and provide a <code>RuntimeConfiguration</code> object with an updated set of server process configurations.</p> <p>Each instance in a Amazon GameLift fleet checks regularly for an updated runtime configuration and changes how it launches server processes to comply with the latest version. Existing server processes are not affected by the update; runtime configuration changes are applied gradually as existing processes shut down and new processes are launched during Amazon GameLift&#39;s normal process recycling activity.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn update_runtime_configuration(
        &self,
        input: UpdateRuntimeConfigurationInput,
    ) -> Result<UpdateRuntimeConfigurationOutput, RusotoError<UpdateRuntimeConfigurationError>>;

    /// <p><p>Updates Realtime script metadata and content.</p> <p>To update script metadata, specify the script ID and provide updated name and/or version values. </p> <p>To update script content, provide an updated zip file by pointing to either a local file or an Amazon S3 bucket location. You can use either method regardless of how the original script was uploaded. Use the <i>Version</i> parameter to track updates to the script.</p> <p>If the call is successful, the updated metadata is stored in the script record and a revised script is uploaded to the Amazon GameLift service. Once the script is updated and acquired by a fleet instance, the new version is used for all new game sessions. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/realtime-intro.html">Amazon GameLift Realtime Servers</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateScript</a> </p> </li> <li> <p> <a>ListScripts</a> </p> </li> <li> <p> <a>DescribeScript</a> </p> </li> <li> <p> <a>UpdateScript</a> </p> </li> <li> <p> <a>DeleteScript</a> </p> </li> </ul></p>
    async fn update_script(
        &self,
        input: UpdateScriptInput,
    ) -> Result<UpdateScriptOutput, RusotoError<UpdateScriptError>>;

    /// <p><p>Validates the syntax of a matchmaking rule or rule set. This operation checks that the rule set is using syntactically correct JSON and that it conforms to allowed property expressions. To validate syntax, provide a rule set JSON string.</p> <p> <b>Learn more</b> </p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-rulesets.html">Build a Rule Set</a> </p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DeleteMatchmakingRuleSet</a> </p> </li> </ul></p>
    async fn validate_matchmaking_rule_set(
        &self,
        input: ValidateMatchmakingRuleSetInput,
    ) -> Result<ValidateMatchmakingRuleSetOutput, RusotoError<ValidateMatchmakingRuleSetError>>;
}
/// A client for the Amazon GameLift API.
#[derive(Clone)]
pub struct GameLiftClient {
    client: Client,
    region: region::Region,
}

impl GameLiftClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> GameLiftClient {
        GameLiftClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> GameLiftClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        GameLiftClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> GameLiftClient {
        GameLiftClient { client, region }
    }
}

#[async_trait]
impl GameLift for GameLiftClient {
    /// <p><p>Registers a player&#39;s acceptance or rejection of a proposed FlexMatch match. A matchmaking configuration may require player acceptance; if so, then matches built with that configuration cannot be completed unless all players accept the proposed match within a specified time limit. </p> <p>When FlexMatch builds a match, all the matchmaking tickets involved in the proposed match are placed into status <code>REQUIRES_ACCEPTANCE</code>. This is a trigger for your game to get acceptance from all players in the ticket. Acceptances are only valid for tickets when they are in this status; all other acceptances result in an error.</p> <p>To register acceptance, specify the ticket ID, a response, and one or more players. Once all players have registered acceptance, the matchmaking tickets advance to status <code>PLACING</code>, where a new game session is created for the match. </p> <p>If any player rejects the match, or if acceptances are not received before a specified timeout, the proposed match is dropped. The matchmaking tickets are then handled in one of two ways: For tickets where one or more players rejected the match, the ticket status is returned to <code>SEARCHING</code> to find a new match. For tickets where one or more players failed to respond, the ticket status is set to <code>CANCELLED</code>, and processing is terminated. A new matchmaking request for these players can be submitted as needed. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-client.html"> Add FlexMatch to a Game Client</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-events.html"> FlexMatch Events Reference</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>StartMatchmaking</a> </p> </li> <li> <p> <a>DescribeMatchmaking</a> </p> </li> <li> <p> <a>StopMatchmaking</a> </p> </li> <li> <p> <a>AcceptMatch</a> </p> </li> <li> <p> <a>StartMatchBackfill</a> </p> </li> </ul></p>
    async fn accept_match(
        &self,
        input: AcceptMatchInput,
    ) -> Result<AcceptMatchOutput, RusotoError<AcceptMatchError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.AcceptMatch");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AcceptMatchError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<AcceptMatchOutput, _>()
    }

    /// <p><p> <b>This operation is used with the Amazon GameLift FleetIQ solution and game server groups.</b> </p> <p>Locates an available game server and temporarily reserves it to host gameplay and players. This operation is called from a game client or client service (such as a matchmaker) to request hosting resources for a new game session. In response, GameLift FleetIQ locates an available game server, places it in <code>CLAIMED</code> status for 60 seconds, and returns connection information that players can use to connect to the game server. </p> <p>To claim a game server, identify a game server group. You can also specify a game server ID, although this approach bypasses GameLift FleetIQ placement optimization. Optionally, include game data to pass to the game server at the start of a game session, such as a game map or player information. </p> <p>When a game server is successfully claimed, connection information is returned. A claimed game server&#39;s utilization status remains <code>AVAILABLE</code> while the claim status is set to <code>CLAIMED</code> for up to 60 seconds. This time period gives the game server time to update its status to <code>UTILIZED</code> (using <a>UpdateGameServer</a>) once players join. If the game server&#39;s status is not updated within 60 seconds, the game server reverts to unclaimed status and is available to be claimed by another request. The claim time period is a fixed value and is not configurable.</p> <p>If you try to claim a specific game server, this request will fail in the following cases:</p> <ul> <li> <p>If the game server utilization status is <code>UTILIZED</code>.</p> </li> <li> <p>If the game server claim status is <code>CLAIMED</code>.</p> </li> </ul> <note> <p>When claiming a specific game server, this request will succeed even if the game server is running on an instance in <code>DRAINING</code> status. To avoid this, first check the instance status by calling <a>DescribeGameServerInstances</a>.</p> </note> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">GameLift FleetIQ Guide</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>RegisterGameServer</a> </p> </li> <li> <p> <a>ListGameServers</a> </p> </li> <li> <p> <a>ClaimGameServer</a> </p> </li> <li> <p> <a>DescribeGameServer</a> </p> </li> <li> <p> <a>UpdateGameServer</a> </p> </li> <li> <p> <a>DeregisterGameServer</a> </p> </li> </ul></p>
    async fn claim_game_server(
        &self,
        input: ClaimGameServerInput,
    ) -> Result<ClaimGameServerOutput, RusotoError<ClaimGameServerError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.ClaimGameServer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ClaimGameServerError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ClaimGameServerOutput, _>()
    }

    /// <p><p>Creates an alias for a fleet. In most situations, you can use an alias ID in place of a fleet ID. An alias provides a level of abstraction for a fleet that is useful when redirecting player traffic from one fleet to another, such as when updating your game build. </p> <p>Amazon GameLift supports two types of routing strategies for aliases: simple and terminal. A simple alias points to an active fleet. A terminal alias is used to display messaging or link to a URL instead of routing players to an active fleet. For example, you might use a terminal alias when a game version is no longer supported and you want to direct players to an upgrade site. </p> <p>To create a fleet alias, specify an alias name, routing strategy, and optional description. Each simple alias can point to only one fleet, but a fleet can have multiple aliases. If successful, a new alias record is returned, including an alias ID and an ARN. You can reassign an alias to another fleet by calling <code>UpdateAlias</code>.</p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>DescribeAlias</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ResolveAlias</a> </p> </li> </ul></p>
    async fn create_alias(
        &self,
        input: CreateAliasInput,
    ) -> Result<CreateAliasOutput, RusotoError<CreateAliasError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.CreateAlias");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateAliasError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateAliasOutput, _>()
    }

    /// <p><p>Creates a new Amazon GameLift build resource for your game server binary files. Game server binaries must be combined into a zip file for use with Amazon GameLift. </p> <important> <p>When setting up a new game build for GameLift, we recommend using the AWS CLI command <b> <a href="https://docs.aws.amazon.com/cli/latest/reference/gamelift/upload-build.html">upload-build</a> </b>. This helper command combines two tasks: (1) it uploads your build files from a file directory to a GameLift Amazon S3 location, and (2) it creates a new build resource. </p> </important> <p>The <code>CreateBuild</code> operation can used in the following scenarios:</p> <ul> <li> <p>To create a new game build with build files that are in an S3 location under an AWS account that you control. To use this option, you must first give Amazon GameLift access to the S3 bucket. With permissions in place, call <code>CreateBuild</code> and specify a build name, operating system, and the S3 storage location of your game build.</p> </li> <li> <p>To directly upload your build files to a GameLift S3 location. To use this option, first call <code>CreateBuild</code> and specify a build name and operating system. This operation creates a new build resource and also returns an S3 location with temporary access credentials. Use the credentials to manually upload your build files to the specified S3 location. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UploadingObjects.html">Uploading Objects</a> in the <i>Amazon S3 Developer Guide</i>. Build files can be uploaded to the GameLift S3 location once only; that can&#39;t be updated. </p> </li> </ul> <p>If successful, this operation creates a new build resource with a unique build ID and places it in <code>INITIALIZED</code> status. A build must be in <code>READY</code> status before you can create fleets with it.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-build-intro.html">Uploading Your Game</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-build-cli-uploading.html#gamelift-build-cli-uploading-create-build"> Create a Build with Files in Amazon S3</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateBuild</a> </p> </li> <li> <p> <a>ListBuilds</a> </p> </li> <li> <p> <a>DescribeBuild</a> </p> </li> <li> <p> <a>UpdateBuild</a> </p> </li> <li> <p> <a>DeleteBuild</a> </p> </li> </ul></p>
    async fn create_build(
        &self,
        input: CreateBuildInput,
    ) -> Result<CreateBuildOutput, RusotoError<CreateBuildError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.CreateBuild");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateBuildError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateBuildOutput, _>()
    }

    /// <p><p>Creates a new fleet to run your game servers. whether they are custom game builds or Realtime Servers with game-specific script. A fleet is a set of Amazon Elastic Compute Cloud (Amazon EC2) instances, each of which can host multiple game sessions. When creating a fleet, you choose the hardware specifications, set some configuration options, and specify the game server to deploy on the new fleet. </p> <p>To create a new fleet, provide the following: (1) a fleet name, (2) an EC2 instance type and fleet type (spot or on-demand), (3) the build ID for your game build or script ID if using Realtime Servers, and (4) a runtime configuration, which determines how game servers will run on each instance in the fleet. </p> <p>If the <code>CreateFleet</code> call is successful, Amazon GameLift performs the following tasks. You can track the process of a fleet by checking the fleet status or by monitoring fleet creation events:</p> <ul> <li> <p>Creates a fleet resource. Status: <code>NEW</code>.</p> </li> <li> <p>Begins writing events to the fleet event log, which can be accessed in the Amazon GameLift console.</p> </li> <li> <p>Sets the fleet&#39;s target capacity to 1 (desired instances), which triggers Amazon GameLift to start one new EC2 instance.</p> </li> <li> <p>Downloads the game build or Realtime script to the new instance and installs it. Statuses: <code>DOWNLOADING</code>, <code>VALIDATING</code>, <code>BUILDING</code>. </p> </li> <li> <p>Starts launching server processes on the instance. If the fleet is configured to run multiple server processes per instance, Amazon GameLift staggers each process launch by a few seconds. Status: <code>ACTIVATING</code>.</p> </li> <li> <p>Sets the fleet&#39;s status to <code>ACTIVE</code> as soon as one server process is ready to host a game session.</p> </li> </ul> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting Up Fleets</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-creating-debug.html#fleets-creating-debug-creation">Debug Fleet Creation Issues</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn create_fleet(
        &self,
        input: CreateFleetInput,
    ) -> Result<CreateFleetOutput, RusotoError<CreateFleetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.CreateFleet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateFleetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateFleetOutput, _>()
    }

    /// <p><p> <b>This operation is used with the Amazon GameLift FleetIQ solution and game server groups.</b> </p> <p>Creates a GameLift FleetIQ game server group for managing game hosting on a collection of Amazon EC2 instances for game hosting. This operation creates the game server group, creates an Auto Scaling group in your AWS account, and establishes a link between the two groups. You can view the status of your game server groups in the GameLift console. Game server group metrics and events are emitted to Amazon CloudWatch.</p> <p>Before creating a new game server group, you must have the following: </p> <ul> <li> <p>An Amazon EC2 launch template that specifies how to launch Amazon EC2 instances with your game server build. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-launch-templates.html"> Launching an Instance from a Launch Template</a> in the <i>Amazon EC2 User Guide</i>. </p> </li> <li> <p>An IAM role that extends limited access to your AWS account to allow GameLift FleetIQ to create and interact with the Auto Scaling group. For more information, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gsg-iam-permissions-roles.html">Create IAM roles for cross-service interaction</a> in the <i>GameLift FleetIQ Developer Guide</i>.</p> </li> </ul> <p>To create a new game server group, specify a unique group name, IAM role and Amazon EC2 launch template, and provide a list of instance types that can be used in the group. You must also set initial maximum and minimum limits on the group&#39;s instance count. You can optionally set an Auto Scaling policy with target tracking based on a GameLift FleetIQ metric.</p> <p>Once the game server group and corresponding Auto Scaling group are created, you have full access to change the Auto Scaling group&#39;s configuration as needed. Several properties that are set when creating a game server group, including maximum/minimum size and auto-scaling policy settings, must be updated directly in the Auto Scaling group. Keep in mind that some Auto Scaling group properties are periodically updated by GameLift FleetIQ as part of its balancing activities to optimize for availability and cost.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">GameLift FleetIQ Guide</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateGameServerGroup</a> </p> </li> <li> <p> <a>ListGameServerGroups</a> </p> </li> <li> <p> <a>DescribeGameServerGroup</a> </p> </li> <li> <p> <a>UpdateGameServerGroup</a> </p> </li> <li> <p> <a>DeleteGameServerGroup</a> </p> </li> <li> <p> <a>ResumeGameServerGroup</a> </p> </li> <li> <p> <a>SuspendGameServerGroup</a> </p> </li> <li> <p> <a>DescribeGameServerInstances</a> </p> </li> </ul></p>
    async fn create_game_server_group(
        &self,
        input: CreateGameServerGroupInput,
    ) -> Result<CreateGameServerGroupOutput, RusotoError<CreateGameServerGroupError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.CreateGameServerGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateGameServerGroupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateGameServerGroupOutput, _>()
    }

    /// <p><p>Creates a multiplayer game session for players. This operation creates a game session record and assigns an available server process in the specified fleet to host the game session. A fleet must have an <code>ACTIVE</code> status before a game session can be created in it.</p> <p>To create a game session, specify either fleet ID or alias ID and indicate a maximum number of players to allow in the game session. You can also provide a name and game-specific properties for this game session. If successful, a <a>GameSession</a> object is returned containing the game session properties and other settings you specified.</p> <p> <b>Idempotency tokens.</b> You can add a token that uniquely identifies game session requests. This is useful for ensuring that game session requests are idempotent. Multiple requests with the same idempotency token are processed only once; subsequent requests return the original result. All response values are the same with the exception of game session status, which may change.</p> <p> <b>Resource creation limits.</b> If you are creating a game session on a fleet with a resource creation limit policy in force, then you must specify a creator ID. Without this ID, Amazon GameLift has no way to evaluate the policy for this new game session request.</p> <p> <b>Player acceptance policy.</b> By default, newly created game sessions are open to new players. You can restrict new player access by using <a>UpdateGameSession</a> to change the game session&#39;s player session creation policy.</p> <p> <b>Game session logs.</b> Logs are retained for all active game sessions for 14 days. To access the logs, call <a>GetGameSessionLogUrl</a> to download the log files.</p> <p> <i>Available in Amazon GameLift Local.</i> </p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul></p>
    async fn create_game_session(
        &self,
        input: CreateGameSessionInput,
    ) -> Result<CreateGameSessionOutput, RusotoError<CreateGameSessionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.CreateGameSession");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateGameSessionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateGameSessionOutput, _>()
    }

    /// <p><p>Establishes a new queue for processing requests to place new game sessions. A queue identifies where new game sessions can be hosted -- by specifying a list of destinations (fleets or aliases) -- and how long requests can wait in the queue before timing out. You can set up a queue to try to place game sessions on fleets in multiple Regions. To add placement requests to a queue, call <a>StartGameSessionPlacement</a> and reference the queue name.</p> <p> <b>Destination order.</b> When processing a request for a game session, Amazon GameLift tries each destination in order until it finds one with available resources to host the new game session. A queue&#39;s default order is determined by how destinations are listed. The default order is overridden when a game session placement request provides player latency information. Player latency information enables Amazon GameLift to prioritize destinations where players report the lowest average latency, as a result placing the new game session where the majority of players will have the best possible gameplay experience.</p> <p> <b>Player latency policies.</b> For placement requests containing player latency information, use player latency policies to protect individual players from very high latencies. With a latency cap, even when a destination can deliver a low latency for most players, the game is not placed where any individual player is reporting latency higher than a policy&#39;s maximum. A queue can have multiple latency policies, which are enforced consecutively starting with the policy with the lowest latency cap. Use multiple policies to gradually relax latency controls; for example, you might set a policy with a low latency cap for the first 60 seconds, a second policy with a higher cap for the next 60 seconds, etc. </p> <p>To create a new queue, provide a name, timeout value, a list of destinations and, if desired, a set of latency policies. If successful, a new queue object is returned.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/queues-design.html"> Design a Game Session Queue</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/queues-creating.html"> Create a Game Session Queue</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateGameSessionQueue</a> </p> </li> <li> <p> <a>DescribeGameSessionQueues</a> </p> </li> <li> <p> <a>UpdateGameSessionQueue</a> </p> </li> <li> <p> <a>DeleteGameSessionQueue</a> </p> </li> </ul></p>
    async fn create_game_session_queue(
        &self,
        input: CreateGameSessionQueueInput,
    ) -> Result<CreateGameSessionQueueOutput, RusotoError<CreateGameSessionQueueError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.CreateGameSessionQueue");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateGameSessionQueueError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateGameSessionQueueOutput, _>()
    }

    /// <p><p>Defines a new matchmaking configuration for use with FlexMatch. Whether your are using FlexMatch with GameLift hosting or as a standalone matchmaking service, the matchmaking configuration sets out rules for matching players and forming teams. If you&#39;re also using GameLift hosting, it defines how to start game sessions for each match. Your matchmaking system can use multiple configurations to handle different game scenarios. All matchmaking requests (<a>StartMatchmaking</a> or <a>StartMatchBackfill</a>) identify the matchmaking configuration to use and provide player attributes consistent with that configuration. </p> <p>To create a matchmaking configuration, you must provide the following: configuration name and FlexMatch mode (with or without GameLift hosting); a rule set that specifies how to evaluate players and find acceptable matches; whether player acceptance is required; and the maximum time allowed for a matchmaking attempt. When using FlexMatch with GameLift hosting, you also need to identify the game session queue to use when starting a game session for the match.</p> <p>In addition, you must set up an Amazon Simple Notification Service (SNS) to receive matchmaking notifications, and provide the topic ARN in the matchmaking configuration. An alternative method, continuously polling ticket status with <a>DescribeMatchmaking</a>, is only suitable for games in development with low matchmaking usage.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/gamelift-match.html"> FlexMatch Developer Guide</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-configuration.html"> Design a FlexMatch Matchmaker</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-notification.html"> Set Up FlexMatch Event Notification</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DeleteMatchmakingRuleSet</a> </p> </li> </ul></p>
    async fn create_matchmaking_configuration(
        &self,
        input: CreateMatchmakingConfigurationInput,
    ) -> Result<
        CreateMatchmakingConfigurationOutput,
        RusotoError<CreateMatchmakingConfigurationError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.CreateMatchmakingConfiguration");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateMatchmakingConfigurationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateMatchmakingConfigurationOutput, _>()
    }

    /// <p><p>Creates a new rule set for FlexMatch matchmaking. A rule set describes the type of match to create, such as the number and size of teams. It also sets the parameters for acceptable player matches, such as minimum skill level or character type. A rule set is used by a <a>MatchmakingConfiguration</a>. </p> <p>To create a matchmaking rule set, provide unique rule set name and the rule set body in JSON format. Rule sets must be defined in the same Region as the matchmaking configuration they are used with.</p> <p>Since matchmaking rule sets cannot be edited, it is a good idea to check the rule set syntax using <a>ValidateMatchmakingRuleSet</a> before creating a new rule set.</p> <p> <b>Learn more</b> </p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-rulesets.html">Build a Rule Set</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-configuration.html">Design a Matchmaker</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-intro.html">Matchmaking with FlexMatch</a> </p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DeleteMatchmakingRuleSet</a> </p> </li> </ul></p>
    async fn create_matchmaking_rule_set(
        &self,
        input: CreateMatchmakingRuleSetInput,
    ) -> Result<CreateMatchmakingRuleSetOutput, RusotoError<CreateMatchmakingRuleSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.CreateMatchmakingRuleSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateMatchmakingRuleSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateMatchmakingRuleSetOutput, _>()
    }

    /// <p><p>Reserves an open player slot in an active game session. Before a player can be added, a game session must have an <code>ACTIVE</code> status, have a creation policy of <code>ALLOW_ALL</code>, and have an open player slot. To add a group of players to a game session, use <a>CreatePlayerSessions</a>. When the player connects to the game server and references a player session ID, the game server contacts the Amazon GameLift service to validate the player reservation and accept the player.</p> <p>To create a player session, specify a game session ID, player ID, and optionally a string of player data. If successful, a slot is reserved in the game session for the player and a new <a>PlayerSession</a> object is returned. Player sessions cannot be updated. </p> <p> <i>Available in Amazon GameLift Local.</i> </p> <ul> <li> <p> <a>CreatePlayerSession</a> </p> </li> <li> <p> <a>CreatePlayerSessions</a> </p> </li> <li> <p> <a>DescribePlayerSessions</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul></p>
    async fn create_player_session(
        &self,
        input: CreatePlayerSessionInput,
    ) -> Result<CreatePlayerSessionOutput, RusotoError<CreatePlayerSessionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.CreatePlayerSession");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreatePlayerSessionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreatePlayerSessionOutput, _>()
    }

    /// <p><p>Reserves open slots in a game session for a group of players. Before players can be added, a game session must have an <code>ACTIVE</code> status, have a creation policy of <code>ALLOW_ALL</code>, and have an open player slot. To add a single player to a game session, use <a>CreatePlayerSession</a>. When a player connects to the game server and references a player session ID, the game server contacts the Amazon GameLift service to validate the player reservation and accept the player.</p> <p>To create player sessions, specify a game session ID, a list of player IDs, and optionally a set of player data strings. If successful, a slot is reserved in the game session for each player and a set of new <a>PlayerSession</a> objects is returned. Player sessions cannot be updated.</p> <p> <i>Available in Amazon GameLift Local.</i> </p> <ul> <li> <p> <a>CreatePlayerSession</a> </p> </li> <li> <p> <a>CreatePlayerSessions</a> </p> </li> <li> <p> <a>DescribePlayerSessions</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul></p>
    async fn create_player_sessions(
        &self,
        input: CreatePlayerSessionsInput,
    ) -> Result<CreatePlayerSessionsOutput, RusotoError<CreatePlayerSessionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.CreatePlayerSessions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreatePlayerSessionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreatePlayerSessionsOutput, _>()
    }

    /// <p><p>Creates a new script record for your Realtime Servers script. Realtime scripts are JavaScript that provide configuration settings and optional custom game logic for your game. The script is deployed when you create a Realtime Servers fleet to host your game sessions. Script logic is executed during an active game session. </p> <p>To create a new script record, specify a script name and provide the script file(s). The script files and all dependencies must be zipped into a single file. You can pull the zip file from either of these locations: </p> <ul> <li> <p>A locally available directory. Use the <i>ZipFile</i> parameter for this option.</p> </li> <li> <p>An Amazon Simple Storage Service (Amazon S3) bucket under your AWS account. Use the <i>StorageLocation</i> parameter for this option. You&#39;ll need to have an Identity Access Management (IAM) role that allows the Amazon GameLift service to access your S3 bucket. </p> </li> </ul> <p>If the call is successful, a new script record is created with a unique script ID. If the script file is provided as a local file, the file is uploaded to an Amazon GameLift-owned S3 bucket and the script record&#39;s storage location reflects this location. If the script file is provided as an S3 bucket, Amazon GameLift accesses the file at this storage location as needed for deployment.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/realtime-intro.html">Amazon GameLift Realtime Servers</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/setting-up-role.html">Set Up a Role for Amazon GameLift Access</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateScript</a> </p> </li> <li> <p> <a>ListScripts</a> </p> </li> <li> <p> <a>DescribeScript</a> </p> </li> <li> <p> <a>UpdateScript</a> </p> </li> <li> <p> <a>DeleteScript</a> </p> </li> </ul></p>
    async fn create_script(
        &self,
        input: CreateScriptInput,
    ) -> Result<CreateScriptOutput, RusotoError<CreateScriptError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.CreateScript");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateScriptError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateScriptOutput, _>()
    }

    /// <p><p>Requests authorization to create or delete a peer connection between the VPC for your Amazon GameLift fleet and a virtual private cloud (VPC) in your AWS account. VPC peering enables the game servers on your fleet to communicate directly with other AWS resources. Once you&#39;ve received authorization, call <a>CreateVpcPeeringConnection</a> to establish the peering connection. For more information, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/vpc-peering.html">VPC Peering with Amazon GameLift Fleets</a>.</p> <p>You can peer with VPCs that are owned by any AWS account you have access to, including the account that you use to manage your Amazon GameLift fleets. You cannot peer with VPCs that are in different Regions.</p> <p>To request authorization to create a connection, call this operation from the AWS account with the VPC that you want to peer to your Amazon GameLift fleet. For example, to enable your game servers to retrieve data from a DynamoDB table, use the account that manages that DynamoDB resource. Identify the following values: (1) The ID of the VPC that you want to peer with, and (2) the ID of the AWS account that you use to manage Amazon GameLift. If successful, VPC peering is authorized for the specified VPC. </p> <p>To request authorization to delete a connection, call this operation from the AWS account with the VPC that is peered with your Amazon GameLift fleet. Identify the following values: (1) VPC ID that you want to delete the peering connection for, and (2) ID of the AWS account that you use to manage Amazon GameLift. </p> <p>The authorization remains valid for 24 hours unless it is canceled by a call to <a>DeleteVpcPeeringAuthorization</a>. You must create or delete the peering connection while the authorization is valid. </p> <ul> <li> <p> <a>CreateVpcPeeringAuthorization</a> </p> </li> <li> <p> <a>DescribeVpcPeeringAuthorizations</a> </p> </li> <li> <p> <a>DeleteVpcPeeringAuthorization</a> </p> </li> <li> <p> <a>CreateVpcPeeringConnection</a> </p> </li> <li> <p> <a>DescribeVpcPeeringConnections</a> </p> </li> <li> <p> <a>DeleteVpcPeeringConnection</a> </p> </li> </ul></p>
    async fn create_vpc_peering_authorization(
        &self,
        input: CreateVpcPeeringAuthorizationInput,
    ) -> Result<CreateVpcPeeringAuthorizationOutput, RusotoError<CreateVpcPeeringAuthorizationError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.CreateVpcPeeringAuthorization");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateVpcPeeringAuthorizationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateVpcPeeringAuthorizationOutput, _>()
    }

    /// <p><p>Establishes a VPC peering connection between a virtual private cloud (VPC) in an AWS account with the VPC for your Amazon GameLift fleet. VPC peering enables the game servers on your fleet to communicate directly with other AWS resources. You can peer with VPCs in any AWS account that you have access to, including the account that you use to manage your Amazon GameLift fleets. You cannot peer with VPCs that are in different Regions. For more information, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/vpc-peering.html">VPC Peering with Amazon GameLift Fleets</a>.</p> <p>Before calling this operation to establish the peering connection, you first need to call <a>CreateVpcPeeringAuthorization</a> and identify the VPC you want to peer with. Once the authorization for the specified VPC is issued, you have 24 hours to establish the connection. These two operations handle all tasks necessary to peer the two VPCs, including acceptance, updating routing tables, etc. </p> <p>To establish the connection, call this operation from the AWS account that is used to manage the Amazon GameLift fleets. Identify the following values: (1) The ID of the fleet you want to be enable a VPC peering connection for; (2) The AWS account with the VPC that you want to peer with; and (3) The ID of the VPC you want to peer with. This operation is asynchronous. If successful, a <a>VpcPeeringConnection</a> request is created. You can use continuous polling to track the request&#39;s status using <a>DescribeVpcPeeringConnections</a>, or by monitoring fleet events for success or failure using <a>DescribeFleetEvents</a>. </p> <ul> <li> <p> <a>CreateVpcPeeringAuthorization</a> </p> </li> <li> <p> <a>DescribeVpcPeeringAuthorizations</a> </p> </li> <li> <p> <a>DeleteVpcPeeringAuthorization</a> </p> </li> <li> <p> <a>CreateVpcPeeringConnection</a> </p> </li> <li> <p> <a>DescribeVpcPeeringConnections</a> </p> </li> <li> <p> <a>DeleteVpcPeeringConnection</a> </p> </li> </ul></p>
    async fn create_vpc_peering_connection(
        &self,
        input: CreateVpcPeeringConnectionInput,
    ) -> Result<CreateVpcPeeringConnectionOutput, RusotoError<CreateVpcPeeringConnectionError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.CreateVpcPeeringConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateVpcPeeringConnectionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateVpcPeeringConnectionOutput, _>()
    }

    /// <p><p>Deletes an alias. This operation removes all record of the alias. Game clients attempting to access a server process using the deleted alias receive an error. To delete an alias, specify the alias ID to be deleted.</p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>DescribeAlias</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ResolveAlias</a> </p> </li> </ul></p>
    async fn delete_alias(
        &self,
        input: DeleteAliasInput,
    ) -> Result<(), RusotoError<DeleteAliasError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DeleteAlias");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteAliasError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p><p>Deletes a build. This operation permanently deletes the build resource and any uploaded build files. Deleting a build does not affect the status of any active fleets using the build, but you can no longer create new fleets with the deleted build.</p> <p>To delete a build, specify the build ID. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-build-intro.html"> Upload a Custom Server Build</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateBuild</a> </p> </li> <li> <p> <a>ListBuilds</a> </p> </li> <li> <p> <a>DescribeBuild</a> </p> </li> <li> <p> <a>UpdateBuild</a> </p> </li> <li> <p> <a>DeleteBuild</a> </p> </li> </ul></p>
    async fn delete_build(
        &self,
        input: DeleteBuildInput,
    ) -> Result<(), RusotoError<DeleteBuildError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DeleteBuild");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteBuildError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p><p>Deletes everything related to a fleet. Before deleting a fleet, you must set the fleet&#39;s desired capacity to zero. See <a>UpdateFleetCapacity</a>.</p> <p>If the fleet being deleted has a VPC peering connection, you first need to get a valid authorization (good for 24 hours) by calling <a>CreateVpcPeeringAuthorization</a>. You do not need to explicitly delete the VPC peering connection--this is done as part of the delete fleet process.</p> <p>This operation removes the fleet and its resources. Once a fleet is deleted, you can no longer use any of the resource in that fleet.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn delete_fleet(
        &self,
        input: DeleteFleetInput,
    ) -> Result<(), RusotoError<DeleteFleetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DeleteFleet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteFleetError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p><p> <b>This operation is used with the Amazon GameLift FleetIQ solution and game server groups.</b> </p> <p>Terminates a game server group and permanently deletes the game server group record. You have several options for how these resources are impacted when deleting the game server group. Depending on the type of delete operation selected, this operation might affect these resources:</p> <ul> <li> <p>The game server group</p> </li> <li> <p>The corresponding Auto Scaling group</p> </li> <li> <p>All game servers that are currently running in the group</p> </li> </ul> <p>To delete a game server group, identify the game server group to delete and specify the type of delete operation to initiate. Game server groups can only be deleted if they are in <code>ACTIVE</code> or <code>ERROR</code> status.</p> <p>If the delete request is successful, a series of operations are kicked off. The game server group status is changed to <code>DELETE_SCHEDULED</code>, which prevents new game servers from being registered and stops automatic scaling activity. Once all game servers in the game server group are deregistered, GameLift FleetIQ can begin deleting resources. If any of the delete operations fail, the game server group is placed in <code>ERROR</code> status.</p> <p>GameLift FleetIQ emits delete events to Amazon CloudWatch.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">GameLift FleetIQ Guide</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateGameServerGroup</a> </p> </li> <li> <p> <a>ListGameServerGroups</a> </p> </li> <li> <p> <a>DescribeGameServerGroup</a> </p> </li> <li> <p> <a>UpdateGameServerGroup</a> </p> </li> <li> <p> <a>DeleteGameServerGroup</a> </p> </li> <li> <p> <a>ResumeGameServerGroup</a> </p> </li> <li> <p> <a>SuspendGameServerGroup</a> </p> </li> <li> <p> <a>DescribeGameServerInstances</a> </p> </li> </ul></p>
    async fn delete_game_server_group(
        &self,
        input: DeleteGameServerGroupInput,
    ) -> Result<DeleteGameServerGroupOutput, RusotoError<DeleteGameServerGroupError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DeleteGameServerGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteGameServerGroupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteGameServerGroupOutput, _>()
    }

    /// <p><p>Deletes a game session queue. Once a queue is successfully deleted, unfulfilled <a>StartGameSessionPlacement</a> requests that reference the queue will fail. To delete a queue, specify the queue name.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/queues-intro.html"> Using Multi-Region Queues</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateGameSessionQueue</a> </p> </li> <li> <p> <a>DescribeGameSessionQueues</a> </p> </li> <li> <p> <a>UpdateGameSessionQueue</a> </p> </li> <li> <p> <a>DeleteGameSessionQueue</a> </p> </li> </ul></p>
    async fn delete_game_session_queue(
        &self,
        input: DeleteGameSessionQueueInput,
    ) -> Result<DeleteGameSessionQueueOutput, RusotoError<DeleteGameSessionQueueError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DeleteGameSessionQueue");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteGameSessionQueueError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteGameSessionQueueOutput, _>()
    }

    /// <p><p>Permanently removes a FlexMatch matchmaking configuration. To delete, specify the configuration name. A matchmaking configuration cannot be deleted if it is being used in any active matchmaking tickets.</p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DeleteMatchmakingRuleSet</a> </p> </li> </ul></p>
    async fn delete_matchmaking_configuration(
        &self,
        input: DeleteMatchmakingConfigurationInput,
    ) -> Result<
        DeleteMatchmakingConfigurationOutput,
        RusotoError<DeleteMatchmakingConfigurationError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DeleteMatchmakingConfiguration");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteMatchmakingConfigurationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteMatchmakingConfigurationOutput, _>()
    }

    /// <p><p>Deletes an existing matchmaking rule set. To delete the rule set, provide the rule set name. Rule sets cannot be deleted if they are currently being used by a matchmaking configuration. </p> <p> <b>Learn more</b> </p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-rulesets.html">Build a Rule Set</a> </p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DeleteMatchmakingRuleSet</a> </p> </li> </ul></p>
    async fn delete_matchmaking_rule_set(
        &self,
        input: DeleteMatchmakingRuleSetInput,
    ) -> Result<DeleteMatchmakingRuleSetOutput, RusotoError<DeleteMatchmakingRuleSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DeleteMatchmakingRuleSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteMatchmakingRuleSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteMatchmakingRuleSetOutput, _>()
    }

    /// <p><p>Deletes a fleet scaling policy. Once deleted, the policy is no longer in force and GameLift removes all record of it. To delete a scaling policy, specify both the scaling policy name and the fleet ID it is associated with.</p> <p>To temporarily suspend scaling policies, call <a>StopFleetActions</a>. This operation suspends all policies for the fleet.</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> <li> <p>Manage scaling policies:</p> <ul> <li> <p> <a>PutScalingPolicy</a> (auto-scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (auto-scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (auto-scaling)</p> </li> </ul> </li> <li> <p>Manage fleet actions:</p> <ul> <li> <p> <a>StartFleetActions</a> </p> </li> <li> <p> <a>StopFleetActions</a> </p> </li> </ul> </li> </ul></p>
    async fn delete_scaling_policy(
        &self,
        input: DeleteScalingPolicyInput,
    ) -> Result<(), RusotoError<DeleteScalingPolicyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DeleteScalingPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteScalingPolicyError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p><p>Deletes a Realtime script. This operation permanently deletes the script record. If script files were uploaded, they are also deleted (files stored in an S3 bucket are not deleted). </p> <p>To delete a script, specify the script ID. Before deleting a script, be sure to terminate all fleets that are deployed with the script being deleted. Fleet instances periodically check for script updates, and if the script record no longer exists, the instance will go into an error state and be unable to host game sessions.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/realtime-intro.html">Amazon GameLift Realtime Servers</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateScript</a> </p> </li> <li> <p> <a>ListScripts</a> </p> </li> <li> <p> <a>DescribeScript</a> </p> </li> <li> <p> <a>UpdateScript</a> </p> </li> <li> <p> <a>DeleteScript</a> </p> </li> </ul></p>
    async fn delete_script(
        &self,
        input: DeleteScriptInput,
    ) -> Result<(), RusotoError<DeleteScriptError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DeleteScript");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteScriptError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p><p>Cancels a pending VPC peering authorization for the specified VPC. If you need to delete an existing VPC peering connection, call <a>DeleteVpcPeeringConnection</a>. </p> <ul> <li> <p> <a>CreateVpcPeeringAuthorization</a> </p> </li> <li> <p> <a>DescribeVpcPeeringAuthorizations</a> </p> </li> <li> <p> <a>DeleteVpcPeeringAuthorization</a> </p> </li> <li> <p> <a>CreateVpcPeeringConnection</a> </p> </li> <li> <p> <a>DescribeVpcPeeringConnections</a> </p> </li> <li> <p> <a>DeleteVpcPeeringConnection</a> </p> </li> </ul></p>
    async fn delete_vpc_peering_authorization(
        &self,
        input: DeleteVpcPeeringAuthorizationInput,
    ) -> Result<DeleteVpcPeeringAuthorizationOutput, RusotoError<DeleteVpcPeeringAuthorizationError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DeleteVpcPeeringAuthorization");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteVpcPeeringAuthorizationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteVpcPeeringAuthorizationOutput, _>()
    }

    /// <p><p>Removes a VPC peering connection. To delete the connection, you must have a valid authorization for the VPC peering connection that you want to delete. You can check for an authorization by calling <a>DescribeVpcPeeringAuthorizations</a> or request a new one using <a>CreateVpcPeeringAuthorization</a>. </p> <p>Once a valid authorization exists, call this operation from the AWS account that is used to manage the Amazon GameLift fleets. Identify the connection to delete by the connection ID and fleet ID. If successful, the connection is removed. </p> <ul> <li> <p> <a>CreateVpcPeeringAuthorization</a> </p> </li> <li> <p> <a>DescribeVpcPeeringAuthorizations</a> </p> </li> <li> <p> <a>DeleteVpcPeeringAuthorization</a> </p> </li> <li> <p> <a>CreateVpcPeeringConnection</a> </p> </li> <li> <p> <a>DescribeVpcPeeringConnections</a> </p> </li> <li> <p> <a>DeleteVpcPeeringConnection</a> </p> </li> </ul></p>
    async fn delete_vpc_peering_connection(
        &self,
        input: DeleteVpcPeeringConnectionInput,
    ) -> Result<DeleteVpcPeeringConnectionOutput, RusotoError<DeleteVpcPeeringConnectionError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DeleteVpcPeeringConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteVpcPeeringConnectionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteVpcPeeringConnectionOutput, _>()
    }

    /// <p><p> <b>This operation is used with the Amazon GameLift FleetIQ solution and game server groups.</b> </p> <p>Removes the game server from a game server group. As a result of this operation, the deregistered game server can no longer be claimed and will not be returned in a list of active game servers. </p> <p>To deregister a game server, specify the game server group and game server ID. If successful, this operation emits a CloudWatch event with termination timestamp and reason.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">GameLift FleetIQ Guide</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>RegisterGameServer</a> </p> </li> <li> <p> <a>ListGameServers</a> </p> </li> <li> <p> <a>ClaimGameServer</a> </p> </li> <li> <p> <a>DescribeGameServer</a> </p> </li> <li> <p> <a>UpdateGameServer</a> </p> </li> <li> <p> <a>DeregisterGameServer</a> </p> </li> </ul></p>
    async fn deregister_game_server(
        &self,
        input: DeregisterGameServerInput,
    ) -> Result<(), RusotoError<DeregisterGameServerError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DeregisterGameServer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeregisterGameServerError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p><p>Retrieves properties for an alias. This operation returns all alias metadata and settings. To get an alias&#39;s target fleet ID only, use <code>ResolveAlias</code>. </p> <p>To get alias properties, specify the alias ID. If successful, the requested alias record is returned.</p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>DescribeAlias</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ResolveAlias</a> </p> </li> </ul></p>
    async fn describe_alias(
        &self,
        input: DescribeAliasInput,
    ) -> Result<DescribeAliasOutput, RusotoError<DescribeAliasError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DescribeAlias");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeAliasError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeAliasOutput, _>()
    }

    /// <p><p>Retrieves properties for a custom game build. To request a build resource, specify a build ID. If successful, an object containing the build properties is returned.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-build-intro.html"> Upload a Custom Server Build</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateBuild</a> </p> </li> <li> <p> <a>ListBuilds</a> </p> </li> <li> <p> <a>DescribeBuild</a> </p> </li> <li> <p> <a>UpdateBuild</a> </p> </li> <li> <p> <a>DeleteBuild</a> </p> </li> </ul></p>
    async fn describe_build(
        &self,
        input: DescribeBuildInput,
    ) -> Result<DescribeBuildOutput, RusotoError<DescribeBuildError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DescribeBuild");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeBuildError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeBuildOutput, _>()
    }

    /// <p><p>Retrieves the following information for the specified EC2 instance type:</p> <ul> <li> <p>Maximum number of instances allowed per AWS account (service limit).</p> </li> <li> <p>Current usage for the AWS account.</p> </li> </ul> <p>To learn more about the capabilities of each instance type, see <a href="http://aws.amazon.com/ec2/instance-types/">Amazon EC2 Instance Types</a>. Note that the instance types offered may vary depending on the region.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn describe_ec2_instance_limits(
        &self,
        input: DescribeEC2InstanceLimitsInput,
    ) -> Result<DescribeEC2InstanceLimitsOutput, RusotoError<DescribeEC2InstanceLimitsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DescribeEC2InstanceLimits");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeEC2InstanceLimitsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeEC2InstanceLimitsOutput, _>()
    }

    /// <p><p>Retrieves core properties, including configuration, status, and metadata, for a fleet. </p> <p>To get attributes for one or more fleets, provide a list of fleet IDs or fleet ARNs. To get attributes for all fleets, do not specify a fleet identifier. When requesting attributes for multiple fleets, use the pagination parameters to retrieve results as a set of sequential pages. If successful, a <a>FleetAttributes</a> object is returned for each fleet requested, unless the fleet identifier is not found.</p> <note> <p>Some API operations may limit the number of fleet IDs allowed in one request. If a request exceeds this limit, the request fails and the error message includes the maximum allowed number.</p> </note> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn describe_fleet_attributes(
        &self,
        input: DescribeFleetAttributesInput,
    ) -> Result<DescribeFleetAttributesOutput, RusotoError<DescribeFleetAttributesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DescribeFleetAttributes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeFleetAttributesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeFleetAttributesOutput, _>()
    }

    /// <p><p>Retrieves the current capacity statistics for one or more fleets. These statistics present a snapshot of the fleet&#39;s instances and provide insight on current or imminent scaling activity. To get statistics on game hosting activity in the fleet, see <a>DescribeFleetUtilization</a>.</p> <p>You can request capacity for all fleets or specify a list of one or more fleet identifiers. When requesting multiple fleets, use the pagination parameters to retrieve results as a set of sequential pages. If successful, a <a>FleetCapacity</a> object is returned for each requested fleet ID. When a list of fleet IDs is provided, attribute objects are returned only for fleets that currently exist.</p> <note> <p>Some API operations may limit the number of fleet IDs allowed in one request. If a request exceeds this limit, the request fails and the error message includes the maximum allowed.</p> </note> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/monitoring-cloudwatch.html#gamelift-metrics-fleet">GameLift Metrics for Fleets</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn describe_fleet_capacity(
        &self,
        input: DescribeFleetCapacityInput,
    ) -> Result<DescribeFleetCapacityOutput, RusotoError<DescribeFleetCapacityError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DescribeFleetCapacity");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeFleetCapacityError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeFleetCapacityOutput, _>()
    }

    /// <p><p>Retrieves entries from the specified fleet&#39;s event log. You can specify a time range to limit the result set. Use the pagination parameters to retrieve results as a set of sequential pages. If successful, a collection of event log entries matching the request are returned.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn describe_fleet_events(
        &self,
        input: DescribeFleetEventsInput,
    ) -> Result<DescribeFleetEventsOutput, RusotoError<DescribeFleetEventsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DescribeFleetEvents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeFleetEventsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeFleetEventsOutput, _>()
    }

    /// <p><p>Retrieves a fleet&#39;s inbound connection permissions. Connection permissions specify the range of IP addresses and port settings that incoming traffic can use to access server processes in the fleet. Game sessions that are running on instances in the fleet use connections that fall in this range. </p> <p>To get a fleet&#39;s inbound connection permissions, specify the fleet&#39;s unique identifier. If successful, a collection of <a>IpPermission</a> objects is returned for the requested fleet ID. If the requested fleet has been deleted, the result set is empty.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn describe_fleet_port_settings(
        &self,
        input: DescribeFleetPortSettingsInput,
    ) -> Result<DescribeFleetPortSettingsOutput, RusotoError<DescribeFleetPortSettingsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DescribeFleetPortSettings");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeFleetPortSettingsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeFleetPortSettingsOutput, _>()
    }

    /// <p><p>Retrieves utilization statistics for one or more fleets. These statistics provide insight into how available hosting resources are currently being used. To get statistics on available hosting resources, see <a>DescribeFleetCapacity</a>.</p> <p>You can request utilization data for all fleets, or specify a list of one or more fleet IDs. When requesting multiple fleets, use the pagination parameters to retrieve results as a set of sequential pages. If successful, a <a>FleetUtilization</a> object is returned for each requested fleet ID, unless the fleet identifier is not found. </p> <note> <p>Some API operations may limit the number of fleet IDs allowed in one request. If a request exceeds this limit, the request fails and the error message includes the maximum allowed.</p> </note> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/monitoring-cloudwatch.html#gamelift-metrics-fleet">GameLift Metrics for Fleets</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn describe_fleet_utilization(
        &self,
        input: DescribeFleetUtilizationInput,
    ) -> Result<DescribeFleetUtilizationOutput, RusotoError<DescribeFleetUtilizationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DescribeFleetUtilization");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeFleetUtilizationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeFleetUtilizationOutput, _>()
    }

    /// <p><p> <b>This operation is used with the Amazon GameLift FleetIQ solution and game server groups.</b> </p> <p>Retrieves information for a registered game server. Information includes game server status, health check info, and the instance that the game server is running on. </p> <p>To retrieve game server information, specify the game server ID. If successful, the requested game server object is returned. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">GameLift FleetIQ Guide</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>RegisterGameServer</a> </p> </li> <li> <p> <a>ListGameServers</a> </p> </li> <li> <p> <a>ClaimGameServer</a> </p> </li> <li> <p> <a>DescribeGameServer</a> </p> </li> <li> <p> <a>UpdateGameServer</a> </p> </li> <li> <p> <a>DeregisterGameServer</a> </p> </li> </ul></p>
    async fn describe_game_server(
        &self,
        input: DescribeGameServerInput,
    ) -> Result<DescribeGameServerOutput, RusotoError<DescribeGameServerError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DescribeGameServer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeGameServerError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeGameServerOutput, _>()
    }

    /// <p><p> <b>This operation is used with the Amazon GameLift FleetIQ solution and game server groups.</b> </p> <p>Retrieves information on a game server group. This operation returns only properties related to GameLift FleetIQ. To view or update properties for the corresponding Auto Scaling group, such as launch template, auto scaling policies, and maximum/minimum group size, access the Auto Scaling group directly.</p> <p>To get attributes for a game server group, provide a group name or ARN value. If successful, a <a>GameServerGroup</a> object is returned.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">GameLift FleetIQ Guide</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateGameServerGroup</a> </p> </li> <li> <p> <a>ListGameServerGroups</a> </p> </li> <li> <p> <a>DescribeGameServerGroup</a> </p> </li> <li> <p> <a>UpdateGameServerGroup</a> </p> </li> <li> <p> <a>DeleteGameServerGroup</a> </p> </li> <li> <p> <a>ResumeGameServerGroup</a> </p> </li> <li> <p> <a>SuspendGameServerGroup</a> </p> </li> <li> <p> <a>DescribeGameServerInstances</a> </p> </li> </ul></p>
    async fn describe_game_server_group(
        &self,
        input: DescribeGameServerGroupInput,
    ) -> Result<DescribeGameServerGroupOutput, RusotoError<DescribeGameServerGroupError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DescribeGameServerGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeGameServerGroupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeGameServerGroupOutput, _>()
    }

    /// <p><p> <b>This operation is used with the Amazon GameLift FleetIQ solution and game server groups.</b> </p> <p>Retrieves status information about the Amazon EC2 instances associated with a GameLift FleetIQ game server group. Use this operation to detect when instances are active or not available to host new game servers. If you are looking for instance configuration information, call <a>DescribeGameServerGroup</a> or access the corresponding Auto Scaling group properties.</p> <p>To request status for all instances in the game server group, provide a game server group ID only. To request status for specific instances, provide the game server group ID and one or more instance IDs. Use the pagination parameters to retrieve results in sequential segments. If successful, a collection of <code>GameServerInstance</code> objects is returned. </p> <p>This operation is not designed to be called with every game server claim request; this practice can cause you to exceed your API limit, which results in errors. Instead, as a best practice, cache the results and refresh your cache no more than once every 10 seconds.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">GameLift FleetIQ Guide</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateGameServerGroup</a> </p> </li> <li> <p> <a>ListGameServerGroups</a> </p> </li> <li> <p> <a>DescribeGameServerGroup</a> </p> </li> <li> <p> <a>UpdateGameServerGroup</a> </p> </li> <li> <p> <a>DeleteGameServerGroup</a> </p> </li> <li> <p> <a>ResumeGameServerGroup</a> </p> </li> <li> <p> <a>SuspendGameServerGroup</a> </p> </li> <li> <p> <a>DescribeGameServerInstances</a> </p> </li> </ul></p>
    async fn describe_game_server_instances(
        &self,
        input: DescribeGameServerInstancesInput,
    ) -> Result<DescribeGameServerInstancesOutput, RusotoError<DescribeGameServerInstancesError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DescribeGameServerInstances");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeGameServerInstancesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeGameServerInstancesOutput, _>()
    }

    /// <p><p>Retrieves properties, including the protection policy in force, for one or more game sessions. This operation can be used in several ways: (1) provide a <code>GameSessionId</code> or <code>GameSessionArn</code> to request details for a specific game session; (2) provide either a <code>FleetId</code> or an <code>AliasId</code> to request properties for all game sessions running on a fleet. </p> <p>To get game session record(s), specify just one of the following: game session ID, fleet ID, or alias ID. You can filter this request by game session status. Use the pagination parameters to retrieve results as a set of sequential pages. If successful, a <a>GameSessionDetail</a> object is returned for each session matching the request.</p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul></p>
    async fn describe_game_session_details(
        &self,
        input: DescribeGameSessionDetailsInput,
    ) -> Result<DescribeGameSessionDetailsOutput, RusotoError<DescribeGameSessionDetailsError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DescribeGameSessionDetails");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeGameSessionDetailsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeGameSessionDetailsOutput, _>()
    }

    /// <p><p>Retrieves properties and current status of a game session placement request. To get game session placement details, specify the placement ID. If successful, a <a>GameSessionPlacement</a> object is returned.</p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul></p>
    async fn describe_game_session_placement(
        &self,
        input: DescribeGameSessionPlacementInput,
    ) -> Result<DescribeGameSessionPlacementOutput, RusotoError<DescribeGameSessionPlacementError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DescribeGameSessionPlacement");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeGameSessionPlacementError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeGameSessionPlacementOutput, _>()
    }

    /// <p><p>Retrieves the properties for one or more game session queues. When requesting multiple queues, use the pagination parameters to retrieve results as a set of sequential pages. If successful, a <a>GameSessionQueue</a> object is returned for each requested queue. When specifying a list of queues, objects are returned only for queues that currently exist in the Region.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/queues-console.html"> View Your Queues</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateGameSessionQueue</a> </p> </li> <li> <p> <a>DescribeGameSessionQueues</a> </p> </li> <li> <p> <a>UpdateGameSessionQueue</a> </p> </li> <li> <p> <a>DeleteGameSessionQueue</a> </p> </li> </ul></p>
    async fn describe_game_session_queues(
        &self,
        input: DescribeGameSessionQueuesInput,
    ) -> Result<DescribeGameSessionQueuesOutput, RusotoError<DescribeGameSessionQueuesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DescribeGameSessionQueues");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeGameSessionQueuesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeGameSessionQueuesOutput, _>()
    }

    /// <p><p>Retrieves a set of one or more game sessions. Request a specific game session or request all game sessions on a fleet. Alternatively, use <a>SearchGameSessions</a> to request a set of active game sessions that are filtered by certain criteria. To retrieve protection policy settings for game sessions, use <a>DescribeGameSessionDetails</a>.</p> <p>To get game sessions, specify one of the following: game session ID, fleet ID, or alias ID. You can filter this request by game session status. Use the pagination parameters to retrieve results as a set of sequential pages. If successful, a <a>GameSession</a> object is returned for each game session matching the request.</p> <p> <i>Available in Amazon GameLift Local.</i> </p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul></p>
    async fn describe_game_sessions(
        &self,
        input: DescribeGameSessionsInput,
    ) -> Result<DescribeGameSessionsOutput, RusotoError<DescribeGameSessionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DescribeGameSessions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeGameSessionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeGameSessionsOutput, _>()
    }

    /// <p><p>Retrieves information about a fleet&#39;s instances, including instance IDs. Use this operation to get details on all instances in the fleet or get details on one specific instance.</p> <p>To get a specific instance, specify fleet ID and instance ID. To get all instances in a fleet, specify a fleet ID only. Use the pagination parameters to retrieve results as a set of sequential pages. If successful, an <a>Instance</a> object is returned for each result.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-remote-access.html">Remotely Access Fleet Instances</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-creating-debug.html">Debug Fleet Issues</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>DescribeInstances</a> </p> </li> <li> <p> <a>GetInstanceAccess</a> </p> </li> </ul></p>
    async fn describe_instances(
        &self,
        input: DescribeInstancesInput,
    ) -> Result<DescribeInstancesOutput, RusotoError<DescribeInstancesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DescribeInstances");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeInstancesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeInstancesOutput, _>()
    }

    /// <p><p>Retrieves one or more matchmaking tickets. Use this operation to retrieve ticket information, including--after a successful match is made--connection information for the resulting new game session. </p> <p>To request matchmaking tickets, provide a list of up to 10 ticket IDs. If the request is successful, a ticket object is returned for each requested ID that currently exists.</p> <p>This operation is not designed to be continually called to track matchmaking ticket status. This practice can cause you to exceed your API limit, which results in errors. Instead, as a best practice, set up an Amazon Simple Notification Service (SNS) to receive notifications, and provide the topic ARN in the matchmaking configuration. Continuously poling ticket status with <a>DescribeMatchmaking</a> should only be used for games in development with low matchmaking usage.</p> <p/> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-client.html"> Add FlexMatch to a Game Client</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-notification.html"> Set Up FlexMatch Event Notification</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>StartMatchmaking</a> </p> </li> <li> <p> <a>DescribeMatchmaking</a> </p> </li> <li> <p> <a>StopMatchmaking</a> </p> </li> <li> <p> <a>AcceptMatch</a> </p> </li> <li> <p> <a>StartMatchBackfill</a> </p> </li> </ul></p>
    async fn describe_matchmaking(
        &self,
        input: DescribeMatchmakingInput,
    ) -> Result<DescribeMatchmakingOutput, RusotoError<DescribeMatchmakingError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DescribeMatchmaking");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeMatchmakingError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeMatchmakingOutput, _>()
    }

    /// <p><p>Retrieves the details of FlexMatch matchmaking configurations. </p> <p>This operation offers the following options: (1) retrieve all matchmaking configurations, (2) retrieve configurations for a specified list, or (3) retrieve all configurations that use a specified rule set name. When requesting multiple items, use the pagination parameters to retrieve results as a set of sequential pages. </p> <p>If successful, a configuration is returned for each requested name. When specifying a list of names, only configurations that currently exist are returned. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/matchmaker-build.html"> Setting Up FlexMatch Matchmakers</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DeleteMatchmakingRuleSet</a> </p> </li> </ul></p>
    async fn describe_matchmaking_configurations(
        &self,
        input: DescribeMatchmakingConfigurationsInput,
    ) -> Result<
        DescribeMatchmakingConfigurationsOutput,
        RusotoError<DescribeMatchmakingConfigurationsError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DescribeMatchmakingConfigurations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                DescribeMatchmakingConfigurationsError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeMatchmakingConfigurationsOutput, _>()
    }

    /// <p><p>Retrieves the details for FlexMatch matchmaking rule sets. You can request all existing rule sets for the Region, or provide a list of one or more rule set names. When requesting multiple items, use the pagination parameters to retrieve results as a set of sequential pages. If successful, a rule set is returned for each requested name. </p> <p> <b>Learn more</b> </p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-rulesets.html">Build a Rule Set</a> </p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DeleteMatchmakingRuleSet</a> </p> </li> </ul></p>
    async fn describe_matchmaking_rule_sets(
        &self,
        input: DescribeMatchmakingRuleSetsInput,
    ) -> Result<DescribeMatchmakingRuleSetsOutput, RusotoError<DescribeMatchmakingRuleSetsError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DescribeMatchmakingRuleSets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeMatchmakingRuleSetsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeMatchmakingRuleSetsOutput, _>()
    }

    /// <p><p>Retrieves properties for one or more player sessions. This operation can be used in several ways: (1) provide a <code>PlayerSessionId</code> to request properties for a specific player session; (2) provide a <code>GameSessionId</code> to request properties for all player sessions in the specified game session; (3) provide a <code>PlayerId</code> to request properties for all player sessions of a specified player. </p> <p>To get game session record(s), specify only one of the following: a player session ID, a game session ID, or a player ID. You can filter this request by player session status. Use the pagination parameters to retrieve results as a set of sequential pages. If successful, a <a>PlayerSession</a> object is returned for each session matching the request.</p> <p> <i>Available in Amazon GameLift Local.</i> </p> <ul> <li> <p> <a>CreatePlayerSession</a> </p> </li> <li> <p> <a>CreatePlayerSessions</a> </p> </li> <li> <p> <a>DescribePlayerSessions</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul></p>
    async fn describe_player_sessions(
        &self,
        input: DescribePlayerSessionsInput,
    ) -> Result<DescribePlayerSessionsOutput, RusotoError<DescribePlayerSessionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DescribePlayerSessions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribePlayerSessionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribePlayerSessionsOutput, _>()
    }

    /// <p><p>Retrieves a fleet&#39;s runtime configuration settings. The runtime configuration tells Amazon GameLift which server processes to run (and how) on each instance in the fleet.</p> <p>To get a runtime configuration, specify the fleet&#39;s unique identifier. If successful, a <a>RuntimeConfiguration</a> object is returned for the requested fleet. If the requested fleet has been deleted, the result set is empty.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-multiprocess.html">Running Multiple Processes on a Fleet</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p>Describe fleets:</p> <ul> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>DescribeFleetPortSettings</a> </p> </li> <li> <p> <a>DescribeFleetUtilization</a> </p> </li> <li> <p> <a>DescribeRuntimeConfiguration</a> </p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> <li> <p> <a>DescribeFleetEvents</a> </p> </li> </ul> </li> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn describe_runtime_configuration(
        &self,
        input: DescribeRuntimeConfigurationInput,
    ) -> Result<DescribeRuntimeConfigurationOutput, RusotoError<DescribeRuntimeConfigurationError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DescribeRuntimeConfiguration");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeRuntimeConfigurationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeRuntimeConfigurationOutput, _>()
    }

    /// <p><p>Retrieves all scaling policies applied to a fleet.</p> <p>To get a fleet&#39;s scaling policies, specify the fleet ID. You can filter this request by policy status, such as to retrieve only active scaling policies. Use the pagination parameters to retrieve results as a set of sequential pages. If successful, set of <a>ScalingPolicy</a> objects is returned for the fleet.</p> <p>A fleet may have all of its scaling policies suspended (<a>StopFleetActions</a>). This operation does not affect the status of the scaling policies, which remains ACTIVE. To see whether a fleet&#39;s scaling policies are in force or suspended, call <a>DescribeFleetAttributes</a> and check the stopped actions.</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> <li> <p>Manage scaling policies:</p> <ul> <li> <p> <a>PutScalingPolicy</a> (auto-scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (auto-scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (auto-scaling)</p> </li> </ul> </li> <li> <p>Manage fleet actions:</p> <ul> <li> <p> <a>StartFleetActions</a> </p> </li> <li> <p> <a>StopFleetActions</a> </p> </li> </ul> </li> </ul></p>
    async fn describe_scaling_policies(
        &self,
        input: DescribeScalingPoliciesInput,
    ) -> Result<DescribeScalingPoliciesOutput, RusotoError<DescribeScalingPoliciesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DescribeScalingPolicies");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeScalingPoliciesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeScalingPoliciesOutput, _>()
    }

    /// <p><p>Retrieves properties for a Realtime script. </p> <p>To request a script record, specify the script ID. If successful, an object containing the script properties is returned.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/realtime-intro.html">Amazon GameLift Realtime Servers</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateScript</a> </p> </li> <li> <p> <a>ListScripts</a> </p> </li> <li> <p> <a>DescribeScript</a> </p> </li> <li> <p> <a>UpdateScript</a> </p> </li> <li> <p> <a>DeleteScript</a> </p> </li> </ul></p>
    async fn describe_script(
        &self,
        input: DescribeScriptInput,
    ) -> Result<DescribeScriptOutput, RusotoError<DescribeScriptError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DescribeScript");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeScriptError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeScriptOutput, _>()
    }

    /// <p><p>Retrieves valid VPC peering authorizations that are pending for the AWS account. This operation returns all VPC peering authorizations and requests for peering. This includes those initiated and received by this account. </p> <ul> <li> <p> <a>CreateVpcPeeringAuthorization</a> </p> </li> <li> <p> <a>DescribeVpcPeeringAuthorizations</a> </p> </li> <li> <p> <a>DeleteVpcPeeringAuthorization</a> </p> </li> <li> <p> <a>CreateVpcPeeringConnection</a> </p> </li> <li> <p> <a>DescribeVpcPeeringConnections</a> </p> </li> <li> <p> <a>DeleteVpcPeeringConnection</a> </p> </li> </ul></p>
    async fn describe_vpc_peering_authorizations(
        &self,
    ) -> Result<
        DescribeVpcPeeringAuthorizationsOutput,
        RusotoError<DescribeVpcPeeringAuthorizationsError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DescribeVpcPeeringAuthorizations");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(
                request,
                DescribeVpcPeeringAuthorizationsError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeVpcPeeringAuthorizationsOutput, _>()
    }

    /// <p><p>Retrieves information on VPC peering connections. Use this operation to get peering information for all fleets or for one specific fleet ID. </p> <p>To retrieve connection information, call this operation from the AWS account that is used to manage the Amazon GameLift fleets. Specify a fleet ID or leave the parameter empty to retrieve all connection records. If successful, the retrieved information includes both active and pending connections. Active connections identify the IpV4 CIDR block that the VPC uses to connect. </p> <ul> <li> <p> <a>CreateVpcPeeringAuthorization</a> </p> </li> <li> <p> <a>DescribeVpcPeeringAuthorizations</a> </p> </li> <li> <p> <a>DeleteVpcPeeringAuthorization</a> </p> </li> <li> <p> <a>CreateVpcPeeringConnection</a> </p> </li> <li> <p> <a>DescribeVpcPeeringConnections</a> </p> </li> <li> <p> <a>DeleteVpcPeeringConnection</a> </p> </li> </ul></p>
    async fn describe_vpc_peering_connections(
        &self,
        input: DescribeVpcPeeringConnectionsInput,
    ) -> Result<DescribeVpcPeeringConnectionsOutput, RusotoError<DescribeVpcPeeringConnectionsError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.DescribeVpcPeeringConnections");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeVpcPeeringConnectionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeVpcPeeringConnectionsOutput, _>()
    }

    /// <p><p>Retrieves the location of stored game session logs for a specified game session. When a game session is terminated, Amazon GameLift automatically stores the logs in Amazon S3 and retains them for 14 days. Use this URL to download the logs.</p> <note> <p>See the <a href="https://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_gamelift">AWS Service Limits</a> page for maximum log file sizes. Log files that exceed this limit are not saved.</p> </note> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul></p>
    async fn get_game_session_log_url(
        &self,
        input: GetGameSessionLogUrlInput,
    ) -> Result<GetGameSessionLogUrlOutput, RusotoError<GetGameSessionLogUrlError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.GetGameSessionLogUrl");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetGameSessionLogUrlError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetGameSessionLogUrlOutput, _>()
    }

    /// <p><p>Requests remote access to a fleet instance. Remote access is useful for debugging, gathering benchmarking data, or observing activity in real time. </p> <p>To remotely access an instance, you need credentials that match the operating system of the instance. For a Windows instance, Amazon GameLift returns a user name and password as strings for use with a Windows Remote Desktop client. For a Linux instance, Amazon GameLift returns a user name and RSA private key, also as strings, for use with an SSH client. The private key must be saved in the proper format to a <code>.pem</code> file before using. If you&#39;re making this request using the AWS CLI, saving the secret can be handled as part of the GetInstanceAccess request, as shown in one of the examples for this operation. </p> <p>To request access to a specific instance, specify the IDs of both the instance and the fleet it belongs to. You can retrieve a fleet&#39;s instance IDs by calling <a>DescribeInstances</a>. If successful, an <a>InstanceAccess</a> object is returned that contains the instance&#39;s IP address and a set of credentials.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-remote-access.html">Remotely Access Fleet Instances</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-creating-debug.html">Debug Fleet Issues</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>DescribeInstances</a> </p> </li> <li> <p> <a>GetInstanceAccess</a> </p> </li> </ul></p>
    async fn get_instance_access(
        &self,
        input: GetInstanceAccessInput,
    ) -> Result<GetInstanceAccessOutput, RusotoError<GetInstanceAccessError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.GetInstanceAccess");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetInstanceAccessError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetInstanceAccessOutput, _>()
    }

    /// <p><p>Retrieves all aliases for this AWS account. You can filter the result set by alias name and/or routing strategy type. Use the pagination parameters to retrieve results in sequential pages.</p> <note> <p>Returned aliases are not listed in any particular order.</p> </note> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>DescribeAlias</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ResolveAlias</a> </p> </li> </ul></p>
    async fn list_aliases(
        &self,
        input: ListAliasesInput,
    ) -> Result<ListAliasesOutput, RusotoError<ListAliasesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.ListAliases");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListAliasesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListAliasesOutput, _>()
    }

    /// <p><p>Retrieves build resources for all builds associated with the AWS account in use. You can limit results to builds that are in a specific status by using the <code>Status</code> parameter. Use the pagination parameters to retrieve results in a set of sequential pages. </p> <note> <p>Build resources are not listed in any particular order.</p> </note> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-build-intro.html"> Upload a Custom Server Build</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateBuild</a> </p> </li> <li> <p> <a>ListBuilds</a> </p> </li> <li> <p> <a>DescribeBuild</a> </p> </li> <li> <p> <a>UpdateBuild</a> </p> </li> <li> <p> <a>DeleteBuild</a> </p> </li> </ul></p>
    async fn list_builds(
        &self,
        input: ListBuildsInput,
    ) -> Result<ListBuildsOutput, RusotoError<ListBuildsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.ListBuilds");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListBuildsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListBuildsOutput, _>()
    }

    /// <p><p>Retrieves a collection of fleet resources for this AWS account. You can filter the result set to find only those fleets that are deployed with a specific build or script. Use the pagination parameters to retrieve results in sequential pages.</p> <note> <p>Fleet resources are not listed in a particular order.</p> </note> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn list_fleets(
        &self,
        input: ListFleetsInput,
    ) -> Result<ListFleetsOutput, RusotoError<ListFleetsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.ListFleets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListFleetsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListFleetsOutput, _>()
    }

    /// <p><p> <b>This operation is used with the Amazon GameLift FleetIQ solution and game server groups.</b> </p> <p>Retrieves information on all game servers groups that exist in the current AWS account for the selected Region. Use the pagination parameters to retrieve results in a set of sequential segments. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">GameLift FleetIQ Guide</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateGameServerGroup</a> </p> </li> <li> <p> <a>ListGameServerGroups</a> </p> </li> <li> <p> <a>DescribeGameServerGroup</a> </p> </li> <li> <p> <a>UpdateGameServerGroup</a> </p> </li> <li> <p> <a>DeleteGameServerGroup</a> </p> </li> <li> <p> <a>ResumeGameServerGroup</a> </p> </li> <li> <p> <a>SuspendGameServerGroup</a> </p> </li> <li> <p> <a>DescribeGameServerInstances</a> </p> </li> </ul></p>
    async fn list_game_server_groups(
        &self,
        input: ListGameServerGroupsInput,
    ) -> Result<ListGameServerGroupsOutput, RusotoError<ListGameServerGroupsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.ListGameServerGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListGameServerGroupsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListGameServerGroupsOutput, _>()
    }

    /// <p><p> <b>This operation is used with the Amazon GameLift FleetIQ solution and game server groups.</b> </p> <p>Retrieves information on all game servers that are currently active in a specified game server group. You can opt to sort the list by game server age. Use the pagination parameters to retrieve results in a set of sequential segments. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">GameLift FleetIQ Guide</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>RegisterGameServer</a> </p> </li> <li> <p> <a>ListGameServers</a> </p> </li> <li> <p> <a>ClaimGameServer</a> </p> </li> <li> <p> <a>DescribeGameServer</a> </p> </li> <li> <p> <a>UpdateGameServer</a> </p> </li> <li> <p> <a>DeregisterGameServer</a> </p> </li> </ul></p>
    async fn list_game_servers(
        &self,
        input: ListGameServersInput,
    ) -> Result<ListGameServersOutput, RusotoError<ListGameServersError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.ListGameServers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListGameServersError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListGameServersOutput, _>()
    }

    /// <p><p>Retrieves script records for all Realtime scripts that are associated with the AWS account in use. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/realtime-intro.html">Amazon GameLift Realtime Servers</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateScript</a> </p> </li> <li> <p> <a>ListScripts</a> </p> </li> <li> <p> <a>DescribeScript</a> </p> </li> <li> <p> <a>UpdateScript</a> </p> </li> <li> <p> <a>DeleteScript</a> </p> </li> </ul></p>
    async fn list_scripts(
        &self,
        input: ListScriptsInput,
    ) -> Result<ListScriptsOutput, RusotoError<ListScriptsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.ListScripts");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListScriptsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListScriptsOutput, _>()
    }

    /// <p><p> Retrieves all tags that are assigned to a GameLift resource. Resource tags are used to organize AWS resources for a range of purposes. This operation handles the permissions necessary to manage tags for the following GameLift resource types:</p> <ul> <li> <p>Build</p> </li> <li> <p>Script</p> </li> <li> <p>Fleet</p> </li> <li> <p>Alias</p> </li> <li> <p>GameSessionQueue</p> </li> <li> <p>MatchmakingConfiguration</p> </li> <li> <p>MatchmakingRuleSet</p> </li> </ul> <p>To list tags for a resource, specify the unique ARN value for the resource.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS Resources</a> in the <i>AWS General Reference</i> </p> <p> <a href="http://aws.amazon.com/answers/account-management/aws-tagging-strategies/"> AWS Tagging Strategies</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>TagResource</a> </p> </li> <li> <p> <a>UntagResource</a> </p> </li> <li> <p> <a>ListTagsForResource</a> </p> </li> </ul></p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.ListTagsForResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTagsForResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTagsForResourceResponse, _>()
    }

    /// <p><p>Creates or updates a scaling policy for a fleet. Scaling policies are used to automatically scale a fleet&#39;s hosting capacity to meet player demand. An active scaling policy instructs Amazon GameLift to track a fleet metric and automatically change the fleet&#39;s capacity when a certain threshold is reached. There are two types of scaling policies: target-based and rule-based. Use a target-based policy to quickly and efficiently manage fleet scaling; this option is the most commonly used. Use rule-based policies when you need to exert fine-grained control over auto-scaling. </p> <p>Fleets can have multiple scaling policies of each type in force at the same time; you can have one target-based policy, one or multiple rule-based scaling policies, or both. We recommend caution, however, because multiple auto-scaling policies can have unintended consequences.</p> <p>You can temporarily suspend all scaling policies for a fleet by calling <a>StopFleetActions</a> with the fleet action AUTO_SCALING. To resume scaling policies, call <a>StartFleetActions</a> with the same fleet action. To stop just one scaling policy--or to permanently remove it, you must delete the policy with <a>DeleteScalingPolicy</a>.</p> <p>Learn more about how to work with auto-scaling in <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-autoscaling.html">Set Up Fleet Automatic Scaling</a>.</p> <p> <b>Target-based policy</b> </p> <p>A target-based policy tracks a single metric: PercentAvailableGameSessions. This metric tells us how much of a fleet&#39;s hosting capacity is ready to host game sessions but is not currently in use. This is the fleet&#39;s buffer; it measures the additional player demand that the fleet could handle at current capacity. With a target-based policy, you set your ideal buffer size and leave it to Amazon GameLift to take whatever action is needed to maintain that target. </p> <p>For example, you might choose to maintain a 10% buffer for a fleet that has the capacity to host 100 simultaneous game sessions. This policy tells Amazon GameLift to take action whenever the fleet&#39;s available capacity falls below or rises above 10 game sessions. Amazon GameLift will start new instances or stop unused instances in order to return to the 10% buffer. </p> <p>To create or update a target-based policy, specify a fleet ID and name, and set the policy type to &quot;TargetBased&quot;. Specify the metric to track (PercentAvailableGameSessions) and reference a <a>TargetConfiguration</a> object with your desired buffer value. Exclude all other parameters. On a successful request, the policy name is returned. The scaling policy is automatically in force as soon as it&#39;s successfully created. If the fleet&#39;s auto-scaling actions are temporarily suspended, the new policy will be in force once the fleet actions are restarted.</p> <p> <b>Rule-based policy</b> </p> <p>A rule-based policy tracks specified fleet metric, sets a threshold value, and specifies the type of action to initiate when triggered. With a rule-based policy, you can select from several available fleet metrics. Each policy specifies whether to scale up or scale down (and by how much), so you need one policy for each type of action. </p> <p>For example, a policy may make the following statement: &quot;If the percentage of idle instances is greater than 20% for more than 15 minutes, then reduce the fleet capacity by 10%.&quot;</p> <p>A policy&#39;s rule statement has the following structure:</p> <p>If <code>[MetricName]</code> is <code>[ComparisonOperator]</code> <code>[Threshold]</code> for <code>[EvaluationPeriods]</code> minutes, then <code>[ScalingAdjustmentType]</code> to/by <code>[ScalingAdjustment]</code>.</p> <p>To implement the example, the rule statement would look like this:</p> <p>If <code>[PercentIdleInstances]</code> is <code>[GreaterThanThreshold]</code> <code>[20]</code> for <code>[15]</code> minutes, then <code>[PercentChangeInCapacity]</code> to/by <code>[10]</code>.</p> <p>To create or update a scaling policy, specify a unique combination of name and fleet ID, and set the policy type to &quot;RuleBased&quot;. Specify the parameter values for a policy rule statement. On a successful request, the policy name is returned. Scaling policies are automatically in force as soon as they&#39;re successfully created. If the fleet&#39;s auto-scaling actions are temporarily suspended, the new policy will be in force once the fleet actions are restarted.</p> <ul> <li> <p> <a>DescribeFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>DescribeEC2InstanceLimits</a> </p> </li> <li> <p>Manage scaling policies:</p> <ul> <li> <p> <a>PutScalingPolicy</a> (auto-scaling)</p> </li> <li> <p> <a>DescribeScalingPolicies</a> (auto-scaling)</p> </li> <li> <p> <a>DeleteScalingPolicy</a> (auto-scaling)</p> </li> </ul> </li> <li> <p>Manage fleet actions:</p> <ul> <li> <p> <a>StartFleetActions</a> </p> </li> <li> <p> <a>StopFleetActions</a> </p> </li> </ul> </li> </ul></p>
    async fn put_scaling_policy(
        &self,
        input: PutScalingPolicyInput,
    ) -> Result<PutScalingPolicyOutput, RusotoError<PutScalingPolicyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.PutScalingPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutScalingPolicyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<PutScalingPolicyOutput, _>()
    }

    /// <p><p> <b>This operation is used with the Amazon GameLift FleetIQ solution and game server groups.</b> </p> <p>Creates a new game server resource and notifies GameLift FleetIQ that the game server is ready to host gameplay and players. This operation is called by a game server process that is running on an instance in a game server group. Registering game servers enables GameLift FleetIQ to track available game servers and enables game clients and services to claim a game server for a new game session. </p> <p>To register a game server, identify the game server group and instance where the game server is running, and provide a unique identifier for the game server. You can also include connection and game server data. When a game client or service requests a game server by calling <a>ClaimGameServer</a>, this information is returned in the response.</p> <p>Once a game server is successfully registered, it is put in status <code>AVAILABLE</code>. A request to register a game server may fail if the instance it is running on is in the process of shutting down as part of instance balancing or scale-down activity. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">GameLift FleetIQ Guide</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>RegisterGameServer</a> </p> </li> <li> <p> <a>ListGameServers</a> </p> </li> <li> <p> <a>ClaimGameServer</a> </p> </li> <li> <p> <a>DescribeGameServer</a> </p> </li> <li> <p> <a>UpdateGameServer</a> </p> </li> <li> <p> <a>DeregisterGameServer</a> </p> </li> </ul></p>
    async fn register_game_server(
        &self,
        input: RegisterGameServerInput,
    ) -> Result<RegisterGameServerOutput, RusotoError<RegisterGameServerError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.RegisterGameServer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RegisterGameServerError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<RegisterGameServerOutput, _>()
    }

    /// <p><p>Retrieves a fresh set of credentials for use when uploading a new set of game build files to Amazon GameLift&#39;s Amazon S3. This is done as part of the build creation process; see <a>CreateBuild</a>.</p> <p>To request new credentials, specify the build ID as returned with an initial <code>CreateBuild</code> request. If successful, a new set of credentials are returned, along with the S3 storage location associated with the build ID.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-build-cli-uploading.html#gamelift-build-cli-uploading-create-build"> Create a Build with Files in S3</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateBuild</a> </p> </li> <li> <p> <a>ListBuilds</a> </p> </li> <li> <p> <a>DescribeBuild</a> </p> </li> <li> <p> <a>UpdateBuild</a> </p> </li> <li> <p> <a>DeleteBuild</a> </p> </li> </ul></p>
    async fn request_upload_credentials(
        &self,
        input: RequestUploadCredentialsInput,
    ) -> Result<RequestUploadCredentialsOutput, RusotoError<RequestUploadCredentialsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.RequestUploadCredentials");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RequestUploadCredentialsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<RequestUploadCredentialsOutput, _>()
    }

    /// <p><p>Retrieves the fleet ID that an alias is currently pointing to.</p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>DescribeAlias</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ResolveAlias</a> </p> </li> </ul></p>
    async fn resolve_alias(
        &self,
        input: ResolveAliasInput,
    ) -> Result<ResolveAliasOutput, RusotoError<ResolveAliasError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.ResolveAlias");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ResolveAliasError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ResolveAliasOutput, _>()
    }

    /// <p><p> <b>This operation is used with the Amazon GameLift FleetIQ solution and game server groups.</b> </p> <p>Reinstates activity on a game server group after it has been suspended. A game server group might be suspended by the<a>SuspendGameServerGroup</a> operation, or it might be suspended involuntarily due to a configuration problem. In the second case, you can manually resume activity on the group once the configuration problem has been resolved. Refer to the game server group status and status reason for more information on why group activity is suspended.</p> <p>To resume activity, specify a game server group ARN and the type of activity to be resumed. If successful, a <a>GameServerGroup</a> object is returned showing that the resumed activity is no longer listed in <code>SuspendedActions</code>. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">GameLift FleetIQ Guide</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateGameServerGroup</a> </p> </li> <li> <p> <a>ListGameServerGroups</a> </p> </li> <li> <p> <a>DescribeGameServerGroup</a> </p> </li> <li> <p> <a>UpdateGameServerGroup</a> </p> </li> <li> <p> <a>DeleteGameServerGroup</a> </p> </li> <li> <p> <a>ResumeGameServerGroup</a> </p> </li> <li> <p> <a>SuspendGameServerGroup</a> </p> </li> <li> <p> <a>DescribeGameServerInstances</a> </p> </li> </ul></p>
    async fn resume_game_server_group(
        &self,
        input: ResumeGameServerGroupInput,
    ) -> Result<ResumeGameServerGroupOutput, RusotoError<ResumeGameServerGroupError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.ResumeGameServerGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ResumeGameServerGroupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ResumeGameServerGroupOutput, _>()
    }

    /// <p><p>Retrieves all active game sessions that match a set of search criteria and sorts them in a specified order. You can search or sort by the following game session attributes:</p> <ul> <li> <p> <b>gameSessionId</b> -- A unique identifier for the game session. You can use either a <code>GameSessionId</code> or <code>GameSessionArn</code> value. </p> </li> <li> <p> <b>gameSessionName</b> -- Name assigned to a game session. This value is set when requesting a new game session with <a>CreateGameSession</a> or updating with <a>UpdateGameSession</a>. Game session names do not need to be unique to a game session.</p> </li> <li> <p> <b>gameSessionProperties</b> -- Custom data defined in a game session&#39;s <code>GameProperty</code> parameter. <code>GameProperty</code> values are stored as key:value pairs; the filter expression must indicate the key and a string to search the data values for. For example, to search for game sessions with custom data containing the key:value pair &quot;gameMode:brawl&quot;, specify the following: <code>gameSessionProperties.gameMode = &quot;brawl&quot;</code>. All custom data values are searched as strings.</p> </li> <li> <p> <b>maximumSessions</b> -- Maximum number of player sessions allowed for a game session. This value is set when requesting a new game session with <a>CreateGameSession</a> or updating with <a>UpdateGameSession</a>.</p> </li> <li> <p> <b>creationTimeMillis</b> -- Value indicating when a game session was created. It is expressed in Unix time as milliseconds.</p> </li> <li> <p> <b>playerSessionCount</b> -- Number of players currently connected to a game session. This value changes rapidly as players join the session or drop out.</p> </li> <li> <p> <b>hasAvailablePlayerSessions</b> -- Boolean value indicating whether a game session has reached its maximum number of players. It is highly recommended that all search requests include this filter attribute to optimize search performance and return only sessions that players can join. </p> </li> </ul> <note> <p>Returned values for <code>playerSessionCount</code> and <code>hasAvailablePlayerSessions</code> change quickly as players join sessions and others drop out. Results should be considered a snapshot in time. Be sure to refresh search results often, and handle sessions that fill up before a player can join. </p> </note> <p>To search or sort, specify either a fleet ID or an alias ID, and provide a search filter expression, a sort expression, or both. If successful, a collection of <a>GameSession</a> objects matching the request is returned. Use the pagination parameters to retrieve results as a set of sequential pages. </p> <p>You can search for game sessions one fleet at a time only. To find game sessions across multiple fleets, you must search each fleet separately and combine the results. This search feature finds only game sessions that are in <code>ACTIVE</code> status. To locate games in statuses other than active, use <a>DescribeGameSessionDetails</a>.</p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul></p>
    async fn search_game_sessions(
        &self,
        input: SearchGameSessionsInput,
    ) -> Result<SearchGameSessionsOutput, RusotoError<SearchGameSessionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.SearchGameSessions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, SearchGameSessionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<SearchGameSessionsOutput, _>()
    }

    /// <p><p>Resumes activity on a fleet that was suspended with <a>StopFleetActions</a>. Currently, this operation is used to restart a fleet&#39;s auto-scaling activity. </p> <p>To start fleet actions, specify the fleet ID and the type of actions to restart. When auto-scaling fleet actions are restarted, Amazon GameLift once again initiates scaling events as triggered by the fleet&#39;s scaling policies. If actions on the fleet were never stopped, this operation will have no effect. You can view a fleet&#39;s stopped actions using <a>DescribeFleetAttributes</a>.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn start_fleet_actions(
        &self,
        input: StartFleetActionsInput,
    ) -> Result<StartFleetActionsOutput, RusotoError<StartFleetActionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.StartFleetActions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StartFleetActionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<StartFleetActionsOutput, _>()
    }

    /// <p><p>Places a request for a new game session in a queue (see <a>CreateGameSessionQueue</a>). When processing a placement request, Amazon GameLift searches for available resources on the queue&#39;s destinations, scanning each until it finds resources or the placement request times out.</p> <p>A game session placement request can also request player sessions. When a new game session is successfully created, Amazon GameLift creates a player session for each player included in the request.</p> <p>When placing a game session, by default Amazon GameLift tries each fleet in the order they are listed in the queue configuration. Ideally, a queue&#39;s destinations are listed in preference order.</p> <p>Alternatively, when requesting a game session with players, you can also provide latency data for each player in relevant Regions. Latency data indicates the performance lag a player experiences when connected to a fleet in the Region. Amazon GameLift uses latency data to reorder the list of destinations to place the game session in a Region with minimal lag. If latency data is provided for multiple players, Amazon GameLift calculates each Region&#39;s average lag for all players and reorders to get the best game play across all players. </p> <p>To place a new game session request, specify the following:</p> <ul> <li> <p>The queue name and a set of game session properties and settings</p> </li> <li> <p>A unique ID (such as a UUID) for the placement. You use this ID to track the status of the placement request</p> </li> <li> <p>(Optional) A set of player data and a unique player ID for each player that you are joining to the new game session (player data is optional, but if you include it, you must also provide a unique ID for each player)</p> </li> <li> <p>Latency data for all players (if you want to optimize game play for the players)</p> </li> </ul> <p>If successful, a new game session placement is created.</p> <p>To track the status of a placement request, call <a>DescribeGameSessionPlacement</a> and check the request&#39;s status. If the status is <code>FULFILLED</code>, a new game session has been created and a game session ARN and Region are referenced. If the placement request times out, you can resubmit the request or retry it with a different queue. </p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul></p>
    async fn start_game_session_placement(
        &self,
        input: StartGameSessionPlacementInput,
    ) -> Result<StartGameSessionPlacementOutput, RusotoError<StartGameSessionPlacementError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.StartGameSessionPlacement");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StartGameSessionPlacementError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<StartGameSessionPlacementOutput, _>()
    }

    /// <p><p>Finds new players to fill open slots in an existing game session. This operation can be used to add players to matched games that start with fewer than the maximum number of players or to replace players when they drop out. By backfilling with the same matchmaker used to create the original match, you ensure that new players meet the match criteria and maintain a consistent experience throughout the game session. You can backfill a match anytime after a game session has been created. </p> <p>To request a match backfill, specify a unique ticket ID, the existing game session&#39;s ARN, a matchmaking configuration, and a set of data that describes all current players in the game session. If successful, a match backfill ticket is created and returned with status set to QUEUED. The ticket is placed in the matchmaker&#39;s ticket pool and processed. Track the status of the ticket to respond as needed. </p> <p>The process of finding backfill matches is essentially identical to the initial matchmaking process. The matchmaker searches the pool and groups tickets together to form potential matches, allowing only one backfill ticket per potential match. Once the a match is formed, the matchmaker creates player sessions for the new players. All tickets in the match are updated with the game session&#39;s connection information, and the <a>GameSession</a> object is updated to include matchmaker data on the new players. For more detail on how match backfill requests are processed, see <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/gamelift-match.html"> How Amazon GameLift FlexMatch Works</a>. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-backfill.html"> Backfill Existing Games with FlexMatch</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/gamelift-match.html"> How GameLift FlexMatch Works</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>StartMatchmaking</a> </p> </li> <li> <p> <a>DescribeMatchmaking</a> </p> </li> <li> <p> <a>StopMatchmaking</a> </p> </li> <li> <p> <a>AcceptMatch</a> </p> </li> <li> <p> <a>StartMatchBackfill</a> </p> </li> </ul></p>
    async fn start_match_backfill(
        &self,
        input: StartMatchBackfillInput,
    ) -> Result<StartMatchBackfillOutput, RusotoError<StartMatchBackfillError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.StartMatchBackfill");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StartMatchBackfillError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<StartMatchBackfillOutput, _>()
    }

    /// <p><p>Uses FlexMatch to create a game match for a group of players based on custom matchmaking rules. If you&#39;re also using GameLift hosting, a new game session is started for the matched players. Each matchmaking request identifies one or more players to find a match for, and specifies the type of match to build, including the team configuration and the rules for an acceptable match. When a matchmaking request identifies a group of players who want to play together, FlexMatch finds additional players to fill the match. Match type, rules, and other features are defined in a <code>MatchmakingConfiguration</code>. </p> <p>To start matchmaking, provide a unique ticket ID, specify a matchmaking configuration, and include the players to be matched. For each player, you must also include the player attribute values that are required by the matchmaking configuration (in the rule set). If successful, a matchmaking ticket is returned with status set to <code>QUEUED</code>. </p> <p>Track the status of the ticket to respond as needed. If you&#39;re also using GameLift hosting, a successfully completed ticket contains game session connection information. Ticket status updates are tracked using event notification through Amazon Simple Notification Service (SNS), which is defined in the matchmaking configuration.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-client.html"> Add FlexMatch to a Game Client</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-notification.html"> Set Up FlexMatch Event Notification</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-tasks.html"> FlexMatch Integration Roadmap</a> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/gamelift-match.html"> How GameLift FlexMatch Works</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>StartMatchmaking</a> </p> </li> <li> <p> <a>DescribeMatchmaking</a> </p> </li> <li> <p> <a>StopMatchmaking</a> </p> </li> <li> <p> <a>AcceptMatch</a> </p> </li> <li> <p> <a>StartMatchBackfill</a> </p> </li> </ul></p>
    async fn start_matchmaking(
        &self,
        input: StartMatchmakingInput,
    ) -> Result<StartMatchmakingOutput, RusotoError<StartMatchmakingError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.StartMatchmaking");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StartMatchmakingError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<StartMatchmakingOutput, _>()
    }

    /// <p><p>Suspends activity on a fleet. Currently, this operation is used to stop a fleet&#39;s auto-scaling activity. It is used to temporarily stop triggering scaling events. The policies can be retained and auto-scaling activity can be restarted using <a>StartFleetActions</a>. You can view a fleet&#39;s stopped actions using <a>DescribeFleetAttributes</a>.</p> <p>To stop fleet actions, specify the fleet ID and the type of actions to suspend. When auto-scaling fleet actions are stopped, Amazon GameLift no longer initiates scaling events except in response to manual changes using <a>UpdateFleetCapacity</a>. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn stop_fleet_actions(
        &self,
        input: StopFleetActionsInput,
    ) -> Result<StopFleetActionsOutput, RusotoError<StopFleetActionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.StopFleetActions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StopFleetActionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<StopFleetActionsOutput, _>()
    }

    /// <p><p>Cancels a game session placement that is in <code>PENDING</code> status. To stop a placement, provide the placement ID values. If successful, the placement is moved to <code>CANCELLED</code> status.</p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul></p>
    async fn stop_game_session_placement(
        &self,
        input: StopGameSessionPlacementInput,
    ) -> Result<StopGameSessionPlacementOutput, RusotoError<StopGameSessionPlacementError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.StopGameSessionPlacement");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StopGameSessionPlacementError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<StopGameSessionPlacementOutput, _>()
    }

    /// <p><p>Cancels a matchmaking ticket or match backfill ticket that is currently being processed. To stop the matchmaking operation, specify the ticket ID. If successful, work on the ticket is stopped, and the ticket status is changed to <code>CANCELLED</code>.</p> <p>This call is also used to turn off automatic backfill for an individual game session. This is for game sessions that are created with a matchmaking configuration that has automatic backfill enabled. The ticket ID is included in the <code>MatchmakerData</code> of an updated game session object, which is provided to the game server.</p> <note> <p>If the operation is successful, the service sends back an empty JSON struct with the HTTP 200 response (not an empty HTTP body).</p> </note> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-client.html"> Add FlexMatch to a Game Client</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>StartMatchmaking</a> </p> </li> <li> <p> <a>DescribeMatchmaking</a> </p> </li> <li> <p> <a>StopMatchmaking</a> </p> </li> <li> <p> <a>AcceptMatch</a> </p> </li> <li> <p> <a>StartMatchBackfill</a> </p> </li> </ul></p>
    async fn stop_matchmaking(
        &self,
        input: StopMatchmakingInput,
    ) -> Result<StopMatchmakingOutput, RusotoError<StopMatchmakingError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.StopMatchmaking");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StopMatchmakingError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<StopMatchmakingOutput, _>()
    }

    /// <p><p> <b>This operation is used with the Amazon GameLift FleetIQ solution and game server groups.</b> </p> <p>Temporarily stops activity on a game server group without terminating instances or the game server group. You can restart activity by calling <a>ResumeGameServerGroup</a>. You can suspend the following activity:</p> <ul> <li> <p> <b>Instance type replacement</b> - This activity evaluates the current game hosting viability of all Spot instance types that are defined for the game server group. It updates the Auto Scaling group to remove nonviable Spot Instance types, which have a higher chance of game server interruptions. It then balances capacity across the remaining viable Spot Instance types. When this activity is suspended, the Auto Scaling group continues with its current balance, regardless of viability. Instance protection, utilization metrics, and capacity scaling activities continue to be active. </p> </li> </ul> <p>To suspend activity, specify a game server group ARN and the type of activity to be suspended. If successful, a <a>GameServerGroup</a> object is returned showing that the activity is listed in <code>SuspendedActions</code>.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">GameLift FleetIQ Guide</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateGameServerGroup</a> </p> </li> <li> <p> <a>ListGameServerGroups</a> </p> </li> <li> <p> <a>DescribeGameServerGroup</a> </p> </li> <li> <p> <a>UpdateGameServerGroup</a> </p> </li> <li> <p> <a>DeleteGameServerGroup</a> </p> </li> <li> <p> <a>ResumeGameServerGroup</a> </p> </li> <li> <p> <a>SuspendGameServerGroup</a> </p> </li> <li> <p> <a>DescribeGameServerInstances</a> </p> </li> </ul></p>
    async fn suspend_game_server_group(
        &self,
        input: SuspendGameServerGroupInput,
    ) -> Result<SuspendGameServerGroupOutput, RusotoError<SuspendGameServerGroupError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.SuspendGameServerGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, SuspendGameServerGroupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<SuspendGameServerGroupOutput, _>()
    }

    /// <p><p> Assigns a tag to a GameLift resource. AWS resource tags provide an additional management tool set. You can use tags to organize resources, create IAM permissions policies to manage access to groups of resources, customize AWS cost breakdowns, etc. This operation handles the permissions necessary to manage tags for the following GameLift resource types:</p> <ul> <li> <p>Build</p> </li> <li> <p>Script</p> </li> <li> <p>Fleet</p> </li> <li> <p>Alias</p> </li> <li> <p>GameSessionQueue</p> </li> <li> <p>MatchmakingConfiguration</p> </li> <li> <p>MatchmakingRuleSet</p> </li> </ul> <p>To add a tag to a resource, specify the unique ARN value for the resource and provide a tag list containing one or more tags. The operation succeeds even if the list includes tags that are already assigned to the specified resource. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS Resources</a> in the <i>AWS General Reference</i> </p> <p> <a href="http://aws.amazon.com/answers/account-management/aws-tagging-strategies/"> AWS Tagging Strategies</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>TagResource</a> </p> </li> <li> <p> <a>UntagResource</a> </p> </li> <li> <p> <a>ListTagsForResource</a> </p> </li> </ul></p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
    }

    /// <p><p>Removes a tag that is assigned to a GameLift resource. Resource tags are used to organize AWS resources for a range of purposes. This operation handles the permissions necessary to manage tags for the following GameLift resource types:</p> <ul> <li> <p>Build</p> </li> <li> <p>Script</p> </li> <li> <p>Fleet</p> </li> <li> <p>Alias</p> </li> <li> <p>GameSessionQueue</p> </li> <li> <p>MatchmakingConfiguration</p> </li> <li> <p>MatchmakingRuleSet</p> </li> </ul> <p>To remove a tag from a resource, specify the unique ARN value for the resource and provide a string list containing one or more tags to be removed. This operation succeeds even if the list includes tags that are not currently assigned to the specified resource.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS Resources</a> in the <i>AWS General Reference</i> </p> <p> <a href="http://aws.amazon.com/answers/account-management/aws-tagging-strategies/"> AWS Tagging Strategies</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>TagResource</a> </p> </li> <li> <p> <a>UntagResource</a> </p> </li> <li> <p> <a>ListTagsForResource</a> </p> </li> </ul></p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UntagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
    }

    /// <p><p>Updates properties for an alias. To update properties, specify the alias ID to be updated and provide the information to be changed. To reassign an alias to another fleet, provide an updated routing strategy. If successful, the updated alias record is returned.</p> <ul> <li> <p> <a>CreateAlias</a> </p> </li> <li> <p> <a>ListAliases</a> </p> </li> <li> <p> <a>DescribeAlias</a> </p> </li> <li> <p> <a>UpdateAlias</a> </p> </li> <li> <p> <a>DeleteAlias</a> </p> </li> <li> <p> <a>ResolveAlias</a> </p> </li> </ul></p>
    async fn update_alias(
        &self,
        input: UpdateAliasInput,
    ) -> Result<UpdateAliasOutput, RusotoError<UpdateAliasError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.UpdateAlias");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateAliasError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateAliasOutput, _>()
    }

    /// <p><p>Updates metadata in a build resource, including the build name and version. To update the metadata, specify the build ID to update and provide the new values. If successful, a build object containing the updated metadata is returned.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-build-intro.html"> Upload a Custom Server Build</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateBuild</a> </p> </li> <li> <p> <a>ListBuilds</a> </p> </li> <li> <p> <a>DescribeBuild</a> </p> </li> <li> <p> <a>UpdateBuild</a> </p> </li> <li> <p> <a>DeleteBuild</a> </p> </li> </ul></p>
    async fn update_build(
        &self,
        input: UpdateBuildInput,
    ) -> Result<UpdateBuildOutput, RusotoError<UpdateBuildError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.UpdateBuild");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateBuildError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateBuildOutput, _>()
    }

    /// <p><p>Updates fleet properties, including name and description, for a fleet. To update metadata, specify the fleet ID and the property values that you want to change. If successful, the fleet ID for the updated fleet is returned.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn update_fleet_attributes(
        &self,
        input: UpdateFleetAttributesInput,
    ) -> Result<UpdateFleetAttributesOutput, RusotoError<UpdateFleetAttributesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.UpdateFleetAttributes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateFleetAttributesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateFleetAttributesOutput, _>()
    }

    /// <p><p>Updates capacity settings for a fleet. Use this operation to specify the number of EC2 instances (hosts) that you want this fleet to contain. Before calling this operation, you may want to call <a>DescribeEC2InstanceLimits</a> to get the maximum capacity based on the fleet&#39;s EC2 instance type.</p> <p>Specify minimum and maximum number of instances. Amazon GameLift will not change fleet capacity to values fall outside of this range. This is particularly important when using auto-scaling (see <a>PutScalingPolicy</a>) to allow capacity to adjust based on player demand while imposing limits on automatic adjustments.</p> <p>To update fleet capacity, specify the fleet ID and the number of instances you want the fleet to host. If successful, Amazon GameLift starts or terminates instances so that the fleet&#39;s active instance count matches the desired instance count. You can view a fleet&#39;s current capacity information by calling <a>DescribeFleetCapacity</a>. If the desired instance count is higher than the instance type&#39;s limit, the &quot;Limit Exceeded&quot; exception occurs.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn update_fleet_capacity(
        &self,
        input: UpdateFleetCapacityInput,
    ) -> Result<UpdateFleetCapacityOutput, RusotoError<UpdateFleetCapacityError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.UpdateFleetCapacity");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateFleetCapacityError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateFleetCapacityOutput, _>()
    }

    /// <p><p>Updates port settings for a fleet. To update settings, specify the fleet ID to be updated and list the permissions you want to update. List the permissions you want to add in <code>InboundPermissionAuthorizations</code>, and permissions you want to remove in <code>InboundPermissionRevocations</code>. Permissions to be removed must match existing fleet permissions. If successful, the fleet ID for the updated fleet is returned.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn update_fleet_port_settings(
        &self,
        input: UpdateFleetPortSettingsInput,
    ) -> Result<UpdateFleetPortSettingsOutput, RusotoError<UpdateFleetPortSettingsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.UpdateFleetPortSettings");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateFleetPortSettingsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateFleetPortSettingsOutput, _>()
    }

    /// <p><p> <b>This operation is used with the Amazon GameLift FleetIQ solution and game server groups.</b> </p> <p>Updates information about a registered game server to help GameLift FleetIQ to track game server availability. This operation is called by a game server process that is running on an instance in a game server group. </p> <p>Use this operation to update the following types of game server information. You can make all three types of updates in the same request:</p> <ul> <li> <p>To update the game server&#39;s utilization status, identify the game server and game server group and specify the current utilization status. Use this status to identify when game servers are currently hosting games and when they are available to be claimed.</p> </li> <li> <p>To report health status, identify the game server and game server group and set health check to <code>HEALTHY</code>. If a game server does not report health status for a certain length of time, the game server is no longer considered healthy. As a result, it will be eventually deregistered from the game server group to avoid affecting utilization metrics. The best practice is to report health every 60 seconds.</p> </li> <li> <p>To change game server metadata, provide updated game server data.</p> </li> </ul> <p>Once a game server is successfully updated, the relevant statuses and timestamps are updated.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">GameLift FleetIQ Guide</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>RegisterGameServer</a> </p> </li> <li> <p> <a>ListGameServers</a> </p> </li> <li> <p> <a>ClaimGameServer</a> </p> </li> <li> <p> <a>DescribeGameServer</a> </p> </li> <li> <p> <a>UpdateGameServer</a> </p> </li> <li> <p> <a>DeregisterGameServer</a> </p> </li> </ul></p>
    async fn update_game_server(
        &self,
        input: UpdateGameServerInput,
    ) -> Result<UpdateGameServerOutput, RusotoError<UpdateGameServerError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.UpdateGameServer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateGameServerError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateGameServerOutput, _>()
    }

    /// <p><p> <b>This operation is used with the Amazon GameLift FleetIQ solution and game server groups.</b> </p> <p>Updates GameLift FleetIQ-specific properties for a game server group. Many Auto Scaling group properties are updated on the Auto Scaling group directly, including the launch template, Auto Scaling policies, and maximum/minimum/desired instance counts.</p> <p>To update the game server group, specify the game server group ID and provide the updated values. Before applying the updates, the new values are validated to ensure that GameLift FleetIQ can continue to perform instance balancing activity. If successful, a <a>GameServerGroup</a> object is returned.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">GameLift FleetIQ Guide</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateGameServerGroup</a> </p> </li> <li> <p> <a>ListGameServerGroups</a> </p> </li> <li> <p> <a>DescribeGameServerGroup</a> </p> </li> <li> <p> <a>UpdateGameServerGroup</a> </p> </li> <li> <p> <a>DeleteGameServerGroup</a> </p> </li> <li> <p> <a>ResumeGameServerGroup</a> </p> </li> <li> <p> <a>SuspendGameServerGroup</a> </p> </li> <li> <p> <a>DescribeGameServerInstances</a> </p> </li> </ul></p>
    async fn update_game_server_group(
        &self,
        input: UpdateGameServerGroupInput,
    ) -> Result<UpdateGameServerGroupOutput, RusotoError<UpdateGameServerGroupError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.UpdateGameServerGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateGameServerGroupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateGameServerGroupOutput, _>()
    }

    /// <p><p>Updates game session properties. This includes the session name, maximum player count, protection policy, which controls whether or not an active game session can be terminated during a scale-down event, and the player session creation policy, which controls whether or not new players can join the session. To update a game session, specify the game session ID and the values you want to change. If successful, an updated <a>GameSession</a> object is returned. </p> <ul> <li> <p> <a>CreateGameSession</a> </p> </li> <li> <p> <a>DescribeGameSessions</a> </p> </li> <li> <p> <a>DescribeGameSessionDetails</a> </p> </li> <li> <p> <a>SearchGameSessions</a> </p> </li> <li> <p> <a>UpdateGameSession</a> </p> </li> <li> <p> <a>GetGameSessionLogUrl</a> </p> </li> <li> <p>Game session placements</p> <ul> <li> <p> <a>StartGameSessionPlacement</a> </p> </li> <li> <p> <a>DescribeGameSessionPlacement</a> </p> </li> <li> <p> <a>StopGameSessionPlacement</a> </p> </li> </ul> </li> </ul></p>
    async fn update_game_session(
        &self,
        input: UpdateGameSessionInput,
    ) -> Result<UpdateGameSessionOutput, RusotoError<UpdateGameSessionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.UpdateGameSession");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateGameSessionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateGameSessionOutput, _>()
    }

    /// <p><p>Updates settings for a game session queue, which determines how new game session requests in the queue are processed. To update settings, specify the queue name to be updated and provide the new settings. When updating destinations, provide a complete list of destinations. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/queues-intro.html"> Using Multi-Region Queues</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateGameSessionQueue</a> </p> </li> <li> <p> <a>DescribeGameSessionQueues</a> </p> </li> <li> <p> <a>UpdateGameSessionQueue</a> </p> </li> <li> <p> <a>DeleteGameSessionQueue</a> </p> </li> </ul></p>
    async fn update_game_session_queue(
        &self,
        input: UpdateGameSessionQueueInput,
    ) -> Result<UpdateGameSessionQueueOutput, RusotoError<UpdateGameSessionQueueError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.UpdateGameSessionQueue");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateGameSessionQueueError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateGameSessionQueueOutput, _>()
    }

    /// <p><p>Updates settings for a FlexMatch matchmaking configuration. These changes affect all matches and game sessions that are created after the update. To update settings, specify the configuration name to be updated and provide the new settings. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-configuration.html"> Design a FlexMatch Matchmaker</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DeleteMatchmakingRuleSet</a> </p> </li> </ul></p>
    async fn update_matchmaking_configuration(
        &self,
        input: UpdateMatchmakingConfigurationInput,
    ) -> Result<
        UpdateMatchmakingConfigurationOutput,
        RusotoError<UpdateMatchmakingConfigurationError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.UpdateMatchmakingConfiguration");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateMatchmakingConfigurationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateMatchmakingConfigurationOutput, _>()
    }

    /// <p><p>Updates the current runtime configuration for the specified fleet, which tells Amazon GameLift how to launch server processes on instances in the fleet. You can update a fleet&#39;s runtime configuration at any time after the fleet is created; it does not need to be in an <code>ACTIVE</code> status.</p> <p>To update runtime configuration, specify the fleet ID and provide a <code>RuntimeConfiguration</code> object with an updated set of server process configurations.</p> <p>Each instance in a Amazon GameLift fleet checks regularly for an updated runtime configuration and changes how it launches server processes to comply with the latest version. Existing server processes are not affected by the update; runtime configuration changes are applied gradually as existing processes shut down and new processes are launched during Amazon GameLift&#39;s normal process recycling activity.</p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up GameLift Fleets</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateFleet</a> </p> </li> <li> <p> <a>ListFleets</a> </p> </li> <li> <p> <a>DeleteFleet</a> </p> </li> <li> <p> <a>DescribeFleetAttributes</a> </p> </li> <li> <p>Update fleets:</p> <ul> <li> <p> <a>UpdateFleetAttributes</a> </p> </li> <li> <p> <a>UpdateFleetCapacity</a> </p> </li> <li> <p> <a>UpdateFleetPortSettings</a> </p> </li> <li> <p> <a>UpdateRuntimeConfiguration</a> </p> </li> </ul> </li> <li> <p> <a>StartFleetActions</a> or <a>StopFleetActions</a> </p> </li> </ul></p>
    async fn update_runtime_configuration(
        &self,
        input: UpdateRuntimeConfigurationInput,
    ) -> Result<UpdateRuntimeConfigurationOutput, RusotoError<UpdateRuntimeConfigurationError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.UpdateRuntimeConfiguration");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateRuntimeConfigurationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateRuntimeConfigurationOutput, _>()
    }

    /// <p><p>Updates Realtime script metadata and content.</p> <p>To update script metadata, specify the script ID and provide updated name and/or version values. </p> <p>To update script content, provide an updated zip file by pointing to either a local file or an Amazon S3 bucket location. You can use either method regardless of how the original script was uploaded. Use the <i>Version</i> parameter to track updates to the script.</p> <p>If the call is successful, the updated metadata is stored in the script record and a revised script is uploaded to the Amazon GameLift service. Once the script is updated and acquired by a fleet instance, the new version is used for all new game sessions. </p> <p> <b>Learn more</b> </p> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/realtime-intro.html">Amazon GameLift Realtime Servers</a> </p> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateScript</a> </p> </li> <li> <p> <a>ListScripts</a> </p> </li> <li> <p> <a>DescribeScript</a> </p> </li> <li> <p> <a>UpdateScript</a> </p> </li> <li> <p> <a>DeleteScript</a> </p> </li> </ul></p>
    async fn update_script(
        &self,
        input: UpdateScriptInput,
    ) -> Result<UpdateScriptOutput, RusotoError<UpdateScriptError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.UpdateScript");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateScriptError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateScriptOutput, _>()
    }

    /// <p><p>Validates the syntax of a matchmaking rule or rule set. This operation checks that the rule set is using syntactically correct JSON and that it conforms to allowed property expressions. To validate syntax, provide a rule set JSON string.</p> <p> <b>Learn more</b> </p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/gamelift/latest/flexmatchguide/match-rulesets.html">Build a Rule Set</a> </p> </li> </ul> <p> <b>Related operations</b> </p> <ul> <li> <p> <a>CreateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DescribeMatchmakingConfigurations</a> </p> </li> <li> <p> <a>UpdateMatchmakingConfiguration</a> </p> </li> <li> <p> <a>DeleteMatchmakingConfiguration</a> </p> </li> <li> <p> <a>CreateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DescribeMatchmakingRuleSets</a> </p> </li> <li> <p> <a>ValidateMatchmakingRuleSet</a> </p> </li> <li> <p> <a>DeleteMatchmakingRuleSet</a> </p> </li> </ul></p>
    async fn validate_matchmaking_rule_set(
        &self,
        input: ValidateMatchmakingRuleSetInput,
    ) -> Result<ValidateMatchmakingRuleSetOutput, RusotoError<ValidateMatchmakingRuleSetError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "GameLift.ValidateMatchmakingRuleSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ValidateMatchmakingRuleSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ValidateMatchmakingRuleSetOutput, _>()
    }
}
