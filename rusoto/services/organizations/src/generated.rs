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

impl OrganizationsClient {
    pub(crate) fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request =
            SignedRequest::new(http_method, "organizations", &self.region, request_uri);

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
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AcceptHandshakeRequest {
    /// <p>The unique identifier (ID) of the handshake that you want to accept.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for handshake ID string requires "h-" followed by from 8 to 32 lowercase letters or digits.</p>
    #[serde(rename = "HandshakeId")]
    pub handshake_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AcceptHandshakeResponse {
    /// <p>A structure that contains details about the accepted handshake.</p>
    #[serde(rename = "Handshake")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake: Option<Handshake>,
}

/// <p>Contains information about an AWS account that is a member of an organization.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Account {
    /// <p>The Amazon Resource Name (ARN) of the account.</p> <p>For more information about ARNs in Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_permissions.html#orgs-permissions-arns">ARN Formats Supported by Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The email address associated with the AWS account.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for this parameter is a string of characters that represents a standard internet email address.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The unique identifier (ID) of the account.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an account ID string requires exactly 12 digits.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The method by which the account joined the organization.</p>
    #[serde(rename = "JoinedMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_method: Option<String>,
    /// <p>The date the account became a part of the organization.</p>
    #[serde(rename = "JoinedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_timestamp: Option<f64>,
    /// <p>The friendly name of the account.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of any of the characters in the ASCII character range.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The status of the account in the organization.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AttachPolicyRequest {
    /// <p>The unique identifier (ID) of the policy that you want to attach to the target. You can get the ID for the policy by calling the <a>ListPolicies</a> operation.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a policy ID string requires "p-" followed by from 8 to 128 lowercase or uppercase letters, digits, or the underscore character (_).</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
    /// <p><p>The unique identifier (ID) of the root, OU, or account that you want to attach the policy to. You can get the ID by calling the <a>ListRoots</a>, <a>ListOrganizationalUnitsForParent</a>, or <a>ListAccounts</a> operations.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a target ID string requires one of the following:</p> <ul> <li> <p> <b>Root</b> - A string that begins with &quot;r-&quot; followed by from 4 to 32 lowercase letters or digits.</p> </li> <li> <p> <b>Account</b> - A string that consists of exactly 12 digits.</p> </li> <li> <p> <b>Organizational unit (OU)</b> - A string that begins with &quot;ou-&quot; followed by from 4 to 32 lowercase letters or digits (the ID of the root that the OU is in). This string is followed by a second &quot;-&quot; dash and from 8 to 32 additional lowercase letters or digits.</p> </li> </ul></p>
    #[serde(rename = "TargetId")]
    pub target_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelHandshakeRequest {
    /// <p>The unique identifier (ID) of the handshake that you want to cancel. You can get the ID from the <a>ListHandshakesForOrganization</a> operation.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for handshake ID string requires "h-" followed by from 8 to 32 lowercase letters or digits.</p>
    #[serde(rename = "HandshakeId")]
    pub handshake_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelHandshakeResponse {
    /// <p>A structure that contains details about the handshake that you canceled.</p>
    #[serde(rename = "Handshake")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake: Option<Handshake>,
}

/// <p>Contains a list of child entities, either OUs or accounts.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Child {
    /// <p><p>The unique identifier (ID) of this child entity.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a child ID string requires one of the following:</p> <ul> <li> <p> <b>Account</b> - A string that consists of exactly 12 digits.</p> </li> <li> <p> <b>Organizational unit (OU)</b> - A string that begins with &quot;ou-&quot; followed by from 4 to 32 lowercase letters or digits (the ID of the root that contains the OU). This string is followed by a second &quot;-&quot; dash and from 8 to 32 additional lowercase letters or digits.</p> </li> </ul></p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The type of this child entity.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateAccountRequest {
    /// <p>The friendly name of the member account.</p>
    #[serde(rename = "AccountName")]
    pub account_name: String,
    /// <p>The email address of the owner to assign to the new member account. This email address must not already be associated with another AWS account. You must use a valid email address to complete account creation. You can't access the root user of the account or remove an account that was created with an invalid email address.</p>
    #[serde(rename = "Email")]
    pub email: String,
    /// <p>If set to <code>ALLOW</code>, the new account enables IAM users to access account billing information <i>if</i> they have the required permissions. If set to <code>DENY</code>, only the root user of the new account can access account billing information. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/grantaccess.html#ControllingAccessWebsite-Activate">Activating Access to the Billing and Cost Management Console</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p> <p>If you don't specify this parameter, the value defaults to <code>ALLOW</code>, and IAM users and roles with the required permissions can access billing information for the new account.</p>
    #[serde(rename = "IamUserAccessToBilling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_user_access_to_billing: Option<String>,
    /// <p>(Optional)</p> <p>The name of an IAM role that AWS Organizations automatically preconfigures in the new member account. This role trusts the management account, allowing users in the management account to assume the role, as permitted by the management account administrator. The role has administrator permissions in the new member account.</p> <p>If you don't specify this parameter, the role name defaults to <code>OrganizationAccountAccessRole</code>.</p> <p>For more information about how to use this role to access the member account, see the following links:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_access.html#orgs_manage_accounts_create-cross-account-role">Accessing and Administering the Member Accounts in Your Organization</a> in the <i>AWS Organizations User Guide</i> </p> </li> <li> <p>Steps 2 and 3 in <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/tutorial_cross-account-with-roles.html">Tutorial: Delegate Access Across AWS Accounts Using IAM Roles</a> in the <i>IAM User Guide</i> </p> </li> </ul> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter. The pattern can include uppercase letters, lowercase letters, digits with no spaces, and any of the following characters: =,.@-</p>
    #[serde(rename = "RoleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// <p><p>A list of tags that you want to attach to the newly created account. For each tag in the list, you must specify both a tag key and a value. You can set the value to an empty string, but you can&#39;t set it to <code>null</code>. For more information about tagging, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_tagging.html">Tagging AWS Organizations resources</a> in the AWS Organizations User Guide.</p> <note> <p>If any one of the tags is invalid or if you exceed the allowed number of tags for an account, then the entire request fails and the account is not created.</p> </note></p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAccountResponse {
    /// <p>A structure that contains details about the request to create an account. This response structure might not be fully populated when you first receive it because account creation is an asynchronous process. You can pass the returned <code>CreateAccountStatus</code> ID as a parameter to <a>DescribeCreateAccountStatus</a> to get status about the progress of the request at later times. You can also check the AWS CloudTrail log for the <code>CreateAccountResult</code> event. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_monitoring.html">Monitoring the Activity in Your Organization</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "CreateAccountStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_account_status: Option<CreateAccountStatus>,
}

/// <p>Contains the status about a <a>CreateAccount</a> or <a>CreateGovCloudAccount</a> request to create an AWS account or an AWS GovCloud (US) account in an organization.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateAccountStatus {
    /// <p>If the account was created successfully, the unique identifier (ID) of the new account.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an account ID string requires exactly 12 digits.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The account name given to the account when it was created.</p>
    #[serde(rename = "AccountName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    /// <p>The date and time that the account was created and the request completed.</p>
    #[serde(rename = "CompletedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_timestamp: Option<f64>,
    /// <p><p>If the request failed, a description of the reason for the failure.</p> <ul> <li> <p>ACCOUNT<em>LIMIT</em>EXCEEDED: The account could not be created because you have reached the limit on the number of accounts in your organization.</p> </li> <li> <p>CONCURRENT<em>ACCOUNT</em>MODIFICATION: You already submitted a request with the same information.</p> </li> <li> <p>EMAIL<em>ALREADY</em>EXISTS: The account could not be created because another AWS account with that email address already exists.</p> </li> <li> <p>GOVCLOUD<em>ACCOUNT</em>ALREADY<em>EXISTS: The account in the AWS GovCloud (US) Region could not be created because this Region already includes an account with that email address.</p> </li> <li> <p>INVALID</em>ADDRESS: The account could not be created because the address you provided is not valid.</p> </li> <li> <p>INVALID<em>EMAIL: The account could not be created because the email address you provided is not valid.</p> </li> <li> <p>INTERNAL</em>FAILURE: The account could not be created because of an internal failure. Try again later. If the problem persists, contact Customer Support.</p> </li> <li> <p>MISSING<em>BUSINESS</em>VALIDATION: The AWS account that owns your organization has not received Business Validation.</p> </li> <li> <p> MISSING<em>PAYMENT</em>INSTRUMENT: You must configure the management account with a valid payment method, such as a credit card.</p> </li> </ul></p>
    #[serde(rename = "FailureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>If the account was created successfully, the unique identifier (ID) of the new account in the AWS GovCloud (US) Region.</p>
    #[serde(rename = "GovCloudAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gov_cloud_account_id: Option<String>,
    /// <p>The unique identifier (ID) that references this request. You get this value from the response of the initial <a>CreateAccount</a> request to create the account.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a create account request ID string requires "car-" followed by from 8 to 32 lowercase letters or digits.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The date and time that the request was made for the account creation.</p>
    #[serde(rename = "RequestedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_timestamp: Option<f64>,
    /// <p>The status of the request.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateGovCloudAccountRequest {
    /// <p>The friendly name of the member account.</p>
    #[serde(rename = "AccountName")]
    pub account_name: String,
    /// <p>The email address of the owner to assign to the new member account in the commercial Region. This email address must not already be associated with another AWS account. You must use a valid email address to complete account creation. You can't access the root user of the account or remove an account that was created with an invalid email address. Like all request parameters for <code>CreateGovCloudAccount</code>, the request for the email address for the AWS GovCloud (US) account originates from the commercial Region, not from the AWS GovCloud (US) Region.</p>
    #[serde(rename = "Email")]
    pub email: String,
    /// <p>If set to <code>ALLOW</code>, the new linked account in the commercial Region enables IAM users to access account billing information <i>if</i> they have the required permissions. If set to <code>DENY</code>, only the root user of the new account can access account billing information. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/grantaccess.html#ControllingAccessWebsite-Activate">Activating Access to the Billing and Cost Management Console</a> in the <i>AWS Billing and Cost Management User Guide.</i> </p> <p>If you don't specify this parameter, the value defaults to <code>ALLOW</code>, and IAM users and roles with the required permissions can access billing information for the new account.</p>
    #[serde(rename = "IamUserAccessToBilling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_user_access_to_billing: Option<String>,
    /// <p>(Optional)</p> <p>The name of an IAM role that AWS Organizations automatically preconfigures in the new member accounts in both the AWS GovCloud (US) Region and in the commercial Region. This role trusts the management account, allowing users in the management account to assume the role, as permitted by the management account administrator. The role has administrator permissions in the new member account.</p> <p>If you don't specify this parameter, the role name defaults to <code>OrganizationAccountAccessRole</code>.</p> <p>For more information about how to use this role to access the member account, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_access.html#orgs_manage_accounts_create-cross-account-role">Accessing and Administering the Member Accounts in Your Organization</a> in the <i>AWS Organizations User Guide</i> and steps 2 and 3 in <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/tutorial_cross-account-with-roles.html">Tutorial: Delegate Access Across AWS Accounts Using IAM Roles</a> in the <i>IAM User Guide.</i> </p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter. The pattern can include uppercase letters, lowercase letters, digits with no spaces, and any of the following characters: =,.@-</p>
    #[serde(rename = "RoleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// <p><p>A list of tags that you want to attach to the newly created account. These tags are attached to the commercial account associated with the GovCloud account, and not to the GovCloud account itself. To add tags to the actual GovCloud account, call the <a>TagResource</a> operation in the GovCloud region after the new GovCloud account exists.</p> <p>For each tag in the list, you must specify both a tag key and a value. You can set the value to an empty string, but you can&#39;t set it to <code>null</code>. For more information about tagging, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_tagging.html">Tagging AWS Organizations resources</a> in the AWS Organizations User Guide.</p> <note> <p>If any one of the tags is invalid or if you exceed the allowed number of tags for an account, then the entire request fails and the account is not created.</p> </note></p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateGovCloudAccountResponse {
    #[serde(rename = "CreateAccountStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_account_status: Option<CreateAccountStatus>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateOrganizationRequest {
    /// <p><p>Specifies the feature set supported by the new organization. Each feature set supports different levels of functionality.</p> <ul> <li> <p> <code>CONSOLIDATED<em>BILLING</code>: All member accounts have their bills consolidated to and paid by the management account. For more information, see &lt;a href=&quot;https://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>getting-started<em>concepts.html#feature-set-cb-only&quot;&gt;Consolidated billing</a> in the <i>AWS Organizations User Guide.</i> </p> <p> The consolidated billing feature subset isn&#39;t available for organizations in the AWS GovCloud (US) Region.</p> </li> <li> <p> <code>ALL</code>: In addition to all the features supported by the consolidated billing feature set, the management account can also apply any policy type to any member account in the organization. For more information, see &lt;a href=&quot;https://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>getting-started_concepts.html#feature-set-all&quot;&gt;All features</a> in the <i>AWS Organizations User Guide.</i> </p> </li> </ul></p>
    #[serde(rename = "FeatureSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_set: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateOrganizationResponse {
    /// <p>A structure that contains details about the newly created organization.</p>
    #[serde(rename = "Organization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateOrganizationalUnitRequest {
    /// <p>The friendly name to assign to the new OU.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>The unique identifier (ID) of the parent root or OU that you want to create the new OU in.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a parent ID string requires one of the following:</p> <ul> <li> <p> <b>Root</b> - A string that begins with &quot;r-&quot; followed by from 4 to 32 lowercase letters or digits.</p> </li> <li> <p> <b>Organizational unit (OU)</b> - A string that begins with &quot;ou-&quot; followed by from 4 to 32 lowercase letters or digits (the ID of the root that the OU is in). This string is followed by a second &quot;-&quot; dash and from 8 to 32 additional lowercase letters or digits.</p> </li> </ul></p>
    #[serde(rename = "ParentId")]
    pub parent_id: String,
    /// <p><p>A list of tags that you want to attach to the newly created OU. For each tag in the list, you must specify both a tag key and a value. You can set the value to an empty string, but you can&#39;t set it to <code>null</code>. For more information about tagging, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_tagging.html">Tagging AWS Organizations resources</a> in the AWS Organizations User Guide.</p> <note> <p>If any one of the tags is invalid or if you exceed the allowed number of tags for an OU, then the entire request fails and the OU is not created.</p> </note></p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateOrganizationalUnitResponse {
    /// <p>A structure that contains details about the newly created OU.</p>
    #[serde(rename = "OrganizationalUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit: Option<OrganizationalUnit>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreatePolicyRequest {
    /// <p>The policy text content to add to the new policy. The text that you supply must adhere to the rules of the policy type you specify in the <code>Type</code> parameter.</p>
    #[serde(rename = "Content")]
    pub content: String,
    /// <p>An optional description to assign to the policy.</p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>The friendly name to assign to the policy.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of any of the characters in the ASCII character range.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>A list of tags that you want to attach to the newly created policy. For each tag in the list, you must specify both a tag key and a value. You can set the value to an empty string, but you can&#39;t set it to <code>null</code>. For more information about tagging, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_tagging.html">Tagging AWS Organizations resources</a> in the AWS Organizations User Guide.</p> <note> <p>If any one of the tags is invalid or if you exceed the allowed number of tags for a policy, then the entire request fails and the policy is not created.</p> </note></p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p><p>The type of policy to create. You can specify one of the following values:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_ai-opt-out.html">AISERVICES<em>OPT</em>OUT<em>POLICY</a> </p> </li> <li> <p> &lt;a href=&quot;https://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>policies</em>backup.html&quot;&gt;BACKUP<em>POLICY</a> </p> </li> <li> <p> &lt;a href=&quot;https://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>policies</em>scp.html&quot;&gt;SERVICE<em>CONTROL</em>POLICY</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies.html">TAG_POLICY</a> </p> </li> </ul></p>
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreatePolicyResponse {
    /// <p>A structure that contains details about the newly created policy.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeclineHandshakeRequest {
    /// <p>The unique identifier (ID) of the handshake that you want to decline. You can get the ID from the <a>ListHandshakesForAccount</a> operation.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for handshake ID string requires "h-" followed by from 8 to 32 lowercase letters or digits.</p>
    #[serde(rename = "HandshakeId")]
    pub handshake_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeclineHandshakeResponse {
    /// <p>A structure that contains details about the declined handshake. The state is updated to show the value <code>DECLINED</code>.</p>
    #[serde(rename = "Handshake")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake: Option<Handshake>,
}

/// <p>Contains information about the delegated administrator.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DelegatedAdministrator {
    /// <p>The Amazon Resource Name (ARN) of the delegated administrator's account.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date when the account was made a delegated administrator.</p>
    #[serde(rename = "DelegationEnabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegation_enabled_date: Option<f64>,
    /// <p>The email address that is associated with the delegated administrator's AWS account.</p>
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// <p>The unique identifier (ID) of the delegated administrator's account.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The method by which the delegated administrator's account joined the organization.</p>
    #[serde(rename = "JoinedMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_method: Option<String>,
    /// <p>The date when the delegated administrator's account became a part of the organization.</p>
    #[serde(rename = "JoinedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_timestamp: Option<f64>,
    /// <p>The friendly name of the delegated administrator's account.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The status of the delegated administrator's account in the organization.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Contains information about the AWS service for which the account is a delegated administrator.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DelegatedService {
    /// <p>The date that the account became a delegated administrator for this service. </p>
    #[serde(rename = "DelegationEnabledDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegation_enabled_date: Option<f64>,
    /// <p>The name of a service that can request an operation for the specified service. This is typically in the form of a URL, such as: <code> <i>servicename</i>.amazonaws.com</code>.</p>
    #[serde(rename = "ServicePrincipal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteOrganizationalUnitRequest {
    /// <p>The unique identifier (ID) of the organizational unit that you want to delete. You can get the ID from the <a>ListOrganizationalUnitsForParent</a> operation.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an organizational unit ID string requires "ou-" followed by from 4 to 32 lowercase letters or digits (the ID of the root that contains the OU). This string is followed by a second "-" dash and from 8 to 32 additional lowercase letters or digits.</p>
    #[serde(rename = "OrganizationalUnitId")]
    pub organizational_unit_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeletePolicyRequest {
    /// <p>The unique identifier (ID) of the policy that you want to delete. You can get the ID from the <a>ListPolicies</a> or <a>ListPoliciesForTarget</a> operations.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a policy ID string requires "p-" followed by from 8 to 128 lowercase or uppercase letters, digits, or the underscore character (_).</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeregisterDelegatedAdministratorRequest {
    /// <p>The account ID number of the member account in the organization that you want to deregister as a delegated administrator.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The service principal name of an AWS service for which the account is a delegated administrator.</p> <p>Delegated administrator privileges are revoked for only the specified AWS service from the member account. If the specified service is the only service for which the member account is a delegated administrator, the operation also revokes Organizations read action permissions.</p>
    #[serde(rename = "ServicePrincipal")]
    pub service_principal: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAccountRequest {
    /// <p>The unique identifier (ID) of the AWS account that you want information about. You can get the ID from the <a>ListAccounts</a> or <a>ListAccountsForParent</a> operations.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an account ID string requires exactly 12 digits.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAccountResponse {
    /// <p>A structure that contains information about the requested account.</p>
    #[serde(rename = "Account")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<Account>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeCreateAccountStatusRequest {
    /// <p>Specifies the <code>Id</code> value that uniquely identifies the <code>CreateAccount</code> request. You can get the value from the <code>CreateAccountStatus.Id</code> response in an earlier <a>CreateAccount</a> request, or from the <a>ListCreateAccountStatus</a> operation.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a create account request ID string requires "car-" followed by from 8 to 32 lowercase letters or digits.</p>
    #[serde(rename = "CreateAccountRequestId")]
    pub create_account_request_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeCreateAccountStatusResponse {
    /// <p>A structure that contains the current status of an account creation request.</p>
    #[serde(rename = "CreateAccountStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_account_status: Option<CreateAccountStatus>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEffectivePolicyRequest {
    /// <p><p>The type of policy that you want information about. You can specify one of the following values:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_ai-opt-out.html">AISERVICES<em>OPT</em>OUT<em>POLICY</a> </p> </li> <li> <p> &lt;a href=&quot;https://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>policies</em>backup.html&quot;&gt;BACKUP<em>POLICY</a> </p> </li> <li> <p> &lt;a href=&quot;https://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>policies</em>tag-policies.html&quot;&gt;TAG_POLICY</a> </p> </li> </ul></p>
    #[serde(rename = "PolicyType")]
    pub policy_type: String,
    /// <p>When you're signed in as the management account, specify the ID of the account that you want details about. Specifying an organization root or organizational unit (OU) as the target is not supported.</p>
    #[serde(rename = "TargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEffectivePolicyResponse {
    /// <p>The contents of the effective policy.</p>
    #[serde(rename = "EffectivePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_policy: Option<EffectivePolicy>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeHandshakeRequest {
    /// <p>The unique identifier (ID) of the handshake that you want information about. You can get the ID from the original call to <a>InviteAccountToOrganization</a>, or from a call to <a>ListHandshakesForAccount</a> or <a>ListHandshakesForOrganization</a>.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for handshake ID string requires "h-" followed by from 8 to 32 lowercase letters or digits.</p>
    #[serde(rename = "HandshakeId")]
    pub handshake_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeHandshakeResponse {
    /// <p>A structure that contains information about the specified handshake.</p>
    #[serde(rename = "Handshake")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake: Option<Handshake>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeOrganizationResponse {
    /// <p><p>A structure that contains information about the organization.</p> <important> <p>The <code>AvailablePolicyTypes</code> part of the response is deprecated, and you shouldn&#39;t use it in your apps. It doesn&#39;t include any policy type supported by Organizations other than SCPs. To determine which policy types are enabled in your organization, use the <code> <a>ListRoots</a> </code> operation.</p> </important></p>
    #[serde(rename = "Organization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeOrganizationalUnitRequest {
    /// <p>The unique identifier (ID) of the organizational unit that you want details about. You can get the ID from the <a>ListOrganizationalUnitsForParent</a> operation.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an organizational unit ID string requires "ou-" followed by from 4 to 32 lowercase letters or digits (the ID of the root that contains the OU). This string is followed by a second "-" dash and from 8 to 32 additional lowercase letters or digits.</p>
    #[serde(rename = "OrganizationalUnitId")]
    pub organizational_unit_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeOrganizationalUnitResponse {
    /// <p>A structure that contains details about the specified OU.</p>
    #[serde(rename = "OrganizationalUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit: Option<OrganizationalUnit>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribePolicyRequest {
    /// <p>The unique identifier (ID) of the policy that you want details about. You can get the ID from the <a>ListPolicies</a> or <a>ListPoliciesForTarget</a> operations.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a policy ID string requires "p-" followed by from 8 to 128 lowercase or uppercase letters, digits, or the underscore character (_).</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribePolicyResponse {
    /// <p>A structure that contains details about the specified policy.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DetachPolicyRequest {
    /// <p>The unique identifier (ID) of the policy you want to detach. You can get the ID from the <a>ListPolicies</a> or <a>ListPoliciesForTarget</a> operations.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a policy ID string requires "p-" followed by from 8 to 128 lowercase or uppercase letters, digits, or the underscore character (_).</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
    /// <p><p>The unique identifier (ID) of the root, OU, or account that you want to detach the policy from. You can get the ID from the <a>ListRoots</a>, <a>ListOrganizationalUnitsForParent</a>, or <a>ListAccounts</a> operations.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a target ID string requires one of the following:</p> <ul> <li> <p> <b>Root</b> - A string that begins with &quot;r-&quot; followed by from 4 to 32 lowercase letters or digits.</p> </li> <li> <p> <b>Account</b> - A string that consists of exactly 12 digits.</p> </li> <li> <p> <b>Organizational unit (OU)</b> - A string that begins with &quot;ou-&quot; followed by from 4 to 32 lowercase letters or digits (the ID of the root that the OU is in). This string is followed by a second &quot;-&quot; dash and from 8 to 32 additional lowercase letters or digits.</p> </li> </ul></p>
    #[serde(rename = "TargetId")]
    pub target_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisableAWSServiceAccessRequest {
    /// <p>The service principal name of the AWS service for which you want to disable integration with your organization. This is typically in the form of a URL, such as <code> <i>service-abbreviation</i>.amazonaws.com</code>.</p>
    #[serde(rename = "ServicePrincipal")]
    pub service_principal: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisablePolicyTypeRequest {
    /// <p><p>The policy type that you want to disable in this root. You can specify one of the following values:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_ai-opt-out.html">AISERVICES<em>OPT</em>OUT<em>POLICY</a> </p> </li> <li> <p> &lt;a href=&quot;https://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>policies</em>backup.html&quot;&gt;BACKUP<em>POLICY</a> </p> </li> <li> <p> &lt;a href=&quot;https://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>policies</em>scp.html&quot;&gt;SERVICE<em>CONTROL</em>POLICY</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies.html">TAG_POLICY</a> </p> </li> </ul></p>
    #[serde(rename = "PolicyType")]
    pub policy_type: String,
    /// <p>The unique identifier (ID) of the root in which you want to disable a policy type. You can get the ID from the <a>ListRoots</a> operation.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a root ID string requires "r-" followed by from 4 to 32 lowercase letters or digits.</p>
    #[serde(rename = "RootId")]
    pub root_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisablePolicyTypeResponse {
    /// <p>A structure that shows the root with the updated list of enabled policy types.</p>
    #[serde(rename = "Root")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<Root>,
}

/// <p>Contains rules to be applied to the affected accounts. The effective policy is the aggregation of any policies the account inherits, plus any policy directly attached to the account.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EffectivePolicy {
    /// <p>The time of the last update to this policy.</p>
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<f64>,
    /// <p>The text content of the policy.</p>
    #[serde(rename = "PolicyContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_content: Option<String>,
    /// <p>The policy type.</p>
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    /// <p>The account ID of the policy target. </p>
    #[serde(rename = "TargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnableAWSServiceAccessRequest {
    /// <p>The service principal name of the AWS service for which you want to enable integration with your organization. This is typically in the form of a URL, such as <code> <i>service-abbreviation</i>.amazonaws.com</code>.</p>
    #[serde(rename = "ServicePrincipal")]
    pub service_principal: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnableAllFeaturesRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EnableAllFeaturesResponse {
    /// <p>A structure that contains details about the handshake created to support this request to enable all features in the organization.</p>
    #[serde(rename = "Handshake")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake: Option<Handshake>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnablePolicyTypeRequest {
    /// <p><p>The policy type that you want to enable. You can specify one of the following values:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_ai-opt-out.html">AISERVICES<em>OPT</em>OUT<em>POLICY</a> </p> </li> <li> <p> &lt;a href=&quot;https://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>policies</em>backup.html&quot;&gt;BACKUP<em>POLICY</a> </p> </li> <li> <p> &lt;a href=&quot;https://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>policies</em>scp.html&quot;&gt;SERVICE<em>CONTROL</em>POLICY</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies.html">TAG_POLICY</a> </p> </li> </ul></p>
    #[serde(rename = "PolicyType")]
    pub policy_type: String,
    /// <p>The unique identifier (ID) of the root in which you want to enable a policy type. You can get the ID from the <a>ListRoots</a> operation.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a root ID string requires "r-" followed by from 4 to 32 lowercase letters or digits.</p>
    #[serde(rename = "RootId")]
    pub root_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EnablePolicyTypeResponse {
    /// <p>A structure that shows the root with the updated list of enabled policy types.</p>
    #[serde(rename = "Root")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<Root>,
}

/// <p>A structure that contains details of a service principal that represents an AWS service that is enabled to integrate with AWS Organizations.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EnabledServicePrincipal {
    /// <p>The date that the service principal was enabled for integration with AWS Organizations.</p>
    #[serde(rename = "DateEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_enabled: Option<f64>,
    /// <p>The name of the service principal. This is typically in the form of a URL, such as: <code> <i>servicename</i>.amazonaws.com</code>.</p>
    #[serde(rename = "ServicePrincipal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal: Option<String>,
}

/// <p>Contains information that must be exchanged to securely establish a relationship between two accounts (an <i>originator</i> and a <i>recipient</i>). For example, when a management account (the originator) invites another account (the recipient) to join its organization, the two accounts exchange information as a series of handshake requests and responses.</p> <p> <b>Note:</b> Handshakes that are CANCELED, ACCEPTED, or DECLINED show up in lists for only 30 days after entering that state After that they are deleted.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Handshake {
    /// <p><p>The type of handshake, indicating what action occurs when the recipient accepts the handshake. The following handshake types are supported:</p> <ul> <li> <p> <b>INVITE</b>: This type of handshake represents a request to join an organization. It is always sent from the management account to only non-member accounts.</p> </li> <li> <p> <b>ENABLE<em>ALL</em>FEATURES</b>: This type of handshake represents a request to enable all features in an organization. It is always sent from the management account to only <i>invited</i> member accounts. Created accounts do not receive this because those accounts were created by the organization&#39;s management account and approval is inferred.</p> </li> <li> <p> <b>APPROVE<em>ALL</em>FEATURES</b>: This type of handshake is sent from the Organizations service when all member accounts have approved the <code>ENABLE<em>ALL</em>FEATURES</code> invitation. It is sent only to the management account and signals the master that it can finalize the process to enable all features.</p> </li> </ul></p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of a handshake.</p> <p>For more information about ARNs in Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_permissions.html#orgs-permissions-arns">ARN Formats Supported by Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date and time that the handshake expires. If the recipient of the handshake request fails to respond before the specified date and time, the handshake becomes inactive and is no longer valid.</p>
    #[serde(rename = "ExpirationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_timestamp: Option<f64>,
    /// <p>The unique identifier (ID) of a handshake. The originating account creates the ID when it initiates the handshake.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for handshake ID string requires "h-" followed by from 8 to 32 lowercase letters or digits.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Information about the two accounts that are participating in the handshake.</p>
    #[serde(rename = "Parties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parties: Option<Vec<HandshakeParty>>,
    /// <p>The date and time that the handshake request was made.</p>
    #[serde(rename = "RequestedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_timestamp: Option<f64>,
    /// <p>Additional information that is needed to process the handshake.</p>
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<HandshakeResource>>,
    /// <p><p>The current state of the handshake. Use the state to trace the flow of the handshake through the process from its creation to its acceptance. The meaning of each of the valid values is as follows:</p> <ul> <li> <p> <b>REQUESTED</b>: This handshake was sent to multiple recipients (applicable to only some handshake types) and not all recipients have responded yet. The request stays in this state until all recipients respond.</p> </li> <li> <p> <b>OPEN</b>: This handshake was sent to multiple recipients (applicable to only some policy types) and all recipients have responded, allowing the originator to complete the handshake action.</p> </li> <li> <p> <b>CANCELED</b>: This handshake is no longer active because it was canceled by the originating account.</p> </li> <li> <p> <b>ACCEPTED</b>: This handshake is complete because it has been accepted by the recipient.</p> </li> <li> <p> <b>DECLINED</b>: This handshake is no longer active because it was declined by the recipient account.</p> </li> <li> <p> <b>EXPIRED</b>: This handshake is no longer active because the originator did not receive a response of any kind from the recipient before the expiration time (15 days).</p> </li> </ul></p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>Specifies the criteria that are used to select the handshakes for the operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct HandshakeFilter {
    /// <p>Specifies the type of handshake action.</p> <p>If you specify <code>ActionType</code>, you cannot also specify <code>ParentHandshakeId</code>.</p>
    #[serde(rename = "ActionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    /// <p>Specifies the parent handshake. Only used for handshake types that are a child of another type.</p> <p>If you specify <code>ParentHandshakeId</code>, you cannot also specify <code>ActionType</code>.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for handshake ID string requires "h-" followed by from 8 to 32 lowercase letters or digits.</p>
    #[serde(rename = "ParentHandshakeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_handshake_id: Option<String>,
}

/// <p>Identifies a participant in a handshake.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HandshakeParty {
    /// <p>The unique identifier (ID) for the party.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for handshake ID string requires "h-" followed by from 8 to 32 lowercase letters or digits.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>The type of party.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>Contains additional data that is needed to process a handshake.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HandshakeResource {
    /// <p>When needed, contains an additional array of <code>HandshakeResource</code> objects.</p>
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<HandshakeResource>>,
    /// <p><p>The type of information being passed, specifying how the value is to be interpreted by the other party:</p> <ul> <li> <p> <code>ACCOUNT</code> - Specifies an AWS account ID number.</p> </li> <li> <p> <code>ORGANIZATION</code> - Specifies an organization ID number.</p> </li> <li> <p> <code>EMAIL</code> - Specifies the email address that is associated with the account that receives the handshake. </p> </li> <li> <p> <code>OWNER<em>EMAIL</code> - Specifies the email address associated with the management account. Included as information about an organization. </p> </li> <li> <p> <code>OWNER</em>NAME</code> - Specifies the name associated with the management account. Included as information about an organization. </p> </li> <li> <p> <code>NOTES</code> - Additional text provided by the handshake initiator and intended for the recipient to read.</p> </li> </ul></p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The information that is passed to the other party in the handshake. The format of the value string must match the requirements of the specified type.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InviteAccountToOrganizationRequest {
    /// <p>Additional information that you want to include in the generated email to the recipient account owner.</p>
    #[serde(rename = "Notes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// <p><p>A list of tags that you want to attach to the account when it becomes a member of the organization. For each tag in the list, you must specify both a tag key and a value. You can set the value to an empty string, but you can&#39;t set it to <code>null</code>. For more information about tagging, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_tagging.html">Tagging AWS Organizations resources</a> in the AWS Organizations User Guide.</p> <important> <p>Any tags in the request are checked for compliance with any applicable tag policies when the request is made. The request is rejected if the tags in the request don&#39;t match the requirements of the policy at that time. Tag policy compliance is <i> <b>not</b> </i> checked again when the invitation is accepted and the tags are actually attached to the account. That means that if the tag policy changes between the invitation and the acceptance, then that tags could potentially be non-compliant.</p> </important> <note> <p>If any one of the tags is invalid or if you exceed the allowed number of tags for an account, then the entire request fails and invitations are not sent.</p> </note></p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The identifier (ID) of the AWS account that you want to invite to join your organization. This is a JSON object that contains the following elements:</p> <p> <code>{ "Type": "ACCOUNT", "Id": "&lt;<i> <b>account id number</b> </i>&gt;" }</code> </p> <p>If you use the AWS CLI, you can submit this as a single string, similar to the following example:</p> <p> <code>--target Id=123456789012,Type=ACCOUNT</code> </p> <p>If you specify <code>"Type": "ACCOUNT"</code>, you must provide the AWS account ID number as the <code>Id</code>. If you specify <code>"Type": "EMAIL"</code>, you must specify the email address that is associated with the account.</p> <p> <code>--target Id=diego@example.com,Type=EMAIL</code> </p>
    #[serde(rename = "Target")]
    pub target: HandshakeParty,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InviteAccountToOrganizationResponse {
    /// <p>A structure that contains details about the handshake that is created to support this invitation request.</p>
    #[serde(rename = "Handshake")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake: Option<Handshake>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAWSServiceAccessForOrganizationRequest {
    /// <p>The total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAWSServiceAccessForOrganizationResponse {
    /// <p>A list of the service principals for the services that are enabled to integrate with your organization. Each principal is a structure that includes the name and the date that it was enabled for integration with AWS Organizations.</p>
    #[serde(rename = "EnabledServicePrincipals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_service_principals: Option<Vec<EnabledServicePrincipal>>,
    /// <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAccountsForParentRequest {
    /// <p>The total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The unique identifier (ID) for the parent root or organization unit (OU) whose accounts you want to list.</p>
    #[serde(rename = "ParentId")]
    pub parent_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAccountsForParentResponse {
    /// <p>A list of the accounts in the specified root or OU.</p>
    #[serde(rename = "Accounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<Account>>,
    /// <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAccountsRequest {
    /// <p>The total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAccountsResponse {
    /// <p>A list of objects in the organization.</p>
    #[serde(rename = "Accounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<Account>>,
    /// <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListChildrenRequest {
    /// <p>Filters the output to include only the specified child type.</p>
    #[serde(rename = "ChildType")]
    pub child_type: String,
    /// <p>The total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The unique identifier (ID) for the parent root or OU whose children you want to list.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a parent ID string requires one of the following:</p> <ul> <li> <p> <b>Root</b> - A string that begins with &quot;r-&quot; followed by from 4 to 32 lowercase letters or digits.</p> </li> <li> <p> <b>Organizational unit (OU)</b> - A string that begins with &quot;ou-&quot; followed by from 4 to 32 lowercase letters or digits (the ID of the root that the OU is in). This string is followed by a second &quot;-&quot; dash and from 8 to 32 additional lowercase letters or digits.</p> </li> </ul></p>
    #[serde(rename = "ParentId")]
    pub parent_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListChildrenResponse {
    /// <p>The list of children of the specified parent container.</p>
    #[serde(rename = "Children")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Child>>,
    /// <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListCreateAccountStatusRequest {
    /// <p>The total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of one or more states that you want included in the response. If this parameter isn't present, all requests are included in the response.</p>
    #[serde(rename = "States")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListCreateAccountStatusResponse {
    /// <p>A list of objects with details about the requests. Certain elements, such as the accountId number, are present in the output only after the account has been successfully created.</p>
    #[serde(rename = "CreateAccountStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_account_statuses: Option<Vec<CreateAccountStatus>>,
    /// <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDelegatedAdministratorsRequest {
    /// <p>The total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Specifies a service principal name. If specified, then the operation lists the delegated administrators only for the specified service.</p> <p>If you don't specify a service principal, the operation lists all delegated administrators for all services in your organization.</p>
    #[serde(rename = "ServicePrincipal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDelegatedAdministratorsResponse {
    /// <p>The list of delegated administrators in your organization.</p>
    #[serde(rename = "DelegatedAdministrators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegated_administrators: Option<Vec<DelegatedAdministrator>>,
    /// <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDelegatedServicesForAccountRequest {
    /// <p>The account ID number of a delegated administrator account in the organization.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDelegatedServicesForAccountResponse {
    /// <p>The services for which the account is a delegated administrator.</p>
    #[serde(rename = "DelegatedServices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegated_services: Option<Vec<DelegatedService>>,
    /// <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListHandshakesForAccountRequest {
    /// <p>Filters the handshakes that you want included in the response. The default is all types. Use the <code>ActionType</code> element to limit the output to only a specified type, such as <code>INVITE</code>, <code>ENABLE_ALL_FEATURES</code>, or <code>APPROVE_ALL_FEATURES</code>. Alternatively, for the <code>ENABLE_ALL_FEATURES</code> handshake that generates a separate child handshake for each member account, you can specify <code>ParentHandshakeId</code> to see only the handshakes that were generated by that parent request.</p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<HandshakeFilter>,
    /// <p>The total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListHandshakesForAccountResponse {
    /// <p>A list of <a>Handshake</a> objects with details about each of the handshakes that is associated with the specified account.</p>
    #[serde(rename = "Handshakes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshakes: Option<Vec<Handshake>>,
    /// <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListHandshakesForOrganizationRequest {
    /// <p>A filter of the handshakes that you want included in the response. The default is all types. Use the <code>ActionType</code> element to limit the output to only a specified type, such as <code>INVITE</code>, <code>ENABLE-ALL-FEATURES</code>, or <code>APPROVE-ALL-FEATURES</code>. Alternatively, for the <code>ENABLE-ALL-FEATURES</code> handshake that generates a separate child handshake for each member account, you can specify the <code>ParentHandshakeId</code> to see only the handshakes that were generated by that parent request.</p>
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<HandshakeFilter>,
    /// <p>The total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListHandshakesForOrganizationResponse {
    /// <p>A list of <a>Handshake</a> objects with details about each of the handshakes that are associated with an organization.</p>
    #[serde(rename = "Handshakes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshakes: Option<Vec<Handshake>>,
    /// <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListOrganizationalUnitsForParentRequest {
    /// <p>The total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The unique identifier (ID) of the root or OU whose child OUs you want to list.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a parent ID string requires one of the following:</p> <ul> <li> <p> <b>Root</b> - A string that begins with &quot;r-&quot; followed by from 4 to 32 lowercase letters or digits.</p> </li> <li> <p> <b>Organizational unit (OU)</b> - A string that begins with &quot;ou-&quot; followed by from 4 to 32 lowercase letters or digits (the ID of the root that the OU is in). This string is followed by a second &quot;-&quot; dash and from 8 to 32 additional lowercase letters or digits.</p> </li> </ul></p>
    #[serde(rename = "ParentId")]
    pub parent_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListOrganizationalUnitsForParentResponse {
    /// <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of the OUs in the specified root or parent OU.</p>
    #[serde(rename = "OrganizationalUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_units: Option<Vec<OrganizationalUnit>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListParentsRequest {
    /// <p><p>The unique identifier (ID) of the OU or account whose parent containers you want to list. Don&#39;t specify a root.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a child ID string requires one of the following:</p> <ul> <li> <p> <b>Account</b> - A string that consists of exactly 12 digits.</p> </li> <li> <p> <b>Organizational unit (OU)</b> - A string that begins with &quot;ou-&quot; followed by from 4 to 32 lowercase letters or digits (the ID of the root that contains the OU). This string is followed by a second &quot;-&quot; dash and from 8 to 32 additional lowercase letters or digits.</p> </li> </ul></p>
    #[serde(rename = "ChildId")]
    pub child_id: String,
    /// <p>The total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListParentsResponse {
    /// <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of parents for the specified child account or OU.</p>
    #[serde(rename = "Parents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<Parent>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPoliciesForTargetRequest {
    /// <p><p>The type of policy that you want to include in the returned list. You must specify one of the following values:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_ai-opt-out.html">AISERVICES<em>OPT</em>OUT<em>POLICY</a> </p> </li> <li> <p> &lt;a href=&quot;https://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>policies</em>backup.html&quot;&gt;BACKUP<em>POLICY</a> </p> </li> <li> <p> &lt;a href=&quot;https://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>policies</em>scp.html&quot;&gt;SERVICE<em>CONTROL</em>POLICY</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies.html">TAG_POLICY</a> </p> </li> </ul></p>
    #[serde(rename = "Filter")]
    pub filter: String,
    /// <p>The total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The unique identifier (ID) of the root, organizational unit, or account whose policies you want to list.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a target ID string requires one of the following:</p> <ul> <li> <p> <b>Root</b> - A string that begins with &quot;r-&quot; followed by from 4 to 32 lowercase letters or digits.</p> </li> <li> <p> <b>Account</b> - A string that consists of exactly 12 digits.</p> </li> <li> <p> <b>Organizational unit (OU)</b> - A string that begins with &quot;ou-&quot; followed by from 4 to 32 lowercase letters or digits (the ID of the root that the OU is in). This string is followed by a second &quot;-&quot; dash and from 8 to 32 additional lowercase letters or digits.</p> </li> </ul></p>
    #[serde(rename = "TargetId")]
    pub target_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPoliciesForTargetResponse {
    /// <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of policies that match the criteria in the request.</p>
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<PolicySummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPoliciesRequest {
    /// <p><p>Specifies the type of policy that you want to include in the response. You must specify one of the following values:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_ai-opt-out.html">AISERVICES<em>OPT</em>OUT<em>POLICY</a> </p> </li> <li> <p> &lt;a href=&quot;https://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>policies</em>backup.html&quot;&gt;BACKUP<em>POLICY</a> </p> </li> <li> <p> &lt;a href=&quot;https://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>policies</em>scp.html&quot;&gt;SERVICE<em>CONTROL</em>POLICY</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies.html">TAG_POLICY</a> </p> </li> </ul></p>
    #[serde(rename = "Filter")]
    pub filter: String,
    /// <p>The total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPoliciesResponse {
    /// <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of policies that match the filter criteria in the request. The output list doesn't include the policy contents. To see the content for a policy, see <a>DescribePolicy</a>.</p>
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<PolicySummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRootsRequest {
    /// <p>The total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRootsResponse {
    /// <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of roots that are defined in an organization.</p>
    #[serde(rename = "Roots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roots: Option<Vec<Root>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The ID of the resource with the tags to list.</p> <p>You can specify any of the following taggable resources.</p> <ul> <li> <p>AWS account – specify the account ID number.</p> </li> <li> <p>Organizational unit – specify the OU ID that begins with <code>ou-</code> and looks similar to: <code>ou-<i>1a2b-34uvwxyz</i> </code> </p> </li> <li> <p>Root – specify the root ID that begins with <code>r-</code> and looks similar to: <code>r-<i>1a2b</i> </code> </p> </li> <li> <p>Policy – specify the policy ID that begins with <code>p-</code> andlooks similar to: <code>p-<i>12abcdefg3</i> </code> </p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The tags that are assigned to the resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTargetsForPolicyRequest {
    /// <p>The total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that Organizations might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The unique identifier (ID) of the policy whose attachments you want to know.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a policy ID string requires "p-" followed by from 8 to 128 lowercase or uppercase letters, digits, or the underscore character (_).</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTargetsForPolicyResponse {
    /// <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of structures, each of which contains details about one of the entities to which the specified policy is attached.</p>
    #[serde(rename = "Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<PolicyTargetSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MoveAccountRequest {
    /// <p>The unique identifier (ID) of the account that you want to move.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an account ID string requires exactly 12 digits.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p><p>The unique identifier (ID) of the root or organizational unit that you want to move the account to.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a parent ID string requires one of the following:</p> <ul> <li> <p> <b>Root</b> - A string that begins with &quot;r-&quot; followed by from 4 to 32 lowercase letters or digits.</p> </li> <li> <p> <b>Organizational unit (OU)</b> - A string that begins with &quot;ou-&quot; followed by from 4 to 32 lowercase letters or digits (the ID of the root that the OU is in). This string is followed by a second &quot;-&quot; dash and from 8 to 32 additional lowercase letters or digits.</p> </li> </ul></p>
    #[serde(rename = "DestinationParentId")]
    pub destination_parent_id: String,
    /// <p><p>The unique identifier (ID) of the root or organizational unit that you want to move the account from.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a parent ID string requires one of the following:</p> <ul> <li> <p> <b>Root</b> - A string that begins with &quot;r-&quot; followed by from 4 to 32 lowercase letters or digits.</p> </li> <li> <p> <b>Organizational unit (OU)</b> - A string that begins with &quot;ou-&quot; followed by from 4 to 32 lowercase letters or digits (the ID of the root that the OU is in). This string is followed by a second &quot;-&quot; dash and from 8 to 32 additional lowercase letters or digits.</p> </li> </ul></p>
    #[serde(rename = "SourceParentId")]
    pub source_parent_id: String,
}

/// <p>Contains details about an organization. An organization is a collection of accounts that are centrally managed together using consolidated billing, organized hierarchically with organizational units (OUs), and controlled with policies .</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Organization {
    /// <p>The Amazon Resource Name (ARN) of an organization.</p> <p>For more information about ARNs in Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_permissions.html#orgs-permissions-arns">ARN Formats Supported by Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p><important> <p>Do not use. This field is deprecated and doesn&#39;t provide complete information about the policies in your organization.</p> </important> <p>To determine the policies that are enabled and available for use in your organization, use the <a>ListRoots</a> operation instead.</p></p>
    #[serde(rename = "AvailablePolicyTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_policy_types: Option<Vec<PolicyTypeSummary>>,
    /// <p>Specifies the functionality that currently is available to the organization. If set to "ALL", then all features are enabled and policies can be applied to accounts in the organization. If set to "CONSOLIDATED_BILLING", then only consolidated billing functionality is available. For more information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">Enabling All Features in Your Organization</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "FeatureSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_set: Option<String>,
    /// <p>The unique identifier (ID) of an organization.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an organization ID string requires "o-" followed by from 10 to 32 lowercase letters or digits.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the account that is designated as the management account for the organization.</p> <p>For more information about ARNs in Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_permissions.html#orgs-permissions-arns">ARN Formats Supported by Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "MasterAccountArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_account_arn: Option<String>,
    /// <p>The email address that is associated with the AWS account that is designated as the management account for the organization.</p>
    #[serde(rename = "MasterAccountEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_account_email: Option<String>,
    /// <p>The unique identifier (ID) of the management account of an organization.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an account ID string requires exactly 12 digits.</p>
    #[serde(rename = "MasterAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_account_id: Option<String>,
}

/// <p>Contains details about an organizational unit (OU). An OU is a container of AWS accounts within a root of an organization. Policies that are attached to an OU apply to all accounts contained in that OU and in any child OUs.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OrganizationalUnit {
    /// <p>The Amazon Resource Name (ARN) of this OU.</p> <p>For more information about ARNs in Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_permissions.html#orgs-permissions-arns">ARN Formats Supported by Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The unique identifier (ID) associated with this OU.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an organizational unit ID string requires "ou-" followed by from 4 to 32 lowercase letters or digits (the ID of the root that contains the OU). This string is followed by a second "-" dash and from 8 to 32 additional lowercase letters or digits.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The friendly name of this OU.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of any of the characters in the ASCII character range.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Contains information about either a root or an organizational unit (OU) that can contain OUs or accounts in an organization.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Parent {
    /// <p><p>The unique identifier (ID) of the parent entity.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a parent ID string requires one of the following:</p> <ul> <li> <p> <b>Root</b> - A string that begins with &quot;r-&quot; followed by from 4 to 32 lowercase letters or digits.</p> </li> <li> <p> <b>Organizational unit (OU)</b> - A string that begins with &quot;ou-&quot; followed by from 4 to 32 lowercase letters or digits (the ID of the root that the OU is in). This string is followed by a second &quot;-&quot; dash and from 8 to 32 additional lowercase letters or digits.</p> </li> </ul></p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The type of the parent entity.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Contains rules to be applied to the affected accounts. Policies can be attached directly to accounts, or to roots and OUs to affect all accounts in those hierarchies.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Policy {
    /// <p>The text content of the policy.</p>
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// <p>A structure that contains additional details about the policy.</p>
    #[serde(rename = "PolicySummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_summary: Option<PolicySummary>,
}

/// <p>Contains information about a policy, but does not include the content. To see the content of a policy, see <a>DescribePolicy</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PolicySummary {
    /// <p>The Amazon Resource Name (ARN) of the policy.</p> <p>For more information about ARNs in Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_permissions.html#orgs-permissions-arns">ARN Formats Supported by Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A boolean value that indicates whether the specified policy is an AWS managed policy. If true, then you can attach the policy to roots, OUs, or accounts, but you cannot edit it.</p>
    #[serde(rename = "AwsManaged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_managed: Option<bool>,
    /// <p>The description of the policy.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The unique identifier (ID) of the policy.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a policy ID string requires "p-" followed by from 8 to 128 lowercase or uppercase letters, digits, or the underscore character (_).</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The friendly name of the policy.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of any of the characters in the ASCII character range.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type of policy.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Contains information about a root, OU, or account that a policy is attached to.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PolicyTargetSummary {
    /// <p>The Amazon Resource Name (ARN) of the policy target.</p> <p>For more information about ARNs in Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_permissions.html#orgs-permissions-arns">ARN Formats Supported by Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The friendly name of the policy target.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of any of the characters in the ASCII character range.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The unique identifier (ID) of the policy target.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a target ID string requires one of the following:</p> <ul> <li> <p> <b>Root</b> - A string that begins with &quot;r-&quot; followed by from 4 to 32 lowercase letters or digits.</p> </li> <li> <p> <b>Account</b> - A string that consists of exactly 12 digits.</p> </li> <li> <p> <b>Organizational unit (OU)</b> - A string that begins with &quot;ou-&quot; followed by from 4 to 32 lowercase letters or digits (the ID of the root that the OU is in). This string is followed by a second &quot;-&quot; dash and from 8 to 32 additional lowercase letters or digits.</p> </li> </ul></p>
    #[serde(rename = "TargetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    /// <p>The type of the policy target.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Contains information about a policy type and its status in the associated root.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PolicyTypeSummary {
    /// <p>The status of the policy type as it relates to the associated root. To attach a policy of the specified type to a root or to an OU or account in that root, it must be available in the organization and enabled for that root.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The name of the policy type.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterDelegatedAdministratorRequest {
    /// <p>The account ID number of the member account in the organization to register as a delegated administrator.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The service principal of the AWS service for which you want to make the member account a delegated administrator.</p>
    #[serde(rename = "ServicePrincipal")]
    pub service_principal: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveAccountFromOrganizationRequest {
    /// <p>The unique identifier (ID) of the member account that you want to remove from the organization.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an account ID string requires exactly 12 digits.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
}

/// <p>Contains details about a root. A root is a top-level parent node in the hierarchy of an organization that can contain organizational units (OUs) and accounts. The root contains every AWS account in the organization.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Root {
    /// <p>The Amazon Resource Name (ARN) of the root.</p> <p>For more information about ARNs in Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_permissions.html#orgs-permissions-arns">ARN Formats Supported by Organizations</a> in the <i>AWS Organizations User Guide</i>.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The unique identifier (ID) for the root.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a root ID string requires "r-" followed by from 4 to 32 lowercase letters or digits.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The friendly name of the root.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of any of the characters in the ASCII character range.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The types of policies that are currently enabled for the root and therefore can be attached to the root or to its OUs or accounts.</p> <note> <p>Even if a policy type is shown as available in the organization, you can separately enable and disable them at the root level by using <a>EnablePolicyType</a> and <a>DisablePolicyType</a>. Use <a>DescribeOrganization</a> to see the availability of the policy types in that organization.</p> </note></p>
    #[serde(rename = "PolicyTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_types: Option<Vec<PolicyTypeSummary>>,
}

/// <p><p>A custom key-value pair associated with a resource within your organization.</p> <p>You can attach tags to any of the following organization resources.</p> <ul> <li> <p>AWS account</p> </li> <li> <p>Organizational unit (OU)</p> </li> <li> <p>Organization root</p> </li> <li> <p>Policy</p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>The key identifier, or name, of the tag.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The string value that's associated with the key of the tag. You can set the value of a tag to an empty string, but you can't set the value of a tag to null.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The ID of the resource to add a tag to.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p><p>A list of tags to add to the specified resource.</p> <p>You can specify any of the following taggable resources.</p> <ul> <li> <p>AWS account – specify the account ID number.</p> </li> <li> <p>Organizational unit – specify the OU ID that begins with <code>ou-</code> and looks similar to: <code>ou-<i>1a2b-34uvwxyz</i> </code> </p> </li> <li> <p>Root – specify the root ID that begins with <code>r-</code> and looks similar to: <code>r-<i>1a2b</i> </code> </p> </li> <li> <p>Policy – specify the policy ID that begins with <code>p-</code> andlooks similar to: <code>p-<i>12abcdefg3</i> </code> </p> </li> </ul> <p>For each tag in the list, you must specify both a tag key and a value. You can set the value to an empty string, but you can&#39;t set it to <code>null</code>.</p> <note> <p>If any one of the tags is invalid or if you exceed the allowed number of tags for an account user, then the entire request fails and the account is not created.</p> </note></p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p><p>The ID of the resource to remove a tag from.</p> <p>You can specify any of the following taggable resources.</p> <ul> <li> <p>AWS account – specify the account ID number.</p> </li> <li> <p>Organizational unit – specify the OU ID that begins with <code>ou-</code> and looks similar to: <code>ou-<i>1a2b-34uvwxyz</i> </code> </p> </li> <li> <p>Root – specify the root ID that begins with <code>r-</code> and looks similar to: <code>r-<i>1a2b</i> </code> </p> </li> <li> <p>Policy – specify the policy ID that begins with <code>p-</code> andlooks similar to: <code>p-<i>12abcdefg3</i> </code> </p> </li> </ul></p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The list of keys for tags to remove from the specified resource.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateOrganizationalUnitRequest {
    /// <p>The new name that you want to assign to the OU.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of any of the characters in the ASCII character range.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The unique identifier (ID) of the OU that you want to rename. You can get the ID from the <a>ListOrganizationalUnitsForParent</a> operation.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for an organizational unit ID string requires "ou-" followed by from 4 to 32 lowercase letters or digits (the ID of the root that contains the OU). This string is followed by a second "-" dash and from 8 to 32 additional lowercase letters or digits.</p>
    #[serde(rename = "OrganizationalUnitId")]
    pub organizational_unit_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateOrganizationalUnitResponse {
    /// <p>A structure that contains the details about the specified OU, including its new name.</p>
    #[serde(rename = "OrganizationalUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit: Option<OrganizationalUnit>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdatePolicyRequest {
    /// <p>If provided, the new content for the policy. The text must be correctly formatted JSON that complies with the syntax for the policy's type. For more information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_scp-syntax.html">Service Control Policy Syntax</a> in the <i>AWS Organizations User Guide.</i> </p>
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// <p>If provided, the new description for the policy.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>If provided, the new name for the policy.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of any of the characters in the ASCII character range.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The unique identifier (ID) of the policy that you want to update.</p> <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a policy ID string requires "p-" followed by from 8 to 128 lowercase or uppercase letters, digits, or the underscore character (_).</p>
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdatePolicyResponse {
    /// <p>A structure that contains details about the updated policy, showing the requested changes.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,
}

/// Errors returned by AcceptHandshake
#[derive(Debug, PartialEq)]
pub enum AcceptHandshakeError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>The operation that you attempted requires you to have the <code>iam:CreateServiceLinkedRole</code> for <code>organizations.amazonaws.com</code> permission so that AWS Organizations can create the required service-linked role. You don't have that permission.</p>
    AccessDeniedForDependency(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p>The specified handshake is already in the requested state. For example, you can't accept a handshake that was already accepted.</p>
    HandshakeAlreadyInState(String),
    /// <p><p>The requested operation would violate the constraint identified in the reason code.</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. Note that deleted and closed accounts still count toward your limit.</p> <important> <p>If you get this exception immediately after creating the organization, wait one hour and try again. If after an hour it continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>ALREADY</em>IN<em>AN</em>ORGANIZATION: The handshake request is invalid because the invited account is already a member of an organization.</p> </li> <li> <p>HANDSHAKE<em>RATE</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>INVITE</em>DISABLED<em>DURING</em>ENABLE<em>ALL</em>FEATURES: You can&#39;t issue new invitations to join an organization while it&#39;s in the process of enabling all features. You can resume inviting accounts after you finalize the process when all accounts have agreed to the change.</p> </li> <li> <p>ORGANIZATION<em>ALREADY</em>HAS<em>ALL</em>FEATURES: The handshake request is invalid because the organization has already enabled all features.</p> </li> <li> <p>ORGANIZATION<em>FROM</em>DIFFERENT<em>SELLER</em>OF<em>RECORD: The request failed because the account is from a different marketplace than the accounts in the organization. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be from the same marketplace.</p> </li> <li> <p>ORGANIZATION</em>MEMBERSHIP<em>CHANGE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to change the membership of an account too quickly after its previous change.</p> </li> <li> <p>PAYMENT<em>INSTRUMENT</em>REQUIRED: You can&#39;t complete the operation with an account that doesn&#39;t have a payment instrument, such as a credit card, associated with it.</p> </li> </ul></p>
    HandshakeConstraintViolation(String),
    /// <p>We can't find a handshake with the <code>HandshakeId</code> that you specified.</p>
    HandshakeNotFound(String),
    /// <p>You can't perform the operation on the handshake in its current state. For example, you can't cancel a handshake that was already accepted or accept a handshake that was already declined.</p>
    InvalidHandshakeTransition(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
}

impl AcceptHandshakeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AcceptHandshakeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(AcceptHandshakeError::AWSOrganizationsNotInUse(
                        err.msg,
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(AcceptHandshakeError::AccessDenied(err.msg))
                }
                "AccessDeniedForDependencyException" => {
                    return RusotoError::Service(AcceptHandshakeError::AccessDeniedForDependency(
                        err.msg,
                    ))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(AcceptHandshakeError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "HandshakeAlreadyInStateException" => {
                    return RusotoError::Service(AcceptHandshakeError::HandshakeAlreadyInState(
                        err.msg,
                    ))
                }
                "HandshakeConstraintViolationException" => {
                    return RusotoError::Service(
                        AcceptHandshakeError::HandshakeConstraintViolation(err.msg),
                    )
                }
                "HandshakeNotFoundException" => {
                    return RusotoError::Service(AcceptHandshakeError::HandshakeNotFound(err.msg))
                }
                "InvalidHandshakeTransitionException" => {
                    return RusotoError::Service(AcceptHandshakeError::InvalidHandshakeTransition(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(AcceptHandshakeError::InvalidInput(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(AcceptHandshakeError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AcceptHandshakeError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AcceptHandshakeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AcceptHandshakeError::AWSOrganizationsNotInUse(ref cause) => write!(f, "{}", cause),
            AcceptHandshakeError::AccessDenied(ref cause) => write!(f, "{}", cause),
            AcceptHandshakeError::AccessDeniedForDependency(ref cause) => write!(f, "{}", cause),
            AcceptHandshakeError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            AcceptHandshakeError::HandshakeAlreadyInState(ref cause) => write!(f, "{}", cause),
            AcceptHandshakeError::HandshakeConstraintViolation(ref cause) => write!(f, "{}", cause),
            AcceptHandshakeError::HandshakeNotFound(ref cause) => write!(f, "{}", cause),
            AcceptHandshakeError::InvalidHandshakeTransition(ref cause) => write!(f, "{}", cause),
            AcceptHandshakeError::InvalidInput(ref cause) => write!(f, "{}", cause),
            AcceptHandshakeError::Service(ref cause) => write!(f, "{}", cause),
            AcceptHandshakeError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AcceptHandshakeError {}
/// Errors returned by AttachPolicy
#[derive(Debug, PartialEq)]
pub enum AttachPolicyError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to remove the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>ORGANIZAION: You attempted to remove the management account from the organization. You can&#39;t remove the management account. Instead, after you remove all member accounts, delete the organization itself.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#orgs<em>manage</em>accounts<em>remove-from-master&quot;&gt;Removing a member account from your organization</a>in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>PHONE</em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#orgs_manage_accounts_remove-from-master">Removing a member account from your organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get this exception when running a command immediately after creating the organization, wait one hour and try again. After an hour, if the command continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>CANNOT</em>REGISTER<em>MASTER</em>AS<em>DELEGATED</em>ADMINISTRATOR: You attempted to register the management account of the organization as a delegated administrator for an AWS service integrated with Organizations. You can designate only a member account as a delegated administrator.</p> </li> <li> <p>CANNOT<em>REMOVE</em>DELEGATED<em>ADMINISTRATOR</em>FROM<em>ORG: You attempted to remove an account that is registered as a delegated administrator for a service integrated with your organization. To complete this operation, you must first deregister this account as a delegated administrator. </p> </li> <li> <p>CREATE</em>ORGANIZATION<em>IN</em>BILLING<em>MODE</em>UNSUPPORTED<em>REGION: To create an organization in the specified region, you must enable all features mode.</p> </li> <li> <p>DELEGATED</em>ADMINISTRATOR<em>EXISTS</em>FOR<em>THIS</em>SERVICE: You attempted to register an AWS account as a delegated administrator for an AWS service that already has a delegated administrator. To complete this operation, you must first deregister any existing delegated administrators for this service.</p> </li> <li> <p>EMAIL<em>VERIFICATION</em>CODE<em>EXPIRED: The email verification code is only valid for a limited period of time. You must resubmit the request and generate a new verfication code.</p> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s management account to the marketplace that corresponds to the management account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>BUSINESS</em>LICENSE: Applies only to the AWS Regions in China. To create an organization, the master must have an valid business license. For more information, contact customer support.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide a valid contact address and phone number for the management account. Then try the operation again.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>NOT<em>GOVCLOUD</em>ENABLED: To complete this operation, the management account must have an associated account in the AWS GovCloud (US-West) Region. For more information, see <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/govcloud-organizations.html">AWS Organizations</a> in the <i>AWS GovCloud User Guide.</i> </p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this management account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MAX<em>DELEGATED</em>ADMINISTRATORS<em>FOR</em>SERVICE<em>LIMIT</em>EXCEEDED: You attempted to register more delegated administrators than allowed for the service principal. </p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MAX</em>TAG<em>LIMIT</em>EXCEEDED: You have exceeded the number of tags allowed on this resource. </p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ORGANIZATION</em>NOT<em>IN</em>ALL<em>FEATURES</em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>OU<em>DEPTH</em>LIMIT<em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>OU</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>POLICY<em>CONTENT</em>LIMIT<em>EXCEEDED: You attempted to create a policy that is larger than the maximum size.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>TAG<em>POLICY</em>VIOLATION: You attempted to create or update a resource with tags that are not compliant with the tag policy requirements for this account.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p>The selected policy is already attached to the specified target.</p>
    DuplicatePolicyAttachment(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>Changes to the effective policy are in progress, and its contents can't be returned. Try the operation again later. </p>
    PolicyChangesInProgress(String),
    /// <p>We can't find a policy with the <code>PolicyId</code> that you specified.</p>
    PolicyNotFound(String),
    /// <p>The specified policy type isn't currently enabled in this root. You can't attach policies of the specified type to entities in a root until you enable that type in the root. For more information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">Enabling All Features in Your Organization</a> in the <i>AWS Organizations User Guide.</i> </p>
    PolicyTypeNotEnabled(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>We can't find a root, OU, account, or policy with the <code>TargetId</code> that you specified.</p>
    TargetNotFound(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
    /// <p>This action isn't available in the current AWS Region.</p>
    UnsupportedAPIEndpoint(String),
}

impl AttachPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AttachPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(AttachPolicyError::AWSOrganizationsNotInUse(
                        err.msg,
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(AttachPolicyError::AccessDenied(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(AttachPolicyError::ConcurrentModification(err.msg))
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(AttachPolicyError::ConstraintViolation(err.msg))
                }
                "DuplicatePolicyAttachmentException" => {
                    return RusotoError::Service(AttachPolicyError::DuplicatePolicyAttachment(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(AttachPolicyError::InvalidInput(err.msg))
                }
                "PolicyChangesInProgressException" => {
                    return RusotoError::Service(AttachPolicyError::PolicyChangesInProgress(
                        err.msg,
                    ))
                }
                "PolicyNotFoundException" => {
                    return RusotoError::Service(AttachPolicyError::PolicyNotFound(err.msg))
                }
                "PolicyTypeNotEnabledException" => {
                    return RusotoError::Service(AttachPolicyError::PolicyTypeNotEnabled(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(AttachPolicyError::Service(err.msg))
                }
                "TargetNotFoundException" => {
                    return RusotoError::Service(AttachPolicyError::TargetNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AttachPolicyError::TooManyRequests(err.msg))
                }
                "UnsupportedAPIEndpointException" => {
                    return RusotoError::Service(AttachPolicyError::UnsupportedAPIEndpoint(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AttachPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AttachPolicyError::AWSOrganizationsNotInUse(ref cause) => write!(f, "{}", cause),
            AttachPolicyError::AccessDenied(ref cause) => write!(f, "{}", cause),
            AttachPolicyError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            AttachPolicyError::ConstraintViolation(ref cause) => write!(f, "{}", cause),
            AttachPolicyError::DuplicatePolicyAttachment(ref cause) => write!(f, "{}", cause),
            AttachPolicyError::InvalidInput(ref cause) => write!(f, "{}", cause),
            AttachPolicyError::PolicyChangesInProgress(ref cause) => write!(f, "{}", cause),
            AttachPolicyError::PolicyNotFound(ref cause) => write!(f, "{}", cause),
            AttachPolicyError::PolicyTypeNotEnabled(ref cause) => write!(f, "{}", cause),
            AttachPolicyError::Service(ref cause) => write!(f, "{}", cause),
            AttachPolicyError::TargetNotFound(ref cause) => write!(f, "{}", cause),
            AttachPolicyError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            AttachPolicyError::UnsupportedAPIEndpoint(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AttachPolicyError {}
/// Errors returned by CancelHandshake
#[derive(Debug, PartialEq)]
pub enum CancelHandshakeError {
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p>The specified handshake is already in the requested state. For example, you can't accept a handshake that was already accepted.</p>
    HandshakeAlreadyInState(String),
    /// <p>We can't find a handshake with the <code>HandshakeId</code> that you specified.</p>
    HandshakeNotFound(String),
    /// <p>You can't perform the operation on the handshake in its current state. For example, you can't cancel a handshake that was already accepted or accept a handshake that was already declined.</p>
    InvalidHandshakeTransition(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
}

impl CancelHandshakeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelHandshakeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CancelHandshakeError::AccessDenied(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CancelHandshakeError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "HandshakeAlreadyInStateException" => {
                    return RusotoError::Service(CancelHandshakeError::HandshakeAlreadyInState(
                        err.msg,
                    ))
                }
                "HandshakeNotFoundException" => {
                    return RusotoError::Service(CancelHandshakeError::HandshakeNotFound(err.msg))
                }
                "InvalidHandshakeTransitionException" => {
                    return RusotoError::Service(CancelHandshakeError::InvalidHandshakeTransition(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CancelHandshakeError::InvalidInput(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CancelHandshakeError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CancelHandshakeError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CancelHandshakeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelHandshakeError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CancelHandshakeError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            CancelHandshakeError::HandshakeAlreadyInState(ref cause) => write!(f, "{}", cause),
            CancelHandshakeError::HandshakeNotFound(ref cause) => write!(f, "{}", cause),
            CancelHandshakeError::InvalidHandshakeTransition(ref cause) => write!(f, "{}", cause),
            CancelHandshakeError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CancelHandshakeError::Service(ref cause) => write!(f, "{}", cause),
            CancelHandshakeError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CancelHandshakeError {}
/// Errors returned by CreateAccount
#[derive(Debug, PartialEq)]
pub enum CreateAccountError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to remove the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>ORGANIZAION: You attempted to remove the management account from the organization. You can&#39;t remove the management account. Instead, after you remove all member accounts, delete the organization itself.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#orgs<em>manage</em>accounts<em>remove-from-master&quot;&gt;Removing a member account from your organization</a>in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>PHONE</em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#orgs_manage_accounts_remove-from-master">Removing a member account from your organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get this exception when running a command immediately after creating the organization, wait one hour and try again. After an hour, if the command continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>CANNOT</em>REGISTER<em>MASTER</em>AS<em>DELEGATED</em>ADMINISTRATOR: You attempted to register the management account of the organization as a delegated administrator for an AWS service integrated with Organizations. You can designate only a member account as a delegated administrator.</p> </li> <li> <p>CANNOT<em>REMOVE</em>DELEGATED<em>ADMINISTRATOR</em>FROM<em>ORG: You attempted to remove an account that is registered as a delegated administrator for a service integrated with your organization. To complete this operation, you must first deregister this account as a delegated administrator. </p> </li> <li> <p>CREATE</em>ORGANIZATION<em>IN</em>BILLING<em>MODE</em>UNSUPPORTED<em>REGION: To create an organization in the specified region, you must enable all features mode.</p> </li> <li> <p>DELEGATED</em>ADMINISTRATOR<em>EXISTS</em>FOR<em>THIS</em>SERVICE: You attempted to register an AWS account as a delegated administrator for an AWS service that already has a delegated administrator. To complete this operation, you must first deregister any existing delegated administrators for this service.</p> </li> <li> <p>EMAIL<em>VERIFICATION</em>CODE<em>EXPIRED: The email verification code is only valid for a limited period of time. You must resubmit the request and generate a new verfication code.</p> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s management account to the marketplace that corresponds to the management account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>BUSINESS</em>LICENSE: Applies only to the AWS Regions in China. To create an organization, the master must have an valid business license. For more information, contact customer support.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide a valid contact address and phone number for the management account. Then try the operation again.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>NOT<em>GOVCLOUD</em>ENABLED: To complete this operation, the management account must have an associated account in the AWS GovCloud (US-West) Region. For more information, see <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/govcloud-organizations.html">AWS Organizations</a> in the <i>AWS GovCloud User Guide.</i> </p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this management account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MAX<em>DELEGATED</em>ADMINISTRATORS<em>FOR</em>SERVICE<em>LIMIT</em>EXCEEDED: You attempted to register more delegated administrators than allowed for the service principal. </p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MAX</em>TAG<em>LIMIT</em>EXCEEDED: You have exceeded the number of tags allowed on this resource. </p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ORGANIZATION</em>NOT<em>IN</em>ALL<em>FEATURES</em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>OU<em>DEPTH</em>LIMIT<em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>OU</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>POLICY<em>CONTENT</em>LIMIT<em>EXCEEDED: You attempted to create a policy that is larger than the maximum size.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>TAG<em>POLICY</em>VIOLATION: You attempted to create or update a resource with tags that are not compliant with the tag policy requirements for this account.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p>AWS Organizations couldn't perform the operation because your organization hasn't finished initializing. This can take up to an hour. Try again later. If after one hour you continue to receive this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p>
    FinalizingOrganization(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
    /// <p>This action isn't available in the current AWS Region.</p>
    UnsupportedAPIEndpoint(String),
}

impl CreateAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAccountError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(CreateAccountError::AWSOrganizationsNotInUse(
                        err.msg,
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateAccountError::AccessDenied(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreateAccountError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(CreateAccountError::ConstraintViolation(err.msg))
                }
                "FinalizingOrganizationException" => {
                    return RusotoError::Service(CreateAccountError::FinalizingOrganization(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateAccountError::InvalidInput(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateAccountError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateAccountError::TooManyRequests(err.msg))
                }
                "UnsupportedAPIEndpointException" => {
                    return RusotoError::Service(CreateAccountError::UnsupportedAPIEndpoint(
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
impl fmt::Display for CreateAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAccountError::AWSOrganizationsNotInUse(ref cause) => write!(f, "{}", cause),
            CreateAccountError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateAccountError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            CreateAccountError::ConstraintViolation(ref cause) => write!(f, "{}", cause),
            CreateAccountError::FinalizingOrganization(ref cause) => write!(f, "{}", cause),
            CreateAccountError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateAccountError::Service(ref cause) => write!(f, "{}", cause),
            CreateAccountError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            CreateAccountError::UnsupportedAPIEndpoint(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateAccountError {}
/// Errors returned by CreateGovCloudAccount
#[derive(Debug, PartialEq)]
pub enum CreateGovCloudAccountError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to remove the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>ORGANIZAION: You attempted to remove the management account from the organization. You can&#39;t remove the management account. Instead, after you remove all member accounts, delete the organization itself.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#orgs<em>manage</em>accounts<em>remove-from-master&quot;&gt;Removing a member account from your organization</a>in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>PHONE</em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#orgs_manage_accounts_remove-from-master">Removing a member account from your organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get this exception when running a command immediately after creating the organization, wait one hour and try again. After an hour, if the command continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>CANNOT</em>REGISTER<em>MASTER</em>AS<em>DELEGATED</em>ADMINISTRATOR: You attempted to register the management account of the organization as a delegated administrator for an AWS service integrated with Organizations. You can designate only a member account as a delegated administrator.</p> </li> <li> <p>CANNOT<em>REMOVE</em>DELEGATED<em>ADMINISTRATOR</em>FROM<em>ORG: You attempted to remove an account that is registered as a delegated administrator for a service integrated with your organization. To complete this operation, you must first deregister this account as a delegated administrator. </p> </li> <li> <p>CREATE</em>ORGANIZATION<em>IN</em>BILLING<em>MODE</em>UNSUPPORTED<em>REGION: To create an organization in the specified region, you must enable all features mode.</p> </li> <li> <p>DELEGATED</em>ADMINISTRATOR<em>EXISTS</em>FOR<em>THIS</em>SERVICE: You attempted to register an AWS account as a delegated administrator for an AWS service that already has a delegated administrator. To complete this operation, you must first deregister any existing delegated administrators for this service.</p> </li> <li> <p>EMAIL<em>VERIFICATION</em>CODE<em>EXPIRED: The email verification code is only valid for a limited period of time. You must resubmit the request and generate a new verfication code.</p> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s management account to the marketplace that corresponds to the management account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>BUSINESS</em>LICENSE: Applies only to the AWS Regions in China. To create an organization, the master must have an valid business license. For more information, contact customer support.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide a valid contact address and phone number for the management account. Then try the operation again.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>NOT<em>GOVCLOUD</em>ENABLED: To complete this operation, the management account must have an associated account in the AWS GovCloud (US-West) Region. For more information, see <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/govcloud-organizations.html">AWS Organizations</a> in the <i>AWS GovCloud User Guide.</i> </p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this management account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MAX<em>DELEGATED</em>ADMINISTRATORS<em>FOR</em>SERVICE<em>LIMIT</em>EXCEEDED: You attempted to register more delegated administrators than allowed for the service principal. </p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MAX</em>TAG<em>LIMIT</em>EXCEEDED: You have exceeded the number of tags allowed on this resource. </p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ORGANIZATION</em>NOT<em>IN</em>ALL<em>FEATURES</em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>OU<em>DEPTH</em>LIMIT<em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>OU</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>POLICY<em>CONTENT</em>LIMIT<em>EXCEEDED: You attempted to create a policy that is larger than the maximum size.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>TAG<em>POLICY</em>VIOLATION: You attempted to create or update a resource with tags that are not compliant with the tag policy requirements for this account.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p>AWS Organizations couldn't perform the operation because your organization hasn't finished initializing. This can take up to an hour. Try again later. If after one hour you continue to receive this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p>
    FinalizingOrganization(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
    /// <p>This action isn't available in the current AWS Region.</p>
    UnsupportedAPIEndpoint(String),
}

impl CreateGovCloudAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateGovCloudAccountError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        CreateGovCloudAccountError::AWSOrganizationsNotInUse(err.msg),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateGovCloudAccountError::AccessDenied(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        CreateGovCloudAccountError::ConcurrentModification(err.msg),
                    )
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(CreateGovCloudAccountError::ConstraintViolation(
                        err.msg,
                    ))
                }
                "FinalizingOrganizationException" => {
                    return RusotoError::Service(
                        CreateGovCloudAccountError::FinalizingOrganization(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateGovCloudAccountError::InvalidInput(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateGovCloudAccountError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateGovCloudAccountError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnsupportedAPIEndpointException" => {
                    return RusotoError::Service(
                        CreateGovCloudAccountError::UnsupportedAPIEndpoint(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateGovCloudAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateGovCloudAccountError::AWSOrganizationsNotInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateGovCloudAccountError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateGovCloudAccountError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            CreateGovCloudAccountError::ConstraintViolation(ref cause) => write!(f, "{}", cause),
            CreateGovCloudAccountError::FinalizingOrganization(ref cause) => write!(f, "{}", cause),
            CreateGovCloudAccountError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateGovCloudAccountError::Service(ref cause) => write!(f, "{}", cause),
            CreateGovCloudAccountError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            CreateGovCloudAccountError::UnsupportedAPIEndpoint(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateGovCloudAccountError {}
/// Errors returned by CreateOrganization
#[derive(Debug, PartialEq)]
pub enum CreateOrganizationError {
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>The operation that you attempted requires you to have the <code>iam:CreateServiceLinkedRole</code> for <code>organizations.amazonaws.com</code> permission so that AWS Organizations can create the required service-linked role. You don't have that permission.</p>
    AccessDeniedForDependency(String),
    /// <p>This account is already a member of an organization. An account can belong to only one organization at a time.</p>
    AlreadyInOrganization(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to remove the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>ORGANIZAION: You attempted to remove the management account from the organization. You can&#39;t remove the management account. Instead, after you remove all member accounts, delete the organization itself.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#orgs<em>manage</em>accounts<em>remove-from-master&quot;&gt;Removing a member account from your organization</a>in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>PHONE</em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#orgs_manage_accounts_remove-from-master">Removing a member account from your organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get this exception when running a command immediately after creating the organization, wait one hour and try again. After an hour, if the command continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>CANNOT</em>REGISTER<em>MASTER</em>AS<em>DELEGATED</em>ADMINISTRATOR: You attempted to register the management account of the organization as a delegated administrator for an AWS service integrated with Organizations. You can designate only a member account as a delegated administrator.</p> </li> <li> <p>CANNOT<em>REMOVE</em>DELEGATED<em>ADMINISTRATOR</em>FROM<em>ORG: You attempted to remove an account that is registered as a delegated administrator for a service integrated with your organization. To complete this operation, you must first deregister this account as a delegated administrator. </p> </li> <li> <p>CREATE</em>ORGANIZATION<em>IN</em>BILLING<em>MODE</em>UNSUPPORTED<em>REGION: To create an organization in the specified region, you must enable all features mode.</p> </li> <li> <p>DELEGATED</em>ADMINISTRATOR<em>EXISTS</em>FOR<em>THIS</em>SERVICE: You attempted to register an AWS account as a delegated administrator for an AWS service that already has a delegated administrator. To complete this operation, you must first deregister any existing delegated administrators for this service.</p> </li> <li> <p>EMAIL<em>VERIFICATION</em>CODE<em>EXPIRED: The email verification code is only valid for a limited period of time. You must resubmit the request and generate a new verfication code.</p> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s management account to the marketplace that corresponds to the management account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>BUSINESS</em>LICENSE: Applies only to the AWS Regions in China. To create an organization, the master must have an valid business license. For more information, contact customer support.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide a valid contact address and phone number for the management account. Then try the operation again.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>NOT<em>GOVCLOUD</em>ENABLED: To complete this operation, the management account must have an associated account in the AWS GovCloud (US-West) Region. For more information, see <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/govcloud-organizations.html">AWS Organizations</a> in the <i>AWS GovCloud User Guide.</i> </p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this management account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MAX<em>DELEGATED</em>ADMINISTRATORS<em>FOR</em>SERVICE<em>LIMIT</em>EXCEEDED: You attempted to register more delegated administrators than allowed for the service principal. </p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MAX</em>TAG<em>LIMIT</em>EXCEEDED: You have exceeded the number of tags allowed on this resource. </p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ORGANIZATION</em>NOT<em>IN</em>ALL<em>FEATURES</em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>OU<em>DEPTH</em>LIMIT<em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>OU</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>POLICY<em>CONTENT</em>LIMIT<em>EXCEEDED: You attempted to create a policy that is larger than the maximum size.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>TAG<em>POLICY</em>VIOLATION: You attempted to create or update a resource with tags that are not compliant with the tag policy requirements for this account.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
}

impl CreateOrganizationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateOrganizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateOrganizationError::AccessDenied(err.msg))
                }
                "AccessDeniedForDependencyException" => {
                    return RusotoError::Service(
                        CreateOrganizationError::AccessDeniedForDependency(err.msg),
                    )
                }
                "AlreadyInOrganizationException" => {
                    return RusotoError::Service(CreateOrganizationError::AlreadyInOrganization(
                        err.msg,
                    ))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreateOrganizationError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(CreateOrganizationError::ConstraintViolation(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateOrganizationError::InvalidInput(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateOrganizationError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateOrganizationError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateOrganizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateOrganizationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateOrganizationError::AccessDeniedForDependency(ref cause) => write!(f, "{}", cause),
            CreateOrganizationError::AlreadyInOrganization(ref cause) => write!(f, "{}", cause),
            CreateOrganizationError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            CreateOrganizationError::ConstraintViolation(ref cause) => write!(f, "{}", cause),
            CreateOrganizationError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateOrganizationError::Service(ref cause) => write!(f, "{}", cause),
            CreateOrganizationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateOrganizationError {}
/// Errors returned by CreateOrganizationalUnit
#[derive(Debug, PartialEq)]
pub enum CreateOrganizationalUnitError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to remove the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>ORGANIZAION: You attempted to remove the management account from the organization. You can&#39;t remove the management account. Instead, after you remove all member accounts, delete the organization itself.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#orgs<em>manage</em>accounts<em>remove-from-master&quot;&gt;Removing a member account from your organization</a>in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>PHONE</em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#orgs_manage_accounts_remove-from-master">Removing a member account from your organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get this exception when running a command immediately after creating the organization, wait one hour and try again. After an hour, if the command continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>CANNOT</em>REGISTER<em>MASTER</em>AS<em>DELEGATED</em>ADMINISTRATOR: You attempted to register the management account of the organization as a delegated administrator for an AWS service integrated with Organizations. You can designate only a member account as a delegated administrator.</p> </li> <li> <p>CANNOT<em>REMOVE</em>DELEGATED<em>ADMINISTRATOR</em>FROM<em>ORG: You attempted to remove an account that is registered as a delegated administrator for a service integrated with your organization. To complete this operation, you must first deregister this account as a delegated administrator. </p> </li> <li> <p>CREATE</em>ORGANIZATION<em>IN</em>BILLING<em>MODE</em>UNSUPPORTED<em>REGION: To create an organization in the specified region, you must enable all features mode.</p> </li> <li> <p>DELEGATED</em>ADMINISTRATOR<em>EXISTS</em>FOR<em>THIS</em>SERVICE: You attempted to register an AWS account as a delegated administrator for an AWS service that already has a delegated administrator. To complete this operation, you must first deregister any existing delegated administrators for this service.</p> </li> <li> <p>EMAIL<em>VERIFICATION</em>CODE<em>EXPIRED: The email verification code is only valid for a limited period of time. You must resubmit the request and generate a new verfication code.</p> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s management account to the marketplace that corresponds to the management account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>BUSINESS</em>LICENSE: Applies only to the AWS Regions in China. To create an organization, the master must have an valid business license. For more information, contact customer support.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide a valid contact address and phone number for the management account. Then try the operation again.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>NOT<em>GOVCLOUD</em>ENABLED: To complete this operation, the management account must have an associated account in the AWS GovCloud (US-West) Region. For more information, see <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/govcloud-organizations.html">AWS Organizations</a> in the <i>AWS GovCloud User Guide.</i> </p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this management account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MAX<em>DELEGATED</em>ADMINISTRATORS<em>FOR</em>SERVICE<em>LIMIT</em>EXCEEDED: You attempted to register more delegated administrators than allowed for the service principal. </p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MAX</em>TAG<em>LIMIT</em>EXCEEDED: You have exceeded the number of tags allowed on this resource. </p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ORGANIZATION</em>NOT<em>IN</em>ALL<em>FEATURES</em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>OU<em>DEPTH</em>LIMIT<em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>OU</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>POLICY<em>CONTENT</em>LIMIT<em>EXCEEDED: You attempted to create a policy that is larger than the maximum size.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>TAG<em>POLICY</em>VIOLATION: You attempted to create or update a resource with tags that are not compliant with the tag policy requirements for this account.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p>An OU with the same name already exists.</p>
    DuplicateOrganizationalUnit(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>We can't find a root or OU with the <code>ParentId</code> that you specified.</p>
    ParentNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
}

impl CreateOrganizationalUnitError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateOrganizationalUnitError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        CreateOrganizationalUnitError::AWSOrganizationsNotInUse(err.msg),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateOrganizationalUnitError::AccessDenied(
                        err.msg,
                    ))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        CreateOrganizationalUnitError::ConcurrentModification(err.msg),
                    )
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(
                        CreateOrganizationalUnitError::ConstraintViolation(err.msg),
                    )
                }
                "DuplicateOrganizationalUnitException" => {
                    return RusotoError::Service(
                        CreateOrganizationalUnitError::DuplicateOrganizationalUnit(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateOrganizationalUnitError::InvalidInput(
                        err.msg,
                    ))
                }
                "ParentNotFoundException" => {
                    return RusotoError::Service(CreateOrganizationalUnitError::ParentNotFound(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateOrganizationalUnitError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateOrganizationalUnitError::TooManyRequests(
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
impl fmt::Display for CreateOrganizationalUnitError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateOrganizationalUnitError::AWSOrganizationsNotInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateOrganizationalUnitError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateOrganizationalUnitError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateOrganizationalUnitError::ConstraintViolation(ref cause) => write!(f, "{}", cause),
            CreateOrganizationalUnitError::DuplicateOrganizationalUnit(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateOrganizationalUnitError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateOrganizationalUnitError::ParentNotFound(ref cause) => write!(f, "{}", cause),
            CreateOrganizationalUnitError::Service(ref cause) => write!(f, "{}", cause),
            CreateOrganizationalUnitError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateOrganizationalUnitError {}
/// Errors returned by CreatePolicy
#[derive(Debug, PartialEq)]
pub enum CreatePolicyError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to remove the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>ORGANIZAION: You attempted to remove the management account from the organization. You can&#39;t remove the management account. Instead, after you remove all member accounts, delete the organization itself.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#orgs<em>manage</em>accounts<em>remove-from-master&quot;&gt;Removing a member account from your organization</a>in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>PHONE</em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#orgs_manage_accounts_remove-from-master">Removing a member account from your organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get this exception when running a command immediately after creating the organization, wait one hour and try again. After an hour, if the command continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>CANNOT</em>REGISTER<em>MASTER</em>AS<em>DELEGATED</em>ADMINISTRATOR: You attempted to register the management account of the organization as a delegated administrator for an AWS service integrated with Organizations. You can designate only a member account as a delegated administrator.</p> </li> <li> <p>CANNOT<em>REMOVE</em>DELEGATED<em>ADMINISTRATOR</em>FROM<em>ORG: You attempted to remove an account that is registered as a delegated administrator for a service integrated with your organization. To complete this operation, you must first deregister this account as a delegated administrator. </p> </li> <li> <p>CREATE</em>ORGANIZATION<em>IN</em>BILLING<em>MODE</em>UNSUPPORTED<em>REGION: To create an organization in the specified region, you must enable all features mode.</p> </li> <li> <p>DELEGATED</em>ADMINISTRATOR<em>EXISTS</em>FOR<em>THIS</em>SERVICE: You attempted to register an AWS account as a delegated administrator for an AWS service that already has a delegated administrator. To complete this operation, you must first deregister any existing delegated administrators for this service.</p> </li> <li> <p>EMAIL<em>VERIFICATION</em>CODE<em>EXPIRED: The email verification code is only valid for a limited period of time. You must resubmit the request and generate a new verfication code.</p> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s management account to the marketplace that corresponds to the management account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>BUSINESS</em>LICENSE: Applies only to the AWS Regions in China. To create an organization, the master must have an valid business license. For more information, contact customer support.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide a valid contact address and phone number for the management account. Then try the operation again.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>NOT<em>GOVCLOUD</em>ENABLED: To complete this operation, the management account must have an associated account in the AWS GovCloud (US-West) Region. For more information, see <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/govcloud-organizations.html">AWS Organizations</a> in the <i>AWS GovCloud User Guide.</i> </p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this management account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MAX<em>DELEGATED</em>ADMINISTRATORS<em>FOR</em>SERVICE<em>LIMIT</em>EXCEEDED: You attempted to register more delegated administrators than allowed for the service principal. </p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MAX</em>TAG<em>LIMIT</em>EXCEEDED: You have exceeded the number of tags allowed on this resource. </p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ORGANIZATION</em>NOT<em>IN</em>ALL<em>FEATURES</em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>OU<em>DEPTH</em>LIMIT<em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>OU</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>POLICY<em>CONTENT</em>LIMIT<em>EXCEEDED: You attempted to create a policy that is larger than the maximum size.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>TAG<em>POLICY</em>VIOLATION: You attempted to create or update a resource with tags that are not compliant with the tag policy requirements for this account.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p>A policy with the same name already exists.</p>
    DuplicatePolicy(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>The provided policy document doesn't meet the requirements of the specified policy type. For example, the syntax might be incorrect. For details about service control policy syntax, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_scp-syntax.html">Service Control Policy Syntax</a> in the <i>AWS Organizations User Guide.</i> </p>
    MalformedPolicyDocument(String),
    /// <p>You can't use the specified policy type with the feature set currently enabled for this organization. For example, you can enable SCPs only after you enable all features in the organization. For more information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies.html#enable_policies_on_root">Managing AWS Organizations Policies</a>in the <i>AWS Organizations User Guide.</i> </p>
    PolicyTypeNotAvailableForOrganization(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
    /// <p>This action isn't available in the current AWS Region.</p>
    UnsupportedAPIEndpoint(String),
}

impl CreatePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(CreatePolicyError::AWSOrganizationsNotInUse(
                        err.msg,
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(CreatePolicyError::AccessDenied(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreatePolicyError::ConcurrentModification(err.msg))
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(CreatePolicyError::ConstraintViolation(err.msg))
                }
                "DuplicatePolicyException" => {
                    return RusotoError::Service(CreatePolicyError::DuplicatePolicy(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreatePolicyError::InvalidInput(err.msg))
                }
                "MalformedPolicyDocumentException" => {
                    return RusotoError::Service(CreatePolicyError::MalformedPolicyDocument(
                        err.msg,
                    ))
                }
                "PolicyTypeNotAvailableForOrganizationException" => {
                    return RusotoError::Service(
                        CreatePolicyError::PolicyTypeNotAvailableForOrganization(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(CreatePolicyError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreatePolicyError::TooManyRequests(err.msg))
                }
                "UnsupportedAPIEndpointException" => {
                    return RusotoError::Service(CreatePolicyError::UnsupportedAPIEndpoint(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreatePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreatePolicyError::AWSOrganizationsNotInUse(ref cause) => write!(f, "{}", cause),
            CreatePolicyError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreatePolicyError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            CreatePolicyError::ConstraintViolation(ref cause) => write!(f, "{}", cause),
            CreatePolicyError::DuplicatePolicy(ref cause) => write!(f, "{}", cause),
            CreatePolicyError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreatePolicyError::MalformedPolicyDocument(ref cause) => write!(f, "{}", cause),
            CreatePolicyError::PolicyTypeNotAvailableForOrganization(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePolicyError::Service(ref cause) => write!(f, "{}", cause),
            CreatePolicyError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            CreatePolicyError::UnsupportedAPIEndpoint(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreatePolicyError {}
/// Errors returned by DeclineHandshake
#[derive(Debug, PartialEq)]
pub enum DeclineHandshakeError {
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p>The specified handshake is already in the requested state. For example, you can't accept a handshake that was already accepted.</p>
    HandshakeAlreadyInState(String),
    /// <p>We can't find a handshake with the <code>HandshakeId</code> that you specified.</p>
    HandshakeNotFound(String),
    /// <p>You can't perform the operation on the handshake in its current state. For example, you can't cancel a handshake that was already accepted or accept a handshake that was already declined.</p>
    InvalidHandshakeTransition(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
}

impl DeclineHandshakeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeclineHandshakeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeclineHandshakeError::AccessDenied(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeclineHandshakeError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "HandshakeAlreadyInStateException" => {
                    return RusotoError::Service(DeclineHandshakeError::HandshakeAlreadyInState(
                        err.msg,
                    ))
                }
                "HandshakeNotFoundException" => {
                    return RusotoError::Service(DeclineHandshakeError::HandshakeNotFound(err.msg))
                }
                "InvalidHandshakeTransitionException" => {
                    return RusotoError::Service(DeclineHandshakeError::InvalidHandshakeTransition(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeclineHandshakeError::InvalidInput(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeclineHandshakeError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeclineHandshakeError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeclineHandshakeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeclineHandshakeError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeclineHandshakeError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeclineHandshakeError::HandshakeAlreadyInState(ref cause) => write!(f, "{}", cause),
            DeclineHandshakeError::HandshakeNotFound(ref cause) => write!(f, "{}", cause),
            DeclineHandshakeError::InvalidHandshakeTransition(ref cause) => write!(f, "{}", cause),
            DeclineHandshakeError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeclineHandshakeError::Service(ref cause) => write!(f, "{}", cause),
            DeclineHandshakeError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeclineHandshakeError {}
/// Errors returned by DeleteOrganization
#[derive(Debug, PartialEq)]
pub enum DeleteOrganizationError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>The organization isn't empty. To delete an organization, you must first remove all accounts except the management account, delete all OUs, and delete all policies.</p>
    OrganizationNotEmpty(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
}

impl DeleteOrganizationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteOrganizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(DeleteOrganizationError::AWSOrganizationsNotInUse(
                        err.msg,
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteOrganizationError::AccessDenied(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteOrganizationError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteOrganizationError::InvalidInput(err.msg))
                }
                "OrganizationNotEmptyException" => {
                    return RusotoError::Service(DeleteOrganizationError::OrganizationNotEmpty(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteOrganizationError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteOrganizationError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteOrganizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteOrganizationError::AWSOrganizationsNotInUse(ref cause) => write!(f, "{}", cause),
            DeleteOrganizationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteOrganizationError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteOrganizationError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteOrganizationError::OrganizationNotEmpty(ref cause) => write!(f, "{}", cause),
            DeleteOrganizationError::Service(ref cause) => write!(f, "{}", cause),
            DeleteOrganizationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteOrganizationError {}
/// Errors returned by DeleteOrganizationalUnit
#[derive(Debug, PartialEq)]
pub enum DeleteOrganizationalUnitError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>The specified OU is not empty. Move all accounts to another root or to other OUs, remove all child OUs, and try the operation again.</p>
    OrganizationalUnitNotEmpty(String),
    /// <p>We can't find an OU with the <code>OrganizationalUnitId</code> that you specified.</p>
    OrganizationalUnitNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
}

impl DeleteOrganizationalUnitError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteOrganizationalUnitError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        DeleteOrganizationalUnitError::AWSOrganizationsNotInUse(err.msg),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteOrganizationalUnitError::AccessDenied(
                        err.msg,
                    ))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DeleteOrganizationalUnitError::ConcurrentModification(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteOrganizationalUnitError::InvalidInput(
                        err.msg,
                    ))
                }
                "OrganizationalUnitNotEmptyException" => {
                    return RusotoError::Service(
                        DeleteOrganizationalUnitError::OrganizationalUnitNotEmpty(err.msg),
                    )
                }
                "OrganizationalUnitNotFoundException" => {
                    return RusotoError::Service(
                        DeleteOrganizationalUnitError::OrganizationalUnitNotFound(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteOrganizationalUnitError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteOrganizationalUnitError::TooManyRequests(
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
impl fmt::Display for DeleteOrganizationalUnitError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteOrganizationalUnitError::AWSOrganizationsNotInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteOrganizationalUnitError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeleteOrganizationalUnitError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteOrganizationalUnitError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteOrganizationalUnitError::OrganizationalUnitNotEmpty(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteOrganizationalUnitError::OrganizationalUnitNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteOrganizationalUnitError::Service(ref cause) => write!(f, "{}", cause),
            DeleteOrganizationalUnitError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteOrganizationalUnitError {}
/// Errors returned by DeletePolicy
#[derive(Debug, PartialEq)]
pub enum DeletePolicyError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>The policy is attached to one or more entities. You must detach it from all roots, OUs, and accounts before performing this operation.</p>
    PolicyInUse(String),
    /// <p>We can't find a policy with the <code>PolicyId</code> that you specified.</p>
    PolicyNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
    /// <p>This action isn't available in the current AWS Region.</p>
    UnsupportedAPIEndpoint(String),
}

impl DeletePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(DeletePolicyError::AWSOrganizationsNotInUse(
                        err.msg,
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(DeletePolicyError::AccessDenied(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeletePolicyError::ConcurrentModification(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeletePolicyError::InvalidInput(err.msg))
                }
                "PolicyInUseException" => {
                    return RusotoError::Service(DeletePolicyError::PolicyInUse(err.msg))
                }
                "PolicyNotFoundException" => {
                    return RusotoError::Service(DeletePolicyError::PolicyNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeletePolicyError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeletePolicyError::TooManyRequests(err.msg))
                }
                "UnsupportedAPIEndpointException" => {
                    return RusotoError::Service(DeletePolicyError::UnsupportedAPIEndpoint(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeletePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeletePolicyError::AWSOrganizationsNotInUse(ref cause) => write!(f, "{}", cause),
            DeletePolicyError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DeletePolicyError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeletePolicyError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeletePolicyError::PolicyInUse(ref cause) => write!(f, "{}", cause),
            DeletePolicyError::PolicyNotFound(ref cause) => write!(f, "{}", cause),
            DeletePolicyError::Service(ref cause) => write!(f, "{}", cause),
            DeletePolicyError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DeletePolicyError::UnsupportedAPIEndpoint(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeletePolicyError {}
/// Errors returned by DeregisterDelegatedAdministrator
#[derive(Debug, PartialEq)]
pub enum DeregisterDelegatedAdministratorError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p> We can't find an AWS account with the <code>AccountId</code> that you specified, or the account whose credentials you used to make this request isn't a member of an organization.</p>
    AccountNotFound(String),
    /// <p>The specified account is not a delegated administrator for this AWS service. </p>
    AccountNotRegistered(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to remove the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>ORGANIZAION: You attempted to remove the management account from the organization. You can&#39;t remove the management account. Instead, after you remove all member accounts, delete the organization itself.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#orgs<em>manage</em>accounts<em>remove-from-master&quot;&gt;Removing a member account from your organization</a>in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>PHONE</em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#orgs_manage_accounts_remove-from-master">Removing a member account from your organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get this exception when running a command immediately after creating the organization, wait one hour and try again. After an hour, if the command continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>CANNOT</em>REGISTER<em>MASTER</em>AS<em>DELEGATED</em>ADMINISTRATOR: You attempted to register the management account of the organization as a delegated administrator for an AWS service integrated with Organizations. You can designate only a member account as a delegated administrator.</p> </li> <li> <p>CANNOT<em>REMOVE</em>DELEGATED<em>ADMINISTRATOR</em>FROM<em>ORG: You attempted to remove an account that is registered as a delegated administrator for a service integrated with your organization. To complete this operation, you must first deregister this account as a delegated administrator. </p> </li> <li> <p>CREATE</em>ORGANIZATION<em>IN</em>BILLING<em>MODE</em>UNSUPPORTED<em>REGION: To create an organization in the specified region, you must enable all features mode.</p> </li> <li> <p>DELEGATED</em>ADMINISTRATOR<em>EXISTS</em>FOR<em>THIS</em>SERVICE: You attempted to register an AWS account as a delegated administrator for an AWS service that already has a delegated administrator. To complete this operation, you must first deregister any existing delegated administrators for this service.</p> </li> <li> <p>EMAIL<em>VERIFICATION</em>CODE<em>EXPIRED: The email verification code is only valid for a limited period of time. You must resubmit the request and generate a new verfication code.</p> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s management account to the marketplace that corresponds to the management account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>BUSINESS</em>LICENSE: Applies only to the AWS Regions in China. To create an organization, the master must have an valid business license. For more information, contact customer support.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide a valid contact address and phone number for the management account. Then try the operation again.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>NOT<em>GOVCLOUD</em>ENABLED: To complete this operation, the management account must have an associated account in the AWS GovCloud (US-West) Region. For more information, see <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/govcloud-organizations.html">AWS Organizations</a> in the <i>AWS GovCloud User Guide.</i> </p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this management account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MAX<em>DELEGATED</em>ADMINISTRATORS<em>FOR</em>SERVICE<em>LIMIT</em>EXCEEDED: You attempted to register more delegated administrators than allowed for the service principal. </p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MAX</em>TAG<em>LIMIT</em>EXCEEDED: You have exceeded the number of tags allowed on this resource. </p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ORGANIZATION</em>NOT<em>IN</em>ALL<em>FEATURES</em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>OU<em>DEPTH</em>LIMIT<em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>OU</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>POLICY<em>CONTENT</em>LIMIT<em>EXCEEDED: You attempted to create a policy that is larger than the maximum size.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>TAG<em>POLICY</em>VIOLATION: You attempted to create or update a resource with tags that are not compliant with the tag policy requirements for this account.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
    /// <p>This action isn't available in the current AWS Region.</p>
    UnsupportedAPIEndpoint(String),
}

impl DeregisterDelegatedAdministratorError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeregisterDelegatedAdministratorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        DeregisterDelegatedAdministratorError::AWSOrganizationsNotInUse(err.msg),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        DeregisterDelegatedAdministratorError::AccessDenied(err.msg),
                    )
                }
                "AccountNotFoundException" => {
                    return RusotoError::Service(
                        DeregisterDelegatedAdministratorError::AccountNotFound(err.msg),
                    )
                }
                "AccountNotRegisteredException" => {
                    return RusotoError::Service(
                        DeregisterDelegatedAdministratorError::AccountNotRegistered(err.msg),
                    )
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DeregisterDelegatedAdministratorError::ConcurrentModification(err.msg),
                    )
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(
                        DeregisterDelegatedAdministratorError::ConstraintViolation(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        DeregisterDelegatedAdministratorError::InvalidInput(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(DeregisterDelegatedAdministratorError::Service(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DeregisterDelegatedAdministratorError::TooManyRequests(err.msg),
                    )
                }
                "UnsupportedAPIEndpointException" => {
                    return RusotoError::Service(
                        DeregisterDelegatedAdministratorError::UnsupportedAPIEndpoint(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeregisterDelegatedAdministratorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeregisterDelegatedAdministratorError::AWSOrganizationsNotInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            DeregisterDelegatedAdministratorError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            DeregisterDelegatedAdministratorError::AccountNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DeregisterDelegatedAdministratorError::AccountNotRegistered(ref cause) => {
                write!(f, "{}", cause)
            }
            DeregisterDelegatedAdministratorError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            DeregisterDelegatedAdministratorError::ConstraintViolation(ref cause) => {
                write!(f, "{}", cause)
            }
            DeregisterDelegatedAdministratorError::InvalidInput(ref cause) => {
                write!(f, "{}", cause)
            }
            DeregisterDelegatedAdministratorError::Service(ref cause) => write!(f, "{}", cause),
            DeregisterDelegatedAdministratorError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
            DeregisterDelegatedAdministratorError::UnsupportedAPIEndpoint(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeregisterDelegatedAdministratorError {}
/// Errors returned by DescribeAccount
#[derive(Debug, PartialEq)]
pub enum DescribeAccountError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p> We can't find an AWS account with the <code>AccountId</code> that you specified, or the account whose credentials you used to make this request isn't a member of an organization.</p>
    AccountNotFound(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
}

impl DescribeAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAccountError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(DescribeAccountError::AWSOrganizationsNotInUse(
                        err.msg,
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeAccountError::AccessDenied(err.msg))
                }
                "AccountNotFoundException" => {
                    return RusotoError::Service(DescribeAccountError::AccountNotFound(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeAccountError::InvalidInput(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribeAccountError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeAccountError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAccountError::AWSOrganizationsNotInUse(ref cause) => write!(f, "{}", cause),
            DescribeAccountError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeAccountError::AccountNotFound(ref cause) => write!(f, "{}", cause),
            DescribeAccountError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeAccountError::Service(ref cause) => write!(f, "{}", cause),
            DescribeAccountError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAccountError {}
/// Errors returned by DescribeCreateAccountStatus
#[derive(Debug, PartialEq)]
pub enum DescribeCreateAccountStatusError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>We can't find an create account request with the <code>CreateAccountRequestId</code> that you specified.</p>
    CreateAccountStatusNotFound(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
    /// <p>This action isn't available in the current AWS Region.</p>
    UnsupportedAPIEndpoint(String),
}

impl DescribeCreateAccountStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeCreateAccountStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        DescribeCreateAccountStatusError::AWSOrganizationsNotInUse(err.msg),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeCreateAccountStatusError::AccessDenied(
                        err.msg,
                    ))
                }
                "CreateAccountStatusNotFoundException" => {
                    return RusotoError::Service(
                        DescribeCreateAccountStatusError::CreateAccountStatusNotFound(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeCreateAccountStatusError::InvalidInput(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribeCreateAccountStatusError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeCreateAccountStatusError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnsupportedAPIEndpointException" => {
                    return RusotoError::Service(
                        DescribeCreateAccountStatusError::UnsupportedAPIEndpoint(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeCreateAccountStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeCreateAccountStatusError::AWSOrganizationsNotInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeCreateAccountStatusError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeCreateAccountStatusError::CreateAccountStatusNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeCreateAccountStatusError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeCreateAccountStatusError::Service(ref cause) => write!(f, "{}", cause),
            DescribeCreateAccountStatusError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DescribeCreateAccountStatusError::UnsupportedAPIEndpoint(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeCreateAccountStatusError {}
/// Errors returned by DescribeEffectivePolicy
#[derive(Debug, PartialEq)]
pub enum DescribeEffectivePolicyError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to remove the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>ORGANIZAION: You attempted to remove the management account from the organization. You can&#39;t remove the management account. Instead, after you remove all member accounts, delete the organization itself.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#orgs<em>manage</em>accounts<em>remove-from-master&quot;&gt;Removing a member account from your organization</a>in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>PHONE</em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#orgs_manage_accounts_remove-from-master">Removing a member account from your organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get this exception when running a command immediately after creating the organization, wait one hour and try again. After an hour, if the command continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>CANNOT</em>REGISTER<em>MASTER</em>AS<em>DELEGATED</em>ADMINISTRATOR: You attempted to register the management account of the organization as a delegated administrator for an AWS service integrated with Organizations. You can designate only a member account as a delegated administrator.</p> </li> <li> <p>CANNOT<em>REMOVE</em>DELEGATED<em>ADMINISTRATOR</em>FROM<em>ORG: You attempted to remove an account that is registered as a delegated administrator for a service integrated with your organization. To complete this operation, you must first deregister this account as a delegated administrator. </p> </li> <li> <p>CREATE</em>ORGANIZATION<em>IN</em>BILLING<em>MODE</em>UNSUPPORTED<em>REGION: To create an organization in the specified region, you must enable all features mode.</p> </li> <li> <p>DELEGATED</em>ADMINISTRATOR<em>EXISTS</em>FOR<em>THIS</em>SERVICE: You attempted to register an AWS account as a delegated administrator for an AWS service that already has a delegated administrator. To complete this operation, you must first deregister any existing delegated administrators for this service.</p> </li> <li> <p>EMAIL<em>VERIFICATION</em>CODE<em>EXPIRED: The email verification code is only valid for a limited period of time. You must resubmit the request and generate a new verfication code.</p> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s management account to the marketplace that corresponds to the management account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>BUSINESS</em>LICENSE: Applies only to the AWS Regions in China. To create an organization, the master must have an valid business license. For more information, contact customer support.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide a valid contact address and phone number for the management account. Then try the operation again.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>NOT<em>GOVCLOUD</em>ENABLED: To complete this operation, the management account must have an associated account in the AWS GovCloud (US-West) Region. For more information, see <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/govcloud-organizations.html">AWS Organizations</a> in the <i>AWS GovCloud User Guide.</i> </p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this management account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MAX<em>DELEGATED</em>ADMINISTRATORS<em>FOR</em>SERVICE<em>LIMIT</em>EXCEEDED: You attempted to register more delegated administrators than allowed for the service principal. </p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MAX</em>TAG<em>LIMIT</em>EXCEEDED: You have exceeded the number of tags allowed on this resource. </p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ORGANIZATION</em>NOT<em>IN</em>ALL<em>FEATURES</em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>OU<em>DEPTH</em>LIMIT<em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>OU</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>POLICY<em>CONTENT</em>LIMIT<em>EXCEEDED: You attempted to create a policy that is larger than the maximum size.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>TAG<em>POLICY</em>VIOLATION: You attempted to create or update a resource with tags that are not compliant with the tag policy requirements for this account.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p>If you ran this action on the management account, this policy type is not enabled. If you ran the action on a member account, the account doesn't have an effective policy of this type. Contact the administrator of your organization about attaching a policy of this type to the account. </p>
    EffectivePolicyNotFound(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>We can't find a root, OU, account, or policy with the <code>TargetId</code> that you specified.</p>
    TargetNotFound(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
    /// <p>This action isn't available in the current AWS Region.</p>
    UnsupportedAPIEndpoint(String),
}

impl DescribeEffectivePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEffectivePolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        DescribeEffectivePolicyError::AWSOrganizationsNotInUse(err.msg),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeEffectivePolicyError::AccessDenied(
                        err.msg,
                    ))
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(DescribeEffectivePolicyError::ConstraintViolation(
                        err.msg,
                    ))
                }
                "EffectivePolicyNotFoundException" => {
                    return RusotoError::Service(
                        DescribeEffectivePolicyError::EffectivePolicyNotFound(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeEffectivePolicyError::InvalidInput(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribeEffectivePolicyError::Service(err.msg))
                }
                "TargetNotFoundException" => {
                    return RusotoError::Service(DescribeEffectivePolicyError::TargetNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeEffectivePolicyError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnsupportedAPIEndpointException" => {
                    return RusotoError::Service(
                        DescribeEffectivePolicyError::UnsupportedAPIEndpoint(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeEffectivePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEffectivePolicyError::AWSOrganizationsNotInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeEffectivePolicyError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeEffectivePolicyError::ConstraintViolation(ref cause) => write!(f, "{}", cause),
            DescribeEffectivePolicyError::EffectivePolicyNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeEffectivePolicyError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeEffectivePolicyError::Service(ref cause) => write!(f, "{}", cause),
            DescribeEffectivePolicyError::TargetNotFound(ref cause) => write!(f, "{}", cause),
            DescribeEffectivePolicyError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DescribeEffectivePolicyError::UnsupportedAPIEndpoint(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeEffectivePolicyError {}
/// Errors returned by DescribeHandshake
#[derive(Debug, PartialEq)]
pub enum DescribeHandshakeError {
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p>We can't find a handshake with the <code>HandshakeId</code> that you specified.</p>
    HandshakeNotFound(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
}

impl DescribeHandshakeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeHandshakeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeHandshakeError::AccessDenied(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DescribeHandshakeError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "HandshakeNotFoundException" => {
                    return RusotoError::Service(DescribeHandshakeError::HandshakeNotFound(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeHandshakeError::InvalidInput(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribeHandshakeError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeHandshakeError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeHandshakeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeHandshakeError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeHandshakeError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DescribeHandshakeError::HandshakeNotFound(ref cause) => write!(f, "{}", cause),
            DescribeHandshakeError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeHandshakeError::Service(ref cause) => write!(f, "{}", cause),
            DescribeHandshakeError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeHandshakeError {}
/// Errors returned by DescribeOrganization
#[derive(Debug, PartialEq)]
pub enum DescribeOrganizationError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
}

impl DescribeOrganizationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeOrganizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        DescribeOrganizationError::AWSOrganizationsNotInUse(err.msg),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeOrganizationError::AccessDenied(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DescribeOrganizationError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribeOrganizationError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeOrganizationError::TooManyRequests(
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
impl fmt::Display for DescribeOrganizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeOrganizationError::AWSOrganizationsNotInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeOrganizationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeOrganizationError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DescribeOrganizationError::Service(ref cause) => write!(f, "{}", cause),
            DescribeOrganizationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeOrganizationError {}
/// Errors returned by DescribeOrganizationalUnit
#[derive(Debug, PartialEq)]
pub enum DescribeOrganizationalUnitError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>We can't find an OU with the <code>OrganizationalUnitId</code> that you specified.</p>
    OrganizationalUnitNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
}

impl DescribeOrganizationalUnitError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeOrganizationalUnitError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        DescribeOrganizationalUnitError::AWSOrganizationsNotInUse(err.msg),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeOrganizationalUnitError::AccessDenied(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeOrganizationalUnitError::InvalidInput(
                        err.msg,
                    ))
                }
                "OrganizationalUnitNotFoundException" => {
                    return RusotoError::Service(
                        DescribeOrganizationalUnitError::OrganizationalUnitNotFound(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribeOrganizationalUnitError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeOrganizationalUnitError::TooManyRequests(
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
impl fmt::Display for DescribeOrganizationalUnitError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeOrganizationalUnitError::AWSOrganizationsNotInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeOrganizationalUnitError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribeOrganizationalUnitError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribeOrganizationalUnitError::OrganizationalUnitNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeOrganizationalUnitError::Service(ref cause) => write!(f, "{}", cause),
            DescribeOrganizationalUnitError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeOrganizationalUnitError {}
/// Errors returned by DescribePolicy
#[derive(Debug, PartialEq)]
pub enum DescribePolicyError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>We can't find a policy with the <code>PolicyId</code> that you specified.</p>
    PolicyNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
    /// <p>This action isn't available in the current AWS Region.</p>
    UnsupportedAPIEndpoint(String),
}

impl DescribePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribePolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(DescribePolicyError::AWSOrganizationsNotInUse(
                        err.msg,
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribePolicyError::AccessDenied(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribePolicyError::InvalidInput(err.msg))
                }
                "PolicyNotFoundException" => {
                    return RusotoError::Service(DescribePolicyError::PolicyNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DescribePolicyError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribePolicyError::TooManyRequests(err.msg))
                }
                "UnsupportedAPIEndpointException" => {
                    return RusotoError::Service(DescribePolicyError::UnsupportedAPIEndpoint(
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
impl fmt::Display for DescribePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribePolicyError::AWSOrganizationsNotInUse(ref cause) => write!(f, "{}", cause),
            DescribePolicyError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DescribePolicyError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DescribePolicyError::PolicyNotFound(ref cause) => write!(f, "{}", cause),
            DescribePolicyError::Service(ref cause) => write!(f, "{}", cause),
            DescribePolicyError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DescribePolicyError::UnsupportedAPIEndpoint(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribePolicyError {}
/// Errors returned by DetachPolicy
#[derive(Debug, PartialEq)]
pub enum DetachPolicyError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to remove the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>ORGANIZAION: You attempted to remove the management account from the organization. You can&#39;t remove the management account. Instead, after you remove all member accounts, delete the organization itself.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#orgs<em>manage</em>accounts<em>remove-from-master&quot;&gt;Removing a member account from your organization</a>in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>PHONE</em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#orgs_manage_accounts_remove-from-master">Removing a member account from your organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get this exception when running a command immediately after creating the organization, wait one hour and try again. After an hour, if the command continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>CANNOT</em>REGISTER<em>MASTER</em>AS<em>DELEGATED</em>ADMINISTRATOR: You attempted to register the management account of the organization as a delegated administrator for an AWS service integrated with Organizations. You can designate only a member account as a delegated administrator.</p> </li> <li> <p>CANNOT<em>REMOVE</em>DELEGATED<em>ADMINISTRATOR</em>FROM<em>ORG: You attempted to remove an account that is registered as a delegated administrator for a service integrated with your organization. To complete this operation, you must first deregister this account as a delegated administrator. </p> </li> <li> <p>CREATE</em>ORGANIZATION<em>IN</em>BILLING<em>MODE</em>UNSUPPORTED<em>REGION: To create an organization in the specified region, you must enable all features mode.</p> </li> <li> <p>DELEGATED</em>ADMINISTRATOR<em>EXISTS</em>FOR<em>THIS</em>SERVICE: You attempted to register an AWS account as a delegated administrator for an AWS service that already has a delegated administrator. To complete this operation, you must first deregister any existing delegated administrators for this service.</p> </li> <li> <p>EMAIL<em>VERIFICATION</em>CODE<em>EXPIRED: The email verification code is only valid for a limited period of time. You must resubmit the request and generate a new verfication code.</p> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s management account to the marketplace that corresponds to the management account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>BUSINESS</em>LICENSE: Applies only to the AWS Regions in China. To create an organization, the master must have an valid business license. For more information, contact customer support.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide a valid contact address and phone number for the management account. Then try the operation again.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>NOT<em>GOVCLOUD</em>ENABLED: To complete this operation, the management account must have an associated account in the AWS GovCloud (US-West) Region. For more information, see <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/govcloud-organizations.html">AWS Organizations</a> in the <i>AWS GovCloud User Guide.</i> </p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this management account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MAX<em>DELEGATED</em>ADMINISTRATORS<em>FOR</em>SERVICE<em>LIMIT</em>EXCEEDED: You attempted to register more delegated administrators than allowed for the service principal. </p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MAX</em>TAG<em>LIMIT</em>EXCEEDED: You have exceeded the number of tags allowed on this resource. </p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ORGANIZATION</em>NOT<em>IN</em>ALL<em>FEATURES</em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>OU<em>DEPTH</em>LIMIT<em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>OU</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>POLICY<em>CONTENT</em>LIMIT<em>EXCEEDED: You attempted to create a policy that is larger than the maximum size.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>TAG<em>POLICY</em>VIOLATION: You attempted to create or update a resource with tags that are not compliant with the tag policy requirements for this account.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>Changes to the effective policy are in progress, and its contents can't be returned. Try the operation again later. </p>
    PolicyChangesInProgress(String),
    /// <p>The policy isn't attached to the specified target in the specified root.</p>
    PolicyNotAttached(String),
    /// <p>We can't find a policy with the <code>PolicyId</code> that you specified.</p>
    PolicyNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>We can't find a root, OU, account, or policy with the <code>TargetId</code> that you specified.</p>
    TargetNotFound(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
    /// <p>This action isn't available in the current AWS Region.</p>
    UnsupportedAPIEndpoint(String),
}

impl DetachPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetachPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(DetachPolicyError::AWSOrganizationsNotInUse(
                        err.msg,
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(DetachPolicyError::AccessDenied(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DetachPolicyError::ConcurrentModification(err.msg))
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(DetachPolicyError::ConstraintViolation(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DetachPolicyError::InvalidInput(err.msg))
                }
                "PolicyChangesInProgressException" => {
                    return RusotoError::Service(DetachPolicyError::PolicyChangesInProgress(
                        err.msg,
                    ))
                }
                "PolicyNotAttachedException" => {
                    return RusotoError::Service(DetachPolicyError::PolicyNotAttached(err.msg))
                }
                "PolicyNotFoundException" => {
                    return RusotoError::Service(DetachPolicyError::PolicyNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DetachPolicyError::Service(err.msg))
                }
                "TargetNotFoundException" => {
                    return RusotoError::Service(DetachPolicyError::TargetNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DetachPolicyError::TooManyRequests(err.msg))
                }
                "UnsupportedAPIEndpointException" => {
                    return RusotoError::Service(DetachPolicyError::UnsupportedAPIEndpoint(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DetachPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetachPolicyError::AWSOrganizationsNotInUse(ref cause) => write!(f, "{}", cause),
            DetachPolicyError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DetachPolicyError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DetachPolicyError::ConstraintViolation(ref cause) => write!(f, "{}", cause),
            DetachPolicyError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DetachPolicyError::PolicyChangesInProgress(ref cause) => write!(f, "{}", cause),
            DetachPolicyError::PolicyNotAttached(ref cause) => write!(f, "{}", cause),
            DetachPolicyError::PolicyNotFound(ref cause) => write!(f, "{}", cause),
            DetachPolicyError::Service(ref cause) => write!(f, "{}", cause),
            DetachPolicyError::TargetNotFound(ref cause) => write!(f, "{}", cause),
            DetachPolicyError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DetachPolicyError::UnsupportedAPIEndpoint(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DetachPolicyError {}
/// Errors returned by DisableAWSServiceAccess
#[derive(Debug, PartialEq)]
pub enum DisableAWSServiceAccessError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to remove the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>ORGANIZAION: You attempted to remove the management account from the organization. You can&#39;t remove the management account. Instead, after you remove all member accounts, delete the organization itself.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#orgs<em>manage</em>accounts<em>remove-from-master&quot;&gt;Removing a member account from your organization</a>in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>PHONE</em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#orgs_manage_accounts_remove-from-master">Removing a member account from your organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get this exception when running a command immediately after creating the organization, wait one hour and try again. After an hour, if the command continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>CANNOT</em>REGISTER<em>MASTER</em>AS<em>DELEGATED</em>ADMINISTRATOR: You attempted to register the management account of the organization as a delegated administrator for an AWS service integrated with Organizations. You can designate only a member account as a delegated administrator.</p> </li> <li> <p>CANNOT<em>REMOVE</em>DELEGATED<em>ADMINISTRATOR</em>FROM<em>ORG: You attempted to remove an account that is registered as a delegated administrator for a service integrated with your organization. To complete this operation, you must first deregister this account as a delegated administrator. </p> </li> <li> <p>CREATE</em>ORGANIZATION<em>IN</em>BILLING<em>MODE</em>UNSUPPORTED<em>REGION: To create an organization in the specified region, you must enable all features mode.</p> </li> <li> <p>DELEGATED</em>ADMINISTRATOR<em>EXISTS</em>FOR<em>THIS</em>SERVICE: You attempted to register an AWS account as a delegated administrator for an AWS service that already has a delegated administrator. To complete this operation, you must first deregister any existing delegated administrators for this service.</p> </li> <li> <p>EMAIL<em>VERIFICATION</em>CODE<em>EXPIRED: The email verification code is only valid for a limited period of time. You must resubmit the request and generate a new verfication code.</p> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s management account to the marketplace that corresponds to the management account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>BUSINESS</em>LICENSE: Applies only to the AWS Regions in China. To create an organization, the master must have an valid business license. For more information, contact customer support.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide a valid contact address and phone number for the management account. Then try the operation again.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>NOT<em>GOVCLOUD</em>ENABLED: To complete this operation, the management account must have an associated account in the AWS GovCloud (US-West) Region. For more information, see <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/govcloud-organizations.html">AWS Organizations</a> in the <i>AWS GovCloud User Guide.</i> </p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this management account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MAX<em>DELEGATED</em>ADMINISTRATORS<em>FOR</em>SERVICE<em>LIMIT</em>EXCEEDED: You attempted to register more delegated administrators than allowed for the service principal. </p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MAX</em>TAG<em>LIMIT</em>EXCEEDED: You have exceeded the number of tags allowed on this resource. </p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ORGANIZATION</em>NOT<em>IN</em>ALL<em>FEATURES</em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>OU<em>DEPTH</em>LIMIT<em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>OU</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>POLICY<em>CONTENT</em>LIMIT<em>EXCEEDED: You attempted to create a policy that is larger than the maximum size.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>TAG<em>POLICY</em>VIOLATION: You attempted to create or update a resource with tags that are not compliant with the tag policy requirements for this account.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
    /// <p>This action isn't available in the current AWS Region.</p>
    UnsupportedAPIEndpoint(String),
}

impl DisableAWSServiceAccessError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisableAWSServiceAccessError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        DisableAWSServiceAccessError::AWSOrganizationsNotInUse(err.msg),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(DisableAWSServiceAccessError::AccessDenied(
                        err.msg,
                    ))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DisableAWSServiceAccessError::ConcurrentModification(err.msg),
                    )
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(DisableAWSServiceAccessError::ConstraintViolation(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DisableAWSServiceAccessError::InvalidInput(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DisableAWSServiceAccessError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DisableAWSServiceAccessError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnsupportedAPIEndpointException" => {
                    return RusotoError::Service(
                        DisableAWSServiceAccessError::UnsupportedAPIEndpoint(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisableAWSServiceAccessError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisableAWSServiceAccessError::AWSOrganizationsNotInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            DisableAWSServiceAccessError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DisableAWSServiceAccessError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            DisableAWSServiceAccessError::ConstraintViolation(ref cause) => write!(f, "{}", cause),
            DisableAWSServiceAccessError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DisableAWSServiceAccessError::Service(ref cause) => write!(f, "{}", cause),
            DisableAWSServiceAccessError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DisableAWSServiceAccessError::UnsupportedAPIEndpoint(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisableAWSServiceAccessError {}
/// Errors returned by DisablePolicyType
#[derive(Debug, PartialEq)]
pub enum DisablePolicyTypeError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to remove the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>ORGANIZAION: You attempted to remove the management account from the organization. You can&#39;t remove the management account. Instead, after you remove all member accounts, delete the organization itself.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#orgs<em>manage</em>accounts<em>remove-from-master&quot;&gt;Removing a member account from your organization</a>in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>PHONE</em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#orgs_manage_accounts_remove-from-master">Removing a member account from your organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get this exception when running a command immediately after creating the organization, wait one hour and try again. After an hour, if the command continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>CANNOT</em>REGISTER<em>MASTER</em>AS<em>DELEGATED</em>ADMINISTRATOR: You attempted to register the management account of the organization as a delegated administrator for an AWS service integrated with Organizations. You can designate only a member account as a delegated administrator.</p> </li> <li> <p>CANNOT<em>REMOVE</em>DELEGATED<em>ADMINISTRATOR</em>FROM<em>ORG: You attempted to remove an account that is registered as a delegated administrator for a service integrated with your organization. To complete this operation, you must first deregister this account as a delegated administrator. </p> </li> <li> <p>CREATE</em>ORGANIZATION<em>IN</em>BILLING<em>MODE</em>UNSUPPORTED<em>REGION: To create an organization in the specified region, you must enable all features mode.</p> </li> <li> <p>DELEGATED</em>ADMINISTRATOR<em>EXISTS</em>FOR<em>THIS</em>SERVICE: You attempted to register an AWS account as a delegated administrator for an AWS service that already has a delegated administrator. To complete this operation, you must first deregister any existing delegated administrators for this service.</p> </li> <li> <p>EMAIL<em>VERIFICATION</em>CODE<em>EXPIRED: The email verification code is only valid for a limited period of time. You must resubmit the request and generate a new verfication code.</p> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s management account to the marketplace that corresponds to the management account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>BUSINESS</em>LICENSE: Applies only to the AWS Regions in China. To create an organization, the master must have an valid business license. For more information, contact customer support.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide a valid contact address and phone number for the management account. Then try the operation again.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>NOT<em>GOVCLOUD</em>ENABLED: To complete this operation, the management account must have an associated account in the AWS GovCloud (US-West) Region. For more information, see <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/govcloud-organizations.html">AWS Organizations</a> in the <i>AWS GovCloud User Guide.</i> </p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this management account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MAX<em>DELEGATED</em>ADMINISTRATORS<em>FOR</em>SERVICE<em>LIMIT</em>EXCEEDED: You attempted to register more delegated administrators than allowed for the service principal. </p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MAX</em>TAG<em>LIMIT</em>EXCEEDED: You have exceeded the number of tags allowed on this resource. </p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ORGANIZATION</em>NOT<em>IN</em>ALL<em>FEATURES</em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>OU<em>DEPTH</em>LIMIT<em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>OU</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>POLICY<em>CONTENT</em>LIMIT<em>EXCEEDED: You attempted to create a policy that is larger than the maximum size.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>TAG<em>POLICY</em>VIOLATION: You attempted to create or update a resource with tags that are not compliant with the tag policy requirements for this account.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>Changes to the effective policy are in progress, and its contents can't be returned. Try the operation again later. </p>
    PolicyChangesInProgress(String),
    /// <p>The specified policy type isn't currently enabled in this root. You can't attach policies of the specified type to entities in a root until you enable that type in the root. For more information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">Enabling All Features in Your Organization</a> in the <i>AWS Organizations User Guide.</i> </p>
    PolicyTypeNotEnabled(String),
    /// <p>We can't find a root with the <code>RootId</code> that you specified.</p>
    RootNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
    /// <p>This action isn't available in the current AWS Region.</p>
    UnsupportedAPIEndpoint(String),
}

impl DisablePolicyTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisablePolicyTypeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(DisablePolicyTypeError::AWSOrganizationsNotInUse(
                        err.msg,
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(DisablePolicyTypeError::AccessDenied(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DisablePolicyTypeError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(DisablePolicyTypeError::ConstraintViolation(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DisablePolicyTypeError::InvalidInput(err.msg))
                }
                "PolicyChangesInProgressException" => {
                    return RusotoError::Service(DisablePolicyTypeError::PolicyChangesInProgress(
                        err.msg,
                    ))
                }
                "PolicyTypeNotEnabledException" => {
                    return RusotoError::Service(DisablePolicyTypeError::PolicyTypeNotEnabled(
                        err.msg,
                    ))
                }
                "RootNotFoundException" => {
                    return RusotoError::Service(DisablePolicyTypeError::RootNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DisablePolicyTypeError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DisablePolicyTypeError::TooManyRequests(err.msg))
                }
                "UnsupportedAPIEndpointException" => {
                    return RusotoError::Service(DisablePolicyTypeError::UnsupportedAPIEndpoint(
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
impl fmt::Display for DisablePolicyTypeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisablePolicyTypeError::AWSOrganizationsNotInUse(ref cause) => write!(f, "{}", cause),
            DisablePolicyTypeError::AccessDenied(ref cause) => write!(f, "{}", cause),
            DisablePolicyTypeError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DisablePolicyTypeError::ConstraintViolation(ref cause) => write!(f, "{}", cause),
            DisablePolicyTypeError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DisablePolicyTypeError::PolicyChangesInProgress(ref cause) => write!(f, "{}", cause),
            DisablePolicyTypeError::PolicyTypeNotEnabled(ref cause) => write!(f, "{}", cause),
            DisablePolicyTypeError::RootNotFound(ref cause) => write!(f, "{}", cause),
            DisablePolicyTypeError::Service(ref cause) => write!(f, "{}", cause),
            DisablePolicyTypeError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            DisablePolicyTypeError::UnsupportedAPIEndpoint(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisablePolicyTypeError {}
/// Errors returned by EnableAWSServiceAccess
#[derive(Debug, PartialEq)]
pub enum EnableAWSServiceAccessError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to remove the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>ORGANIZAION: You attempted to remove the management account from the organization. You can&#39;t remove the management account. Instead, after you remove all member accounts, delete the organization itself.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#orgs<em>manage</em>accounts<em>remove-from-master&quot;&gt;Removing a member account from your organization</a>in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>PHONE</em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#orgs_manage_accounts_remove-from-master">Removing a member account from your organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get this exception when running a command immediately after creating the organization, wait one hour and try again. After an hour, if the command continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>CANNOT</em>REGISTER<em>MASTER</em>AS<em>DELEGATED</em>ADMINISTRATOR: You attempted to register the management account of the organization as a delegated administrator for an AWS service integrated with Organizations. You can designate only a member account as a delegated administrator.</p> </li> <li> <p>CANNOT<em>REMOVE</em>DELEGATED<em>ADMINISTRATOR</em>FROM<em>ORG: You attempted to remove an account that is registered as a delegated administrator for a service integrated with your organization. To complete this operation, you must first deregister this account as a delegated administrator. </p> </li> <li> <p>CREATE</em>ORGANIZATION<em>IN</em>BILLING<em>MODE</em>UNSUPPORTED<em>REGION: To create an organization in the specified region, you must enable all features mode.</p> </li> <li> <p>DELEGATED</em>ADMINISTRATOR<em>EXISTS</em>FOR<em>THIS</em>SERVICE: You attempted to register an AWS account as a delegated administrator for an AWS service that already has a delegated administrator. To complete this operation, you must first deregister any existing delegated administrators for this service.</p> </li> <li> <p>EMAIL<em>VERIFICATION</em>CODE<em>EXPIRED: The email verification code is only valid for a limited period of time. You must resubmit the request and generate a new verfication code.</p> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s management account to the marketplace that corresponds to the management account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>BUSINESS</em>LICENSE: Applies only to the AWS Regions in China. To create an organization, the master must have an valid business license. For more information, contact customer support.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide a valid contact address and phone number for the management account. Then try the operation again.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>NOT<em>GOVCLOUD</em>ENABLED: To complete this operation, the management account must have an associated account in the AWS GovCloud (US-West) Region. For more information, see <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/govcloud-organizations.html">AWS Organizations</a> in the <i>AWS GovCloud User Guide.</i> </p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this management account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MAX<em>DELEGATED</em>ADMINISTRATORS<em>FOR</em>SERVICE<em>LIMIT</em>EXCEEDED: You attempted to register more delegated administrators than allowed for the service principal. </p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MAX</em>TAG<em>LIMIT</em>EXCEEDED: You have exceeded the number of tags allowed on this resource. </p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ORGANIZATION</em>NOT<em>IN</em>ALL<em>FEATURES</em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>OU<em>DEPTH</em>LIMIT<em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>OU</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>POLICY<em>CONTENT</em>LIMIT<em>EXCEEDED: You attempted to create a policy that is larger than the maximum size.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>TAG<em>POLICY</em>VIOLATION: You attempted to create or update a resource with tags that are not compliant with the tag policy requirements for this account.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
    /// <p>This action isn't available in the current AWS Region.</p>
    UnsupportedAPIEndpoint(String),
}

impl EnableAWSServiceAccessError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnableAWSServiceAccessError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        EnableAWSServiceAccessError::AWSOrganizationsNotInUse(err.msg),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(EnableAWSServiceAccessError::AccessDenied(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        EnableAWSServiceAccessError::ConcurrentModification(err.msg),
                    )
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(EnableAWSServiceAccessError::ConstraintViolation(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(EnableAWSServiceAccessError::InvalidInput(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(EnableAWSServiceAccessError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(EnableAWSServiceAccessError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnsupportedAPIEndpointException" => {
                    return RusotoError::Service(
                        EnableAWSServiceAccessError::UnsupportedAPIEndpoint(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for EnableAWSServiceAccessError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableAWSServiceAccessError::AWSOrganizationsNotInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            EnableAWSServiceAccessError::AccessDenied(ref cause) => write!(f, "{}", cause),
            EnableAWSServiceAccessError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            EnableAWSServiceAccessError::ConstraintViolation(ref cause) => write!(f, "{}", cause),
            EnableAWSServiceAccessError::InvalidInput(ref cause) => write!(f, "{}", cause),
            EnableAWSServiceAccessError::Service(ref cause) => write!(f, "{}", cause),
            EnableAWSServiceAccessError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            EnableAWSServiceAccessError::UnsupportedAPIEndpoint(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for EnableAWSServiceAccessError {}
/// Errors returned by EnableAllFeatures
#[derive(Debug, PartialEq)]
pub enum EnableAllFeaturesError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>The requested operation would violate the constraint identified in the reason code.</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. Note that deleted and closed accounts still count toward your limit.</p> <important> <p>If you get this exception immediately after creating the organization, wait one hour and try again. If after an hour it continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>ALREADY</em>IN<em>AN</em>ORGANIZATION: The handshake request is invalid because the invited account is already a member of an organization.</p> </li> <li> <p>HANDSHAKE<em>RATE</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>INVITE</em>DISABLED<em>DURING</em>ENABLE<em>ALL</em>FEATURES: You can&#39;t issue new invitations to join an organization while it&#39;s in the process of enabling all features. You can resume inviting accounts after you finalize the process when all accounts have agreed to the change.</p> </li> <li> <p>ORGANIZATION<em>ALREADY</em>HAS<em>ALL</em>FEATURES: The handshake request is invalid because the organization has already enabled all features.</p> </li> <li> <p>ORGANIZATION<em>FROM</em>DIFFERENT<em>SELLER</em>OF<em>RECORD: The request failed because the account is from a different marketplace than the accounts in the organization. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be from the same marketplace.</p> </li> <li> <p>ORGANIZATION</em>MEMBERSHIP<em>CHANGE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to change the membership of an account too quickly after its previous change.</p> </li> <li> <p>PAYMENT<em>INSTRUMENT</em>REQUIRED: You can&#39;t complete the operation with an account that doesn&#39;t have a payment instrument, such as a credit card, associated with it.</p> </li> </ul></p>
    HandshakeConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
}

impl EnableAllFeaturesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnableAllFeaturesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(EnableAllFeaturesError::AWSOrganizationsNotInUse(
                        err.msg,
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(EnableAllFeaturesError::AccessDenied(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(EnableAllFeaturesError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "HandshakeConstraintViolationException" => {
                    return RusotoError::Service(
                        EnableAllFeaturesError::HandshakeConstraintViolation(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(EnableAllFeaturesError::InvalidInput(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(EnableAllFeaturesError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(EnableAllFeaturesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for EnableAllFeaturesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableAllFeaturesError::AWSOrganizationsNotInUse(ref cause) => write!(f, "{}", cause),
            EnableAllFeaturesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            EnableAllFeaturesError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            EnableAllFeaturesError::HandshakeConstraintViolation(ref cause) => {
                write!(f, "{}", cause)
            }
            EnableAllFeaturesError::InvalidInput(ref cause) => write!(f, "{}", cause),
            EnableAllFeaturesError::Service(ref cause) => write!(f, "{}", cause),
            EnableAllFeaturesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for EnableAllFeaturesError {}
/// Errors returned by EnablePolicyType
#[derive(Debug, PartialEq)]
pub enum EnablePolicyTypeError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to remove the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>ORGANIZAION: You attempted to remove the management account from the organization. You can&#39;t remove the management account. Instead, after you remove all member accounts, delete the organization itself.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#orgs<em>manage</em>accounts<em>remove-from-master&quot;&gt;Removing a member account from your organization</a>in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>PHONE</em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#orgs_manage_accounts_remove-from-master">Removing a member account from your organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get this exception when running a command immediately after creating the organization, wait one hour and try again. After an hour, if the command continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>CANNOT</em>REGISTER<em>MASTER</em>AS<em>DELEGATED</em>ADMINISTRATOR: You attempted to register the management account of the organization as a delegated administrator for an AWS service integrated with Organizations. You can designate only a member account as a delegated administrator.</p> </li> <li> <p>CANNOT<em>REMOVE</em>DELEGATED<em>ADMINISTRATOR</em>FROM<em>ORG: You attempted to remove an account that is registered as a delegated administrator for a service integrated with your organization. To complete this operation, you must first deregister this account as a delegated administrator. </p> </li> <li> <p>CREATE</em>ORGANIZATION<em>IN</em>BILLING<em>MODE</em>UNSUPPORTED<em>REGION: To create an organization in the specified region, you must enable all features mode.</p> </li> <li> <p>DELEGATED</em>ADMINISTRATOR<em>EXISTS</em>FOR<em>THIS</em>SERVICE: You attempted to register an AWS account as a delegated administrator for an AWS service that already has a delegated administrator. To complete this operation, you must first deregister any existing delegated administrators for this service.</p> </li> <li> <p>EMAIL<em>VERIFICATION</em>CODE<em>EXPIRED: The email verification code is only valid for a limited period of time. You must resubmit the request and generate a new verfication code.</p> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s management account to the marketplace that corresponds to the management account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>BUSINESS</em>LICENSE: Applies only to the AWS Regions in China. To create an organization, the master must have an valid business license. For more information, contact customer support.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide a valid contact address and phone number for the management account. Then try the operation again.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>NOT<em>GOVCLOUD</em>ENABLED: To complete this operation, the management account must have an associated account in the AWS GovCloud (US-West) Region. For more information, see <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/govcloud-organizations.html">AWS Organizations</a> in the <i>AWS GovCloud User Guide.</i> </p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this management account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MAX<em>DELEGATED</em>ADMINISTRATORS<em>FOR</em>SERVICE<em>LIMIT</em>EXCEEDED: You attempted to register more delegated administrators than allowed for the service principal. </p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MAX</em>TAG<em>LIMIT</em>EXCEEDED: You have exceeded the number of tags allowed on this resource. </p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ORGANIZATION</em>NOT<em>IN</em>ALL<em>FEATURES</em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>OU<em>DEPTH</em>LIMIT<em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>OU</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>POLICY<em>CONTENT</em>LIMIT<em>EXCEEDED: You attempted to create a policy that is larger than the maximum size.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>TAG<em>POLICY</em>VIOLATION: You attempted to create or update a resource with tags that are not compliant with the tag policy requirements for this account.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>Changes to the effective policy are in progress, and its contents can't be returned. Try the operation again later. </p>
    PolicyChangesInProgress(String),
    /// <p>The specified policy type is already enabled in the specified root.</p>
    PolicyTypeAlreadyEnabled(String),
    /// <p>You can't use the specified policy type with the feature set currently enabled for this organization. For example, you can enable SCPs only after you enable all features in the organization. For more information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies.html#enable_policies_on_root">Managing AWS Organizations Policies</a>in the <i>AWS Organizations User Guide.</i> </p>
    PolicyTypeNotAvailableForOrganization(String),
    /// <p>We can't find a root with the <code>RootId</code> that you specified.</p>
    RootNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
    /// <p>This action isn't available in the current AWS Region.</p>
    UnsupportedAPIEndpoint(String),
}

impl EnablePolicyTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnablePolicyTypeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(EnablePolicyTypeError::AWSOrganizationsNotInUse(
                        err.msg,
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(EnablePolicyTypeError::AccessDenied(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(EnablePolicyTypeError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(EnablePolicyTypeError::ConstraintViolation(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(EnablePolicyTypeError::InvalidInput(err.msg))
                }
                "PolicyChangesInProgressException" => {
                    return RusotoError::Service(EnablePolicyTypeError::PolicyChangesInProgress(
                        err.msg,
                    ))
                }
                "PolicyTypeAlreadyEnabledException" => {
                    return RusotoError::Service(EnablePolicyTypeError::PolicyTypeAlreadyEnabled(
                        err.msg,
                    ))
                }
                "PolicyTypeNotAvailableForOrganizationException" => {
                    return RusotoError::Service(
                        EnablePolicyTypeError::PolicyTypeNotAvailableForOrganization(err.msg),
                    )
                }
                "RootNotFoundException" => {
                    return RusotoError::Service(EnablePolicyTypeError::RootNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(EnablePolicyTypeError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(EnablePolicyTypeError::TooManyRequests(err.msg))
                }
                "UnsupportedAPIEndpointException" => {
                    return RusotoError::Service(EnablePolicyTypeError::UnsupportedAPIEndpoint(
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
impl fmt::Display for EnablePolicyTypeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnablePolicyTypeError::AWSOrganizationsNotInUse(ref cause) => write!(f, "{}", cause),
            EnablePolicyTypeError::AccessDenied(ref cause) => write!(f, "{}", cause),
            EnablePolicyTypeError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            EnablePolicyTypeError::ConstraintViolation(ref cause) => write!(f, "{}", cause),
            EnablePolicyTypeError::InvalidInput(ref cause) => write!(f, "{}", cause),
            EnablePolicyTypeError::PolicyChangesInProgress(ref cause) => write!(f, "{}", cause),
            EnablePolicyTypeError::PolicyTypeAlreadyEnabled(ref cause) => write!(f, "{}", cause),
            EnablePolicyTypeError::PolicyTypeNotAvailableForOrganization(ref cause) => {
                write!(f, "{}", cause)
            }
            EnablePolicyTypeError::RootNotFound(ref cause) => write!(f, "{}", cause),
            EnablePolicyTypeError::Service(ref cause) => write!(f, "{}", cause),
            EnablePolicyTypeError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            EnablePolicyTypeError::UnsupportedAPIEndpoint(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for EnablePolicyTypeError {}
/// Errors returned by InviteAccountToOrganization
#[derive(Debug, PartialEq)]
pub enum InviteAccountToOrganizationError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>You can't invite an existing account to your organization until you verify that you own the email address associated with the management account. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_create.html#about-email-verification">Email Address Verification</a> in the <i>AWS Organizations User Guide.</i> </p>
    AccountOwnerNotVerified(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to remove the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>ORGANIZAION: You attempted to remove the management account from the organization. You can&#39;t remove the management account. Instead, after you remove all member accounts, delete the organization itself.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#orgs<em>manage</em>accounts<em>remove-from-master&quot;&gt;Removing a member account from your organization</a>in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>PHONE</em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#orgs_manage_accounts_remove-from-master">Removing a member account from your organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get this exception when running a command immediately after creating the organization, wait one hour and try again. After an hour, if the command continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>CANNOT</em>REGISTER<em>MASTER</em>AS<em>DELEGATED</em>ADMINISTRATOR: You attempted to register the management account of the organization as a delegated administrator for an AWS service integrated with Organizations. You can designate only a member account as a delegated administrator.</p> </li> <li> <p>CANNOT<em>REMOVE</em>DELEGATED<em>ADMINISTRATOR</em>FROM<em>ORG: You attempted to remove an account that is registered as a delegated administrator for a service integrated with your organization. To complete this operation, you must first deregister this account as a delegated administrator. </p> </li> <li> <p>CREATE</em>ORGANIZATION<em>IN</em>BILLING<em>MODE</em>UNSUPPORTED<em>REGION: To create an organization in the specified region, you must enable all features mode.</p> </li> <li> <p>DELEGATED</em>ADMINISTRATOR<em>EXISTS</em>FOR<em>THIS</em>SERVICE: You attempted to register an AWS account as a delegated administrator for an AWS service that already has a delegated administrator. To complete this operation, you must first deregister any existing delegated administrators for this service.</p> </li> <li> <p>EMAIL<em>VERIFICATION</em>CODE<em>EXPIRED: The email verification code is only valid for a limited period of time. You must resubmit the request and generate a new verfication code.</p> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s management account to the marketplace that corresponds to the management account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>BUSINESS</em>LICENSE: Applies only to the AWS Regions in China. To create an organization, the master must have an valid business license. For more information, contact customer support.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide a valid contact address and phone number for the management account. Then try the operation again.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>NOT<em>GOVCLOUD</em>ENABLED: To complete this operation, the management account must have an associated account in the AWS GovCloud (US-West) Region. For more information, see <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/govcloud-organizations.html">AWS Organizations</a> in the <i>AWS GovCloud User Guide.</i> </p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this management account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MAX<em>DELEGATED</em>ADMINISTRATORS<em>FOR</em>SERVICE<em>LIMIT</em>EXCEEDED: You attempted to register more delegated administrators than allowed for the service principal. </p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MAX</em>TAG<em>LIMIT</em>EXCEEDED: You have exceeded the number of tags allowed on this resource. </p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ORGANIZATION</em>NOT<em>IN</em>ALL<em>FEATURES</em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>OU<em>DEPTH</em>LIMIT<em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>OU</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>POLICY<em>CONTENT</em>LIMIT<em>EXCEEDED: You attempted to create a policy that is larger than the maximum size.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>TAG<em>POLICY</em>VIOLATION: You attempted to create or update a resource with tags that are not compliant with the tag policy requirements for this account.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p>A handshake with the same action and target already exists. For example, if you invited an account to join your organization, the invited account might already have a pending invitation from this organization. If you intend to resend an invitation to an account, ensure that existing handshakes that might be considered duplicates are canceled or declined.</p>
    DuplicateHandshake(String),
    /// <p>AWS Organizations couldn't perform the operation because your organization hasn't finished initializing. This can take up to an hour. Try again later. If after one hour you continue to receive this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p>
    FinalizingOrganization(String),
    /// <p><p>The requested operation would violate the constraint identified in the reason code.</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation:</p> </note> <ul> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. Note that deleted and closed accounts still count toward your limit.</p> <important> <p>If you get this exception immediately after creating the organization, wait one hour and try again. If after an hour it continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>ALREADY</em>IN<em>AN</em>ORGANIZATION: The handshake request is invalid because the invited account is already a member of an organization.</p> </li> <li> <p>HANDSHAKE<em>RATE</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>INVITE</em>DISABLED<em>DURING</em>ENABLE<em>ALL</em>FEATURES: You can&#39;t issue new invitations to join an organization while it&#39;s in the process of enabling all features. You can resume inviting accounts after you finalize the process when all accounts have agreed to the change.</p> </li> <li> <p>ORGANIZATION<em>ALREADY</em>HAS<em>ALL</em>FEATURES: The handshake request is invalid because the organization has already enabled all features.</p> </li> <li> <p>ORGANIZATION<em>FROM</em>DIFFERENT<em>SELLER</em>OF<em>RECORD: The request failed because the account is from a different marketplace than the accounts in the organization. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be from the same marketplace.</p> </li> <li> <p>ORGANIZATION</em>MEMBERSHIP<em>CHANGE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to change the membership of an account too quickly after its previous change.</p> </li> <li> <p>PAYMENT<em>INSTRUMENT</em>REQUIRED: You can&#39;t complete the operation with an account that doesn&#39;t have a payment instrument, such as a credit card, associated with it.</p> </li> </ul></p>
    HandshakeConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
}

impl InviteAccountToOrganizationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<InviteAccountToOrganizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        InviteAccountToOrganizationError::AWSOrganizationsNotInUse(err.msg),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(InviteAccountToOrganizationError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountOwnerNotVerifiedException" => {
                    return RusotoError::Service(
                        InviteAccountToOrganizationError::AccountOwnerNotVerified(err.msg),
                    )
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        InviteAccountToOrganizationError::ConcurrentModification(err.msg),
                    )
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(
                        InviteAccountToOrganizationError::ConstraintViolation(err.msg),
                    )
                }
                "DuplicateHandshakeException" => {
                    return RusotoError::Service(
                        InviteAccountToOrganizationError::DuplicateHandshake(err.msg),
                    )
                }
                "FinalizingOrganizationException" => {
                    return RusotoError::Service(
                        InviteAccountToOrganizationError::FinalizingOrganization(err.msg),
                    )
                }
                "HandshakeConstraintViolationException" => {
                    return RusotoError::Service(
                        InviteAccountToOrganizationError::HandshakeConstraintViolation(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(InviteAccountToOrganizationError::InvalidInput(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(InviteAccountToOrganizationError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(InviteAccountToOrganizationError::TooManyRequests(
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
impl fmt::Display for InviteAccountToOrganizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InviteAccountToOrganizationError::AWSOrganizationsNotInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            InviteAccountToOrganizationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            InviteAccountToOrganizationError::AccountOwnerNotVerified(ref cause) => {
                write!(f, "{}", cause)
            }
            InviteAccountToOrganizationError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            InviteAccountToOrganizationError::ConstraintViolation(ref cause) => {
                write!(f, "{}", cause)
            }
            InviteAccountToOrganizationError::DuplicateHandshake(ref cause) => {
                write!(f, "{}", cause)
            }
            InviteAccountToOrganizationError::FinalizingOrganization(ref cause) => {
                write!(f, "{}", cause)
            }
            InviteAccountToOrganizationError::HandshakeConstraintViolation(ref cause) => {
                write!(f, "{}", cause)
            }
            InviteAccountToOrganizationError::InvalidInput(ref cause) => write!(f, "{}", cause),
            InviteAccountToOrganizationError::Service(ref cause) => write!(f, "{}", cause),
            InviteAccountToOrganizationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for InviteAccountToOrganizationError {}
/// Errors returned by LeaveOrganization
#[derive(Debug, PartialEq)]
pub enum LeaveOrganizationError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p> We can't find an AWS account with the <code>AccountId</code> that you specified, or the account whose credentials you used to make this request isn't a member of an organization.</p>
    AccountNotFound(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to remove the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>ORGANIZAION: You attempted to remove the management account from the organization. You can&#39;t remove the management account. Instead, after you remove all member accounts, delete the organization itself.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#orgs<em>manage</em>accounts<em>remove-from-master&quot;&gt;Removing a member account from your organization</a>in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>PHONE</em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#orgs_manage_accounts_remove-from-master">Removing a member account from your organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get this exception when running a command immediately after creating the organization, wait one hour and try again. After an hour, if the command continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>CANNOT</em>REGISTER<em>MASTER</em>AS<em>DELEGATED</em>ADMINISTRATOR: You attempted to register the management account of the organization as a delegated administrator for an AWS service integrated with Organizations. You can designate only a member account as a delegated administrator.</p> </li> <li> <p>CANNOT<em>REMOVE</em>DELEGATED<em>ADMINISTRATOR</em>FROM<em>ORG: You attempted to remove an account that is registered as a delegated administrator for a service integrated with your organization. To complete this operation, you must first deregister this account as a delegated administrator. </p> </li> <li> <p>CREATE</em>ORGANIZATION<em>IN</em>BILLING<em>MODE</em>UNSUPPORTED<em>REGION: To create an organization in the specified region, you must enable all features mode.</p> </li> <li> <p>DELEGATED</em>ADMINISTRATOR<em>EXISTS</em>FOR<em>THIS</em>SERVICE: You attempted to register an AWS account as a delegated administrator for an AWS service that already has a delegated administrator. To complete this operation, you must first deregister any existing delegated administrators for this service.</p> </li> <li> <p>EMAIL<em>VERIFICATION</em>CODE<em>EXPIRED: The email verification code is only valid for a limited period of time. You must resubmit the request and generate a new verfication code.</p> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s management account to the marketplace that corresponds to the management account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>BUSINESS</em>LICENSE: Applies only to the AWS Regions in China. To create an organization, the master must have an valid business license. For more information, contact customer support.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide a valid contact address and phone number for the management account. Then try the operation again.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>NOT<em>GOVCLOUD</em>ENABLED: To complete this operation, the management account must have an associated account in the AWS GovCloud (US-West) Region. For more information, see <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/govcloud-organizations.html">AWS Organizations</a> in the <i>AWS GovCloud User Guide.</i> </p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this management account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MAX<em>DELEGATED</em>ADMINISTRATORS<em>FOR</em>SERVICE<em>LIMIT</em>EXCEEDED: You attempted to register more delegated administrators than allowed for the service principal. </p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MAX</em>TAG<em>LIMIT</em>EXCEEDED: You have exceeded the number of tags allowed on this resource. </p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ORGANIZATION</em>NOT<em>IN</em>ALL<em>FEATURES</em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>OU<em>DEPTH</em>LIMIT<em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>OU</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>POLICY<em>CONTENT</em>LIMIT<em>EXCEEDED: You attempted to create a policy that is larger than the maximum size.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>TAG<em>POLICY</em>VIOLATION: You attempted to create or update a resource with tags that are not compliant with the tag policy requirements for this account.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>You can't remove a management account from an organization. If you want the management account to become a member account in another organization, you must first delete the current organization of the management account.</p>
    MasterCannotLeaveOrganization(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
}

impl LeaveOrganizationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<LeaveOrganizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(LeaveOrganizationError::AWSOrganizationsNotInUse(
                        err.msg,
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(LeaveOrganizationError::AccessDenied(err.msg))
                }
                "AccountNotFoundException" => {
                    return RusotoError::Service(LeaveOrganizationError::AccountNotFound(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(LeaveOrganizationError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(LeaveOrganizationError::ConstraintViolation(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(LeaveOrganizationError::InvalidInput(err.msg))
                }
                "MasterCannotLeaveOrganizationException" => {
                    return RusotoError::Service(
                        LeaveOrganizationError::MasterCannotLeaveOrganization(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(LeaveOrganizationError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(LeaveOrganizationError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for LeaveOrganizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LeaveOrganizationError::AWSOrganizationsNotInUse(ref cause) => write!(f, "{}", cause),
            LeaveOrganizationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            LeaveOrganizationError::AccountNotFound(ref cause) => write!(f, "{}", cause),
            LeaveOrganizationError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            LeaveOrganizationError::ConstraintViolation(ref cause) => write!(f, "{}", cause),
            LeaveOrganizationError::InvalidInput(ref cause) => write!(f, "{}", cause),
            LeaveOrganizationError::MasterCannotLeaveOrganization(ref cause) => {
                write!(f, "{}", cause)
            }
            LeaveOrganizationError::Service(ref cause) => write!(f, "{}", cause),
            LeaveOrganizationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for LeaveOrganizationError {}
/// Errors returned by ListAWSServiceAccessForOrganization
#[derive(Debug, PartialEq)]
pub enum ListAWSServiceAccessForOrganizationError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to remove the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>ORGANIZAION: You attempted to remove the management account from the organization. You can&#39;t remove the management account. Instead, after you remove all member accounts, delete the organization itself.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#orgs<em>manage</em>accounts<em>remove-from-master&quot;&gt;Removing a member account from your organization</a>in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>PHONE</em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#orgs_manage_accounts_remove-from-master">Removing a member account from your organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get this exception when running a command immediately after creating the organization, wait one hour and try again. After an hour, if the command continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>CANNOT</em>REGISTER<em>MASTER</em>AS<em>DELEGATED</em>ADMINISTRATOR: You attempted to register the management account of the organization as a delegated administrator for an AWS service integrated with Organizations. You can designate only a member account as a delegated administrator.</p> </li> <li> <p>CANNOT<em>REMOVE</em>DELEGATED<em>ADMINISTRATOR</em>FROM<em>ORG: You attempted to remove an account that is registered as a delegated administrator for a service integrated with your organization. To complete this operation, you must first deregister this account as a delegated administrator. </p> </li> <li> <p>CREATE</em>ORGANIZATION<em>IN</em>BILLING<em>MODE</em>UNSUPPORTED<em>REGION: To create an organization in the specified region, you must enable all features mode.</p> </li> <li> <p>DELEGATED</em>ADMINISTRATOR<em>EXISTS</em>FOR<em>THIS</em>SERVICE: You attempted to register an AWS account as a delegated administrator for an AWS service that already has a delegated administrator. To complete this operation, you must first deregister any existing delegated administrators for this service.</p> </li> <li> <p>EMAIL<em>VERIFICATION</em>CODE<em>EXPIRED: The email verification code is only valid for a limited period of time. You must resubmit the request and generate a new verfication code.</p> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s management account to the marketplace that corresponds to the management account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>BUSINESS</em>LICENSE: Applies only to the AWS Regions in China. To create an organization, the master must have an valid business license. For more information, contact customer support.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide a valid contact address and phone number for the management account. Then try the operation again.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>NOT<em>GOVCLOUD</em>ENABLED: To complete this operation, the management account must have an associated account in the AWS GovCloud (US-West) Region. For more information, see <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/govcloud-organizations.html">AWS Organizations</a> in the <i>AWS GovCloud User Guide.</i> </p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this management account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MAX<em>DELEGATED</em>ADMINISTRATORS<em>FOR</em>SERVICE<em>LIMIT</em>EXCEEDED: You attempted to register more delegated administrators than allowed for the service principal. </p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MAX</em>TAG<em>LIMIT</em>EXCEEDED: You have exceeded the number of tags allowed on this resource. </p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ORGANIZATION</em>NOT<em>IN</em>ALL<em>FEATURES</em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>OU<em>DEPTH</em>LIMIT<em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>OU</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>POLICY<em>CONTENT</em>LIMIT<em>EXCEEDED: You attempted to create a policy that is larger than the maximum size.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>TAG<em>POLICY</em>VIOLATION: You attempted to create or update a resource with tags that are not compliant with the tag policy requirements for this account.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
    /// <p>This action isn't available in the current AWS Region.</p>
    UnsupportedAPIEndpoint(String),
}

impl ListAWSServiceAccessForOrganizationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListAWSServiceAccessForOrganizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        ListAWSServiceAccessForOrganizationError::AWSOrganizationsNotInUse(err.msg),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        ListAWSServiceAccessForOrganizationError::AccessDenied(err.msg),
                    )
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(
                        ListAWSServiceAccessForOrganizationError::ConstraintViolation(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        ListAWSServiceAccessForOrganizationError::InvalidInput(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(ListAWSServiceAccessForOrganizationError::Service(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        ListAWSServiceAccessForOrganizationError::TooManyRequests(err.msg),
                    )
                }
                "UnsupportedAPIEndpointException" => {
                    return RusotoError::Service(
                        ListAWSServiceAccessForOrganizationError::UnsupportedAPIEndpoint(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAWSServiceAccessForOrganizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAWSServiceAccessForOrganizationError::AWSOrganizationsNotInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            ListAWSServiceAccessForOrganizationError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            ListAWSServiceAccessForOrganizationError::ConstraintViolation(ref cause) => {
                write!(f, "{}", cause)
            }
            ListAWSServiceAccessForOrganizationError::InvalidInput(ref cause) => {
                write!(f, "{}", cause)
            }
            ListAWSServiceAccessForOrganizationError::Service(ref cause) => write!(f, "{}", cause),
            ListAWSServiceAccessForOrganizationError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
            ListAWSServiceAccessForOrganizationError::UnsupportedAPIEndpoint(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListAWSServiceAccessForOrganizationError {}
/// Errors returned by ListAccounts
#[derive(Debug, PartialEq)]
pub enum ListAccountsError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
}

impl ListAccountsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAccountsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(ListAccountsError::AWSOrganizationsNotInUse(
                        err.msg,
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(ListAccountsError::AccessDenied(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListAccountsError::InvalidInput(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListAccountsError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListAccountsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAccountsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAccountsError::AWSOrganizationsNotInUse(ref cause) => write!(f, "{}", cause),
            ListAccountsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListAccountsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListAccountsError::Service(ref cause) => write!(f, "{}", cause),
            ListAccountsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAccountsError {}
/// Errors returned by ListAccountsForParent
#[derive(Debug, PartialEq)]
pub enum ListAccountsForParentError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>We can't find a root or OU with the <code>ParentId</code> that you specified.</p>
    ParentNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
}

impl ListAccountsForParentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAccountsForParentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        ListAccountsForParentError::AWSOrganizationsNotInUse(err.msg),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(ListAccountsForParentError::AccessDenied(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListAccountsForParentError::InvalidInput(err.msg))
                }
                "ParentNotFoundException" => {
                    return RusotoError::Service(ListAccountsForParentError::ParentNotFound(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListAccountsForParentError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListAccountsForParentError::TooManyRequests(
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
impl fmt::Display for ListAccountsForParentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAccountsForParentError::AWSOrganizationsNotInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            ListAccountsForParentError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListAccountsForParentError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListAccountsForParentError::ParentNotFound(ref cause) => write!(f, "{}", cause),
            ListAccountsForParentError::Service(ref cause) => write!(f, "{}", cause),
            ListAccountsForParentError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAccountsForParentError {}
/// Errors returned by ListChildren
#[derive(Debug, PartialEq)]
pub enum ListChildrenError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>We can't find a root or OU with the <code>ParentId</code> that you specified.</p>
    ParentNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
}

impl ListChildrenError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListChildrenError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(ListChildrenError::AWSOrganizationsNotInUse(
                        err.msg,
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(ListChildrenError::AccessDenied(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListChildrenError::InvalidInput(err.msg))
                }
                "ParentNotFoundException" => {
                    return RusotoError::Service(ListChildrenError::ParentNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListChildrenError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListChildrenError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListChildrenError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListChildrenError::AWSOrganizationsNotInUse(ref cause) => write!(f, "{}", cause),
            ListChildrenError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListChildrenError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListChildrenError::ParentNotFound(ref cause) => write!(f, "{}", cause),
            ListChildrenError::Service(ref cause) => write!(f, "{}", cause),
            ListChildrenError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListChildrenError {}
/// Errors returned by ListCreateAccountStatus
#[derive(Debug, PartialEq)]
pub enum ListCreateAccountStatusError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
    /// <p>This action isn't available in the current AWS Region.</p>
    UnsupportedAPIEndpoint(String),
}

impl ListCreateAccountStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListCreateAccountStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        ListCreateAccountStatusError::AWSOrganizationsNotInUse(err.msg),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(ListCreateAccountStatusError::AccessDenied(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListCreateAccountStatusError::InvalidInput(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListCreateAccountStatusError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListCreateAccountStatusError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnsupportedAPIEndpointException" => {
                    return RusotoError::Service(
                        ListCreateAccountStatusError::UnsupportedAPIEndpoint(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListCreateAccountStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListCreateAccountStatusError::AWSOrganizationsNotInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            ListCreateAccountStatusError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListCreateAccountStatusError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListCreateAccountStatusError::Service(ref cause) => write!(f, "{}", cause),
            ListCreateAccountStatusError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ListCreateAccountStatusError::UnsupportedAPIEndpoint(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListCreateAccountStatusError {}
/// Errors returned by ListDelegatedAdministrators
#[derive(Debug, PartialEq)]
pub enum ListDelegatedAdministratorsError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to remove the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>ORGANIZAION: You attempted to remove the management account from the organization. You can&#39;t remove the management account. Instead, after you remove all member accounts, delete the organization itself.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#orgs<em>manage</em>accounts<em>remove-from-master&quot;&gt;Removing a member account from your organization</a>in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>PHONE</em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#orgs_manage_accounts_remove-from-master">Removing a member account from your organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get this exception when running a command immediately after creating the organization, wait one hour and try again. After an hour, if the command continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>CANNOT</em>REGISTER<em>MASTER</em>AS<em>DELEGATED</em>ADMINISTRATOR: You attempted to register the management account of the organization as a delegated administrator for an AWS service integrated with Organizations. You can designate only a member account as a delegated administrator.</p> </li> <li> <p>CANNOT<em>REMOVE</em>DELEGATED<em>ADMINISTRATOR</em>FROM<em>ORG: You attempted to remove an account that is registered as a delegated administrator for a service integrated with your organization. To complete this operation, you must first deregister this account as a delegated administrator. </p> </li> <li> <p>CREATE</em>ORGANIZATION<em>IN</em>BILLING<em>MODE</em>UNSUPPORTED<em>REGION: To create an organization in the specified region, you must enable all features mode.</p> </li> <li> <p>DELEGATED</em>ADMINISTRATOR<em>EXISTS</em>FOR<em>THIS</em>SERVICE: You attempted to register an AWS account as a delegated administrator for an AWS service that already has a delegated administrator. To complete this operation, you must first deregister any existing delegated administrators for this service.</p> </li> <li> <p>EMAIL<em>VERIFICATION</em>CODE<em>EXPIRED: The email verification code is only valid for a limited period of time. You must resubmit the request and generate a new verfication code.</p> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s management account to the marketplace that corresponds to the management account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>BUSINESS</em>LICENSE: Applies only to the AWS Regions in China. To create an organization, the master must have an valid business license. For more information, contact customer support.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide a valid contact address and phone number for the management account. Then try the operation again.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>NOT<em>GOVCLOUD</em>ENABLED: To complete this operation, the management account must have an associated account in the AWS GovCloud (US-West) Region. For more information, see <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/govcloud-organizations.html">AWS Organizations</a> in the <i>AWS GovCloud User Guide.</i> </p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this management account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MAX<em>DELEGATED</em>ADMINISTRATORS<em>FOR</em>SERVICE<em>LIMIT</em>EXCEEDED: You attempted to register more delegated administrators than allowed for the service principal. </p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MAX</em>TAG<em>LIMIT</em>EXCEEDED: You have exceeded the number of tags allowed on this resource. </p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ORGANIZATION</em>NOT<em>IN</em>ALL<em>FEATURES</em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>OU<em>DEPTH</em>LIMIT<em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>OU</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>POLICY<em>CONTENT</em>LIMIT<em>EXCEEDED: You attempted to create a policy that is larger than the maximum size.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>TAG<em>POLICY</em>VIOLATION: You attempted to create or update a resource with tags that are not compliant with the tag policy requirements for this account.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
    /// <p>This action isn't available in the current AWS Region.</p>
    UnsupportedAPIEndpoint(String),
}

impl ListDelegatedAdministratorsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListDelegatedAdministratorsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        ListDelegatedAdministratorsError::AWSOrganizationsNotInUse(err.msg),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(ListDelegatedAdministratorsError::AccessDenied(
                        err.msg,
                    ))
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(
                        ListDelegatedAdministratorsError::ConstraintViolation(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListDelegatedAdministratorsError::InvalidInput(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListDelegatedAdministratorsError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListDelegatedAdministratorsError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnsupportedAPIEndpointException" => {
                    return RusotoError::Service(
                        ListDelegatedAdministratorsError::UnsupportedAPIEndpoint(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDelegatedAdministratorsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDelegatedAdministratorsError::AWSOrganizationsNotInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            ListDelegatedAdministratorsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListDelegatedAdministratorsError::ConstraintViolation(ref cause) => {
                write!(f, "{}", cause)
            }
            ListDelegatedAdministratorsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListDelegatedAdministratorsError::Service(ref cause) => write!(f, "{}", cause),
            ListDelegatedAdministratorsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ListDelegatedAdministratorsError::UnsupportedAPIEndpoint(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListDelegatedAdministratorsError {}
/// Errors returned by ListDelegatedServicesForAccount
#[derive(Debug, PartialEq)]
pub enum ListDelegatedServicesForAccountError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p> We can't find an AWS account with the <code>AccountId</code> that you specified, or the account whose credentials you used to make this request isn't a member of an organization.</p>
    AccountNotFound(String),
    /// <p>The specified account is not a delegated administrator for this AWS service. </p>
    AccountNotRegistered(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to remove the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>ORGANIZAION: You attempted to remove the management account from the organization. You can&#39;t remove the management account. Instead, after you remove all member accounts, delete the organization itself.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#orgs<em>manage</em>accounts<em>remove-from-master&quot;&gt;Removing a member account from your organization</a>in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>PHONE</em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#orgs_manage_accounts_remove-from-master">Removing a member account from your organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get this exception when running a command immediately after creating the organization, wait one hour and try again. After an hour, if the command continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>CANNOT</em>REGISTER<em>MASTER</em>AS<em>DELEGATED</em>ADMINISTRATOR: You attempted to register the management account of the organization as a delegated administrator for an AWS service integrated with Organizations. You can designate only a member account as a delegated administrator.</p> </li> <li> <p>CANNOT<em>REMOVE</em>DELEGATED<em>ADMINISTRATOR</em>FROM<em>ORG: You attempted to remove an account that is registered as a delegated administrator for a service integrated with your organization. To complete this operation, you must first deregister this account as a delegated administrator. </p> </li> <li> <p>CREATE</em>ORGANIZATION<em>IN</em>BILLING<em>MODE</em>UNSUPPORTED<em>REGION: To create an organization in the specified region, you must enable all features mode.</p> </li> <li> <p>DELEGATED</em>ADMINISTRATOR<em>EXISTS</em>FOR<em>THIS</em>SERVICE: You attempted to register an AWS account as a delegated administrator for an AWS service that already has a delegated administrator. To complete this operation, you must first deregister any existing delegated administrators for this service.</p> </li> <li> <p>EMAIL<em>VERIFICATION</em>CODE<em>EXPIRED: The email verification code is only valid for a limited period of time. You must resubmit the request and generate a new verfication code.</p> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s management account to the marketplace that corresponds to the management account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>BUSINESS</em>LICENSE: Applies only to the AWS Regions in China. To create an organization, the master must have an valid business license. For more information, contact customer support.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide a valid contact address and phone number for the management account. Then try the operation again.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>NOT<em>GOVCLOUD</em>ENABLED: To complete this operation, the management account must have an associated account in the AWS GovCloud (US-West) Region. For more information, see <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/govcloud-organizations.html">AWS Organizations</a> in the <i>AWS GovCloud User Guide.</i> </p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this management account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MAX<em>DELEGATED</em>ADMINISTRATORS<em>FOR</em>SERVICE<em>LIMIT</em>EXCEEDED: You attempted to register more delegated administrators than allowed for the service principal. </p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MAX</em>TAG<em>LIMIT</em>EXCEEDED: You have exceeded the number of tags allowed on this resource. </p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ORGANIZATION</em>NOT<em>IN</em>ALL<em>FEATURES</em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>OU<em>DEPTH</em>LIMIT<em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>OU</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>POLICY<em>CONTENT</em>LIMIT<em>EXCEEDED: You attempted to create a policy that is larger than the maximum size.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>TAG<em>POLICY</em>VIOLATION: You attempted to create or update a resource with tags that are not compliant with the tag policy requirements for this account.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
    /// <p>This action isn't available in the current AWS Region.</p>
    UnsupportedAPIEndpoint(String),
}

impl ListDelegatedServicesForAccountError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListDelegatedServicesForAccountError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        ListDelegatedServicesForAccountError::AWSOrganizationsNotInUse(err.msg),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        ListDelegatedServicesForAccountError::AccessDenied(err.msg),
                    )
                }
                "AccountNotFoundException" => {
                    return RusotoError::Service(
                        ListDelegatedServicesForAccountError::AccountNotFound(err.msg),
                    )
                }
                "AccountNotRegisteredException" => {
                    return RusotoError::Service(
                        ListDelegatedServicesForAccountError::AccountNotRegistered(err.msg),
                    )
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(
                        ListDelegatedServicesForAccountError::ConstraintViolation(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        ListDelegatedServicesForAccountError::InvalidInput(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(ListDelegatedServicesForAccountError::Service(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        ListDelegatedServicesForAccountError::TooManyRequests(err.msg),
                    )
                }
                "UnsupportedAPIEndpointException" => {
                    return RusotoError::Service(
                        ListDelegatedServicesForAccountError::UnsupportedAPIEndpoint(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDelegatedServicesForAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDelegatedServicesForAccountError::AWSOrganizationsNotInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            ListDelegatedServicesForAccountError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListDelegatedServicesForAccountError::AccountNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            ListDelegatedServicesForAccountError::AccountNotRegistered(ref cause) => {
                write!(f, "{}", cause)
            }
            ListDelegatedServicesForAccountError::ConstraintViolation(ref cause) => {
                write!(f, "{}", cause)
            }
            ListDelegatedServicesForAccountError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListDelegatedServicesForAccountError::Service(ref cause) => write!(f, "{}", cause),
            ListDelegatedServicesForAccountError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
            ListDelegatedServicesForAccountError::UnsupportedAPIEndpoint(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListDelegatedServicesForAccountError {}
/// Errors returned by ListHandshakesForAccount
#[derive(Debug, PartialEq)]
pub enum ListHandshakesForAccountError {
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
}

impl ListHandshakesForAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListHandshakesForAccountError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListHandshakesForAccountError::AccessDenied(
                        err.msg,
                    ))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        ListHandshakesForAccountError::ConcurrentModification(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListHandshakesForAccountError::InvalidInput(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListHandshakesForAccountError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListHandshakesForAccountError::TooManyRequests(
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
impl fmt::Display for ListHandshakesForAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListHandshakesForAccountError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListHandshakesForAccountError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            ListHandshakesForAccountError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListHandshakesForAccountError::Service(ref cause) => write!(f, "{}", cause),
            ListHandshakesForAccountError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListHandshakesForAccountError {}
/// Errors returned by ListHandshakesForOrganization
#[derive(Debug, PartialEq)]
pub enum ListHandshakesForOrganizationError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
}

impl ListHandshakesForOrganizationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListHandshakesForOrganizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        ListHandshakesForOrganizationError::AWSOrganizationsNotInUse(err.msg),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(ListHandshakesForOrganizationError::AccessDenied(
                        err.msg,
                    ))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        ListHandshakesForOrganizationError::ConcurrentModification(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListHandshakesForOrganizationError::InvalidInput(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListHandshakesForOrganizationError::Service(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        ListHandshakesForOrganizationError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListHandshakesForOrganizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListHandshakesForOrganizationError::AWSOrganizationsNotInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            ListHandshakesForOrganizationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListHandshakesForOrganizationError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            ListHandshakesForOrganizationError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListHandshakesForOrganizationError::Service(ref cause) => write!(f, "{}", cause),
            ListHandshakesForOrganizationError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListHandshakesForOrganizationError {}
/// Errors returned by ListOrganizationalUnitsForParent
#[derive(Debug, PartialEq)]
pub enum ListOrganizationalUnitsForParentError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>We can't find a root or OU with the <code>ParentId</code> that you specified.</p>
    ParentNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
}

impl ListOrganizationalUnitsForParentError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListOrganizationalUnitsForParentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        ListOrganizationalUnitsForParentError::AWSOrganizationsNotInUse(err.msg),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        ListOrganizationalUnitsForParentError::AccessDenied(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        ListOrganizationalUnitsForParentError::InvalidInput(err.msg),
                    )
                }
                "ParentNotFoundException" => {
                    return RusotoError::Service(
                        ListOrganizationalUnitsForParentError::ParentNotFound(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(ListOrganizationalUnitsForParentError::Service(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        ListOrganizationalUnitsForParentError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListOrganizationalUnitsForParentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListOrganizationalUnitsForParentError::AWSOrganizationsNotInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            ListOrganizationalUnitsForParentError::AccessDenied(ref cause) => {
                write!(f, "{}", cause)
            }
            ListOrganizationalUnitsForParentError::InvalidInput(ref cause) => {
                write!(f, "{}", cause)
            }
            ListOrganizationalUnitsForParentError::ParentNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            ListOrganizationalUnitsForParentError::Service(ref cause) => write!(f, "{}", cause),
            ListOrganizationalUnitsForParentError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListOrganizationalUnitsForParentError {}
/// Errors returned by ListParents
#[derive(Debug, PartialEq)]
pub enum ListParentsError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>We can't find an organizational unit (OU) or AWS account with the <code>ChildId</code> that you specified.</p>
    ChildNotFound(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
}

impl ListParentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListParentsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(ListParentsError::AWSOrganizationsNotInUse(
                        err.msg,
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(ListParentsError::AccessDenied(err.msg))
                }
                "ChildNotFoundException" => {
                    return RusotoError::Service(ListParentsError::ChildNotFound(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListParentsError::InvalidInput(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListParentsError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListParentsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListParentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListParentsError::AWSOrganizationsNotInUse(ref cause) => write!(f, "{}", cause),
            ListParentsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListParentsError::ChildNotFound(ref cause) => write!(f, "{}", cause),
            ListParentsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListParentsError::Service(ref cause) => write!(f, "{}", cause),
            ListParentsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListParentsError {}
/// Errors returned by ListPolicies
#[derive(Debug, PartialEq)]
pub enum ListPoliciesError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
    /// <p>This action isn't available in the current AWS Region.</p>
    UnsupportedAPIEndpoint(String),
}

impl ListPoliciesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPoliciesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(ListPoliciesError::AWSOrganizationsNotInUse(
                        err.msg,
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(ListPoliciesError::AccessDenied(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListPoliciesError::InvalidInput(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListPoliciesError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListPoliciesError::TooManyRequests(err.msg))
                }
                "UnsupportedAPIEndpointException" => {
                    return RusotoError::Service(ListPoliciesError::UnsupportedAPIEndpoint(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListPoliciesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPoliciesError::AWSOrganizationsNotInUse(ref cause) => write!(f, "{}", cause),
            ListPoliciesError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListPoliciesError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListPoliciesError::Service(ref cause) => write!(f, "{}", cause),
            ListPoliciesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ListPoliciesError::UnsupportedAPIEndpoint(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPoliciesError {}
/// Errors returned by ListPoliciesForTarget
#[derive(Debug, PartialEq)]
pub enum ListPoliciesForTargetError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>We can't find a root, OU, account, or policy with the <code>TargetId</code> that you specified.</p>
    TargetNotFound(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
    /// <p>This action isn't available in the current AWS Region.</p>
    UnsupportedAPIEndpoint(String),
}

impl ListPoliciesForTargetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPoliciesForTargetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        ListPoliciesForTargetError::AWSOrganizationsNotInUse(err.msg),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(ListPoliciesForTargetError::AccessDenied(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListPoliciesForTargetError::InvalidInput(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListPoliciesForTargetError::Service(err.msg))
                }
                "TargetNotFoundException" => {
                    return RusotoError::Service(ListPoliciesForTargetError::TargetNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListPoliciesForTargetError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnsupportedAPIEndpointException" => {
                    return RusotoError::Service(
                        ListPoliciesForTargetError::UnsupportedAPIEndpoint(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListPoliciesForTargetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPoliciesForTargetError::AWSOrganizationsNotInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            ListPoliciesForTargetError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListPoliciesForTargetError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListPoliciesForTargetError::Service(ref cause) => write!(f, "{}", cause),
            ListPoliciesForTargetError::TargetNotFound(ref cause) => write!(f, "{}", cause),
            ListPoliciesForTargetError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ListPoliciesForTargetError::UnsupportedAPIEndpoint(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPoliciesForTargetError {}
/// Errors returned by ListRoots
#[derive(Debug, PartialEq)]
pub enum ListRootsError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
}

impl ListRootsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRootsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(ListRootsError::AWSOrganizationsNotInUse(err.msg))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(ListRootsError::AccessDenied(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListRootsError::InvalidInput(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListRootsError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListRootsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListRootsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRootsError::AWSOrganizationsNotInUse(ref cause) => write!(f, "{}", cause),
            ListRootsError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListRootsError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListRootsError::Service(ref cause) => write!(f, "{}", cause),
            ListRootsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRootsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>We can't find a root, OU, account, or policy with the <code>TargetId</code> that you specified.</p>
    TargetNotFound(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        ListTagsForResourceError::AWSOrganizationsNotInUse(err.msg),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(ListTagsForResourceError::AccessDenied(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidInput(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListTagsForResourceError::Service(err.msg))
                }
                "TargetNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::TargetNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListTagsForResourceError::TooManyRequests(err.msg))
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
            ListTagsForResourceError::AWSOrganizationsNotInUse(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::Service(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::TargetNotFound(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ListTargetsForPolicy
#[derive(Debug, PartialEq)]
pub enum ListTargetsForPolicyError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>We can't find a policy with the <code>PolicyId</code> that you specified.</p>
    PolicyNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
    /// <p>This action isn't available in the current AWS Region.</p>
    UnsupportedAPIEndpoint(String),
}

impl ListTargetsForPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTargetsForPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        ListTargetsForPolicyError::AWSOrganizationsNotInUse(err.msg),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(ListTargetsForPolicyError::AccessDenied(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListTargetsForPolicyError::InvalidInput(err.msg))
                }
                "PolicyNotFoundException" => {
                    return RusotoError::Service(ListTargetsForPolicyError::PolicyNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListTargetsForPolicyError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListTargetsForPolicyError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnsupportedAPIEndpointException" => {
                    return RusotoError::Service(ListTargetsForPolicyError::UnsupportedAPIEndpoint(
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
impl fmt::Display for ListTargetsForPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTargetsForPolicyError::AWSOrganizationsNotInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            ListTargetsForPolicyError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListTargetsForPolicyError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListTargetsForPolicyError::PolicyNotFound(ref cause) => write!(f, "{}", cause),
            ListTargetsForPolicyError::Service(ref cause) => write!(f, "{}", cause),
            ListTargetsForPolicyError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ListTargetsForPolicyError::UnsupportedAPIEndpoint(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTargetsForPolicyError {}
/// Errors returned by MoveAccount
#[derive(Debug, PartialEq)]
pub enum MoveAccountError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p> We can't find an AWS account with the <code>AccountId</code> that you specified, or the account whose credentials you used to make this request isn't a member of an organization.</p>
    AccountNotFound(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p>We can't find the destination container (a root or OU) with the <code>ParentId</code> that you specified.</p>
    DestinationParentNotFound(String),
    /// <p>That account is already present in the specified destination.</p>
    DuplicateAccount(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>We can't find a source root or OU with the <code>ParentId</code> that you specified.</p>
    SourceParentNotFound(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
}

impl MoveAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<MoveAccountError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(MoveAccountError::AWSOrganizationsNotInUse(
                        err.msg,
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(MoveAccountError::AccessDenied(err.msg))
                }
                "AccountNotFoundException" => {
                    return RusotoError::Service(MoveAccountError::AccountNotFound(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(MoveAccountError::ConcurrentModification(err.msg))
                }
                "DestinationParentNotFoundException" => {
                    return RusotoError::Service(MoveAccountError::DestinationParentNotFound(
                        err.msg,
                    ))
                }
                "DuplicateAccountException" => {
                    return RusotoError::Service(MoveAccountError::DuplicateAccount(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(MoveAccountError::InvalidInput(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(MoveAccountError::Service(err.msg))
                }
                "SourceParentNotFoundException" => {
                    return RusotoError::Service(MoveAccountError::SourceParentNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(MoveAccountError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for MoveAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MoveAccountError::AWSOrganizationsNotInUse(ref cause) => write!(f, "{}", cause),
            MoveAccountError::AccessDenied(ref cause) => write!(f, "{}", cause),
            MoveAccountError::AccountNotFound(ref cause) => write!(f, "{}", cause),
            MoveAccountError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            MoveAccountError::DestinationParentNotFound(ref cause) => write!(f, "{}", cause),
            MoveAccountError::DuplicateAccount(ref cause) => write!(f, "{}", cause),
            MoveAccountError::InvalidInput(ref cause) => write!(f, "{}", cause),
            MoveAccountError::Service(ref cause) => write!(f, "{}", cause),
            MoveAccountError::SourceParentNotFound(ref cause) => write!(f, "{}", cause),
            MoveAccountError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for MoveAccountError {}
/// Errors returned by RegisterDelegatedAdministrator
#[derive(Debug, PartialEq)]
pub enum RegisterDelegatedAdministratorError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>The specified account is already a delegated administrator for this AWS service.</p>
    AccountAlreadyRegistered(String),
    /// <p> We can't find an AWS account with the <code>AccountId</code> that you specified, or the account whose credentials you used to make this request isn't a member of an organization.</p>
    AccountNotFound(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to remove the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>ORGANIZAION: You attempted to remove the management account from the organization. You can&#39;t remove the management account. Instead, after you remove all member accounts, delete the organization itself.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#orgs<em>manage</em>accounts<em>remove-from-master&quot;&gt;Removing a member account from your organization</a>in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>PHONE</em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#orgs_manage_accounts_remove-from-master">Removing a member account from your organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get this exception when running a command immediately after creating the organization, wait one hour and try again. After an hour, if the command continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>CANNOT</em>REGISTER<em>MASTER</em>AS<em>DELEGATED</em>ADMINISTRATOR: You attempted to register the management account of the organization as a delegated administrator for an AWS service integrated with Organizations. You can designate only a member account as a delegated administrator.</p> </li> <li> <p>CANNOT<em>REMOVE</em>DELEGATED<em>ADMINISTRATOR</em>FROM<em>ORG: You attempted to remove an account that is registered as a delegated administrator for a service integrated with your organization. To complete this operation, you must first deregister this account as a delegated administrator. </p> </li> <li> <p>CREATE</em>ORGANIZATION<em>IN</em>BILLING<em>MODE</em>UNSUPPORTED<em>REGION: To create an organization in the specified region, you must enable all features mode.</p> </li> <li> <p>DELEGATED</em>ADMINISTRATOR<em>EXISTS</em>FOR<em>THIS</em>SERVICE: You attempted to register an AWS account as a delegated administrator for an AWS service that already has a delegated administrator. To complete this operation, you must first deregister any existing delegated administrators for this service.</p> </li> <li> <p>EMAIL<em>VERIFICATION</em>CODE<em>EXPIRED: The email verification code is only valid for a limited period of time. You must resubmit the request and generate a new verfication code.</p> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s management account to the marketplace that corresponds to the management account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>BUSINESS</em>LICENSE: Applies only to the AWS Regions in China. To create an organization, the master must have an valid business license. For more information, contact customer support.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide a valid contact address and phone number for the management account. Then try the operation again.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>NOT<em>GOVCLOUD</em>ENABLED: To complete this operation, the management account must have an associated account in the AWS GovCloud (US-West) Region. For more information, see <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/govcloud-organizations.html">AWS Organizations</a> in the <i>AWS GovCloud User Guide.</i> </p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this management account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MAX<em>DELEGATED</em>ADMINISTRATORS<em>FOR</em>SERVICE<em>LIMIT</em>EXCEEDED: You attempted to register more delegated administrators than allowed for the service principal. </p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MAX</em>TAG<em>LIMIT</em>EXCEEDED: You have exceeded the number of tags allowed on this resource. </p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ORGANIZATION</em>NOT<em>IN</em>ALL<em>FEATURES</em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>OU<em>DEPTH</em>LIMIT<em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>OU</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>POLICY<em>CONTENT</em>LIMIT<em>EXCEEDED: You attempted to create a policy that is larger than the maximum size.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>TAG<em>POLICY</em>VIOLATION: You attempted to create or update a resource with tags that are not compliant with the tag policy requirements for this account.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
    /// <p>This action isn't available in the current AWS Region.</p>
    UnsupportedAPIEndpoint(String),
}

impl RegisterDelegatedAdministratorError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RegisterDelegatedAdministratorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        RegisterDelegatedAdministratorError::AWSOrganizationsNotInUse(err.msg),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(RegisterDelegatedAdministratorError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountAlreadyRegisteredException" => {
                    return RusotoError::Service(
                        RegisterDelegatedAdministratorError::AccountAlreadyRegistered(err.msg),
                    )
                }
                "AccountNotFoundException" => {
                    return RusotoError::Service(
                        RegisterDelegatedAdministratorError::AccountNotFound(err.msg),
                    )
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        RegisterDelegatedAdministratorError::ConcurrentModification(err.msg),
                    )
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(
                        RegisterDelegatedAdministratorError::ConstraintViolation(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(RegisterDelegatedAdministratorError::InvalidInput(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(RegisterDelegatedAdministratorError::Service(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        RegisterDelegatedAdministratorError::TooManyRequests(err.msg),
                    )
                }
                "UnsupportedAPIEndpointException" => {
                    return RusotoError::Service(
                        RegisterDelegatedAdministratorError::UnsupportedAPIEndpoint(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RegisterDelegatedAdministratorError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterDelegatedAdministratorError::AWSOrganizationsNotInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            RegisterDelegatedAdministratorError::AccessDenied(ref cause) => write!(f, "{}", cause),
            RegisterDelegatedAdministratorError::AccountAlreadyRegistered(ref cause) => {
                write!(f, "{}", cause)
            }
            RegisterDelegatedAdministratorError::AccountNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            RegisterDelegatedAdministratorError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            RegisterDelegatedAdministratorError::ConstraintViolation(ref cause) => {
                write!(f, "{}", cause)
            }
            RegisterDelegatedAdministratorError::InvalidInput(ref cause) => write!(f, "{}", cause),
            RegisterDelegatedAdministratorError::Service(ref cause) => write!(f, "{}", cause),
            RegisterDelegatedAdministratorError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
            RegisterDelegatedAdministratorError::UnsupportedAPIEndpoint(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RegisterDelegatedAdministratorError {}
/// Errors returned by RemoveAccountFromOrganization
#[derive(Debug, PartialEq)]
pub enum RemoveAccountFromOrganizationError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p> We can't find an AWS account with the <code>AccountId</code> that you specified, or the account whose credentials you used to make this request isn't a member of an organization.</p>
    AccountNotFound(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to remove the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>ORGANIZAION: You attempted to remove the management account from the organization. You can&#39;t remove the management account. Instead, after you remove all member accounts, delete the organization itself.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#orgs<em>manage</em>accounts<em>remove-from-master&quot;&gt;Removing a member account from your organization</a>in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>PHONE</em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#orgs_manage_accounts_remove-from-master">Removing a member account from your organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get this exception when running a command immediately after creating the organization, wait one hour and try again. After an hour, if the command continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>CANNOT</em>REGISTER<em>MASTER</em>AS<em>DELEGATED</em>ADMINISTRATOR: You attempted to register the management account of the organization as a delegated administrator for an AWS service integrated with Organizations. You can designate only a member account as a delegated administrator.</p> </li> <li> <p>CANNOT<em>REMOVE</em>DELEGATED<em>ADMINISTRATOR</em>FROM<em>ORG: You attempted to remove an account that is registered as a delegated administrator for a service integrated with your organization. To complete this operation, you must first deregister this account as a delegated administrator. </p> </li> <li> <p>CREATE</em>ORGANIZATION<em>IN</em>BILLING<em>MODE</em>UNSUPPORTED<em>REGION: To create an organization in the specified region, you must enable all features mode.</p> </li> <li> <p>DELEGATED</em>ADMINISTRATOR<em>EXISTS</em>FOR<em>THIS</em>SERVICE: You attempted to register an AWS account as a delegated administrator for an AWS service that already has a delegated administrator. To complete this operation, you must first deregister any existing delegated administrators for this service.</p> </li> <li> <p>EMAIL<em>VERIFICATION</em>CODE<em>EXPIRED: The email verification code is only valid for a limited period of time. You must resubmit the request and generate a new verfication code.</p> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s management account to the marketplace that corresponds to the management account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>BUSINESS</em>LICENSE: Applies only to the AWS Regions in China. To create an organization, the master must have an valid business license. For more information, contact customer support.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide a valid contact address and phone number for the management account. Then try the operation again.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>NOT<em>GOVCLOUD</em>ENABLED: To complete this operation, the management account must have an associated account in the AWS GovCloud (US-West) Region. For more information, see <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/govcloud-organizations.html">AWS Organizations</a> in the <i>AWS GovCloud User Guide.</i> </p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this management account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MAX<em>DELEGATED</em>ADMINISTRATORS<em>FOR</em>SERVICE<em>LIMIT</em>EXCEEDED: You attempted to register more delegated administrators than allowed for the service principal. </p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MAX</em>TAG<em>LIMIT</em>EXCEEDED: You have exceeded the number of tags allowed on this resource. </p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ORGANIZATION</em>NOT<em>IN</em>ALL<em>FEATURES</em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>OU<em>DEPTH</em>LIMIT<em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>OU</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>POLICY<em>CONTENT</em>LIMIT<em>EXCEEDED: You attempted to create a policy that is larger than the maximum size.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>TAG<em>POLICY</em>VIOLATION: You attempted to create or update a resource with tags that are not compliant with the tag policy requirements for this account.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>You can't remove a management account from an organization. If you want the management account to become a member account in another organization, you must first delete the current organization of the management account.</p>
    MasterCannotLeaveOrganization(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
}

impl RemoveAccountFromOrganizationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RemoveAccountFromOrganizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        RemoveAccountFromOrganizationError::AWSOrganizationsNotInUse(err.msg),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(RemoveAccountFromOrganizationError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountNotFoundException" => {
                    return RusotoError::Service(
                        RemoveAccountFromOrganizationError::AccountNotFound(err.msg),
                    )
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        RemoveAccountFromOrganizationError::ConcurrentModification(err.msg),
                    )
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(
                        RemoveAccountFromOrganizationError::ConstraintViolation(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(RemoveAccountFromOrganizationError::InvalidInput(
                        err.msg,
                    ))
                }
                "MasterCannotLeaveOrganizationException" => {
                    return RusotoError::Service(
                        RemoveAccountFromOrganizationError::MasterCannotLeaveOrganization(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(RemoveAccountFromOrganizationError::Service(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        RemoveAccountFromOrganizationError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RemoveAccountFromOrganizationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveAccountFromOrganizationError::AWSOrganizationsNotInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            RemoveAccountFromOrganizationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            RemoveAccountFromOrganizationError::AccountNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            RemoveAccountFromOrganizationError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            RemoveAccountFromOrganizationError::ConstraintViolation(ref cause) => {
                write!(f, "{}", cause)
            }
            RemoveAccountFromOrganizationError::InvalidInput(ref cause) => write!(f, "{}", cause),
            RemoveAccountFromOrganizationError::MasterCannotLeaveOrganization(ref cause) => {
                write!(f, "{}", cause)
            }
            RemoveAccountFromOrganizationError::Service(ref cause) => write!(f, "{}", cause),
            RemoveAccountFromOrganizationError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RemoveAccountFromOrganizationError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to remove the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>ORGANIZAION: You attempted to remove the management account from the organization. You can&#39;t remove the management account. Instead, after you remove all member accounts, delete the organization itself.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#orgs<em>manage</em>accounts<em>remove-from-master&quot;&gt;Removing a member account from your organization</a>in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>PHONE</em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#orgs_manage_accounts_remove-from-master">Removing a member account from your organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get this exception when running a command immediately after creating the organization, wait one hour and try again. After an hour, if the command continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>CANNOT</em>REGISTER<em>MASTER</em>AS<em>DELEGATED</em>ADMINISTRATOR: You attempted to register the management account of the organization as a delegated administrator for an AWS service integrated with Organizations. You can designate only a member account as a delegated administrator.</p> </li> <li> <p>CANNOT<em>REMOVE</em>DELEGATED<em>ADMINISTRATOR</em>FROM<em>ORG: You attempted to remove an account that is registered as a delegated administrator for a service integrated with your organization. To complete this operation, you must first deregister this account as a delegated administrator. </p> </li> <li> <p>CREATE</em>ORGANIZATION<em>IN</em>BILLING<em>MODE</em>UNSUPPORTED<em>REGION: To create an organization in the specified region, you must enable all features mode.</p> </li> <li> <p>DELEGATED</em>ADMINISTRATOR<em>EXISTS</em>FOR<em>THIS</em>SERVICE: You attempted to register an AWS account as a delegated administrator for an AWS service that already has a delegated administrator. To complete this operation, you must first deregister any existing delegated administrators for this service.</p> </li> <li> <p>EMAIL<em>VERIFICATION</em>CODE<em>EXPIRED: The email verification code is only valid for a limited period of time. You must resubmit the request and generate a new verfication code.</p> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s management account to the marketplace that corresponds to the management account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>BUSINESS</em>LICENSE: Applies only to the AWS Regions in China. To create an organization, the master must have an valid business license. For more information, contact customer support.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide a valid contact address and phone number for the management account. Then try the operation again.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>NOT<em>GOVCLOUD</em>ENABLED: To complete this operation, the management account must have an associated account in the AWS GovCloud (US-West) Region. For more information, see <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/govcloud-organizations.html">AWS Organizations</a> in the <i>AWS GovCloud User Guide.</i> </p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this management account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MAX<em>DELEGATED</em>ADMINISTRATORS<em>FOR</em>SERVICE<em>LIMIT</em>EXCEEDED: You attempted to register more delegated administrators than allowed for the service principal. </p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MAX</em>TAG<em>LIMIT</em>EXCEEDED: You have exceeded the number of tags allowed on this resource. </p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ORGANIZATION</em>NOT<em>IN</em>ALL<em>FEATURES</em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>OU<em>DEPTH</em>LIMIT<em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>OU</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>POLICY<em>CONTENT</em>LIMIT<em>EXCEEDED: You attempted to create a policy that is larger than the maximum size.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>TAG<em>POLICY</em>VIOLATION: You attempted to create or update a resource with tags that are not compliant with the tag policy requirements for this account.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>We can't find a root, OU, account, or policy with the <code>TargetId</code> that you specified.</p>
    TargetNotFound(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(TagResourceError::AWSOrganizationsNotInUse(
                        err.msg,
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(TagResourceError::AccessDenied(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(TagResourceError::ConcurrentModification(err.msg))
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(TagResourceError::ConstraintViolation(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(TagResourceError::InvalidInput(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(TagResourceError::Service(err.msg))
                }
                "TargetNotFoundException" => {
                    return RusotoError::Service(TagResourceError::TargetNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(TagResourceError::TooManyRequests(err.msg))
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
            TagResourceError::AWSOrganizationsNotInUse(ref cause) => write!(f, "{}", cause),
            TagResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            TagResourceError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            TagResourceError::ConstraintViolation(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            TagResourceError::Service(ref cause) => write!(f, "{}", cause),
            TagResourceError::TargetNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to remove the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>ORGANIZAION: You attempted to remove the management account from the organization. You can&#39;t remove the management account. Instead, after you remove all member accounts, delete the organization itself.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#orgs<em>manage</em>accounts<em>remove-from-master&quot;&gt;Removing a member account from your organization</a>in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>PHONE</em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#orgs_manage_accounts_remove-from-master">Removing a member account from your organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get this exception when running a command immediately after creating the organization, wait one hour and try again. After an hour, if the command continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>CANNOT</em>REGISTER<em>MASTER</em>AS<em>DELEGATED</em>ADMINISTRATOR: You attempted to register the management account of the organization as a delegated administrator for an AWS service integrated with Organizations. You can designate only a member account as a delegated administrator.</p> </li> <li> <p>CANNOT<em>REMOVE</em>DELEGATED<em>ADMINISTRATOR</em>FROM<em>ORG: You attempted to remove an account that is registered as a delegated administrator for a service integrated with your organization. To complete this operation, you must first deregister this account as a delegated administrator. </p> </li> <li> <p>CREATE</em>ORGANIZATION<em>IN</em>BILLING<em>MODE</em>UNSUPPORTED<em>REGION: To create an organization in the specified region, you must enable all features mode.</p> </li> <li> <p>DELEGATED</em>ADMINISTRATOR<em>EXISTS</em>FOR<em>THIS</em>SERVICE: You attempted to register an AWS account as a delegated administrator for an AWS service that already has a delegated administrator. To complete this operation, you must first deregister any existing delegated administrators for this service.</p> </li> <li> <p>EMAIL<em>VERIFICATION</em>CODE<em>EXPIRED: The email verification code is only valid for a limited period of time. You must resubmit the request and generate a new verfication code.</p> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s management account to the marketplace that corresponds to the management account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>BUSINESS</em>LICENSE: Applies only to the AWS Regions in China. To create an organization, the master must have an valid business license. For more information, contact customer support.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide a valid contact address and phone number for the management account. Then try the operation again.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>NOT<em>GOVCLOUD</em>ENABLED: To complete this operation, the management account must have an associated account in the AWS GovCloud (US-West) Region. For more information, see <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/govcloud-organizations.html">AWS Organizations</a> in the <i>AWS GovCloud User Guide.</i> </p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this management account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MAX<em>DELEGATED</em>ADMINISTRATORS<em>FOR</em>SERVICE<em>LIMIT</em>EXCEEDED: You attempted to register more delegated administrators than allowed for the service principal. </p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MAX</em>TAG<em>LIMIT</em>EXCEEDED: You have exceeded the number of tags allowed on this resource. </p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ORGANIZATION</em>NOT<em>IN</em>ALL<em>FEATURES</em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>OU<em>DEPTH</em>LIMIT<em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>OU</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>POLICY<em>CONTENT</em>LIMIT<em>EXCEEDED: You attempted to create a policy that is larger than the maximum size.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>TAG<em>POLICY</em>VIOLATION: You attempted to create or update a resource with tags that are not compliant with the tag policy requirements for this account.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>We can't find a root, OU, account, or policy with the <code>TargetId</code> that you specified.</p>
    TargetNotFound(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(UntagResourceError::AWSOrganizationsNotInUse(
                        err.msg,
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(UntagResourceError::AccessDenied(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UntagResourceError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(UntagResourceError::ConstraintViolation(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UntagResourceError::InvalidInput(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(UntagResourceError::Service(err.msg))
                }
                "TargetNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::TargetNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UntagResourceError::TooManyRequests(err.msg))
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
            UntagResourceError::AWSOrganizationsNotInUse(ref cause) => write!(f, "{}", cause),
            UntagResourceError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ConstraintViolation(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Service(ref cause) => write!(f, "{}", cause),
            UntagResourceError::TargetNotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateOrganizationalUnit
#[derive(Debug, PartialEq)]
pub enum UpdateOrganizationalUnitError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p>An OU with the same name already exists.</p>
    DuplicateOrganizationalUnit(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>We can't find an OU with the <code>OrganizationalUnitId</code> that you specified.</p>
    OrganizationalUnitNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
}

impl UpdateOrganizationalUnitError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateOrganizationalUnitError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(
                        UpdateOrganizationalUnitError::AWSOrganizationsNotInUse(err.msg),
                    )
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateOrganizationalUnitError::AccessDenied(
                        err.msg,
                    ))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        UpdateOrganizationalUnitError::ConcurrentModification(err.msg),
                    )
                }
                "DuplicateOrganizationalUnitException" => {
                    return RusotoError::Service(
                        UpdateOrganizationalUnitError::DuplicateOrganizationalUnit(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateOrganizationalUnitError::InvalidInput(
                        err.msg,
                    ))
                }
                "OrganizationalUnitNotFoundException" => {
                    return RusotoError::Service(
                        UpdateOrganizationalUnitError::OrganizationalUnitNotFound(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(UpdateOrganizationalUnitError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateOrganizationalUnitError::TooManyRequests(
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
impl fmt::Display for UpdateOrganizationalUnitError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateOrganizationalUnitError::AWSOrganizationsNotInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateOrganizationalUnitError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdateOrganizationalUnitError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateOrganizationalUnitError::DuplicateOrganizationalUnit(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateOrganizationalUnitError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateOrganizationalUnitError::OrganizationalUnitNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateOrganizationalUnitError::Service(ref cause) => write!(f, "{}", cause),
            UpdateOrganizationalUnitError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateOrganizationalUnitError {}
/// Errors returned by UpdatePolicy
#[derive(Debug, PartialEq)]
pub enum UpdatePolicyError {
    /// <p>Your account isn't a member of an organization. To make this request, you must use the credentials of an account that belongs to an organization.</p>
    AWSOrganizationsNotInUse(String),
    /// <p>You don't have permissions to perform the requested operation. The user or role that is making the request must have at least one IAM permissions policy attached that grants the required permissions. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access.html">Access Management</a> in the <i>IAM User Guide.</i> </p>
    AccessDenied(String),
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>Performing this operation violates a minimum or maximum value limit. For example, attempting to remove the last service control policy (SCP) from an OU or root, inviting or creating too many accounts to the organization, or attaching too many policies to an account, OU, or root. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>ACCOUNT<em>CANNOT</em>LEAVE<em>ORGANIZAION: You attempted to remove the management account from the organization. You can&#39;t remove the management account. Instead, after you remove all member accounts, delete the organization itself.</p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>EULA: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first agree to the AWS Customer Agreement. Follow the steps at &lt;a href=&quot;http://docs.aws.amazon.com/organizations/latest/userguide/orgs</em>manage<em>accounts</em>remove.html#orgs<em>manage</em>accounts<em>remove-from-master&quot;&gt;Removing a member account from your organization</a>in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT</em>CANNOT<em>LEAVE</em>WITHOUT<em>PHONE</em>VERIFICATION: You attempted to remove an account from the organization that doesn&#39;t yet have enough information to exist as a standalone account. This account requires you to first complete phone verification. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#orgs_manage_accounts_remove-from-master">Removing a member account from your organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>ACCOUNT<em>CREATION</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of accounts that you can create in one day.</p> </li> <li> <p>ACCOUNT<em>NUMBER</em>LIMIT<em>EXCEEDED: You attempted to exceed the limit on the number of accounts in an organization. If you need more accounts, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a> to request an increase in your limit. </p> <p>Or the number of invitations that you tried to send would cause you to exceed the limit of accounts in your organization. Send fewer invitations or contact AWS Support to request an increase in the number of accounts.</p> <note> <p>Deleted and closed accounts still count toward your limit.</p> </note> <important> <p>If you get this exception when running a command immediately after creating the organization, wait one hour and try again. After an hour, if the command continues to fail with this error, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </important> </li> <li> <p>CANNOT</em>REGISTER<em>MASTER</em>AS<em>DELEGATED</em>ADMINISTRATOR: You attempted to register the management account of the organization as a delegated administrator for an AWS service integrated with Organizations. You can designate only a member account as a delegated administrator.</p> </li> <li> <p>CANNOT<em>REMOVE</em>DELEGATED<em>ADMINISTRATOR</em>FROM<em>ORG: You attempted to remove an account that is registered as a delegated administrator for a service integrated with your organization. To complete this operation, you must first deregister this account as a delegated administrator. </p> </li> <li> <p>CREATE</em>ORGANIZATION<em>IN</em>BILLING<em>MODE</em>UNSUPPORTED<em>REGION: To create an organization in the specified region, you must enable all features mode.</p> </li> <li> <p>DELEGATED</em>ADMINISTRATOR<em>EXISTS</em>FOR<em>THIS</em>SERVICE: You attempted to register an AWS account as a delegated administrator for an AWS service that already has a delegated administrator. To complete this operation, you must first deregister any existing delegated administrators for this service.</p> </li> <li> <p>EMAIL<em>VERIFICATION</em>CODE<em>EXPIRED: The email verification code is only valid for a limited period of time. You must resubmit the request and generate a new verfication code.</p> </li> <li> <p>HANDSHAKE</em>RATE<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of handshakes that you can send in one day.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>ADDRESS<em>DOES</em>NOT<em>MATCH</em>MARKETPLACE: To create an account in this organization, you first must migrate the organization&#39;s management account to the marketplace that corresponds to the management account&#39;s address. For example, accounts with India addresses must be associated with the AISPL marketplace. All accounts in an organization must be associated with the same marketplace.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>BUSINESS</em>LICENSE: Applies only to the AWS Regions in China. To create an organization, the master must have an valid business license. For more information, contact customer support.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>MISSING<em>CONTACT</em>INFO: To complete this operation, you must first provide a valid contact address and phone number for the management account. Then try the operation again.</p> </li> <li> <p>MASTER<em>ACCOUNT</em>NOT<em>GOVCLOUD</em>ENABLED: To complete this operation, the management account must have an associated account in the AWS GovCloud (US-West) Region. For more information, see <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/govcloud-organizations.html">AWS Organizations</a> in the <i>AWS GovCloud User Guide.</i> </p> </li> <li> <p>MASTER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To create an organization with this management account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MAX<em>DELEGATED</em>ADMINISTRATORS<em>FOR</em>SERVICE<em>LIMIT</em>EXCEEDED: You attempted to register more delegated administrators than allowed for the service principal. </p> </li> <li> <p>MAX<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to exceed the number of policies of a certain type that can be attached to an entity at one time.</p> </li> <li> <p>MAX</em>TAG<em>LIMIT</em>EXCEEDED: You have exceeded the number of tags allowed on this resource. </p> </li> <li> <p>MEMBER<em>ACCOUNT</em>PAYMENT<em>INSTRUMENT</em>REQUIRED: To complete this operation with this member account, you first must associate a valid payment instrument, such as a credit card, with the account. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info">To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>MIN<em>POLICY</em>TYPE<em>ATTACHMENT</em>LIMIT<em>EXCEEDED: You attempted to detach a policy from an entity that would cause the entity to have fewer than the minimum number of policies of a certain type required.</p> </li> <li> <p>ORGANIZATION</em>NOT<em>IN</em>ALL<em>FEATURES</em>MODE: You attempted to perform an operation that requires the organization to be configured to support all features. An organization that supports only consolidated billing features can&#39;t perform this operation.</p> </li> <li> <p>OU<em>DEPTH</em>LIMIT<em>EXCEEDED: You attempted to create an OU tree that is too many levels deep.</p> </li> <li> <p>OU</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of OUs that you can have in an organization.</p> </li> <li> <p>POLICY<em>CONTENT</em>LIMIT<em>EXCEEDED: You attempted to create a policy that is larger than the maximum size.</p> </li> <li> <p>POLICY</em>NUMBER<em>LIMIT</em>EXCEEDED: You attempted to exceed the number of policies that you can have in an organization.</p> </li> <li> <p>TAG<em>POLICY</em>VIOLATION: You attempted to create or update a resource with tags that are not compliant with the tag policy requirements for this account.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p>A policy with the same name already exists.</p>
    DuplicatePolicy(String),
    /// <p><p>The requested operation failed because you provided invalid values for one or more of the request parameters. This exception includes a reason that contains additional information about the violated limit:</p> <note> <p>Some of the reasons in the following list might not be applicable to this specific API or operation.</p> </note> <ul> <li> <p>DUPLICATE<em>TAG</em>KEY: Tag keys must be unique among the tags attached to the same entity.</p> </li> <li> <p>IMMUTABLE<em>POLICY: You specified a policy that is managed by AWS and can&#39;t be modified.</p> </li> <li> <p>INPUT</em>REQUIRED: You must include a value for all required parameters.</p> </li> <li> <p>INVALID<em>ENUM: You specified an invalid value.</p> </li> <li> <p>INVALID</em>ENUM<em>POLICY</em>TYPE: You specified an invalid policy type string.</p> </li> <li> <p>INVALID<em>FULL</em>NAME<em>TARGET: You specified a full name that contains invalid characters.</p> </li> <li> <p>INVALID</em>LIST<em>MEMBER: You provided a list to a parameter that contains at least one invalid value.</p> </li> <li> <p>INVALID</em>PAGINATION<em>TOKEN: Get the value for the <code>NextToken</code> parameter from the response to a previous call of the operation.</p> </li> <li> <p>INVALID</em>PARTY<em>TYPE</em>TARGET: You specified the wrong type of entity (account, organization, or email) as a party.</p> </li> <li> <p>INVALID<em>PATTERN: You provided a value that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID</em>PATTERN<em>TARGET</em>ID: You specified a policy target ID that doesn&#39;t match the required pattern.</p> </li> <li> <p>INVALID<em>ROLE</em>NAME: You provided a role name that isn&#39;t valid. A role name can&#39;t begin with the reserved prefix <code>AWSServiceRoleFor</code>.</p> </li> <li> <p>INVALID<em>SYNTAX</em>ORGANIZATION<em>ARN: You specified an invalid Amazon Resource Name (ARN) for the organization.</p> </li> <li> <p>INVALID</em>SYNTAX<em>POLICY</em>ID: You specified an invalid policy ID. </p> </li> <li> <p>INVALID<em>SYSTEM</em>TAGS<em>PARAMETER: You specified a tag key that is a system tag. You can’t add, edit, or delete system tag keys because they&#39;re reserved for AWS use. System tags don’t count against your tags per resource limit.</p> </li> <li> <p>MAX</em>FILTER<em>LIMIT</em>EXCEEDED: You can specify only one filter parameter for the operation.</p> </li> <li> <p>MAX<em>LENGTH</em>EXCEEDED: You provided a string parameter that is longer than allowed.</p> </li> <li> <p>MAX<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a larger value than allowed.</p> </li> <li> <p>MIN<em>LENGTH</em>EXCEEDED: You provided a string parameter that is shorter than allowed.</p> </li> <li> <p>MIN<em>VALUE</em>EXCEEDED: You provided a numeric parameter that has a smaller value than allowed.</p> </li> <li> <p>MOVING<em>ACCOUNT</em>BETWEEN<em>DIFFERENT</em>ROOTS: You can move an account only between entities in the same root.</p> </li> <li> <p>TARGET<em>NOT</em>SUPPORTED: You can&#39;t perform the specified operation on that target entity.</p> </li> <li> <p>UNRECOGNIZED<em>SERVICE</em>PRINCIPAL: You specified a service principal that isn&#39;t recognized.</p> </li> </ul></p>
    InvalidInput(String),
    /// <p>The provided policy document doesn't meet the requirements of the specified policy type. For example, the syntax might be incorrect. For details about service control policy syntax, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_scp-syntax.html">Service Control Policy Syntax</a> in the <i>AWS Organizations User Guide.</i> </p>
    MalformedPolicyDocument(String),
    /// <p>Changes to the effective policy are in progress, and its contents can't be returned. Try the operation again later. </p>
    PolicyChangesInProgress(String),
    /// <p>We can't find a policy with the <code>PolicyId</code> that you specified.</p>
    PolicyNotFound(String),
    /// <p>AWS Organizations can't complete your request because of an internal service error. Try again later.</p>
    Service(String),
    /// <p>You have sent too many requests in too short a period of time. The quota helps protect against denial-of-service attacks. Try again later.</p> <p>For information about quotas that affect AWS Organizations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_reference_limits.html">Quotas for AWS Organizations</a>in the <i>AWS Organizations User Guide.</i> </p>
    TooManyRequests(String),
    /// <p>This action isn't available in the current AWS Region.</p>
    UnsupportedAPIEndpoint(String),
}

impl UpdatePolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdatePolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AWSOrganizationsNotInUseException" => {
                    return RusotoError::Service(UpdatePolicyError::AWSOrganizationsNotInUse(
                        err.msg,
                    ))
                }
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdatePolicyError::AccessDenied(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdatePolicyError::ConcurrentModification(err.msg))
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(UpdatePolicyError::ConstraintViolation(err.msg))
                }
                "DuplicatePolicyException" => {
                    return RusotoError::Service(UpdatePolicyError::DuplicatePolicy(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdatePolicyError::InvalidInput(err.msg))
                }
                "MalformedPolicyDocumentException" => {
                    return RusotoError::Service(UpdatePolicyError::MalformedPolicyDocument(
                        err.msg,
                    ))
                }
                "PolicyChangesInProgressException" => {
                    return RusotoError::Service(UpdatePolicyError::PolicyChangesInProgress(
                        err.msg,
                    ))
                }
                "PolicyNotFoundException" => {
                    return RusotoError::Service(UpdatePolicyError::PolicyNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(UpdatePolicyError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdatePolicyError::TooManyRequests(err.msg))
                }
                "UnsupportedAPIEndpointException" => {
                    return RusotoError::Service(UpdatePolicyError::UnsupportedAPIEndpoint(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdatePolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdatePolicyError::AWSOrganizationsNotInUse(ref cause) => write!(f, "{}", cause),
            UpdatePolicyError::AccessDenied(ref cause) => write!(f, "{}", cause),
            UpdatePolicyError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UpdatePolicyError::ConstraintViolation(ref cause) => write!(f, "{}", cause),
            UpdatePolicyError::DuplicatePolicy(ref cause) => write!(f, "{}", cause),
            UpdatePolicyError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdatePolicyError::MalformedPolicyDocument(ref cause) => write!(f, "{}", cause),
            UpdatePolicyError::PolicyChangesInProgress(ref cause) => write!(f, "{}", cause),
            UpdatePolicyError::PolicyNotFound(ref cause) => write!(f, "{}", cause),
            UpdatePolicyError::Service(ref cause) => write!(f, "{}", cause),
            UpdatePolicyError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            UpdatePolicyError::UnsupportedAPIEndpoint(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdatePolicyError {}
/// Trait representing the capabilities of the Organizations API. Organizations clients implement this trait.
#[async_trait]
pub trait Organizations {
    /// <p>Sends a response to the originator of a handshake agreeing to the action proposed by the handshake request.</p> <p>This operation can be called only by the following principals when they also have the relevant IAM permissions:</p> <ul> <li> <p> <b>Invitation to join</b> or <b>Approve all features request</b> handshakes: only a principal from the member account.</p> <p>The user who calls the API for an invitation to join must have the <code>organizations:AcceptHandshake</code> permission. If you enabled all features in the organization, the user must also have the <code>iam:CreateServiceLinkedRole</code> permission so that AWS Organizations can create the required service-linked role named <code>AWSServiceRoleForOrganizations</code>. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integration_services.html#orgs_integration_service-linked-roles">AWS Organizations and Service-Linked Roles</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p> <b>Enable all features final confirmation</b> handshake: only a principal from the management account.</p> <p>For more information about invitations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_invites.html">Inviting an AWS Account to Join Your Organization</a> in the <i>AWS Organizations User Guide.</i> For more information about requests to enable all features in the organization, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">Enabling All Features in Your Organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> </ul> <p>After you accept a handshake, it continues to appear in the results of relevant APIs for only 30 days. After that, it's deleted.</p>
    async fn accept_handshake(
        &self,
        input: AcceptHandshakeRequest,
    ) -> Result<AcceptHandshakeResponse, RusotoError<AcceptHandshakeError>>;

    /// <p>Attaches a policy to a root, an organizational unit (OU), or an individual account. How the policy affects accounts depends on the type of policy. Refer to the <i>AWS Organizations User Guide</i> for information about each policy type:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_ai-opt-out.html">AISERVICES_OPT_OUT_POLICY</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_backup.html">BACKUP_POLICY</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_scp.html">SERVICE_CONTROL_POLICY</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies.html">TAG_POLICY</a> </p> </li> </ul> <p>This operation can be called only from the organization's management account.</p>
    async fn attach_policy(
        &self,
        input: AttachPolicyRequest,
    ) -> Result<(), RusotoError<AttachPolicyError>>;

    /// <p>Cancels a handshake. Canceling a handshake sets the handshake state to <code>CANCELED</code>.</p> <p>This operation can be called only from the account that originated the handshake. The recipient of the handshake can't cancel it, but can use <a>DeclineHandshake</a> instead. After a handshake is canceled, the recipient can no longer respond to that handshake.</p> <p>After you cancel a handshake, it continues to appear in the results of relevant APIs for only 30 days. After that, it's deleted.</p>
    async fn cancel_handshake(
        &self,
        input: CancelHandshakeRequest,
    ) -> Result<CancelHandshakeResponse, RusotoError<CancelHandshakeError>>;

    /// <p><p>Creates an AWS account that is automatically a member of the organization whose credentials made the request. This is an asynchronous request that AWS performs in the background. Because <code>CreateAccount</code> operates asynchronously, it can return a successful completion message even though account initialization might still be in progress. You might need to wait a few minutes before you can successfully access the account. To check the status of the request, do one of the following:</p> <ul> <li> <p>Use the <code>Id</code> member of the <code>CreateAccountStatus</code> response element from this operation to provide as a parameter to the <a>DescribeCreateAccountStatus</a> operation.</p> </li> <li> <p>Check the AWS CloudTrail log for the <code>CreateAccountResult</code> event. For information on using AWS CloudTrail with AWS Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_monitoring.html">Monitoring the Activity in Your Organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> </ul> <p>The user who calls the API to create an account must have the <code>organizations:CreateAccount</code> permission. If you enabled all features in the organization, AWS Organizations creates the required service-linked role named <code>AWSServiceRoleForOrganizations</code>. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html#orgs_integrate_services-using_slrs">AWS Organizations and Service-Linked Roles</a> in the <i>AWS Organizations User Guide</i>.</p> <p>If the request includes tags, then the requester must have the <code>organizations:TagResource</code> permission.</p> <p>AWS Organizations preconfigures the new member account with a role (named <code>OrganizationAccountAccessRole</code> by default) that grants users in the management account administrator permissions in the new member account. Principals in the management account can assume the role. AWS Organizations clones the company name and address information for the new account from the organization&#39;s management account.</p> <p>This operation can be called only from the organization&#39;s management account.</p> <p>For more information about creating accounts, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_create.html">Creating an AWS Account in Your Organization</a> in the <i>AWS Organizations User Guide.</i> </p> <important> <ul> <li> <p>When you create an account in an organization using the AWS Organizations console, API, or CLI commands, the information required for the account to operate as a standalone account, such as a payment method and signing the end user license agreement (EULA) is <i>not</i> automatically collected. If you must remove an account from your organization later, you can do so only after you provide the missing information. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info"> To leave an organization as a member account</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>If you get an exception that indicates that you exceeded your account limits for the organization, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </li> <li> <p>If you get an exception that indicates that the operation failed because your organization is still initializing, wait one hour and then try again. If the error persists, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </li> <li> <p>Using <code>CreateAccount</code> to create multiple temporary accounts isn&#39;t recommended. You can only close an account from the Billing and Cost Management Console, and you must be signed in as the root user. For information on the requirements and process for closing an account, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_close.html">Closing an AWS Account</a> in the <i>AWS Organizations User Guide</i>.</p> </li> </ul> </important> <note> <p>When you create a member account with this operation, you can choose whether to create the account with the <b>IAM User and Role Access to Billing Information</b> switch enabled. If you enable it, IAM users and roles that have appropriate permissions can view billing information for the account. If you disable it, only the account root user can access billing information. For information about how to disable this switch for an account, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/grantaccess.html">Granting Access to Your Billing Information and Tools</a>.</p> </note></p>
    async fn create_account(
        &self,
        input: CreateAccountRequest,
    ) -> Result<CreateAccountResponse, RusotoError<CreateAccountError>>;

    /// <p><p>This action is available if all of the following are true:</p> <ul> <li> <p>You&#39;re authorized to create accounts in the AWS GovCloud (US) Region. For more information on the AWS GovCloud (US) Region, see the <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/welcome.html"> <i>AWS GovCloud User Guide</i>.</a> </p> </li> <li> <p>You already have an account in the AWS GovCloud (US) Region that is paired with a management account of an organization in the commercial Region.</p> </li> <li> <p>You call this action from the management account of your organization in the commercial Region.</p> </li> <li> <p>You have the <code>organizations:CreateGovCloudAccount</code> permission. </p> </li> </ul> <p>AWS Organizations automatically creates the required service-linked role named <code>AWSServiceRoleForOrganizations</code>. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html#orgs_integrate_services-using_slrs">AWS Organizations and Service-Linked Roles</a> in the <i>AWS Organizations User Guide.</i> </p> <p>AWS automatically enables AWS CloudTrail for AWS GovCloud (US) accounts, but you should also do the following:</p> <ul> <li> <p>Verify that AWS CloudTrail is enabled to store logs.</p> </li> <li> <p>Create an S3 bucket for AWS CloudTrail log storage.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/verifying-cloudtrail.html">Verifying AWS CloudTrail Is Enabled</a> in the <i>AWS GovCloud User Guide</i>. </p> </li> </ul> <p>If the request includes tags, then the requester must have the <code>organizations:TagResource</code> permission. The tags are attached to the commercial account associated with the GovCloud account, rather than the GovCloud account itself. To add tags to the GovCloud account, call the <a>TagResource</a> operation in the GovCloud Region after the new GovCloud account exists.</p> <p>You call this action from the management account of your organization in the commercial Region to create a standalone AWS account in the AWS GovCloud (US) Region. After the account is created, the management account of an organization in the AWS GovCloud (US) Region can invite it to that organization. For more information on inviting standalone accounts in the AWS GovCloud (US) to join an organization, see <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/govcloud-organizations.html">AWS Organizations</a> in the <i>AWS GovCloud User Guide.</i> </p> <p>Calling <code>CreateGovCloudAccount</code> is an asynchronous request that AWS performs in the background. Because <code>CreateGovCloudAccount</code> operates asynchronously, it can return a successful completion message even though account initialization might still be in progress. You might need to wait a few minutes before you can successfully access the account. To check the status of the request, do one of the following:</p> <ul> <li> <p>Use the <code>OperationId</code> response element from this operation to provide as a parameter to the <a>DescribeCreateAccountStatus</a> operation.</p> </li> <li> <p>Check the AWS CloudTrail log for the <code>CreateAccountResult</code> event. For information on using AWS CloudTrail with Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_monitoring.html">Monitoring the Activity in Your Organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> </ul> <p/> <p>When you call the <code>CreateGovCloudAccount</code> action, you create two accounts: a standalone account in the AWS GovCloud (US) Region and an associated account in the commercial Region for billing and support purposes. The account in the commercial Region is automatically a member of the organization whose credentials made the request. Both accounts are associated with the same email address.</p> <p>A role is created in the new account in the commercial Region that allows the management account in the organization in the commercial Region to assume it. An AWS GovCloud (US) account is then created and associated with the commercial account that you just created. A role is also created in the new AWS GovCloud (US) account that can be assumed by the AWS GovCloud (US) account that is associated with the management account of the commercial organization. For more information and to view a diagram that explains how account access works, see <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/govcloud-organizations.html">AWS Organizations</a> in the <i>AWS GovCloud User Guide.</i> </p> <p>For more information about creating accounts, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_create.html">Creating an AWS Account in Your Organization</a> in the <i>AWS Organizations User Guide.</i> </p> <important> <ul> <li> <p>When you create an account in an organization using the AWS Organizations console, API, or CLI commands, the information required for the account to operate as a standalone account is <i>not</i> automatically collected. This includes a payment method and signing the end user license agreement (EULA). If you must remove an account from your organization later, you can do so only after you provide the missing information. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info"> To leave an organization as a member account</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>If you get an exception that indicates that you exceeded your account limits for the organization, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </li> <li> <p>If you get an exception that indicates that the operation failed because your organization is still initializing, wait one hour and then try again. If the error persists, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </li> <li> <p>Using <code>CreateGovCloudAccount</code> to create multiple temporary accounts isn&#39;t recommended. You can only close an account from the AWS Billing and Cost Management console, and you must be signed in as the root user. For information on the requirements and process for closing an account, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_close.html">Closing an AWS Account</a> in the <i>AWS Organizations User Guide</i>.</p> </li> </ul> </important> <note> <p>When you create a member account with this operation, you can choose whether to create the account with the <b>IAM User and Role Access to Billing Information</b> switch enabled. If you enable it, IAM users and roles that have appropriate permissions can view billing information for the account. If you disable it, only the account root user can access billing information. For information about how to disable this switch for an account, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/grantaccess.html">Granting Access to Your Billing Information and Tools</a>.</p> </note></p>
    async fn create_gov_cloud_account(
        &self,
        input: CreateGovCloudAccountRequest,
    ) -> Result<CreateGovCloudAccountResponse, RusotoError<CreateGovCloudAccountError>>;

    /// <p>Creates an AWS organization. The account whose user is calling the <code>CreateOrganization</code> operation automatically becomes the <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_getting-started_concepts.html#account">management account</a> of the new organization.</p> <p>This operation must be called using credentials from the account that is to become the new organization's management account. The principal must also have the relevant IAM permissions.</p> <p>By default (or if you set the <code>FeatureSet</code> parameter to <code>ALL</code>), the new organization is created with all features enabled and service control policies automatically enabled in the root. If you instead choose to create the organization supporting only the consolidated billing features by setting the <code>FeatureSet</code> parameter to <code>CONSOLIDATED_BILLING"</code>, no policy types are enabled by default, and you can't use organization policies</p>
    async fn create_organization(
        &self,
        input: CreateOrganizationRequest,
    ) -> Result<CreateOrganizationResponse, RusotoError<CreateOrganizationError>>;

    /// <p>Creates an organizational unit (OU) within a root or parent OU. An OU is a container for accounts that enables you to organize your accounts to apply policies according to your business requirements. The number of levels deep that you can nest OUs is dependent upon the policy types enabled for that root. For service control policies, the limit is five.</p> <p>For more information about OUs, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_ous.html">Managing Organizational Units</a> in the <i>AWS Organizations User Guide.</i> </p> <p>If the request includes tags, then the requester must have the <code>organizations:TagResource</code> permission.</p> <p>This operation can be called only from the organization's management account.</p>
    async fn create_organizational_unit(
        &self,
        input: CreateOrganizationalUnitRequest,
    ) -> Result<CreateOrganizationalUnitResponse, RusotoError<CreateOrganizationalUnitError>>;

    /// <p>Creates a policy of a specified type that you can attach to a root, an organizational unit (OU), or an individual AWS account.</p> <p>For more information about policies and their use, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies.html">Managing Organization Policies</a>.</p> <p>If the request includes tags, then the requester must have the <code>organizations:TagResource</code> permission.</p> <p>This operation can be called only from the organization's management account.</p>
    async fn create_policy(
        &self,
        input: CreatePolicyRequest,
    ) -> Result<CreatePolicyResponse, RusotoError<CreatePolicyError>>;

    /// <p>Declines a handshake request. This sets the handshake state to <code>DECLINED</code> and effectively deactivates the request.</p> <p>This operation can be called only from the account that received the handshake. The originator of the handshake can use <a>CancelHandshake</a> instead. The originator can't reactivate a declined request, but can reinitiate the process with a new handshake request.</p> <p>After you decline a handshake, it continues to appear in the results of relevant APIs for only 30 days. After that, it's deleted.</p>
    async fn decline_handshake(
        &self,
        input: DeclineHandshakeRequest,
    ) -> Result<DeclineHandshakeResponse, RusotoError<DeclineHandshakeError>>;

    /// <p>Deletes the organization. You can delete an organization only by using credentials from the management account. The organization must be empty of member accounts.</p>
    async fn delete_organization(&self) -> Result<(), RusotoError<DeleteOrganizationError>>;

    /// <p>Deletes an organizational unit (OU) from a root or another OU. You must first remove all accounts and child OUs from the OU that you want to delete.</p> <p>This operation can be called only from the organization's management account.</p>
    async fn delete_organizational_unit(
        &self,
        input: DeleteOrganizationalUnitRequest,
    ) -> Result<(), RusotoError<DeleteOrganizationalUnitError>>;

    /// <p>Deletes the specified policy from your organization. Before you perform this operation, you must first detach the policy from all organizational units (OUs), roots, and accounts.</p> <p>This operation can be called only from the organization's management account.</p>
    async fn delete_policy(
        &self,
        input: DeletePolicyRequest,
    ) -> Result<(), RusotoError<DeletePolicyError>>;

    /// <p>Removes the specified member AWS account as a delegated administrator for the specified AWS service.</p> <important> <p>Deregistering a delegated administrator can have unintended impacts on the functionality of the enabled AWS service. See the documentation for the enabled service before you deregister a delegated administrator so that you understand any potential impacts.</p> </important> <p>You can run this action only for AWS services that support this feature. For a current list of services that support it, see the column <i>Supports Delegated Administrator</i> in the table at <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrated-services-list.html">AWS Services that you can use with AWS Organizations</a> in the <i>AWS Organizations User Guide.</i> </p> <p>This operation can be called only from the organization's management account.</p>
    async fn deregister_delegated_administrator(
        &self,
        input: DeregisterDelegatedAdministratorRequest,
    ) -> Result<(), RusotoError<DeregisterDelegatedAdministratorError>>;

    /// <p>Retrieves AWS Organizations-related information about the specified account.</p> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn describe_account(
        &self,
        input: DescribeAccountRequest,
    ) -> Result<DescribeAccountResponse, RusotoError<DescribeAccountError>>;

    /// <p>Retrieves the current status of an asynchronous request to create an account.</p> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn describe_create_account_status(
        &self,
        input: DescribeCreateAccountStatusRequest,
    ) -> Result<DescribeCreateAccountStatusResponse, RusotoError<DescribeCreateAccountStatusError>>;

    /// <p>Returns the contents of the effective policy for specified policy type and account. The effective policy is the aggregation of any policies of the specified type that the account inherits, plus any policy of that type that is directly attached to the account.</p> <p>This operation applies only to policy types <i>other</i> than service control policies (SCPs).</p> <p>For more information about policy inheritance, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies-inheritance.html">How Policy Inheritance Works</a> in the <i>AWS Organizations User Guide</i>.</p> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn describe_effective_policy(
        &self,
        input: DescribeEffectivePolicyRequest,
    ) -> Result<DescribeEffectivePolicyResponse, RusotoError<DescribeEffectivePolicyError>>;

    /// <p>Retrieves information about a previously requested handshake. The handshake ID comes from the response to the original <a>InviteAccountToOrganization</a> operation that generated the handshake.</p> <p>You can access handshakes that are <code>ACCEPTED</code>, <code>DECLINED</code>, or <code>CANCELED</code> for only 30 days after they change to that state. They're then deleted and no longer accessible.</p> <p>This operation can be called from any account in the organization.</p>
    async fn describe_handshake(
        &self,
        input: DescribeHandshakeRequest,
    ) -> Result<DescribeHandshakeResponse, RusotoError<DescribeHandshakeError>>;

    /// <p><p>Retrieves information about the organization that the user&#39;s account belongs to.</p> <p>This operation can be called from any account in the organization.</p> <note> <p>Even if a policy type is shown as available in the organization, you can disable it separately at the root level with <a>DisablePolicyType</a>. Use <a>ListRoots</a> to see the status of policy types for a specified root.</p> </note></p>
    async fn describe_organization(
        &self,
    ) -> Result<DescribeOrganizationResponse, RusotoError<DescribeOrganizationError>>;

    /// <p>Retrieves information about an organizational unit (OU).</p> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn describe_organizational_unit(
        &self,
        input: DescribeOrganizationalUnitRequest,
    ) -> Result<DescribeOrganizationalUnitResponse, RusotoError<DescribeOrganizationalUnitError>>;

    /// <p>Retrieves information about a policy.</p> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn describe_policy(
        &self,
        input: DescribePolicyRequest,
    ) -> Result<DescribePolicyResponse, RusotoError<DescribePolicyError>>;

    /// <p>Detaches a policy from a target root, organizational unit (OU), or account.</p> <important> <p>If the policy being detached is a service control policy (SCP), the changes to permissions for AWS Identity and Access Management (IAM) users and roles in affected accounts are immediate.</p> </important> <p>Every root, OU, and account must have at least one SCP attached. If you want to replace the default <code>FullAWSAccess</code> policy with an SCP that limits the permissions that can be delegated, you must attach the replacement SCP before you can remove the default SCP. This is the authorization strategy of an "<a href="https://docs.aws.amazon.com/organizations/latest/userguide/SCP_strategies.html#orgs_policies_allowlist">allow list</a>". If you instead attach a second SCP and leave the <code>FullAWSAccess</code> SCP still attached, and specify <code>"Effect": "Deny"</code> in the second SCP to override the <code>"Effect": "Allow"</code> in the <code>FullAWSAccess</code> policy (or any other attached SCP), you're using the authorization strategy of a "<a href="https://docs.aws.amazon.com/organizations/latest/userguide/SCP_strategies.html#orgs_policies_denylist">deny list</a>".</p> <p>This operation can be called only from the organization's management account.</p>
    async fn detach_policy(
        &self,
        input: DetachPolicyRequest,
    ) -> Result<(), RusotoError<DetachPolicyError>>;

    /// <p>Disables the integration of an AWS service (the service that is specified by <code>ServicePrincipal</code>) with AWS Organizations. When you disable integration, the specified service no longer can create a <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/using-service-linked-roles.html">service-linked role</a> in <i>new</i> accounts in your organization. This means the service can't perform operations on your behalf on any new accounts in your organization. The service can still perform operations in older accounts until the service completes its clean-up from AWS Organizations.</p> <p/> <important> <p>We recommend that you disable integration between AWS Organizations and the specified AWS service by using the console or commands that are provided by the specified service. Doing so ensures that the other service is aware that it can clean up any resources that are required only for the integration. How the service cleans up its resources in the organization's accounts depends on that service. For more information, see the documentation for the other AWS service.</p> </important> <p>After you perform the <code>DisableAWSServiceAccess</code> operation, the specified service can no longer perform operations in your organization's accounts unless the operations are explicitly permitted by the IAM policies that are attached to your roles.</p> <p>For more information about integrating other services with AWS Organizations, including the list of services that work with Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html">Integrating AWS Organizations with Other AWS Services</a> in the <i>AWS Organizations User Guide.</i> </p> <p>This operation can be called only from the organization's management account.</p>
    async fn disable_aws_service_access(
        &self,
        input: DisableAWSServiceAccessRequest,
    ) -> Result<(), RusotoError<DisableAWSServiceAccessError>>;

    /// <p>Disables an organizational policy type in a root. A policy of a certain type can be attached to entities in a root only if that type is enabled in the root. After you perform this operation, you no longer can attach policies of the specified type to that root or to any organizational unit (OU) or account in that root. You can undo this by using the <a>EnablePolicyType</a> operation.</p> <p>This is an asynchronous request that AWS performs in the background. If you disable a policy type for a root, it still appears enabled for the organization if <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">all features</a> are enabled for the organization. AWS recommends that you first use <a>ListRoots</a> to see the status of policy types for a specified root, and then use this operation.</p> <p>This operation can be called only from the organization's management account.</p> <p> To view the status of available policy types in the organization, use <a>DescribeOrganization</a>.</p>
    async fn disable_policy_type(
        &self,
        input: DisablePolicyTypeRequest,
    ) -> Result<DisablePolicyTypeResponse, RusotoError<DisablePolicyTypeError>>;

    /// <p>Enables the integration of an AWS service (the service that is specified by <code>ServicePrincipal</code>) with AWS Organizations. When you enable integration, you allow the specified service to create a <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/using-service-linked-roles.html">service-linked role</a> in all the accounts in your organization. This allows the service to perform operations on your behalf in your organization and its accounts.</p> <important> <p>We recommend that you enable integration between AWS Organizations and the specified AWS service by using the console or commands that are provided by the specified service. Doing so ensures that the service is aware that it can create the resources that are required for the integration. How the service creates those resources in the organization's accounts depends on that service. For more information, see the documentation for the other AWS service.</p> </important> <p>For more information about enabling services to integrate with AWS Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html">Integrating AWS Organizations with Other AWS Services</a> in the <i>AWS Organizations User Guide.</i> </p> <p>This operation can be called only from the organization's management account and only if the organization has <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">enabled all features</a>.</p>
    async fn enable_aws_service_access(
        &self,
        input: EnableAWSServiceAccessRequest,
    ) -> Result<(), RusotoError<EnableAWSServiceAccessError>>;

    /// <p>Enables all features in an organization. This enables the use of organization policies that can restrict the services and actions that can be called in each account. Until you enable all features, you have access only to consolidated billing, and you can't use any of the advanced account administration features that AWS Organizations supports. For more information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">Enabling All Features in Your Organization</a> in the <i>AWS Organizations User Guide.</i> </p> <important> <p>This operation is required only for organizations that were created explicitly with only the consolidated billing features enabled. Calling this operation sends a handshake to every invited account in the organization. The feature set change can be finalized and the additional features enabled only after all administrators in the invited accounts approve the change by accepting the handshake.</p> </important> <p>After you enable all features, you can separately enable or disable individual policy types in a root using <a>EnablePolicyType</a> and <a>DisablePolicyType</a>. To see the status of policy types in a root, use <a>ListRoots</a>.</p> <p>After all invited member accounts accept the handshake, you finalize the feature set change by accepting the handshake that contains <code>"Action": "ENABLE_ALL_FEATURES"</code>. This completes the change.</p> <p>After you enable all features in your organization, the management account in the organization can apply policies on all member accounts. These policies can restrict what users and even administrators in those accounts can do. The management account can apply policies that prevent accounts from leaving the organization. Ensure that your account administrators are aware of this.</p> <p>This operation can be called only from the organization's management account.</p>
    async fn enable_all_features(
        &self,
    ) -> Result<EnableAllFeaturesResponse, RusotoError<EnableAllFeaturesError>>;

    /// <p>Enables a policy type in a root. After you enable a policy type in a root, you can attach policies of that type to the root, any organizational unit (OU), or account in that root. You can undo this by using the <a>DisablePolicyType</a> operation.</p> <p>This is an asynchronous request that AWS performs in the background. AWS recommends that you first use <a>ListRoots</a> to see the status of policy types for a specified root, and then use this operation.</p> <p>This operation can be called only from the organization's management account.</p> <p>You can enable a policy type in a root only if that policy type is available in the organization. To view the status of available policy types in the organization, use <a>DescribeOrganization</a>.</p>
    async fn enable_policy_type(
        &self,
        input: EnablePolicyTypeRequest,
    ) -> Result<EnablePolicyTypeResponse, RusotoError<EnablePolicyTypeError>>;

    /// <p>Sends an invitation to another account to join your organization as a member account. AWS Organizations sends email on your behalf to the email address that is associated with the other account's owner. The invitation is implemented as a <a>Handshake</a> whose details are in the response.</p> <important> <ul> <li> <p>You can invite AWS accounts only from the same seller as the management account. For example, if your organization's management account was created by Amazon Internet Services Pvt. Ltd (AISPL), an AWS seller in India, you can invite only other AISPL accounts to your organization. You can't combine accounts from AISPL and AWS or from any other AWS seller. For more information, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/useconsolidatedbilliing-India.html">Consolidated Billing in India</a>.</p> </li> <li> <p>If you receive an exception that indicates that you exceeded your account limits for the organization or that the operation failed because your organization is still initializing, wait one hour and then try again. If the error persists after an hour, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </li> </ul> </important> <p>If the request includes tags, then the requester must have the <code>organizations:TagResource</code> permission.</p> <p>This operation can be called only from the organization's management account.</p>
    async fn invite_account_to_organization(
        &self,
        input: InviteAccountToOrganizationRequest,
    ) -> Result<InviteAccountToOrganizationResponse, RusotoError<InviteAccountToOrganizationError>>;

    /// <p><p>Removes a member account from its parent organization. This version of the operation is performed by the account that wants to leave. To remove a member account as a user in the management account, use <a>RemoveAccountFromOrganization</a> instead.</p> <p>This operation can be called only from a member account in the organization.</p> <important> <ul> <li> <p>The management account in an organization with all features enabled can set service control policies (SCPs) that can restrict what administrators of member accounts can do. This includes preventing them from successfully calling <code>LeaveOrganization</code> and leaving the organization.</p> </li> <li> <p>You can leave an organization as a member account only if the account is configured with the information required to operate as a standalone account. When you create an account in an organization using the AWS Organizations console, API, or CLI commands, the information required of standalone accounts is <i>not</i> automatically collected. For each account that you want to make standalone, you must perform the following steps. If any of the steps are already completed for this account, that step doesn&#39;t appear.</p> <ul> <li> <p>Choose a support plan</p> </li> <li> <p>Provide and verify the required contact information</p> </li> <li> <p>Provide a current payment method</p> </li> </ul> <p>AWS uses the payment method to charge for any billable (not free tier) AWS activity that occurs while the account isn&#39;t attached to an organization. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info"> To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>You can leave an organization only after you enable IAM user access to billing in your account. For more information, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/grantaccess.html#ControllingAccessWebsite-Activate">Activating Access to the Billing and Cost Management Console</a> in the <i>AWS Billing and Cost Management User Guide.</i> </p> </li> <li> <p>After the account leaves the organization, all tags that were attached to the account object in the organization are deleted. AWS accounts outside of an organization do not support tags.</p> </li> </ul> </important></p>
    async fn leave_organization(&self) -> Result<(), RusotoError<LeaveOrganizationError>>;

    /// <p>Returns a list of the AWS services that you enabled to integrate with your organization. After a service on this list creates the resources that it requires for the integration, it can perform operations on your organization and its accounts.</p> <p>For more information about integrating other services with AWS Organizations, including the list of services that currently work with Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html">Integrating AWS Organizations with Other AWS Services</a> in the <i>AWS Organizations User Guide.</i> </p> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn list_aws_service_access_for_organization(
        &self,
        input: ListAWSServiceAccessForOrganizationRequest,
    ) -> Result<
        ListAWSServiceAccessForOrganizationResponse,
        RusotoError<ListAWSServiceAccessForOrganizationError>,
    >;

    /// <p>Lists all the accounts in the organization. To request only the accounts in a specified root or organizational unit (OU), use the <a>ListAccountsForParent</a> operation instead.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn list_accounts(
        &self,
        input: ListAccountsRequest,
    ) -> Result<ListAccountsResponse, RusotoError<ListAccountsError>>;

    /// <p>Lists the accounts in an organization that are contained by the specified target root or organizational unit (OU). If you specify the root, you get a list of all the accounts that aren't in any OU. If you specify an OU, you get a list of all the accounts in only that OU and not in any child OUs. To get a list of all accounts in the organization, use the <a>ListAccounts</a> operation.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn list_accounts_for_parent(
        &self,
        input: ListAccountsForParentRequest,
    ) -> Result<ListAccountsForParentResponse, RusotoError<ListAccountsForParentError>>;

    /// <p>Lists all of the organizational units (OUs) or accounts that are contained in the specified parent OU or root. This operation, along with <a>ListParents</a> enables you to traverse the tree structure that makes up this root.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn list_children(
        &self,
        input: ListChildrenRequest,
    ) -> Result<ListChildrenResponse, RusotoError<ListChildrenError>>;

    /// <p>Lists the account creation requests that match the specified status that is currently being tracked for the organization.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn list_create_account_status(
        &self,
        input: ListCreateAccountStatusRequest,
    ) -> Result<ListCreateAccountStatusResponse, RusotoError<ListCreateAccountStatusError>>;

    /// <p>Lists the AWS accounts that are designated as delegated administrators in this organization.</p> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn list_delegated_administrators(
        &self,
        input: ListDelegatedAdministratorsRequest,
    ) -> Result<ListDelegatedAdministratorsResponse, RusotoError<ListDelegatedAdministratorsError>>;

    /// <p>List the AWS services for which the specified account is a delegated administrator.</p> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn list_delegated_services_for_account(
        &self,
        input: ListDelegatedServicesForAccountRequest,
    ) -> Result<
        ListDelegatedServicesForAccountResponse,
        RusotoError<ListDelegatedServicesForAccountError>,
    >;

    /// <p>Lists the current handshakes that are associated with the account of the requesting user.</p> <p>Handshakes that are <code>ACCEPTED</code>, <code>DECLINED</code>, or <code>CANCELED</code> appear in the results of this API for only 30 days after changing to that state. After that, they're deleted and no longer accessible.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called from any account in the organization.</p>
    async fn list_handshakes_for_account(
        &self,
        input: ListHandshakesForAccountRequest,
    ) -> Result<ListHandshakesForAccountResponse, RusotoError<ListHandshakesForAccountError>>;

    /// <p>Lists the handshakes that are associated with the organization that the requesting user is part of. The <code>ListHandshakesForOrganization</code> operation returns a list of handshake structures. Each structure contains details and status about a handshake.</p> <p>Handshakes that are <code>ACCEPTED</code>, <code>DECLINED</code>, or <code>CANCELED</code> appear in the results of this API for only 30 days after changing to that state. After that, they're deleted and no longer accessible.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn list_handshakes_for_organization(
        &self,
        input: ListHandshakesForOrganizationRequest,
    ) -> Result<
        ListHandshakesForOrganizationResponse,
        RusotoError<ListHandshakesForOrganizationError>,
    >;

    /// <p>Lists the organizational units (OUs) in a parent organizational unit or root.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn list_organizational_units_for_parent(
        &self,
        input: ListOrganizationalUnitsForParentRequest,
    ) -> Result<
        ListOrganizationalUnitsForParentResponse,
        RusotoError<ListOrganizationalUnitsForParentError>,
    >;

    /// <p><p>Lists the root or organizational units (OUs) that serve as the immediate parent of the specified child OU or account. This operation, along with <a>ListChildren</a> enables you to traverse the tree structure that makes up this root.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization&#39;s management account or by a member account that is a delegated administrator for an AWS service.</p> <note> <p>In the current release, a child can have only a single parent.</p> </note></p>
    async fn list_parents(
        &self,
        input: ListParentsRequest,
    ) -> Result<ListParentsResponse, RusotoError<ListParentsError>>;

    /// <p>Retrieves the list of all policies in an organization of a specified type.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn list_policies(
        &self,
        input: ListPoliciesRequest,
    ) -> Result<ListPoliciesResponse, RusotoError<ListPoliciesError>>;

    /// <p>Lists the policies that are directly attached to the specified target root, organizational unit (OU), or account. You must specify the policy type that you want included in the returned list.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn list_policies_for_target(
        &self,
        input: ListPoliciesForTargetRequest,
    ) -> Result<ListPoliciesForTargetResponse, RusotoError<ListPoliciesForTargetError>>;

    /// <p><p>Lists the roots that are defined in the current organization.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization&#39;s management account or by a member account that is a delegated administrator for an AWS service.</p> <note> <p>Policy types can be enabled and disabled in roots. This is distinct from whether they&#39;re available in the organization. When you enable all features, you make policy types available for use in that organization. Individual policy types can then be enabled and disabled in a root. To see the availability of a policy type in an organization, use <a>DescribeOrganization</a>.</p> </note></p>
    async fn list_roots(
        &self,
        input: ListRootsRequest,
    ) -> Result<ListRootsResponse, RusotoError<ListRootsError>>;

    /// <p>Lists tags that are attached to the specified resource.</p> <p>You can attach tags to the following resources in AWS Organizations.</p> <ul> <li> <p>AWS account</p> </li> <li> <p>Organization root</p> </li> <li> <p>Organizational unit (OU)</p> </li> <li> <p>Policy (any type)</p> </li> </ul> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Lists all the roots, organizational units (OUs), and accounts that the specified policy is attached to.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn list_targets_for_policy(
        &self,
        input: ListTargetsForPolicyRequest,
    ) -> Result<ListTargetsForPolicyResponse, RusotoError<ListTargetsForPolicyError>>;

    /// <p>Moves an account from its current source parent root or organizational unit (OU) to the specified destination parent root or OU.</p> <p>This operation can be called only from the organization's management account.</p>
    async fn move_account(
        &self,
        input: MoveAccountRequest,
    ) -> Result<(), RusotoError<MoveAccountError>>;

    /// <p>Enables the specified member account to administer the Organizations features of the specified AWS service. It grants read-only access to AWS Organizations service data. The account still requires IAM permissions to access and administer the AWS service.</p> <p>You can run this action only for AWS services that support this feature. For a current list of services that support it, see the column <i>Supports Delegated Administrator</i> in the table at <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrated-services-list.html">AWS Services that you can use with AWS Organizations</a> in the <i>AWS Organizations User Guide.</i> </p> <p>This operation can be called only from the organization's management account.</p>
    async fn register_delegated_administrator(
        &self,
        input: RegisterDelegatedAdministratorRequest,
    ) -> Result<(), RusotoError<RegisterDelegatedAdministratorError>>;

    /// <p><p>Removes the specified account from the organization.</p> <p>The removed account becomes a standalone account that isn&#39;t a member of any organization. It&#39;s no longer subject to any policies and is responsible for its own bill payments. The organization&#39;s management account is no longer charged for any expenses accrued by the member account after it&#39;s removed from the organization.</p> <p>This operation can be called only from the organization&#39;s management account. Member accounts can remove themselves with <a>LeaveOrganization</a> instead.</p> <important> <ul> <li> <p>You can remove an account from your organization only if the account is configured with the information required to operate as a standalone account. When you create an account in an organization using the AWS Organizations console, API, or CLI commands, the information required of standalone accounts is <i>not</i> automatically collected. For an account that you want to make standalone, you must choose a support plan, provide and verify the required contact information, and provide a current payment method. AWS uses the payment method to charge for any billable (not free tier) AWS activity that occurs while the account isn&#39;t attached to an organization. To remove an account that doesn&#39;t yet have this information, you must sign in as the member account and follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info"> To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>After the account leaves the organization, all tags that were attached to the account object in the organization are deleted. AWS accounts outside of an organization do not support tags.</p> </li> </ul> </important></p>
    async fn remove_account_from_organization(
        &self,
        input: RemoveAccountFromOrganizationRequest,
    ) -> Result<(), RusotoError<RemoveAccountFromOrganizationError>>;

    /// <p>Adds one or more tags to the specified resource.</p> <p>Currently, you can attach tags to the following resources in AWS Organizations.</p> <ul> <li> <p>AWS account</p> </li> <li> <p>Organization root</p> </li> <li> <p>Organizational unit (OU)</p> </li> <li> <p>Policy (any type)</p> </li> </ul> <p>This operation can be called only from the organization's management account.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>>;

    /// <p>Removes any tags with the specified keys from the specified resource.</p> <p>You can attach tags to the following resources in AWS Organizations.</p> <ul> <li> <p>AWS account</p> </li> <li> <p>Organization root</p> </li> <li> <p>Organizational unit (OU)</p> </li> <li> <p>Policy (any type)</p> </li> </ul> <p>This operation can be called only from the organization's management account.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>>;

    /// <p>Renames the specified organizational unit (OU). The ID and ARN don't change. The child OUs and accounts remain in place, and any attached policies of the OU remain attached.</p> <p>This operation can be called only from the organization's management account.</p>
    async fn update_organizational_unit(
        &self,
        input: UpdateOrganizationalUnitRequest,
    ) -> Result<UpdateOrganizationalUnitResponse, RusotoError<UpdateOrganizationalUnitError>>;

    /// <p>Updates an existing policy with a new name, description, or content. If you don't supply any parameter, that value remains unchanged. You can't change a policy's type.</p> <p>This operation can be called only from the organization's management account.</p>
    async fn update_policy(
        &self,
        input: UpdatePolicyRequest,
    ) -> Result<UpdatePolicyResponse, RusotoError<UpdatePolicyError>>;
}
/// A client for the Organizations API.
#[derive(Clone)]
pub struct OrganizationsClient {
    client: Client,
    region: region::Region,
}

impl OrganizationsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> OrganizationsClient {
        OrganizationsClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> OrganizationsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        OrganizationsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> OrganizationsClient {
        OrganizationsClient { client, region }
    }
}

#[async_trait]
impl Organizations for OrganizationsClient {
    /// <p>Sends a response to the originator of a handshake agreeing to the action proposed by the handshake request.</p> <p>This operation can be called only by the following principals when they also have the relevant IAM permissions:</p> <ul> <li> <p> <b>Invitation to join</b> or <b>Approve all features request</b> handshakes: only a principal from the member account.</p> <p>The user who calls the API for an invitation to join must have the <code>organizations:AcceptHandshake</code> permission. If you enabled all features in the organization, the user must also have the <code>iam:CreateServiceLinkedRole</code> permission so that AWS Organizations can create the required service-linked role named <code>AWSServiceRoleForOrganizations</code>. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integration_services.html#orgs_integration_service-linked-roles">AWS Organizations and Service-Linked Roles</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p> <b>Enable all features final confirmation</b> handshake: only a principal from the management account.</p> <p>For more information about invitations, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_invites.html">Inviting an AWS Account to Join Your Organization</a> in the <i>AWS Organizations User Guide.</i> For more information about requests to enable all features in the organization, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">Enabling All Features in Your Organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> </ul> <p>After you accept a handshake, it continues to appear in the results of relevant APIs for only 30 days. After that, it's deleted.</p>
    async fn accept_handshake(
        &self,
        input: AcceptHandshakeRequest,
    ) -> Result<AcceptHandshakeResponse, RusotoError<AcceptHandshakeError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.AcceptHandshake");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AcceptHandshakeError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<AcceptHandshakeResponse, _>()
    }

    /// <p>Attaches a policy to a root, an organizational unit (OU), or an individual account. How the policy affects accounts depends on the type of policy. Refer to the <i>AWS Organizations User Guide</i> for information about each policy type:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_ai-opt-out.html">AISERVICES_OPT_OUT_POLICY</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_backup.html">BACKUP_POLICY</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_scp.html">SERVICE_CONTROL_POLICY</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies.html">TAG_POLICY</a> </p> </li> </ul> <p>This operation can be called only from the organization's management account.</p>
    async fn attach_policy(
        &self,
        input: AttachPolicyRequest,
    ) -> Result<(), RusotoError<AttachPolicyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.AttachPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AttachPolicyError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Cancels a handshake. Canceling a handshake sets the handshake state to <code>CANCELED</code>.</p> <p>This operation can be called only from the account that originated the handshake. The recipient of the handshake can't cancel it, but can use <a>DeclineHandshake</a> instead. After a handshake is canceled, the recipient can no longer respond to that handshake.</p> <p>After you cancel a handshake, it continues to appear in the results of relevant APIs for only 30 days. After that, it's deleted.</p>
    async fn cancel_handshake(
        &self,
        input: CancelHandshakeRequest,
    ) -> Result<CancelHandshakeResponse, RusotoError<CancelHandshakeError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.CancelHandshake");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CancelHandshakeError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CancelHandshakeResponse, _>()
    }

    /// <p><p>Creates an AWS account that is automatically a member of the organization whose credentials made the request. This is an asynchronous request that AWS performs in the background. Because <code>CreateAccount</code> operates asynchronously, it can return a successful completion message even though account initialization might still be in progress. You might need to wait a few minutes before you can successfully access the account. To check the status of the request, do one of the following:</p> <ul> <li> <p>Use the <code>Id</code> member of the <code>CreateAccountStatus</code> response element from this operation to provide as a parameter to the <a>DescribeCreateAccountStatus</a> operation.</p> </li> <li> <p>Check the AWS CloudTrail log for the <code>CreateAccountResult</code> event. For information on using AWS CloudTrail with AWS Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_monitoring.html">Monitoring the Activity in Your Organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> </ul> <p>The user who calls the API to create an account must have the <code>organizations:CreateAccount</code> permission. If you enabled all features in the organization, AWS Organizations creates the required service-linked role named <code>AWSServiceRoleForOrganizations</code>. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html#orgs_integrate_services-using_slrs">AWS Organizations and Service-Linked Roles</a> in the <i>AWS Organizations User Guide</i>.</p> <p>If the request includes tags, then the requester must have the <code>organizations:TagResource</code> permission.</p> <p>AWS Organizations preconfigures the new member account with a role (named <code>OrganizationAccountAccessRole</code> by default) that grants users in the management account administrator permissions in the new member account. Principals in the management account can assume the role. AWS Organizations clones the company name and address information for the new account from the organization&#39;s management account.</p> <p>This operation can be called only from the organization&#39;s management account.</p> <p>For more information about creating accounts, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_create.html">Creating an AWS Account in Your Organization</a> in the <i>AWS Organizations User Guide.</i> </p> <important> <ul> <li> <p>When you create an account in an organization using the AWS Organizations console, API, or CLI commands, the information required for the account to operate as a standalone account, such as a payment method and signing the end user license agreement (EULA) is <i>not</i> automatically collected. If you must remove an account from your organization later, you can do so only after you provide the missing information. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info"> To leave an organization as a member account</a> in the <i>AWS Organizations User Guide</i>.</p> </li> <li> <p>If you get an exception that indicates that you exceeded your account limits for the organization, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </li> <li> <p>If you get an exception that indicates that the operation failed because your organization is still initializing, wait one hour and then try again. If the error persists, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </li> <li> <p>Using <code>CreateAccount</code> to create multiple temporary accounts isn&#39;t recommended. You can only close an account from the Billing and Cost Management Console, and you must be signed in as the root user. For information on the requirements and process for closing an account, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_close.html">Closing an AWS Account</a> in the <i>AWS Organizations User Guide</i>.</p> </li> </ul> </important> <note> <p>When you create a member account with this operation, you can choose whether to create the account with the <b>IAM User and Role Access to Billing Information</b> switch enabled. If you enable it, IAM users and roles that have appropriate permissions can view billing information for the account. If you disable it, only the account root user can access billing information. For information about how to disable this switch for an account, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/grantaccess.html">Granting Access to Your Billing Information and Tools</a>.</p> </note></p>
    async fn create_account(
        &self,
        input: CreateAccountRequest,
    ) -> Result<CreateAccountResponse, RusotoError<CreateAccountError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.CreateAccount");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateAccountError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateAccountResponse, _>()
    }

    /// <p><p>This action is available if all of the following are true:</p> <ul> <li> <p>You&#39;re authorized to create accounts in the AWS GovCloud (US) Region. For more information on the AWS GovCloud (US) Region, see the <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/welcome.html"> <i>AWS GovCloud User Guide</i>.</a> </p> </li> <li> <p>You already have an account in the AWS GovCloud (US) Region that is paired with a management account of an organization in the commercial Region.</p> </li> <li> <p>You call this action from the management account of your organization in the commercial Region.</p> </li> <li> <p>You have the <code>organizations:CreateGovCloudAccount</code> permission. </p> </li> </ul> <p>AWS Organizations automatically creates the required service-linked role named <code>AWSServiceRoleForOrganizations</code>. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html#orgs_integrate_services-using_slrs">AWS Organizations and Service-Linked Roles</a> in the <i>AWS Organizations User Guide.</i> </p> <p>AWS automatically enables AWS CloudTrail for AWS GovCloud (US) accounts, but you should also do the following:</p> <ul> <li> <p>Verify that AWS CloudTrail is enabled to store logs.</p> </li> <li> <p>Create an S3 bucket for AWS CloudTrail log storage.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/verifying-cloudtrail.html">Verifying AWS CloudTrail Is Enabled</a> in the <i>AWS GovCloud User Guide</i>. </p> </li> </ul> <p>If the request includes tags, then the requester must have the <code>organizations:TagResource</code> permission. The tags are attached to the commercial account associated with the GovCloud account, rather than the GovCloud account itself. To add tags to the GovCloud account, call the <a>TagResource</a> operation in the GovCloud Region after the new GovCloud account exists.</p> <p>You call this action from the management account of your organization in the commercial Region to create a standalone AWS account in the AWS GovCloud (US) Region. After the account is created, the management account of an organization in the AWS GovCloud (US) Region can invite it to that organization. For more information on inviting standalone accounts in the AWS GovCloud (US) to join an organization, see <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/govcloud-organizations.html">AWS Organizations</a> in the <i>AWS GovCloud User Guide.</i> </p> <p>Calling <code>CreateGovCloudAccount</code> is an asynchronous request that AWS performs in the background. Because <code>CreateGovCloudAccount</code> operates asynchronously, it can return a successful completion message even though account initialization might still be in progress. You might need to wait a few minutes before you can successfully access the account. To check the status of the request, do one of the following:</p> <ul> <li> <p>Use the <code>OperationId</code> response element from this operation to provide as a parameter to the <a>DescribeCreateAccountStatus</a> operation.</p> </li> <li> <p>Check the AWS CloudTrail log for the <code>CreateAccountResult</code> event. For information on using AWS CloudTrail with Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_monitoring.html">Monitoring the Activity in Your Organization</a> in the <i>AWS Organizations User Guide.</i> </p> </li> </ul> <p/> <p>When you call the <code>CreateGovCloudAccount</code> action, you create two accounts: a standalone account in the AWS GovCloud (US) Region and an associated account in the commercial Region for billing and support purposes. The account in the commercial Region is automatically a member of the organization whose credentials made the request. Both accounts are associated with the same email address.</p> <p>A role is created in the new account in the commercial Region that allows the management account in the organization in the commercial Region to assume it. An AWS GovCloud (US) account is then created and associated with the commercial account that you just created. A role is also created in the new AWS GovCloud (US) account that can be assumed by the AWS GovCloud (US) account that is associated with the management account of the commercial organization. For more information and to view a diagram that explains how account access works, see <a href="http://docs.aws.amazon.com/govcloud-us/latest/UserGuide/govcloud-organizations.html">AWS Organizations</a> in the <i>AWS GovCloud User Guide.</i> </p> <p>For more information about creating accounts, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_create.html">Creating an AWS Account in Your Organization</a> in the <i>AWS Organizations User Guide.</i> </p> <important> <ul> <li> <p>When you create an account in an organization using the AWS Organizations console, API, or CLI commands, the information required for the account to operate as a standalone account is <i>not</i> automatically collected. This includes a payment method and signing the end user license agreement (EULA). If you must remove an account from your organization later, you can do so only after you provide the missing information. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info"> To leave an organization as a member account</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>If you get an exception that indicates that you exceeded your account limits for the organization, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </li> <li> <p>If you get an exception that indicates that the operation failed because your organization is still initializing, wait one hour and then try again. If the error persists, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </li> <li> <p>Using <code>CreateGovCloudAccount</code> to create multiple temporary accounts isn&#39;t recommended. You can only close an account from the AWS Billing and Cost Management console, and you must be signed in as the root user. For information on the requirements and process for closing an account, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_close.html">Closing an AWS Account</a> in the <i>AWS Organizations User Guide</i>.</p> </li> </ul> </important> <note> <p>When you create a member account with this operation, you can choose whether to create the account with the <b>IAM User and Role Access to Billing Information</b> switch enabled. If you enable it, IAM users and roles that have appropriate permissions can view billing information for the account. If you disable it, only the account root user can access billing information. For information about how to disable this switch for an account, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/grantaccess.html">Granting Access to Your Billing Information and Tools</a>.</p> </note></p>
    async fn create_gov_cloud_account(
        &self,
        input: CreateGovCloudAccountRequest,
    ) -> Result<CreateGovCloudAccountResponse, RusotoError<CreateGovCloudAccountError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.CreateGovCloudAccount",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateGovCloudAccountError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateGovCloudAccountResponse, _>()
    }

    /// <p>Creates an AWS organization. The account whose user is calling the <code>CreateOrganization</code> operation automatically becomes the <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_getting-started_concepts.html#account">management account</a> of the new organization.</p> <p>This operation must be called using credentials from the account that is to become the new organization's management account. The principal must also have the relevant IAM permissions.</p> <p>By default (or if you set the <code>FeatureSet</code> parameter to <code>ALL</code>), the new organization is created with all features enabled and service control policies automatically enabled in the root. If you instead choose to create the organization supporting only the consolidated billing features by setting the <code>FeatureSet</code> parameter to <code>CONSOLIDATED_BILLING"</code>, no policy types are enabled by default, and you can't use organization policies</p>
    async fn create_organization(
        &self,
        input: CreateOrganizationRequest,
    ) -> Result<CreateOrganizationResponse, RusotoError<CreateOrganizationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.CreateOrganization",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateOrganizationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateOrganizationResponse, _>()
    }

    /// <p>Creates an organizational unit (OU) within a root or parent OU. An OU is a container for accounts that enables you to organize your accounts to apply policies according to your business requirements. The number of levels deep that you can nest OUs is dependent upon the policy types enabled for that root. For service control policies, the limit is five.</p> <p>For more information about OUs, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_ous.html">Managing Organizational Units</a> in the <i>AWS Organizations User Guide.</i> </p> <p>If the request includes tags, then the requester must have the <code>organizations:TagResource</code> permission.</p> <p>This operation can be called only from the organization's management account.</p>
    async fn create_organizational_unit(
        &self,
        input: CreateOrganizationalUnitRequest,
    ) -> Result<CreateOrganizationalUnitResponse, RusotoError<CreateOrganizationalUnitError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.CreateOrganizationalUnit",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateOrganizationalUnitError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateOrganizationalUnitResponse, _>()
    }

    /// <p>Creates a policy of a specified type that you can attach to a root, an organizational unit (OU), or an individual AWS account.</p> <p>For more information about policies and their use, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies.html">Managing Organization Policies</a>.</p> <p>If the request includes tags, then the requester must have the <code>organizations:TagResource</code> permission.</p> <p>This operation can be called only from the organization's management account.</p>
    async fn create_policy(
        &self,
        input: CreatePolicyRequest,
    ) -> Result<CreatePolicyResponse, RusotoError<CreatePolicyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.CreatePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreatePolicyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreatePolicyResponse, _>()
    }

    /// <p>Declines a handshake request. This sets the handshake state to <code>DECLINED</code> and effectively deactivates the request.</p> <p>This operation can be called only from the account that received the handshake. The originator of the handshake can use <a>CancelHandshake</a> instead. The originator can't reactivate a declined request, but can reinitiate the process with a new handshake request.</p> <p>After you decline a handshake, it continues to appear in the results of relevant APIs for only 30 days. After that, it's deleted.</p>
    async fn decline_handshake(
        &self,
        input: DeclineHandshakeRequest,
    ) -> Result<DeclineHandshakeResponse, RusotoError<DeclineHandshakeError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.DeclineHandshake");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeclineHandshakeError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeclineHandshakeResponse, _>()
    }

    /// <p>Deletes the organization. You can delete an organization only by using credentials from the management account. The organization must be empty of member accounts.</p>
    async fn delete_organization(&self) -> Result<(), RusotoError<DeleteOrganizationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.DeleteOrganization",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, DeleteOrganizationError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes an organizational unit (OU) from a root or another OU. You must first remove all accounts and child OUs from the OU that you want to delete.</p> <p>This operation can be called only from the organization's management account.</p>
    async fn delete_organizational_unit(
        &self,
        input: DeleteOrganizationalUnitRequest,
    ) -> Result<(), RusotoError<DeleteOrganizationalUnitError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.DeleteOrganizationalUnit",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteOrganizationalUnitError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deletes the specified policy from your organization. Before you perform this operation, you must first detach the policy from all organizational units (OUs), roots, and accounts.</p> <p>This operation can be called only from the organization's management account.</p>
    async fn delete_policy(
        &self,
        input: DeletePolicyRequest,
    ) -> Result<(), RusotoError<DeletePolicyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.DeletePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeletePolicyError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Removes the specified member AWS account as a delegated administrator for the specified AWS service.</p> <important> <p>Deregistering a delegated administrator can have unintended impacts on the functionality of the enabled AWS service. See the documentation for the enabled service before you deregister a delegated administrator so that you understand any potential impacts.</p> </important> <p>You can run this action only for AWS services that support this feature. For a current list of services that support it, see the column <i>Supports Delegated Administrator</i> in the table at <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrated-services-list.html">AWS Services that you can use with AWS Organizations</a> in the <i>AWS Organizations User Guide.</i> </p> <p>This operation can be called only from the organization's management account.</p>
    async fn deregister_delegated_administrator(
        &self,
        input: DeregisterDelegatedAdministratorRequest,
    ) -> Result<(), RusotoError<DeregisterDelegatedAdministratorError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.DeregisterDelegatedAdministrator",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                DeregisterDelegatedAdministratorError::from_response,
            )
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Retrieves AWS Organizations-related information about the specified account.</p> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn describe_account(
        &self,
        input: DescribeAccountRequest,
    ) -> Result<DescribeAccountResponse, RusotoError<DescribeAccountError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.DescribeAccount");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeAccountError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeAccountResponse, _>()
    }

    /// <p>Retrieves the current status of an asynchronous request to create an account.</p> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn describe_create_account_status(
        &self,
        input: DescribeCreateAccountStatusRequest,
    ) -> Result<DescribeCreateAccountStatusResponse, RusotoError<DescribeCreateAccountStatusError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.DescribeCreateAccountStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeCreateAccountStatusError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeCreateAccountStatusResponse, _>()
    }

    /// <p>Returns the contents of the effective policy for specified policy type and account. The effective policy is the aggregation of any policies of the specified type that the account inherits, plus any policy of that type that is directly attached to the account.</p> <p>This operation applies only to policy types <i>other</i> than service control policies (SCPs).</p> <p>For more information about policy inheritance, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies-inheritance.html">How Policy Inheritance Works</a> in the <i>AWS Organizations User Guide</i>.</p> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn describe_effective_policy(
        &self,
        input: DescribeEffectivePolicyRequest,
    ) -> Result<DescribeEffectivePolicyResponse, RusotoError<DescribeEffectivePolicyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.DescribeEffectivePolicy",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeEffectivePolicyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeEffectivePolicyResponse, _>()
    }

    /// <p>Retrieves information about a previously requested handshake. The handshake ID comes from the response to the original <a>InviteAccountToOrganization</a> operation that generated the handshake.</p> <p>You can access handshakes that are <code>ACCEPTED</code>, <code>DECLINED</code>, or <code>CANCELED</code> for only 30 days after they change to that state. They're then deleted and no longer accessible.</p> <p>This operation can be called from any account in the organization.</p>
    async fn describe_handshake(
        &self,
        input: DescribeHandshakeRequest,
    ) -> Result<DescribeHandshakeResponse, RusotoError<DescribeHandshakeError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.DescribeHandshake",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeHandshakeError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeHandshakeResponse, _>()
    }

    /// <p><p>Retrieves information about the organization that the user&#39;s account belongs to.</p> <p>This operation can be called from any account in the organization.</p> <note> <p>Even if a policy type is shown as available in the organization, you can disable it separately at the root level with <a>DisablePolicyType</a>. Use <a>ListRoots</a> to see the status of policy types for a specified root.</p> </note></p>
    async fn describe_organization(
        &self,
    ) -> Result<DescribeOrganizationResponse, RusotoError<DescribeOrganizationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.DescribeOrganization",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, DescribeOrganizationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeOrganizationResponse, _>()
    }

    /// <p>Retrieves information about an organizational unit (OU).</p> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn describe_organizational_unit(
        &self,
        input: DescribeOrganizationalUnitRequest,
    ) -> Result<DescribeOrganizationalUnitResponse, RusotoError<DescribeOrganizationalUnitError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.DescribeOrganizationalUnit",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeOrganizationalUnitError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeOrganizationalUnitResponse, _>()
    }

    /// <p>Retrieves information about a policy.</p> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn describe_policy(
        &self,
        input: DescribePolicyRequest,
    ) -> Result<DescribePolicyResponse, RusotoError<DescribePolicyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.DescribePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribePolicyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribePolicyResponse, _>()
    }

    /// <p>Detaches a policy from a target root, organizational unit (OU), or account.</p> <important> <p>If the policy being detached is a service control policy (SCP), the changes to permissions for AWS Identity and Access Management (IAM) users and roles in affected accounts are immediate.</p> </important> <p>Every root, OU, and account must have at least one SCP attached. If you want to replace the default <code>FullAWSAccess</code> policy with an SCP that limits the permissions that can be delegated, you must attach the replacement SCP before you can remove the default SCP. This is the authorization strategy of an "<a href="https://docs.aws.amazon.com/organizations/latest/userguide/SCP_strategies.html#orgs_policies_allowlist">allow list</a>". If you instead attach a second SCP and leave the <code>FullAWSAccess</code> SCP still attached, and specify <code>"Effect": "Deny"</code> in the second SCP to override the <code>"Effect": "Allow"</code> in the <code>FullAWSAccess</code> policy (or any other attached SCP), you're using the authorization strategy of a "<a href="https://docs.aws.amazon.com/organizations/latest/userguide/SCP_strategies.html#orgs_policies_denylist">deny list</a>".</p> <p>This operation can be called only from the organization's management account.</p>
    async fn detach_policy(
        &self,
        input: DetachPolicyRequest,
    ) -> Result<(), RusotoError<DetachPolicyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.DetachPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DetachPolicyError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Disables the integration of an AWS service (the service that is specified by <code>ServicePrincipal</code>) with AWS Organizations. When you disable integration, the specified service no longer can create a <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/using-service-linked-roles.html">service-linked role</a> in <i>new</i> accounts in your organization. This means the service can't perform operations on your behalf on any new accounts in your organization. The service can still perform operations in older accounts until the service completes its clean-up from AWS Organizations.</p> <p/> <important> <p>We recommend that you disable integration between AWS Organizations and the specified AWS service by using the console or commands that are provided by the specified service. Doing so ensures that the other service is aware that it can clean up any resources that are required only for the integration. How the service cleans up its resources in the organization's accounts depends on that service. For more information, see the documentation for the other AWS service.</p> </important> <p>After you perform the <code>DisableAWSServiceAccess</code> operation, the specified service can no longer perform operations in your organization's accounts unless the operations are explicitly permitted by the IAM policies that are attached to your roles.</p> <p>For more information about integrating other services with AWS Organizations, including the list of services that work with Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html">Integrating AWS Organizations with Other AWS Services</a> in the <i>AWS Organizations User Guide.</i> </p> <p>This operation can be called only from the organization's management account.</p>
    async fn disable_aws_service_access(
        &self,
        input: DisableAWSServiceAccessRequest,
    ) -> Result<(), RusotoError<DisableAWSServiceAccessError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.DisableAWSServiceAccess",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DisableAWSServiceAccessError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Disables an organizational policy type in a root. A policy of a certain type can be attached to entities in a root only if that type is enabled in the root. After you perform this operation, you no longer can attach policies of the specified type to that root or to any organizational unit (OU) or account in that root. You can undo this by using the <a>EnablePolicyType</a> operation.</p> <p>This is an asynchronous request that AWS performs in the background. If you disable a policy type for a root, it still appears enabled for the organization if <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">all features</a> are enabled for the organization. AWS recommends that you first use <a>ListRoots</a> to see the status of policy types for a specified root, and then use this operation.</p> <p>This operation can be called only from the organization's management account.</p> <p> To view the status of available policy types in the organization, use <a>DescribeOrganization</a>.</p>
    async fn disable_policy_type(
        &self,
        input: DisablePolicyTypeRequest,
    ) -> Result<DisablePolicyTypeResponse, RusotoError<DisablePolicyTypeError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.DisablePolicyType",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DisablePolicyTypeError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DisablePolicyTypeResponse, _>()
    }

    /// <p>Enables the integration of an AWS service (the service that is specified by <code>ServicePrincipal</code>) with AWS Organizations. When you enable integration, you allow the specified service to create a <a href="http://docs.aws.amazon.com/IAM/latest/UserGuide/using-service-linked-roles.html">service-linked role</a> in all the accounts in your organization. This allows the service to perform operations on your behalf in your organization and its accounts.</p> <important> <p>We recommend that you enable integration between AWS Organizations and the specified AWS service by using the console or commands that are provided by the specified service. Doing so ensures that the service is aware that it can create the resources that are required for the integration. How the service creates those resources in the organization's accounts depends on that service. For more information, see the documentation for the other AWS service.</p> </important> <p>For more information about enabling services to integrate with AWS Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html">Integrating AWS Organizations with Other AWS Services</a> in the <i>AWS Organizations User Guide.</i> </p> <p>This operation can be called only from the organization's management account and only if the organization has <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">enabled all features</a>.</p>
    async fn enable_aws_service_access(
        &self,
        input: EnableAWSServiceAccessRequest,
    ) -> Result<(), RusotoError<EnableAWSServiceAccessError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.EnableAWSServiceAccess",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, EnableAWSServiceAccessError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Enables all features in an organization. This enables the use of organization policies that can restrict the services and actions that can be called in each account. Until you enable all features, you have access only to consolidated billing, and you can't use any of the advanced account administration features that AWS Organizations supports. For more information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_org_support-all-features.html">Enabling All Features in Your Organization</a> in the <i>AWS Organizations User Guide.</i> </p> <important> <p>This operation is required only for organizations that were created explicitly with only the consolidated billing features enabled. Calling this operation sends a handshake to every invited account in the organization. The feature set change can be finalized and the additional features enabled only after all administrators in the invited accounts approve the change by accepting the handshake.</p> </important> <p>After you enable all features, you can separately enable or disable individual policy types in a root using <a>EnablePolicyType</a> and <a>DisablePolicyType</a>. To see the status of policy types in a root, use <a>ListRoots</a>.</p> <p>After all invited member accounts accept the handshake, you finalize the feature set change by accepting the handshake that contains <code>"Action": "ENABLE_ALL_FEATURES"</code>. This completes the change.</p> <p>After you enable all features in your organization, the management account in the organization can apply policies on all member accounts. These policies can restrict what users and even administrators in those accounts can do. The management account can apply policies that prevent accounts from leaving the organization. Ensure that your account administrators are aware of this.</p> <p>This operation can be called only from the organization's management account.</p>
    async fn enable_all_features(
        &self,
    ) -> Result<EnableAllFeaturesResponse, RusotoError<EnableAllFeaturesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.EnableAllFeatures",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, EnableAllFeaturesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<EnableAllFeaturesResponse, _>()
    }

    /// <p>Enables a policy type in a root. After you enable a policy type in a root, you can attach policies of that type to the root, any organizational unit (OU), or account in that root. You can undo this by using the <a>DisablePolicyType</a> operation.</p> <p>This is an asynchronous request that AWS performs in the background. AWS recommends that you first use <a>ListRoots</a> to see the status of policy types for a specified root, and then use this operation.</p> <p>This operation can be called only from the organization's management account.</p> <p>You can enable a policy type in a root only if that policy type is available in the organization. To view the status of available policy types in the organization, use <a>DescribeOrganization</a>.</p>
    async fn enable_policy_type(
        &self,
        input: EnablePolicyTypeRequest,
    ) -> Result<EnablePolicyTypeResponse, RusotoError<EnablePolicyTypeError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.EnablePolicyType");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, EnablePolicyTypeError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<EnablePolicyTypeResponse, _>()
    }

    /// <p>Sends an invitation to another account to join your organization as a member account. AWS Organizations sends email on your behalf to the email address that is associated with the other account's owner. The invitation is implemented as a <a>Handshake</a> whose details are in the response.</p> <important> <ul> <li> <p>You can invite AWS accounts only from the same seller as the management account. For example, if your organization's management account was created by Amazon Internet Services Pvt. Ltd (AISPL), an AWS seller in India, you can invite only other AISPL accounts to your organization. You can't combine accounts from AISPL and AWS or from any other AWS seller. For more information, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/useconsolidatedbilliing-India.html">Consolidated Billing in India</a>.</p> </li> <li> <p>If you receive an exception that indicates that you exceeded your account limits for the organization or that the operation failed because your organization is still initializing, wait one hour and then try again. If the error persists after an hour, contact <a href="https://console.aws.amazon.com/support/home#/">AWS Support</a>.</p> </li> </ul> </important> <p>If the request includes tags, then the requester must have the <code>organizations:TagResource</code> permission.</p> <p>This operation can be called only from the organization's management account.</p>
    async fn invite_account_to_organization(
        &self,
        input: InviteAccountToOrganizationRequest,
    ) -> Result<InviteAccountToOrganizationResponse, RusotoError<InviteAccountToOrganizationError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.InviteAccountToOrganization",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, InviteAccountToOrganizationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<InviteAccountToOrganizationResponse, _>()
    }

    /// <p><p>Removes a member account from its parent organization. This version of the operation is performed by the account that wants to leave. To remove a member account as a user in the management account, use <a>RemoveAccountFromOrganization</a> instead.</p> <p>This operation can be called only from a member account in the organization.</p> <important> <ul> <li> <p>The management account in an organization with all features enabled can set service control policies (SCPs) that can restrict what administrators of member accounts can do. This includes preventing them from successfully calling <code>LeaveOrganization</code> and leaving the organization.</p> </li> <li> <p>You can leave an organization as a member account only if the account is configured with the information required to operate as a standalone account. When you create an account in an organization using the AWS Organizations console, API, or CLI commands, the information required of standalone accounts is <i>not</i> automatically collected. For each account that you want to make standalone, you must perform the following steps. If any of the steps are already completed for this account, that step doesn&#39;t appear.</p> <ul> <li> <p>Choose a support plan</p> </li> <li> <p>Provide and verify the required contact information</p> </li> <li> <p>Provide a current payment method</p> </li> </ul> <p>AWS uses the payment method to charge for any billable (not free tier) AWS activity that occurs while the account isn&#39;t attached to an organization. Follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info"> To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>You can leave an organization only after you enable IAM user access to billing in your account. For more information, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/grantaccess.html#ControllingAccessWebsite-Activate">Activating Access to the Billing and Cost Management Console</a> in the <i>AWS Billing and Cost Management User Guide.</i> </p> </li> <li> <p>After the account leaves the organization, all tags that were attached to the account object in the organization are deleted. AWS accounts outside of an organization do not support tags.</p> </li> </ul> </important></p>
    async fn leave_organization(&self) -> Result<(), RusotoError<LeaveOrganizationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.LeaveOrganization",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, LeaveOrganizationError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Returns a list of the AWS services that you enabled to integrate with your organization. After a service on this list creates the resources that it requires for the integration, it can perform operations on your organization and its accounts.</p> <p>For more information about integrating other services with AWS Organizations, including the list of services that currently work with Organizations, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrate_services.html">Integrating AWS Organizations with Other AWS Services</a> in the <i>AWS Organizations User Guide.</i> </p> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn list_aws_service_access_for_organization(
        &self,
        input: ListAWSServiceAccessForOrganizationRequest,
    ) -> Result<
        ListAWSServiceAccessForOrganizationResponse,
        RusotoError<ListAWSServiceAccessForOrganizationError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.ListAWSServiceAccessForOrganization",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                ListAWSServiceAccessForOrganizationError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListAWSServiceAccessForOrganizationResponse, _>()
    }

    /// <p>Lists all the accounts in the organization. To request only the accounts in a specified root or organizational unit (OU), use the <a>ListAccountsForParent</a> operation instead.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn list_accounts(
        &self,
        input: ListAccountsRequest,
    ) -> Result<ListAccountsResponse, RusotoError<ListAccountsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.ListAccounts");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListAccountsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListAccountsResponse, _>()
    }

    /// <p>Lists the accounts in an organization that are contained by the specified target root or organizational unit (OU). If you specify the root, you get a list of all the accounts that aren't in any OU. If you specify an OU, you get a list of all the accounts in only that OU and not in any child OUs. To get a list of all accounts in the organization, use the <a>ListAccounts</a> operation.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn list_accounts_for_parent(
        &self,
        input: ListAccountsForParentRequest,
    ) -> Result<ListAccountsForParentResponse, RusotoError<ListAccountsForParentError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.ListAccountsForParent",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListAccountsForParentError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListAccountsForParentResponse, _>()
    }

    /// <p>Lists all of the organizational units (OUs) or accounts that are contained in the specified parent OU or root. This operation, along with <a>ListParents</a> enables you to traverse the tree structure that makes up this root.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn list_children(
        &self,
        input: ListChildrenRequest,
    ) -> Result<ListChildrenResponse, RusotoError<ListChildrenError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.ListChildren");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListChildrenError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListChildrenResponse, _>()
    }

    /// <p>Lists the account creation requests that match the specified status that is currently being tracked for the organization.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn list_create_account_status(
        &self,
        input: ListCreateAccountStatusRequest,
    ) -> Result<ListCreateAccountStatusResponse, RusotoError<ListCreateAccountStatusError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.ListCreateAccountStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListCreateAccountStatusError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListCreateAccountStatusResponse, _>()
    }

    /// <p>Lists the AWS accounts that are designated as delegated administrators in this organization.</p> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn list_delegated_administrators(
        &self,
        input: ListDelegatedAdministratorsRequest,
    ) -> Result<ListDelegatedAdministratorsResponse, RusotoError<ListDelegatedAdministratorsError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.ListDelegatedAdministrators",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListDelegatedAdministratorsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListDelegatedAdministratorsResponse, _>()
    }

    /// <p>List the AWS services for which the specified account is a delegated administrator.</p> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn list_delegated_services_for_account(
        &self,
        input: ListDelegatedServicesForAccountRequest,
    ) -> Result<
        ListDelegatedServicesForAccountResponse,
        RusotoError<ListDelegatedServicesForAccountError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.ListDelegatedServicesForAccount",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListDelegatedServicesForAccountError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListDelegatedServicesForAccountResponse, _>()
    }

    /// <p>Lists the current handshakes that are associated with the account of the requesting user.</p> <p>Handshakes that are <code>ACCEPTED</code>, <code>DECLINED</code>, or <code>CANCELED</code> appear in the results of this API for only 30 days after changing to that state. After that, they're deleted and no longer accessible.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called from any account in the organization.</p>
    async fn list_handshakes_for_account(
        &self,
        input: ListHandshakesForAccountRequest,
    ) -> Result<ListHandshakesForAccountResponse, RusotoError<ListHandshakesForAccountError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.ListHandshakesForAccount",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListHandshakesForAccountError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListHandshakesForAccountResponse, _>()
    }

    /// <p>Lists the handshakes that are associated with the organization that the requesting user is part of. The <code>ListHandshakesForOrganization</code> operation returns a list of handshake structures. Each structure contains details and status about a handshake.</p> <p>Handshakes that are <code>ACCEPTED</code>, <code>DECLINED</code>, or <code>CANCELED</code> appear in the results of this API for only 30 days after changing to that state. After that, they're deleted and no longer accessible.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn list_handshakes_for_organization(
        &self,
        input: ListHandshakesForOrganizationRequest,
    ) -> Result<
        ListHandshakesForOrganizationResponse,
        RusotoError<ListHandshakesForOrganizationError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.ListHandshakesForOrganization",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListHandshakesForOrganizationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListHandshakesForOrganizationResponse, _>()
    }

    /// <p>Lists the organizational units (OUs) in a parent organizational unit or root.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn list_organizational_units_for_parent(
        &self,
        input: ListOrganizationalUnitsForParentRequest,
    ) -> Result<
        ListOrganizationalUnitsForParentResponse,
        RusotoError<ListOrganizationalUnitsForParentError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.ListOrganizationalUnitsForParent",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                ListOrganizationalUnitsForParentError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListOrganizationalUnitsForParentResponse, _>()
    }

    /// <p><p>Lists the root or organizational units (OUs) that serve as the immediate parent of the specified child OU or account. This operation, along with <a>ListChildren</a> enables you to traverse the tree structure that makes up this root.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization&#39;s management account or by a member account that is a delegated administrator for an AWS service.</p> <note> <p>In the current release, a child can have only a single parent.</p> </note></p>
    async fn list_parents(
        &self,
        input: ListParentsRequest,
    ) -> Result<ListParentsResponse, RusotoError<ListParentsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.ListParents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListParentsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListParentsResponse, _>()
    }

    /// <p>Retrieves the list of all policies in an organization of a specified type.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn list_policies(
        &self,
        input: ListPoliciesRequest,
    ) -> Result<ListPoliciesResponse, RusotoError<ListPoliciesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.ListPolicies");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListPoliciesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListPoliciesResponse, _>()
    }

    /// <p>Lists the policies that are directly attached to the specified target root, organizational unit (OU), or account. You must specify the policy type that you want included in the returned list.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn list_policies_for_target(
        &self,
        input: ListPoliciesForTargetRequest,
    ) -> Result<ListPoliciesForTargetResponse, RusotoError<ListPoliciesForTargetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.ListPoliciesForTarget",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListPoliciesForTargetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListPoliciesForTargetResponse, _>()
    }

    /// <p><p>Lists the roots that are defined in the current organization.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization&#39;s management account or by a member account that is a delegated administrator for an AWS service.</p> <note> <p>Policy types can be enabled and disabled in roots. This is distinct from whether they&#39;re available in the organization. When you enable all features, you make policy types available for use in that organization. Individual policy types can then be enabled and disabled in a root. To see the availability of a policy type in an organization, use <a>DescribeOrganization</a>.</p> </note></p>
    async fn list_roots(
        &self,
        input: ListRootsRequest,
    ) -> Result<ListRootsResponse, RusotoError<ListRootsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.ListRoots");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListRootsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListRootsResponse, _>()
    }

    /// <p>Lists tags that are attached to the specified resource.</p> <p>You can attach tags to the following resources in AWS Organizations.</p> <ul> <li> <p>AWS account</p> </li> <li> <p>Organization root</p> </li> <li> <p>Organizational unit (OU)</p> </li> <li> <p>Policy (any type)</p> </li> </ul> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.ListTagsForResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTagsForResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTagsForResourceResponse, _>()
    }

    /// <p>Lists all the roots, organizational units (OUs), and accounts that the specified policy is attached to.</p> <note> <p>Always check the <code>NextToken</code> response parameter for a <code>null</code> value when calling a <code>List*</code> operation. These operations can occasionally return an empty set of results even when there are more results available. The <code>NextToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display.</p> </note> <p>This operation can be called only from the organization's management account or by a member account that is a delegated administrator for an AWS service.</p>
    async fn list_targets_for_policy(
        &self,
        input: ListTargetsForPolicyRequest,
    ) -> Result<ListTargetsForPolicyResponse, RusotoError<ListTargetsForPolicyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.ListTargetsForPolicy",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTargetsForPolicyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListTargetsForPolicyResponse, _>()
    }

    /// <p>Moves an account from its current source parent root or organizational unit (OU) to the specified destination parent root or OU.</p> <p>This operation can be called only from the organization's management account.</p>
    async fn move_account(
        &self,
        input: MoveAccountRequest,
    ) -> Result<(), RusotoError<MoveAccountError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.MoveAccount");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, MoveAccountError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Enables the specified member account to administer the Organizations features of the specified AWS service. It grants read-only access to AWS Organizations service data. The account still requires IAM permissions to access and administer the AWS service.</p> <p>You can run this action only for AWS services that support this feature. For a current list of services that support it, see the column <i>Supports Delegated Administrator</i> in the table at <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_integrated-services-list.html">AWS Services that you can use with AWS Organizations</a> in the <i>AWS Organizations User Guide.</i> </p> <p>This operation can be called only from the organization's management account.</p>
    async fn register_delegated_administrator(
        &self,
        input: RegisterDelegatedAdministratorRequest,
    ) -> Result<(), RusotoError<RegisterDelegatedAdministratorError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.RegisterDelegatedAdministrator",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RegisterDelegatedAdministratorError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p><p>Removes the specified account from the organization.</p> <p>The removed account becomes a standalone account that isn&#39;t a member of any organization. It&#39;s no longer subject to any policies and is responsible for its own bill payments. The organization&#39;s management account is no longer charged for any expenses accrued by the member account after it&#39;s removed from the organization.</p> <p>This operation can be called only from the organization&#39;s management account. Member accounts can remove themselves with <a>LeaveOrganization</a> instead.</p> <important> <ul> <li> <p>You can remove an account from your organization only if the account is configured with the information required to operate as a standalone account. When you create an account in an organization using the AWS Organizations console, API, or CLI commands, the information required of standalone accounts is <i>not</i> automatically collected. For an account that you want to make standalone, you must choose a support plan, provide and verify the required contact information, and provide a current payment method. AWS uses the payment method to charge for any billable (not free tier) AWS activity that occurs while the account isn&#39;t attached to an organization. To remove an account that doesn&#39;t yet have this information, you must sign in as the member account and follow the steps at <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_accounts_remove.html#leave-without-all-info"> To leave an organization when all required account information has not yet been provided</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>After the account leaves the organization, all tags that were attached to the account object in the organization are deleted. AWS accounts outside of an organization do not support tags.</p> </li> </ul> </important></p>
    async fn remove_account_from_organization(
        &self,
        input: RemoveAccountFromOrganizationRequest,
    ) -> Result<(), RusotoError<RemoveAccountFromOrganizationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.RemoveAccountFromOrganization",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RemoveAccountFromOrganizationError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Adds one or more tags to the specified resource.</p> <p>Currently, you can attach tags to the following resources in AWS Organizations.</p> <ul> <li> <p>AWS account</p> </li> <li> <p>Organization root</p> </li> <li> <p>Organizational unit (OU)</p> </li> <li> <p>Policy (any type)</p> </li> </ul> <p>This operation can be called only from the organization's management account.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TagResourceError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Removes any tags with the specified keys from the specified resource.</p> <p>You can attach tags to the following resources in AWS Organizations.</p> <ul> <li> <p>AWS account</p> </li> <li> <p>Organization root</p> </li> <li> <p>Organizational unit (OU)</p> </li> <li> <p>Policy (any type)</p> </li> </ul> <p>This operation can be called only from the organization's management account.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UntagResourceError::from_response)
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Renames the specified organizational unit (OU). The ID and ARN don't change. The child OUs and accounts remain in place, and any attached policies of the OU remain attached.</p> <p>This operation can be called only from the organization's management account.</p>
    async fn update_organizational_unit(
        &self,
        input: UpdateOrganizationalUnitRequest,
    ) -> Result<UpdateOrganizationalUnitResponse, RusotoError<UpdateOrganizationalUnitError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSOrganizationsV20161128.UpdateOrganizationalUnit",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateOrganizationalUnitError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateOrganizationalUnitResponse, _>()
    }

    /// <p>Updates an existing policy with a new name, description, or content. If you don't supply any parameter, that value remains unchanged. You can't change a policy's type.</p> <p>This operation can be called only from the organization's management account.</p>
    async fn update_policy(
        &self,
        input: UpdatePolicyRequest,
    ) -> Result<UpdatePolicyResponse, RusotoError<UpdatePolicyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSOrganizationsV20161128.UpdatePolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdatePolicyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdatePolicyResponse, _>()
    }
}
