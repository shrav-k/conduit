# Secrets in AWS, Kubernetes and the systems around them

> Research; it does not bind. Written on 2026-11-06 because the owner, ruling on point 1 of
> [ruling 39](../proposals/ruling-39/README.md), asked for more research before deciding the secret
> type.

The owner's words were: "I feel we need more research into how secrets are handled by cloud
providers like AWS and in kubernetes before making a determination". This packet is that research.
It recommends nothing. What it means for Conduit is argued in the revised
[secrets proposal](../proposals/secrets/README.md), which waits for the owner. AWS and Kubernetes
are what the owner asked about, so they get the most depth. The other sections cover the systems
around them: the Kubernetes ecosystem, the other cloud and secret stores, and the configuration
tools and agent tooling a developer uses beside them.

**Method.** Six researchers read the primary documentation of each system. A fact-checker opened
every cited page again and gave each claim a verdict: confirmed, wrong or unverifiable. This packet
states as fact only what the fact-checker confirmed, in the form in which it was confirmed, and
quotes the page wherever the wording matters. Five pages were opened again by the editor on
2026-11-06, to settle claims that the check had found wrong or only partly supported; each use is
marked "re-opened". The claims that were not established are listed under
[Not established](#not-established). Nothing was tested: no cloud account, cluster or credential
was used. Each section describes its system in that system's own terms; it does not read the system
for Conduit.

**Second round.** A review of the revised proposal, on the same day, found five topics that the
packet left thin. Each was researched again from primary pages and checked the same way, and only
the claims the check confirmed are stated here, with the limits the check put on them. They are
Parameter Store, added to the [first section](#aws-secrets-manager-and-parameter-store), and four
new sections: [who may give a workload a read](#who-may-give-a-workload-a-read),
[AWS and Kubernetes on a developer's machine](#aws-and-kubernetes-on-a-developers-machine),
[ciphertext kept beside configuration](#ciphertext-kept-beside-configuration), and
[delivering a credential to an existing MCP
server](#delivering-a-credential-to-an-existing-mcp-server). The claims of this round that the
check found wrong or could not confirm are listed under [Not established](#not-established), with
what the page does say.

**Third round.** A review of the second revision of the proposal, on the same day, found five more
topics thin. They were researched and checked as before, and only the claims that the check
confirmed are stated, with the limits it put on them. They are five new sections:
[a list of readers on the secret](#a-list-of-readers-on-the-secret),
[what Kubernetes gives every container](#what-kubernetes-gives-every-container),
[an Agent's own model credential](#an-agents-own-model-credential),
[processes of the same user on Linux and macOS](#processes-of-the-same-user-on-linux-and-macos),
and [delivering to an EC2 instance](#delivering-to-an-ec2-instance). The editor opened four pages
again in this round: two that the review had re-opened to support facts the proposal used, and two
manual pages to settle a claim that the check found wrong; each use is marked "re-opened". One
researcher also ran commands on one macOS machine. What was seen there is not a source, and it is
reported only under [Not established](#not-established).

## Contents

- [AWS Secrets Manager and Parameter Store](#aws-secrets-manager-and-parameter-store)
- [How AWS delivers to workloads](#how-aws-delivers-to-workloads)
- [The Kubernetes Secret](#the-kubernetes-secret)
- [The Kubernetes ecosystem](#the-kubernetes-ecosystem)
- [GCP, Azure and HashiCorp Vault](#gcp-azure-and-hashicorp-vault)
- [Configuration tools, MCP and agent tooling](#configuration-tools-mcp-and-agent-tooling)
- [Who may give a workload a read](#who-may-give-a-workload-a-read)
- [AWS and Kubernetes on a developer's machine](#aws-and-kubernetes-on-a-developers-machine)
- [Ciphertext kept beside configuration](#ciphertext-kept-beside-configuration)
- [Delivering a credential to an existing MCP
  server](#delivering-a-credential-to-an-existing-mcp-server)
- [A list of readers on the secret](#a-list-of-readers-on-the-secret)
- [What Kubernetes gives every container](#what-kubernetes-gives-every-container)
- [An Agent's own model credential](#an-agents-own-model-credential)
- [Processes of the same user on Linux and
  macOS](#processes-of-the-same-user-on-linux-and-macos)
- [Delivering to an EC2 instance](#delivering-to-an-ec2-instance)
- [Patterns across the systems](#patterns-across-the-systems)
- [Not established](#not-established)
- [Sources](#sources)

Each system's section answers the same seven questions, in this order: what a secret is; how it is
referenced; who may read it; how it reaches code; rotation; auditing; and what the system
guarantees and leaves to the user. The sections of the second and third rounds each follow one
question across several systems, and do not keep that order.

## AWS Secrets Manager and Parameter Store

**What a secret is.** In Secrets Manager a secret is an encrypted value with metadata that is kept
in the clear. The value is protected by envelope encryption: "Whenever the secret value in a secret
changes, Secrets Manager requests a new data key from AWS KMS to protect it." The name, the
description, the rotation settings, the ARN of the KMS key and the tags are not encrypted
([encryption][sm-enc]). A secret has versions, but "Secrets Manager doesn't store a linear history
of secrets with versions. Instead, it keeps track of three specific versions by labeling them":
AWSCURRENT, AWSPREVIOUS and AWSPENDING. Users may attach labels of their own, and versions that
have no label are kept as deprecated until there are more than 100
([what's in a secret][sm-whats]). For several values that belong together, such as a host, a port,
a username and a password, AWS recommends "a JSON text string with key-value pairs"
([what's in a secret][sm-whats]).

Parameter Store has three types of parameter: String, StringList and SecureString. "Only the value
of a SecureString parameter is encrypted. Parameter names, descriptions, and other properties
aren't encrypted." Secrecy is the author's choice of type: "Don't store sensitive data in a String
or StringList parameter." For credentials the same page now points elsewhere: "For secrets such as
database credentials, API keys, or tokens, we recommend AWS Secrets Manager"
([what is a parameter][ps-param]).

**How it is referenced.** By name or by ARN. The ARN ends in six random characters, so a secret
that is deleted and made again under the same name is a new resource: "Users with access to the old
secret don't automatically get access to the new secret because the ARNs are different"
([what's in a secret][sm-whats]). A policy that matches the name with the wildcard `??????` does
carry over: "the user automatically receives permission to the new secret, even though the 6
characters changed" ([IAM policies][sm-iam]). `GetSecretValue` takes "The ARN or name of the secret
to retrieve. To retrieve a secret from another account, you must use an ARN", and AWS recommends a
complete ARN over a partial one ([GetSecretValue][sm-get]). A read that names no version returns
AWSCURRENT: "A secret always has a version labeled AWSCURRENT, and Secrets Manager returns that
version by default" ([what's in a secret][sm-whats]).

The references of the services around the store carry the same parts. CloudFormation's is
`{{resolve:secretsmanager:secret-id:secret-string:json-key:version-stage:version-id}}`. Of the
`secret-string` segment the page says: "The only supported value is `SecretString`. The default is
`SecretString`." `json-key` picks one key of a JSON value; and "If you don't specify either
`version-stage` or `version-id`, then the default is the `AWSCURRENT` version"
([CloudFormation reference][cfn-sm]; re-opened, and the quote on `SecretString` re-opened again in
the third round). AWS's reference form for AI agents has
the same parts, and its field type "Must be SecretString" ([AI coding agents][sm-agents]). In ECS
"you can specify the full contents of a secret, a specific JSON key within a secret"
([ECS secrets][ecs-secrets]). Parameter Store can stand in front of Secrets Manager under the
prefix `/aws/reference/secretsmanager`: it "functions as a pass-through service for references to
Secrets Manager secrets. Parameter Store doesn't retain data or metadata about secrets. The
reference is stateless", and it "honors AWS Identity and Access Management (IAM) policies attached
to Secrets Manager secrets" ([Parameter Store references][ps-ref]).

CloudFormation keeps only the reference. For a SecureString, "CloudFormation never stores the
actual secure string value. Instead, it only stores the literal dynamic reference, which contains
the plaintext parameter name of the secure string", and its APIs return only that reference. It
also limits where such a reference may stand: "Secure strings can only be used for resource
properties that support the ssm-secure dynamic reference pattern"
([CloudFormation SecureString][cfn-ssm]).

**Who may read it.** Reading can need two grants: `secretsmanager:GetSecretValue`, and, where the
secret is encrypted with a customer-managed key, "kms:Decrypt permissions for that key"
([GetSecretValue][sm-get]). AWS recommends "two IAM roles": one "that manages the secrets", with
which "You'll create and rotate the secret", and one "that can use the secret at runtime"; its
tutorial gives the second "only the permission to retrieve one secret value"
([hardcoded credentials][sm-hardcoded]). It recommends against administrator rights for end users,
because "the permission required to enable rotation (IAMFullAccess) grants significant permissions
that are not appropriate for end users" ([authentication and access][sm-access]). A rotation
function is "a privileged deputy that has the authorization to access and modify customer
credentials" ([rotation functions][sm-rot-lambda]).

AWS now addresses AI coding agents directly: "When AI coding agents have shell or AWS API access,
they can call `get-secret-value` and receive plaintext secrets in their context window. This
creates multiple risks: secret values can leak into conversation history, logs, or downstream tool
calls." Its answer is an optional plugin with two parts. A wrapper, `asm-exec`, "resolves
`{{resolve:...}}` references in command arguments, then runs the target command"; "The plaintext
value exists only in the child process and never enters the agent's context window", and more
exactly "Secret values exist only in the `asm-exec` process memory and the child process
arguments". A `PreToolUse` hook, once the plugin is enabled, blocks `get-secret-value` and
`batch-get-secret-value` through the CLI, SDK calls in scripts, the local provider's path, and
"`GetSecretValue` operations via MCP tools or structured AWS API calls". AWS limits the claim:
"This is a best-effort defense, not a security boundary. It prevents the most common leakage path
but cannot stop all evasion vectors. Combine with IAM least-privilege, CloudTrail monitoring, and
VPC endpoint policies." ([AI coding agents][sm-agents]; re-opened.) Elsewhere AWS warns that "other
utilities that work in the background might have access to your command parameters", and shows a
secret passed in a file instead ([CLI exposure][sm-cli]).

**How it reaches code.** In three ways, set out with their costs in
[the next section](#how-aws-delivers-to-workloads): injected by the platform when a task or a stack
is made; fetched by the workload under its own identity; or served by a local caching provider. AWS
recommends caching on the client: "We recommend that you cache your secret values by using
client-side caching. Caching secrets improves speed and reduces your costs"
([GetSecretValue][sm-get]). Its local provider, the Workload Credentials Provider, has a "default
refresh frequency (TTL)" of 300 seconds, settable from 0 to 3600, where 0 means no caching. It
"doesn't include cache invalidation. If a secret rotates before the cache entry expires, the AWS
Workload Credentials Provider might return a stale secret value"; a caller can pass
`refreshNow=true`, which "Bypasses the cache entirely" ([Workload Credentials Provider][sm-wcp]).
Every caller sends a token header, `X-Aws-Parameters-Secrets-Token`, read from a file that one
group may read or from an environment variable, and the provider "denies requests that don't have
this header or that have an invalid SSRF token". AWS names the trust domain plainly: "any user with
access to the compute environment and SSRF token can access the secret from the AWS Workload
Credentials Provider cache"; "the domain of trust is where the provider endpoint and SSRF token are
accessible, which is usually the entire host"; and "Secret values aren't encrypted in the cache"
([Workload Credentials Provider][sm-wcp]).

**Rotation.** Rotation is a staged protocol run by a function. `createSecret` stores a new value
with the label AWSPENDING; `setSecret` changes the credential in the database or service;
`testSecret` tests the pending version; and `finishSecret` "moves the label AWSCURRENT from the
previous secret version to this version", while AWSPREVIOUS goes to the old one "so that you retain
the last known good version of the secret" ([rotation functions][sm-rot-lambda]). With a single
user, "there is a short period of time between when the password in the database changes and when
the secret is updated. During this time, there is a low risk of the database denying calls that use
the rotated credentials. You can mitigate this risk with an appropriate retry strategy." With
alternating users, "If an application retrieves the secret during rotation, the application still
gets a valid set of credentials" ([rotation strategies][sm-rot-strategy]; re-opened). The store is
eventually consistent: "Any change that you make in Secrets Manager (or other AWS services) takes
time to become visible from all possible endpoints" ([troubleshooting][sm-trouble]).

CloudFormation recommends references with no version: "This ensures your credentials are regularly
updated without requiring template changes". It keeps versioned references for "specific scenarios
like testing or rollback situations". But "Updating only the secret value in Secrets Manager
doesn't automatically cause CloudFormation to retrieve the new value": it reads the value only when
the resource that holds the reference is made or changed ([CloudFormation reference][cfn-sm];
re-opened). Deleting a secret makes it unreadable at once: Secrets Manager "immediately makes the
secrets inaccessible and scheduled for deletion after a recovery window of a minimum of seven
days", and AWS suggests an alarm on any attempt to read it in that window
([deleting a secret][sm-delete]). `GetSecretValue` is limited to 10,000 requests per second in each
Region, a quota that cannot be raised, and throttled calls should be retried with backoff
([quotas][sm-limits]).

**Auditing.** Every read is logged, and the value never is. `SecretString` and `SecretBinary` are
marked Sensitive: "the service does not include it in AWS CloudTrail log entries. If you create
your own log entries, you must also avoid logging the information in this field". The read itself
is logged: "Secrets Manager generates a CloudTrail log entry when you call this action"
([GetSecretValue][sm-get]). CloudTrail also receives the events of the store's own life, apart from
API calls: rotation started, succeeded, failed or abandoned; version deletion; and replication
([CloudTrail entries][sm-trail]). Reads served by the local provider are not in CloudTrail:
"Requests to get secrets from the AWS Workload Credentials Provider do not appear in those logs";
they are in the provider's local log, and only the provider's own calls to Secrets Manager reach
CloudTrail ([Workload Credentials Provider][sm-wcp]).

**What AWS guarantees and leaves to the user.** Under the shared responsibility model, "You are
responsible for maintaining control over your content that is hosted on this infrastructure. This
content includes the security configuration and management tasks for the AWS services that you use"
([data protection][sm-data]). What happens after a read is the user's. Of CloudFormation: "neither
Secrets Manager nor CloudFormation logs should persist any resolved secret value. However, the
secret value may show up in the service whose resource it's being used in. Review your usage to
avoid leaking secret data." ([CloudFormation reference][cfn-sm]; re-opened.) The agent hook is
"best-effort" ([AI coding agents][sm-agents]). For Lambda, "we recommend that you use AWS Secrets
Manager instead of environment variables to store database credentials", and users without
`kms:Decrypt` on a customer-managed key "can still manage functions, but they can't view
environment variables" ([Lambda encryption][lambda-envenc]).

**Parameter Store in more detail** (second round). A read names a parameter and may add one
selector after a colon: "To query by parameter label, use "Name": "name:label". To query by
parameter version, use "Name": "name:version"." The response carries the `Version` served and the
`Selector` used beside the `Value` ([GetParameter][ps-get]). "If you don't specify a version
number, the system automatically uses the latest version. If you specify the number for a version
that doesn't exist, the system returns an error rather than falling back to the latest or default
version of the parameter." History is capped: "After you have created 100 versions of a parameter,
each time you create a new version, the oldest version of the parameter is removed from history",
except that where a label is on the oldest version, that version is kept and "the request to
create a new parameter version fails" ([versions][ps-versions]). The pass-through to Secrets
Manager uses the same form for a Secrets Manager version ID or stage, as in
`/aws/reference/secretsmanager/s1-secret:AWSCURRENT`: "Parameter Store honors Secrets Manager
version stages", and such a parameter "can't use the Parameter Store versioning or history
features" ([Parameter Store references][ps-ref]).

The services that deliver a parameter default to the latest version. ASCP's `objectVersion` is
"generally not recommended since updates to the secret require updating this field", and of
`objectVersionLabel` it says "Most applications should not use this field since the most recent
version of the secret is used by default" ([ASCP repository][ascp-repo]). CloudFormation's
`ssm-secure` reference takes an optional version number and "doesn't support using Systems Manager
parameter labels"; without a version it "uses the latest version of the parameter whenever you
create or update the stack", and "If CloudFormation needs to roll back a stack update, and the
previously specified version of a secure string parameter is no longer available, the rollback
operation will fail" ([CloudFormation SecureString][cfn-ssm]).

Reading is gated in layers. "Four Parameter Store operations can return a parameter value, and each
one is authorized against its own IAM action": `GetParameter`, `GetParameters`,
`GetParameterHistory` and `GetParametersByPath`. "Denying ssm:GetParameter has no effect on calls
to" the other three, and `GetParametersByPath` "authorizes against the path in the request rather
than against the parameters it returns" ([retrieval authorization][ps-retrieval]). So "A deny on the
parameter's own ARN doesn't stop the parameter from being returned by a call against a path above
it"; to protect one parameter, the path action is denied on every path above it. AWS advises:
"Prefer granting narrow access over denying broad access", and "Separate sensitive parameters into
their own path" ([restricting access][ps-restrict]). For a SecureString, decryption is a further
gate. "When you get a SecureString parameter from Parameter Store, its value is encrypted"; with
`WithDecryption`, Parameter Store "calls the AWS KMS Decrypt operation on your behalf" and returns
the plaintext ([KMS and Parameter Store][kms-ps]). "Any principal that retrieves the decrypted value
must have kms:Decrypt permission on that key, whichever Parameter Store operation they call", so
"a single AWS KMS policy statement applies to all retrieval paths at once". It hides no metadata:
restricting decryption "doesn't hide the parameter's name, description, or other metadata, which
aren't encrypted, and it doesn't prevent the parameter from being listed"
([restricting decryption][ps-decrypt]). Parameter Store binds each ciphertext to the parameter's
ARN as KMS encryption context, under the key `PARAMETER_ARN`, so that a policy can "permit a user to
decrypt only one particular parameter value"; "The encryption context appears in plaintext in logs,
such as AWS CloudTrail logs" ([KMS and Parameter Store][kms-ps]).

Parameter Store does not rotate a value. Its parameter policies, for the advanced tier only, are
`Expiration`, which "deletes the parameter", and two that send an event to EventBridge: one before
the expiry, and one when a parameter "has not been modified for a specified period of time".
"Parameter Store enforces parameter policies by using asynchronous, periodic scans", and "If you
manage credentials that require automatic rotation ... we recommend using AWS Secrets Manager"
([parameter policies][ps-policies]).

## How AWS delivers to workloads

**What a secret is, beside an identity.** For AWS's own APIs, AWS removes the stored secret. Its
best practices say "Require workloads to use temporary credentials with IAM roles to access AWS":
"AWS delivers the temporary credentials of an IAM role to that compute resource", and "there is no
need to distribute long lived credentials for an IAM user to your workloads"
([IAM best practices][iam-bp]). Temporary credentials "are not stored with the user but are
generated dynamically and provided to the user when requested", and "After the credentials expire,
AWS no longer recognizes them" ([temporary credentials][sts-temp]). The Well-Architected security
pillar orders the work: "The best way to approach secrets management is to follow the guidance of
remove, replace, and rotate. The most secure credential is one that you do not have to store,
manage, or handle." What cannot be replaced by a short-term credential is stored in a service such
as Secrets Manager and rotated. It lists as an anti-pattern "Storing long-term credentials in
source code or configuration files" ([Well-Architected][wa-secrets]). Any other credential, such as
a token for a service outside AWS, is a stored secret, named by a reference.

**How it is referenced.** In an ECS task definition the plain `environment` parameter holds literal
values, and AWS says: "We do not recommend using plaintext environment variables for sensitive
information, such as credential data." The `secrets` parameter's `valueFrom` names "the full Amazon
Resource Name (ARN) of the AWS Secrets Manager secret or the full ARN of the parameter in the AWS
Systems Manager Parameter Store" ([task definition parameters][ecs-taskdef]). On EKS, the AWS
provider for the Secrets Store CSI driver (ASCP) names secrets in a `SecretProviderClass`. Of
pinning a version ID it says "Not recommended because you must update the version ID every time you
update the secret. By default the most recent version is used", and the default label is
AWSCURRENT. Its `jmesPath` is "A map of the keys in the secret to the files to be mounted"
([ASCP examples][ascp-examples]). CloudFormation's display mark `NoEcho` masks a parameter "as
asterisks (*****) for any calls that describe the stack or stack events", with exceptions, and AWS
recommends instead that templates "use dynamic parameters in the stack template to reference
sensitive information that is stored and managed outside of CloudFormation"
([CloudFormation parameters][cfn-params]).

**Who may read it.** AWS uses two models. ECS reads a secret for injection under the task execution
role, which "grants the Amazon ECS container and Fargate agents permission to make AWS API calls on
your behalf". Those permissions "aren't directly accessible by the containers in the task", whose
code runs under a separate task role ([task execution role][ecs-exec-role]). ASCP reads under the
Pod's own role: "ASCP assumes the IAM role of the Pod, and then it can retrieve secrets from
Secrets Manager that are authorized for that role" ([EKS secrets][eks-secrets]), and "Other
containers can't access the secrets unless you also associate them with the IAM role"
([ASCP][ascp-irsa]).

The role itself is bound to the workload. On EKS, "Instead of creating and distributing your AWS
credentials to the containers or using the Amazon EC2 instance's role, you associate an IAM role
with a Kubernetes service account" ([IRSA][eks-irsa]). With EKS Pod Identity the cluster adds to
the Pod the variables `AWS_CONTAINER_CREDENTIALS_FULL_URI` and
`AWS_CONTAINER_AUTHORIZATION_TOKEN_FILE` and a projected token with a 24-hour expiry, and the node
agent gets temporary credentials from the EKS Auth API: the environment holds a pointer and a path,
not a credential ([Pod Identity][eks-podid]). For tasks on EC2, AWS "strongly recommend[s]"
blocking containers from the instance metadata service, "to prevent your containers from accessing
sensitive instance-level data, including the instance profile credentials of the instance"
([ECS roles][ecs-roles]).

**How it reaches code.** ECS injects a secret at start: "Sensitive data is injected into your
container when the container is initially started. If the secret is subsequently updated or
rotated, the container will not receive the updated value automatically." A new task, or a forced
new deployment, is needed. "Applications that run on the container and container logs and debugging
tools have access to the environment variables" ([ECS secrets][ecs-secrets]). ECS's own guidance
recommends other paths because of that risk: "Because there's an elevated risk of data leakage with
environment variables, you should run a sidecar container that reads your secrets from AWS Secrets
Manager and write them to a shared volume", or have the application read an encrypted S3 object
under its task role; either way "your application must be written to read" it
([sensitive data][ecs-sensitive]). AWS's ASCP project calls file mounts compatibility for legacy
applications: "Security conscious applications should use the native AWS APIs to fetch secrets and
optionally cache them in memory rather than storing them in the file system"
([ASCP repository][ascp-repo]). The EKS best practices say: "Use volume mounts instead of
environment variables The values of environment variables can unintentionally appear in logs"
([EKS best practices][eks-bp]).

Lambda keeps environment variables in configuration: "An environment variable is a pair of strings
that is stored in a function's version-specific configuration", encrypted at rest. Lambda itself
sets the execution role's keys in reserved variables, `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`
and `AWS_SESSION_TOKEN`, at initialization. For anything else it recommends "AWS Secrets Manager
instead of environment variables to store database credentials and other sensitive information like
API keys or authorization tokens" ([Lambda environment][lambda-env]). Its Parameters and Secrets
extension caches for 300 seconds by default, and "If you rotate secrets frequently, the default
300-second cache duration might cause your function to use outdated secrets"
([Lambda and Secrets Manager][lambda-sm]).

**Rotation and revocation.** Instance-profile credentials "are temporary and we rotate them
automatically. We make new credentials available at least five minutes before the expiration of the
old credentials" ([instance credentials][ec2-imds]). A value injected at start does not change
until a new task starts ([ECS secrets][ecs-secrets]). A value resolved by CloudFormation does not
change until its resource is updated ([CloudFormation reference][cfn-sm]). A cached value is stale
for up to the cache's lifetime ([Lambda and Secrets Manager][lambda-sm]). ASCP's rotation is an
optional alpha reconciler that remounts secrets on a poll, and "A one hour poll interval is
recommended as a default to reduce excessive API costs" ([ASCP repository][ascp-repo]). Issued role
credentials are revoked at the verifier, not recalled: AWS attaches a policy that denies everything
to sessions issued before a time, and "All temporary credentials for that role issued before the
specified time become invalid" ([revoking sessions][sts-revoke]).

**Auditing.** Each call is attributed to the workload: "Task credentials have a context of
'taskArn' that is attached to the session, so CloudTrail logs show which task the role credentials
were vended for" ([task role][ecs-task-role]). Reads served by a local caching provider are not in
CloudTrail, as the previous section says.

**What AWS guarantees and leaves to the user.** AWS states the cost of each path rather than hiding
it. Values in an environment are visible to the application, its logs and its debugging tools, and
do not follow rotation without a restart ([ECS secrets][ecs-secrets]). Caches go stale
([Lambda and Secrets Manager][lambda-sm]). Command arguments can be seen by other utilities
([CLI exposure][sm-cli]). It recommends against keeping sensitive data on a machine image that is
shared, because "Users who launch a shared AMI might be able to rebundle it and register it as
their own" ([shared AMIs][ec2-ami]).

## The Kubernetes Secret

**What a secret is.** A Secret is an API object that holds the value itself. "Secret holds secret
data of a certain type", and "The serialized form of the secret data is a base64 encoded string"
([Secret API][k8s-secret-api]). "Individual Secrets are limited to 1MiB in size. This is to
discourage creation of very large Secrets that could exhaust the API server and kubelet memory"
([Secrets][k8s-secret]). Confidential data is kept apart from configuration by the kind of object,
not by a mark on a field: "A ConfigMap is an API object used to store non-confidential data in
key-value pairs", and "ConfigMap does not provide secrecy or encryption. If the data you want to
store are confidential, use a Secret rather than a ConfigMap" ([ConfigMap][k8s-configmap]). A
Secret's `type` names a convention for its keys. The basic-auth type "is provided only for
convenience", and it "helps other people to understand the purpose of your Secret, and sets a
convention for what key names to expect" ([Secrets][k8s-secret]). The reason given for a separate
object is exposure: "Because Secrets can be created independently of the Pods that use them, there
is less risk of the Secret (and its data) being exposed during the workflow of creating, viewing,
and editing Pods" ([Secrets][k8s-secret]).

The value may still be written in a manifest, and then it is exposed: "If you configure a Secret
through a manifest, with the secret data encoded as base64, sharing this file or checking it in to
a source repository means the secret is available to everyone who can read the manifest." And:
"Base64 encoding is not an encryption method, it provides no additional confidentiality over plain
text" ([good practices][k8s-gp]). The field `stringData` "is provided as a write-only input field
for convenience. All keys and values are merged into the data field on write", and "The stringData
field is never output when reading from the API" ([Secret API][k8s-secret-api]): the value is still
returned, under `data`.

**How it is referenced.** By the Secret's name, and for one value by a key in it:
`env[].valueFrom.secretKeyRef` with `name` and `key`. `envFrom` imports every key: "Use envFrom to
define all of the Secret's data as container environment variables. The key from the Secret becomes
the environment variable name in the Pod" ([distribute credentials][k8s-distribute]). A volume
names the whole Secret. Identifiers of keys are public: a KMS key's `key_id` "is the public,
non-secret name of the remote KMS KEK that is currently in use. It may be logged during regular
operation of the API server, and thus must not contain any private data" ([KMS provider][k8s-kms]).

**Who may read it.** RBAC decides, and Kubernetes documents three gaps. First, the store: "Anyone
with API access can retrieve or modify a Secret, and so can anyone with access to etcd"
([Secrets][k8s-secret]). Second, workloads: "anyone who is authorized to create a Pod in a
namespace can use that access to read any Secret in that namespace; this includes indirect access
such as the ability to create a Deployment" ([Secrets][k8s-secret]); "A user who can create a Pod
that uses a Secret can also see the value of that Secret" ([good practices][k8s-gp]). The RBAC good
practices list workload creation among the privilege-escalation risks, since it "implicitly grants
access to many other resources in that namespace, such as Secrets", and say "boundaries within a
namespace should be considered weak" ([RBAC good practices][k8s-rbac-gp]). Third, listing:
"Granting list access to Secrets implicitly lets the subject fetch the contents of the Secrets"
([good practices][k8s-gp]), and "list and watch access also effectively allow for users to reveal
the Secret contents" ([RBAC good practices][k8s-rbac-gp]).

Access can be narrowed to one named Secret with `resourceNames`, by choice: "For resourceNames, an
empty set means that everything is allowed", and "Permissions are purely additive (there are no
"deny" rules)" ([RBAC][k8s-rbac]). The mitigations offered detect and limit: "Use short-lived
Secrets" and "Implement audit rules that alert on specific events, such as concurrent reading of
multiple Secrets by a single user" ([good practices][k8s-gp]). `kubectl get` and `kubectl describe`
"avoid showing the contents of a Secret by default", so that it is not "stored in a terminal log";
the same page then prints one with `-o jsonpath` and `base64 --decode` ([kubectl][k8s-kubectl]).
The deliverer is limited too: the Node authorizer lets a kubelet read "secrets, configmaps,
persistent volume claims and persistent volumes related to pods bound to the kubelet's node"
([Node authorization][k8s-node]).

**How it reaches code.** As an environment variable fixed at container start, or as a file. "A
Secret is only sent to a node if a Pod on that node requires it. For mounting Secrets into Pods,
the kubelet stores a copy of the data into a tmpfs so that the confidential data is not written to
durable storage." The copy is deleted with the Pod, and "one Pod does not have access to the
Secrets of another Pod"; but "Any containers that run with privileged: true on a node can access
all Secrets used on that node" ([Secrets][k8s-secret]). A workload fails closed: "By default,
Secrets are required. None of a Pod's containers will start until all non-optional Secrets are
available." A missing named key fails the Pod, the kubelet retries and reports an event, and "If an
optional Secret doesn't exist, Kubernetes ignores it" ([Secrets][k8s-secret]). The platform can use
a Secret for a workload without showing it: "Secrets can also be used by other parts of the system,
without being directly exposed to the Pod", as the kubelet does when it pulls a private image
([Secrets][k8s-secret]). Among the "Alternatives to Secrets" Kubernetes names "a service that Pods
access over HTTPS, that reveals a Secret if the client correctly authenticates (for example, with a
ServiceAccount token)" ([Secrets][k8s-secret]).

**Storage and encryption.** "Kubernetes Secrets are, by default, stored unencrypted in the API
server's underlying data store (etcd)" ([Secrets][k8s-secret]). "By default, the API server stores
plain-text representations of resources into etcd, with no at-rest encryption." Encryption is an
ordered list of providers that the operator configures, and "The first provider in the list is used
to encrypt resources written into the storage". A key kept locally "protects against an etcd
compromise, but it fails to protect against a host compromise", because the keys sit in a file on
the host. Turning encryption on does not encrypt what is already stored: the documented step "reads
all Secrets and then updates them with the same data" ([encrypting data][k8s-encrypt]). KMS v2
wraps data keys with a key encryption key held in a remote KMS, and "All data encryption keys
(DEKs) will be cached in the clear once the server has unwrapped them via a call to the KMS". The
KMS plugin is a gRPC server on a UNIX socket on each control-plane host, and "All configuration
data, including authentication credentials the KMS plugin uses to communicate with the remote KMS,
are stored and managed by the KMS plugin independently" ([KMS provider][k8s-kms]).

**Rotation and revocation.** "If a container already consumes a Secret in an environment variable,
a Secret update will not be seen by the container unless it is restarted. There are third party
solutions for triggering restarts when secrets change" ([distribute credentials][k8s-distribute]).
A mounted volume is updated "using an eventually-consistent approach", from a kubelet cache, with a
delay of up to "the kubelet sync period + cache propagation delay"; "A container using a Secret as
a subPath volume mount does not receive automated Secret updates." An immutable Secret cannot be
changed or made mutable again: "You can only delete and recreate the Secret", and Pods should be
recreated ([Secrets][k8s-secret]).

Kubernetes has moved its own workload identity out of the Secret object. A service-account token
kept in a Secret "is a legacy mechanism that provides long-lived ServiceAccount credentials to
Pods"; "the recommended approach is to obtain a short-lived, automatically rotating ServiceAccount
token by using the TokenRequest API instead" ([Secrets][k8s-secret]). With a stored token, "any
user with read access to those Secrets can authenticate as the service account"
([authentication][k8s-authn]), and "Long-lived bearer tokens represent a security risk as, once
disclosed, the token can be misused" ([service accounts][k8s-sa]). A bound token "expires either
when the pod is deleted or after a defined lifespan (by default, that is 1 hour)", and "There is no
specific mechanism to invalidate a token issued via TokenRequest. If you no longer trust a bound
service account token for a Pod, you can delete that Pod"
([service account administration][k8s-sa-admin]). Whether a token stops working at once depends on
who checks it: after the Pod is deleted "a TokenReview immediately fails", while "If you use OIDC
validation instead, your clients continue to treat the token as valid until the token reaches its
expiration timestamp" ([service accounts][k8s-sa]). Tokens are bound to an audience: "Your
application should always define the audience that it accepts, and should check that the token's
audiences match the audiences that the application expects" ([service accounts][k8s-sa]). The
kubelet refreshes a projected token at 80% of its lifetime or after 24 hours, and "The application
is responsible for reloading the token when it rotates"
([service account configuration][k8s-sa-config]).

Credentials can also come from a command. `kubectl` and the kubelet "are able to execute an
external command to receive user credentials", and a credential with an expiry is "cached until the
expiry time is reached, or if the server responds with a 401 HTTP status code, or when the process
exits" ([authentication][k8s-authn]). Image credential provider plugins "allow the kubelet to
request credentials for a container registry dynamically as opposed to storing static credentials
on disk" ([credential provider][k8s-credprov]).

**Auditing.** Audit is the operator's policy; with no policy "no events are logged". The example
policy logs Secrets at the Metadata level, which records "requesting user, timestamp, resource,
verb, etc." "but not request or response body" ([auditing][k8s-audit]).

**What Kubernetes guarantees and leaves to the user.** Little by default. After delivery:
"Applications still need to protect the value of confidential information after reading it from an
environment variable or volume. For example, your application must avoid logging the secret data in
the clear or transmitting it to an untrusted party" ([good practices][k8s-gp]). Encryption at rest,
the permissions of the key file, access to etcd, the audit policy, and restarting workloads when a
value changes are the operator's ([encrypting data][k8s-encrypt], [auditing][k8s-audit],
[distribute credentials][k8s-distribute]).

## The Kubernetes ecosystem

Two families keep the value outside the cluster's own store, or copy it in. AWS states the line
between them: "rather than retrieving secrets from the external secret store, external-secrets
copies secrets from these backends to Kubernetes as Secrets" ([EKS best practices][eks-bp]). The
first family reads the outside store and mounts the value at use: the Secrets Store CSI driver, the
Vault Agent Injector, and the Vault Secrets Operator's CSI driver. The second copies the value into
an ordinary Kubernetes Secret: the External Secrets Operator (ESO), the Vault Secrets Operator by
default, Sealed Secrets, and SOPS files decrypted by Flux.

**What a secret is and how it is referenced.** One reference kind fronts many stores. The CSI
driver reads a `SecretProviderClass`, and "communicates with the provider using gRPC to fetch the
mount contents from external Secrets Store" ([CSI concepts][csi-concepts]). ESO separates how a
store is reached from what is fetched: the SecretStore exists "to separate concerns of
authentication/access and the actual Secret and configuration needed for workloads", and "The
ExternalSecret specifies what to fetch, the SecretStore specifies how to access"
([ESO overview][eso-overview]). A reference can pick one key of a JSON secret: ASCP's `jmesPath`
([ASCP examples][ascp-examples]), and ESO's `property`, "Used to select a specific property of the
Provider value (if a map), if supported" ([ESO API][eso-spec]). Sealed Secrets keeps ciphertext in
Git instead: a SealedSecret "is safe to store - even inside a public repository", and "can be
decrypted only by the controller running in the target cluster" ([Sealed Secrets][sealed]).

**Who may read it.** In the mount family the store can see the workload's own identity.
Service-account tokens "may be forwarded from the kubelet process to the driver and then to
provider plugins. This allows the provider to impersonate the pod when contacting the external
secret API" ([CSI concepts][csi-concepts]). With AWS's provider, "Other containers can't access the
secrets unless you also associate them with the IAM role" ([ASCP][ascp-irsa]). In the copy family
one controller reads for everyone: ESO "operates with elevated privileges within your Kubernetes
cluster, allowing it to read and write to all secrets across all namespaces", in its usual
cluster-wide deployment ([ESO security][eso-security]). Where tenants share one ClusterSecretStore,
"Now all application developers have access to all the secrets", and "ESO does not provide a
mechanic to limit access to certain keys per namespace" ([ESO multi-tenancy][eso-tenancy]).

**How it reaches code.** Google ranks the paths for GKE. "You should access Secret Manager secrets
from your application code whenever possible, using the Secret Manager API"; mount only where the
application needs files. "Mounted volumes are susceptible to directory traversal attacks.
Environment variables can be compromised due to misconfigurations such as opening a debug endpoint"
([GKE secrets][gke-wi-secrets]). "The CSI driver is invoked by kubelet only during the pod volume
mount" ([CSI limitations][csi-limits]). Its optional copy into a Kubernetes Secret syncs only "once
you start a pod mounting the secrets", and "When all the pods consuming the secret are deleted, the
Kubernetes secret is also deleted" ([CSI sync][csi-sync]). ESO "reads information from external
APIs and automatically injects the values into a Kubernetes Secret" ([ESO][eso]). Flux's
kustomize-controller "decrypts the Kubernetes secrets with SOPS and applies them on the cluster"
([Flux and SOPS][flux-sops]). Unsealing a SealedSecret makes "This normal kubernetes secret"
([Sealed Secrets][sealed]).

Argo CD warns against resolving secrets while rendering manifests: "We strongly caution against
this style of secret management", since "Argo CD stores generated manifests in plaintext in its
Redis cache, so injecting secrets into the manifests increases risk", and the manifests are also
served by its repo-server API. It recommends populating secrets on the destination cluster
([Argo CD][argo-secrets]). ESO can render secrets into a configuration file "that you mount into
your pod", and removed `env`, `expandenv`, `getHostByName` and `fail` from its template functions
"for security reasons" ([ESO templating][eso-templating]). Vault Agent's process supervisor mode
sets secrets in a child's environment; "it will wait until each environment variable template has
rendered at least once before starting the process", and by default it "will restart the process
whenever an update to an injected secret is detected"
([Vault process supervisor][vault-supervisor]).

**Rotation.** CSI rotation is alpha and "not enabled by default". `--rotation-poll-interval`,
"default `2m`", "controls the minimum cache duration between rotations". "The pod needs to be
restarted to get the latest secret as environment variable", and for files the "application will
need to watch for the file change" ([CSI rotation][csi-rotation]). ESO refreshes on
`refreshInterval`, which "May be set to zero to fetch and create it once". Its default deletion
policy, Retain, "Retains the Secret if all Secret data fields have been deleted from the provider"
([ExternalSecret][eso-es]): the copy can outlive its source. Sealed Secrets' "key renewal and
re-encryption features are not a substitute for periodical rotation of your actual secret values"
([Sealed Secrets][sealed]). HashiCorp draws the family line by lifecycle: with the Vault Secrets
Operator "Secret lifecycle is independent of Pod lifecycle due to caching in Kubernetes Secrets",
while with the CSI provider and the Agent Injector it "depends on Pod lifecycle"
([Vault on Kubernetes][vault-k8s-compare]). The operator's own CSI driver "never stores secrets in
etcd" ([VSO CSI driver][vso-csi]).

**Auditing.** The ecosystem relies on the Kubernetes audit policy and on the store's own audit log
([auditing][k8s-audit]). None of the pages read describes a separate audit record of each mount or
copy.

**What the ecosystem guarantees and leaves to the user.** A copy brings every gap of the Kubernetes
Secret with it, and the copy can outlive its source ([ExternalSecret][eso-es]). A component that
reads for many workloads holds broad rights ([ESO security][eso-security]). After delivery, the
application protects the value, as the Kubernetes good practices say ([good practices][k8s-gp]).

## GCP, Azure and HashiCorp Vault

**What a secret is.** All three keep a named object with versions, in a store with access control
of its own. In GCP, "Secret data is immutable and most operations take place on secret versions. A
secret version contains the actual secret data, along with state and metadata about the secret"
([GCP versions][gsm-add]). In Azure, "Key Vault APIs accept and return secret values as strings";
internally they are octets "with a maximum size of 25 KB each"; "The Key Vault service doesn't
provide semantics for secrets", and the optional `contentType` is "a hint for interpreting the
secret data" ([Key Vault secrets][akv-secrets]). In Vault's KV version 2, a secret is a map: "The
contents of the data map will be stored and returned on read"; each write "creates a new version",
and by default Vault keeps 10 ([KV v2][vault-kv2]).

**How it is referenced.** By name and version. In GCP a read names a version ID, an alias, or
`latest`. Reading by number right after adding a version is strongly consistent; "This doesn't
apply when you access a secret version using `aliases` or `latest`", and other operations
"typically converge within minutes, but may take a few hours" ([GCP access][gsm-access]). Google's
best practice is to pin: "Reference secrets by their version number rather than using the latest
alias", so that "the configuration can be validated and rolled back using your existing release
processes" ([GCP best practices][gsm-bp]). In Azure an identifier names the vault, the object type,
the name and an optional version, and "You can retrieve objects in Key Vault by specifying a
version or by omitting the version to get the latest version of the object". Microsoft warns that
values given as identifiers may be copied globally: "Don't include personally identifiable or
sensitive information in the value you provide" ([Key Vault objects][akv-objects]). App Service
resolves a setting written `@Microsoft.KeyVault(SecretUri=...)` or
`VaultName=...;SecretName=...;SecretVersion=...`, and "Your app can reference the secret through
its key as normal. No code changes are required" ([Key Vault references][appsvc-kv]).

**Who may read it.** Reading the value is a permission of its own. In GCP the Secret Accessor role
"Allows accessing the payload of secrets", and Viewer, which sees metadata, lacks
`secretmanager.versions.access`. Granted on one secret it reaches that one; granted on the project
it reaches every secret in it; and "Always apply permissions at the lowest level in the resource
hierarchy" ([GCP access control][gsm-iam]). In Azure, Key Vault Reader "Cannot read sensitive
values such as secret contents or key material", and Key Vault Secrets User can
([Key Vault RBAC][akv-rbac]). Under Key Vault's older access-policy model, users who may write the
vault "can grant themselves data plane access by configuring a Key Vault access policy". Microsoft
recommends RBAC, the default for new vaults from API version 2026-02-01, and notes that "RBAC
restricts permission management to only the 'Owner' and 'User Access Administrator' roles"
([RBAC and access policies][akv-rbac-vs]). Granting can be delegated within limits: Key Vault Data
Access Administrator assigns only Key Vault roles, and "Includes an ABAC condition to constrain
role assignments" ([Key Vault RBAC][akv-rbac]).

The reader's identity comes from the platform rather than from a stored secret. GKE Workload
Identity gives Pods access "without needing manual configuration or less secure methods like
service account key files", with tokens that last an hour by default
([GKE workload identity][gke-wi]). Azure managed identities obtain tokens "without having to manage
any credentials", and "Credentials aren't even accessible to you" ([managed identities][entra-mi]).
Under Vault's AWS method a client signs a request, and "The actual secret (i.e., the AWS secret
access key) is never transmitted over the wire" ([Vault AWS auth][vault-aws]). For the first
credential, HashiCorp describes a "Trusted Orchestrator" that is already authenticated and that
"launches new applications and inject a mechanism they can use to authenticate"
([secure introduction][vault-intro]).

For agents, Vault Enterprise's on-behalf-of tokens name the person and the agent, and "Vault allows
the request only if both the subject baseline and the OBO ceiling permit it"
([Vault agent permissions][vault-obo]). HashiCorp's validated pattern puts the credential in the
MCP server, which "Validates the OBO token, authenticates to Vault, retrieves dynamic credentials,
and accesses API/services using dynamic credentials", while the agent "cannot directly access
sensitive resources and only calls the Model Context Protocol (MCP) Server"
([Vault AI agent identity][vault-ai]).

**How it reaches code.** Google: "We recommend using the Secret Manager API directly using one of
the provided client libraries", and "Avoid passing secrets to your application through the file
system or through the environment variables", though "for some product integrations such as
serverless integrations, you may". The stated risk: "misconfigurations such as enabling debug
endpoints or including dependencies that log process environment details may leak secrets"
([GCP best practices][gsm-bp]). Cloud Run resolves a secret exposed as an environment variable "at
instance startup time", for which Google recommends pinning a version; "Cloud Run retrieves the
value of the secret prior to starting the instance. If the secret retrieval process fails, the
instance doesn't start." A secret mounted as a volume is read at each use: "When reading a volume,
Cloud Run always fetches the secret value from the Secret Manager". At deployment Cloud Run checks
"that the service account that runs the container has permission to access these secrets"
([Cloud Run secrets][run-secrets]).

App Service resolves Key Vault references with the app's own identity: "Key vault references use
the app's system-assigned identity by default", which needs the Key Vault Secrets User role. "If a
reference isn't resolved properly, the reference string is used instead"
([Key Vault references][appsvc-kv]). Vault Agent's process supervisor sets secrets in a child's
environment and waits for every one before starting it
([Vault process supervisor][vault-supervisor]). Its Agent Injector renders secrets to a shared
memory volume, so that "containers within the pod can consume Vault secrets without being Vault
aware" ([Vault injector][vault-injector]). Response wrapping puts a response behind a single-use
token, so "the value being transmitted across the wire is not the actual secret but a reference to
such a secret" ([response wrapping][vault-wrap]).

**Rotation and revocation.** Rotation adds a version. GCP's scheduled rotation only notifies:
"Secret Manager triggers a `SECRET_ROTATE` message to the designated Pub/Sub topics at the secret's
`next_rotation_time`", and "You must configure a Pub/Sub subscriber to receive and act on the
`SECRET_ROTATE` messages", with further workflows "such as creating a new version of the secret"
([GCP rotation][gsm-rotation]; re-opened). App Service picks up a new version of an unpinned
reference "within 24 hours", because it "caches the values of the Key Vault references and
refetches them every 24 hours"; "Any configuration change to the app causes an app restart and an
immediate refetch" ([Key Vault references][appsvc-kv]). Vault Agent restarts a supervised process
when a secret changes, unless told never to ([Vault process supervisor][vault-supervisor]).
Revocation reaches a copy only where the store issued it: "When a lease is revoked, it invalidates
that secret immediately and prevents any further renewals", and the KV store issues no leases
([leases][vault-lease]). "When a parent token is revoked, all of its child tokens -- and all of
their leases -- are revoked as well" ([tokens][vault-tokens]).

**Auditing.** Every read can be audited with the caller and the object, never the value. In GCP,
reading a value is a Data Access event (`AccessSecretVersion`, `DATA_READ`)
([GCP audit logging][gsm-audit]), and "Data Access audit logs are disabled by default for all
services but some BigQuery services" ([data access logs][gcp-audit-config]); the best practices say
to enable them ([GCP best practices][gsm-bp]). An Azure Key Vault record holds the time, the
operation, such as `SecretGet`, the result, the caller's address and the "Identity from the token
that was presented in the REST API request"; no field holds the value
([Key Vault logging][akv-logging]). Vault goes further: "By default, Vault only writes a keyed hash
(HMAC-SHA256) of most string values to audit logs", and "To calculate the keyed hash of any known
value for a given audit device, use the /sys/audit-hash API endpoint". It fails closed: where it
"cannot log information to at least one of the enabled devices, Vault refuses to service the
corresponding API request" ([Vault audit][vault-audit]).

**What they guarantee and leave to the user.** Access to the value is an explicit grant on the
reader's identity; what the reader does after a read is the reader's. Only an issuer with leases
makes a copy already handed out invalid ([leases][vault-lease]). GCP leaves the audit of reads off
until the user turns it on ([data access logs][gcp-audit-config]).

## Configuration tools, MCP and agent tooling

**What a secret is and how it is referenced.** Marking a value for display hides nothing.
Terraform's `sensitive` redacts output, but "Terraform stores values with the sensitive argument in
both state and plan files, and anyone who can access those files can access your sensitive values".
Ephemeral values "are available at runtime, but Terraform omits them from state and plan files
entirely" ([Terraform sensitive data][tf-sensitive]). "You can only refer to ephemeral resources in
specific ephemeral contexts. Otherwise, Terraform throws an error when it plans your changes"
([Terraform ephemeral][tf-ephemeral]). "You cannot add the `ephemeral` argument to `output` blocks
in the root module" ([Terraform output][tf-output]; re-opened). Of write-only arguments, "Terraform
has no way of knowing if a write-only argument value has changed", so a separate version argument
is incremented to change one ([Terraform write-only][tf-wo]). Helm keeps release records in
Kubernetes Secrets by default, and those records include values files, which "might contain
sensitive data (like passwords, private keys, and other credentials)" ([Helm][helm-adv]).

Pulumi takes the other path: the engine keeps secrets, encrypted in its state file, so that "these
values never appear as plain text in your state file", with keys from Pulumi Cloud by default or
from a passphrase, AWS KMS, Azure Key Vault, GCP KMS or Vault. A value computed from a secret is a
secret too: "the resulting output is also marked as a secret" ([Pulumi secrets][pulumi]).

The others refer to a value in a store. 1Password references are written
`op://vault/item/[section/]field`, and "reflect changes you make in your 1Password account, so when
you run a script you get the latest value" ([1Password references][op-ref]). A Compose secret's
source "is either file or environment" ([Compose secrets][compose-secrets]). Claude Code's
`.mcp.json` writes `${VAR}` in a server's command, arguments, environment, URL or headers; where it
warns of a conflict it quotes the reference unexpanded, "so it never shows a resolved value such as
an API key" ([Claude Code MCP][cc-mcp]). The MCP registry's `server.json` schema (2025-12-11) lets
a package declare "A mapping of environment variables to be set when running the package", and
remote headers in the same form; each input has `isSecret`: "If true, clients should handle the
value securely" ([MCP registry schema][mcp-registry]).

**Who may read it.** Compose: "Defining a secret in the top-level secrets must not imply granting
any service access to it. Such grant must be explicit within service specification as secrets
service element" ([Compose services][compose-services]). The Model Context Protocol, revision
2026-07-28: "MCP servers MUST validate that access tokens were issued specifically for them as the
intended audience", "MCP servers MUST NOT accept or transit any other tokens"
([MCP authorization][mcp-auth]), and "The MCP server MUST NOT pass through the token it received
from the MCP client". A server that calls another API "may act as an OAuth client to them", and
"The access token used at the upstream API is a separate token" ([MCP security][mcp-sec]). For a
user's credential to a third party, servers "MUST NOT use form mode elicitation to request
sensitive information such as passwords, API keys, access tokens, or payment credentials", and
"MUST use URL mode". "The third-party credentials MUST NOT transit through the MCP client"; the
server stores them, bound to the user's identity, and "MUST NOT transmit credentials obtained
through URL mode elicitation to the MCP client". The aim is that "sensitive credentials never pass
through the LLM context, MCP client or any intermediate MCP servers"
([MCP elicitation][mcp-elicit]). direnv does not load a new `.envrc` until the user runs
`direnv allow`: "This is the security mechanism to avoid loading new files automatically"
([direnv][direnv]).

**How it reaches code.** Over stdio, "Implementations using an STDIO transport SHOULD NOT follow
this specification, and instead retrieve credentials from the environment"
([MCP authorization][mcp-auth]). `op run` scans for references and runs "a provided command in a
subprocess with the secrets made available as environment variables only for the duration of the
subprocess"; "Secrets printed to stdout or stderr are concealed by default" ([op run][op-run]).
1Password warns: "You should assume that processes on your computer can access the environment of
other processes run by the same user" ([1Password environment][op-env]). Doppler injects with
`doppler run`, but "We strongly recommend against downloading secrets in plain text to the file
system"; its mount is "a Linux named pipe" that "is automatically cleaned up when the Doppler
process exits", with an optional limit on reads ([Doppler][doppler]). Docker warns: "Environment
variables are often available to all processes, and it can be difficult to track access. They can
also be printed in logs when debugging errors without your knowledge." Compose mounts a secret "as
a file in /run/secrets/<secret_name>", and some images, including official ones such as mysql and
postgres, read a variable ending `_FILE` that names such a file ([Compose how-to][compose-howto]).
Swarm keeps secrets in its encrypted Raft log and mounts them in memory; a secret is reachable
"only while those service tasks are running", and is "flushed from the node's memory" when a task
stops ([Swarm secrets][swarm]). Kubernetes's own instructions write a file-sourced secret with
`echo -n`: "The -n flag ensures that there's no newline character at the end of your files"
([Kustomize][k8s-kustomize]).

Agent tooling takes its own key from the environment or a store. The Anthropic Python SDK reads
`ANTHROPIC_API_KEY` by default ([Anthropic SDK][anthropic-sdk]). Claude Code's order of precedence
puts `ANTHROPIC_AUTH_TOKEN` and `ANTHROPIC_API_KEY` above `apiKeyHelper` and the stored login. It
stores its login in the macOS Keychain, or in a file of mode 0600; on macOS "When the Keychain
rejects the write, such as when it's locked in an SSH session", it uses the file
([Claude Code IAM][cc-iam]). `apiKeyHelper` runs a command for "dynamic or rotating credentials";
Claude Code "caches the value and reruns the command" after five minutes by default, and after a
401 or 403 in stated cases ([Claude Code settings][cc-settings]). With
`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB` set, Claude Code "removes credentials from the environments of
the subprocesses it starts, such as Bash commands, hooks, and stdio MCP servers. This reduces what
a prompt injection attack can read. The Claude Code process keeps the credentials for its own API
calls" ([Claude Code environment][cc-env]). A `headersHelper` that a repository or plugin supplies
"is a command you didn't write, so Claude Code runs it without the credential variables from your
environment", and in a remote server's URL and headers it "reads credential variables from your
environment as empty" for a defined set of names ([Claude Code MCP][cc-mcp]). Codex keeps its login
in `~/.codex/auth.json` or the operating system's store; its `keyring` mode "stores credentials in
your operating system credential store and fails if it is unavailable"
([Codex authentication][codex-auth]).

**Rotation.** A reference resolved at each use sees a new value: 1Password references, and
`apiKeyHelper` after its cache lifetime or a refusal ([1Password references][op-ref],
[Claude Code settings][cc-settings]). A tool that keeps or copies a value changes by an explicit
step: Terraform's version argument for a write-only value ([Terraform write-only][tf-wo]).

**Auditing.** The pages read for these tools do not describe an audit record of each read. Claude
Code keeps resolved values out of its warnings ([Claude Code MCP][cc-mcp]).

**What they guarantee and leave to the user.** Pulumi: "Pulumi cannot guarantee that the apply
callback itself will not expose the secret value" ([Pulumi secrets][pulumi]). MCP asks clients to
"Warn that MCP servers run with the same privileges as the client" and to "Execute MCP server
commands in a sandboxed environment with minimal default privileges"
([MCP security best practices][mcp-bp]).

## Who may give a workload a read

Second round. The question here is not who may read a secret but who may arrange for a workload to
read one.

**AWS.** Attaching a role to a service is a permission of its own, `iam:PassRole`: "To pass a role
(and its permissions) to an AWS service, a user must have permissions to pass the role to the
service. This helps administrators ensure that only approved users can configure a service with a
role that grants permissions." It is checked when a role's ARN is passed to an operation that uses
the role, and "PassRole is not an API call. PassRole is a permission". AWS gives the reason: "you
should make sure that a user doesn't pass a role where the role has more permissions than you want
the user to have. For example, Alice might not be allowed to perform any Amazon S3 actions. If Alice
could pass a role to a service that allows Amazon S3 actions, the service could perform Amazon S3
actions on behalf of Alice when executing the job." The permission is scoped twice: to named roles,
by the policy's `Resource` element, and to the service that may receive the role, by the condition
key `iam:PassedToService`, which "can be used to specify the service principal of the service to
which a role can be passed" ([PassRole][iam-passrole]). Lambda's example policy for a developer
allows them to "View and pass only a role named intern-lambda-execution-role, which must be created
and managed by a user with IAM permissions. PassRole is used when you assign an execution role to a
function" ([Lambda user permissions][lambda-user]).

AWS delegates the creation of principals with permissions boundaries. Its example "allows Zhang to
create IAM users but only if he uses the XCompanyBoundaries policy to set the permissions boundary",
through the condition key `iam:PermissionsBoundary`, and denies him the right to change that policy
or to delete the boundary from any user. The boundary does not hold him to what he has himself:
"Zhang's policies allow him to create a user that can then access Amazon S3 resources that he can't
access. By delegating these administrative actions, Maria effectively trusts Zhang with access to
Amazon S3" ([permissions boundaries][iam-boundaries]).

**Kubernetes.** RBAC prevents escalation in its API. "You can only create/update a role if at least
one of the following things is true: 1. You already have all the permissions contained in the role,
at the same scope as the object being modified ... 2. You are granted explicit permission to perform
the escalate verb", so "if user-1 does not have the ability to list Secrets cluster-wide, they
cannot create a ClusterRole containing that permission". Binding follows the same rule: "You can
only create/update a role binding if you already have all the permissions contained in the
referenced role (at the same scope as the role binding) or if you have been authorized to perform
the bind verb on the referenced role." The `bind` verb can name particular roles with
`resourceNames`; leaving them out allows binding any role. The first grants need a superuser: "When
bootstrapping the first roles and role bindings, it is necessary for the initial user to grant
permissions they do not yet have", with "a credential with the "system:masters" group, which is
bound to the "cluster-admin" super-user role" ([RBAC][k8s-rbac]). The good practices call both verbs
a way around that protection. Of `bind`: "granting users this right allows for the bypass of
Kubernetes in-built protections against privilege escalation, allowing users to create bindings to
roles with rights they do not already have"; of `escalate`, "users with this right can effectively
escalate their privileges" ([RBAC good practices][k8s-rbac-gp]).

A workload's identity is not checked in the same way. "Since Pods can run as any ServiceAccount,
granting permission to create workloads also implicitly grants the API access levels of any service
account in that namespace" ([RBAC good practices][k8s-rbac-gp]). The annotation that limited which
Secrets the Pods of a ServiceAccount may mount, `kubernetes.io/enforce-mountable-secrets`, "is
deprecated since Kubernetes v1.32. Use separate namespaces to isolate access to mounted secrets"
([service accounts][k8s-sa]).

**GCP and Azure.** In GCP, the permission `iam.serviceAccounts.actAs` "is required to attach a
service account to a resource" ([attaching service accounts][gcp-attach]). Azure's Key Vault Data
Access Administrator may "Manage access to Azure Key Vault by adding or removing role assignments
for" eight Key Vault roles, among them Key Vault Secrets User, which may "Read secret contents"; its
own definition lists no data actions ([built-in roles][az-roles]). Its built-in condition limits
which roles it assigns. "If you want to further constrain the Key Vault Data Access Administrator
role assignment, you can add your own condition to constrain the types of principals (users, groups,
or service principals) or specific principals that can be assigned the Key Vault roles"
([delegating role assignments][az-delegate]).

## AWS and Kubernetes on a developer's machine

Second round. The sections above describe workloads in a cloud or a cluster. This one describes how
the AWS tools and `kubectl` find credentials on a person's own computer.

**The AWS credential chain.** "All SDKs have a series of places (or sources) that they check in
order to find valid credentials ... After valid credentials are found, the search is stopped." The
chains most often include access keys, web identity, a console login, IAM Identity Center, assume
role, the container provider, a process and IMDS, and "Not all SDKs support all providers, or even
all aspects within a provider" ([credential providers][aws-chain]). The SDK for Rust checks, in
order: the variables `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` and `AWS_SESSION_TOKEN`; the
`[default]` profile of the shared files, or the profile that `AWS_PROFILE` names; web identity;
container credentials; and IMDS, where "The SDK for Rust only supports IMDSv2". It "loads
credentials upon first request, caches them, and then attempts to refresh them during another
request when they are close to expiring" ([SDK for Rust][rust-chain]). "The shared AWS config and
credentials files are plaintext files", by default `~/.aws/config` and `~/.aws/credentials`, found
through the variable `HOME` (`USERPROFILE` on Windows), and moved with `AWS_CONFIG_FILE` and
`AWS_SHARED_CREDENTIALS_FILE` ([file location][aws-files]).

AWS advises against long-term keys for development: "To avoid security risks, don't use IAM users
for authentication when developing purpose-built software or working with real data. Instead, use
federation with an identity provider such as AWS IAM Identity Center." "Using long-term credentials
is not recommended", and where access keys are used, "The shared AWS credentials file is the
recommended location for storing credentials information" ([access keys][aws-static]). IAM Identity
Center needs a person to sign in, with the CLI's `aws sso login`, which "updates an SSO cache JSON
file with a new access token and refresh token and makes it available for use by SDKs". With the
SSO token provider configuration, "the hourly access token obtained from IAM Identity Center is
automatically refreshed using the refresh token". When the session ends, "Any code that creates a
new client will fail authentication as soon as the IAM Identity Center session expires", until the
person signs in again; a client already made keeps working until its own session times out
([Identity Center][aws-sso]).

**A command as the source.** `credential_process` names a program in the shared config file. It
prints JSON with `Version` 1, `AccessKeyId`, `SecretAccessKey`, `SessionToken`, `Expiration` and an
optional `AccountId`, and "can return a non-zero return code to indicate that an error occurred".
Without `Expiration` the keys are taken as long-term; otherwise they are "automatically refreshed by
rerunning the credential_process command before the credentials expire", and "The SDK does not cache
external process credentials the way it does assume-role credentials". The setting may not use `~`
or environment variables; a bare name is looked for on `PATH`; and the tool must not write "any
secret information to StdErr". AWS warns that "anyone with write access to your shared config file
can change the command that is run" ([process credentials][aws-process]).

**Machines outside AWS.** IAM Roles Anywhere gives temporary credentials "for workloads such as
servers, containers, and applications that run outside of AWS", against an X.509 certificate and
its private key; the SDKs and CLI can take them through `credential_process`
([Roles Anywhere][ra-sdk]). Its credential helper can keep the private key in a file, a PKCS#11
token, a TPM, the macOS Keychain or the Windows CNG store. Its `serve` command "Implements
the same URIs and request headers as IMDSv2" on 127.0.0.1, and AWS warns that "any process running
on the system that can reach 127.0.0.1 will be able to retrieve AWS credentials from the credential
helper". Its `update` command writes the keys into `~/.aws/credentials`, where "Any user or process
who can read the credentials file may be able to read and use those AWS credentials". Version 1.8.6
made `serve` reject a request whose Host or Origin header does not match, "which prevents DNS
rebinding attacks" ([credential helper][ra-helper]).

**Local credential endpoints.** IMDSv2 "uses session-oriented requests". A PUT returns a token,
with a lifetime of "up to a maximum of six hours", which later requests carry in
`X-aws-ec2-metadata-token`. Where tokens are required, a request without a valid token receives
a 401 error. "PUT requests are rejected if they contain an X-Forwarded-For header", and by default
the response to a PUT has an IP hop limit of 1 ([IMDS configuration][imdsv2]). The container
provider reads an endpoint from `AWS_CONTAINER_CREDENTIALS_FULL_URI`, and a token from the file
that `AWS_CONTAINER_AUTHORIZATION_TOKEN_FILE` names or, where that is not set, from
`AWS_CONTAINER_AUTHORIZATION_TOKEN`, of which the page says "If this variable is set, the SDK will
set the Authorization header on the HTTP request with the environment variable's value". "When
configured, Amazon EKS sets the AWS_CONTAINER_CREDENTIALS_FULL_URI and
AWS_CONTAINER_AUTHORIZATION_TOKEN_FILE environment variables" ([container credentials][aws-cont]).
The page's table of support, which "Any partial exceptions are noted" introduces, marks every SDK
and tool it lists as supporting the container provider, from the AWS CLI v2 to the SDK for Rust
and the Tools for PowerShell; its only notes concern Lambda SnapStart
([container credentials][aws-cont]; re-opened in the third round). That table is for this provider
alone: of the chain as a whole, "Not all SDKs support all providers".
The SDK for Rust accepts any https URI there, and a plain http URI only where every address it
resolves to is a loopback address or the link-local address of ECS or EKS Pod Identity
([aws-config source][rust-ecs]). The Workload Credentials Provider, described in the
[first section](#aws-secrets-manager-and-parameter-store), takes a token header on each request,
and "uses the AWS SDK for Rust, which uses the AWS credential provider chain" for its own
credentials ([Workload Credentials Provider][sm-wcp]).

**kubectl.** "By default, kubectl looks for a file named config in the $HOME/.kube directory"; "The
KUBECONFIG environment variable holds a list of kubeconfig files", and "If the --kubeconfig flag is
set, use only the specified file". Kubernetes warns: "Only use kubeconfig files from trusted
sources. Using a specially-crafted kubeconfig file could result in malicious code execution or file
exposure" ([kubeconfig][k8s-kubeconfig]). An exec plugin is a command whose output is the
credential: "The executed command prints an ExecCredential object to stdout", and "is passed an
ExecCredential object as input via the KUBERNETES_EXEC_INFO environment variable". The object, of
`apiVersion` `client.authentication.k8s.io/v1`, holds a bearer token or a client certificate and
key, and "Optionally, the response can include the expiry of the credential formatted as a RFC 3339
timestamp". A credential is "cached until the expiry time is reached, or if the server responds
with a 401 HTTP status code, or when the process exits". A plugin declares whether it needs a
person: under `interactiveMode: Always`, "the exec plugin will only be run if standard input is
available for user input. If standard input is not available for user input, then the exec plugin
will not be run and an error will be returned" ([authentication][k8s-authn]). Since Kubernetes 1.35,
in beta, kubectl's preference file `kuberc` can set `credentialPluginPolicy` to `AllowAll`, the
default, `DenyAll` or `Allowlist`, the last with a `credentialPluginAllowlist`. The page says:
"Editors of a kubeconfig can specify an executable plugin that will be used to acquire credentials
to authenticate the client to the cluster". "Unlike the kubeconfig file, a kuberc configuration
file does not contain cluster details, usernames or passwords" ([kuberc][kuberc]).

On EKS the two meet. "Amazon EKS uses the aws eks get-token command with kubectl for cluster
authentication", and unless a role is named, "the IAM principal in your default AWS CLI or SDK
credential chain is used" ([EKS kubeconfig][eks-kubeconfig]).

**Other command formats.** Google's client libraries run an executable for a credential only where
the person opts in: "To use executable-sourced credentials, you must set the environment variable
GOOGLE_EXTERNAL_ACCOUNT_ALLOW_EXECUTABLES to 1". The executable writes JSON with `version`,
`success`, `token_type`, an `id_token` or a `saml_response`, and `expiration_time`, and its output
can be cached in a file ([GCP executable credentials][gcp-wif]). Docker's credential helpers are
programs named `docker-credential-` and a suffix, found on `PATH`, and "There are only three
possible values for that argument: store, get, and erase". "If you don't configure a credential
store, Docker stores credentials in the config.json file in a base64-encoded format"
([docker login][docker-login]).

## Ciphertext kept beside configuration

Second round. Two tools keep encrypted values in Git next to the configuration that uses them.

**Sealed Secrets.** Its problem statement is "I can manage all my K8s config in git, except
Secrets", and its answer is to "Encrypt your Secret into a SealedSecret, which *is* safe to store -
even inside a public repository". "The SealedSecret can be decrypted only by the controller running
in the target cluster and nobody else (not even the original author)", and "This normal kubernetes
secret will appear in the cluster". The controller's private keys "are just normal k8s secrets
living in the same namespace where the `SealedSecret` controller lives". Outside the cluster, a
backed-up key can decrypt a sealed file with `kubeseal --recovery-unseal --recovery-private-key`
([Sealed Secrets][sealed]).

A sealed value is tied to where it may be used, by scope. "`strict` (default): the secret must be
sealed with exactly the same *name* and *namespace*. These attributes become *part of the encrypted
data* and thus changing name and/or namespace would lead to 'decryption error'." Under
`namespace-wide` it may be renamed within its namespace, and under `cluster-wide` "the secret can be
unsealed in *any* namespace and can be given *any* name". Sealing does not identify who sealed: "By
design, this scheme *does not authenticate the user*. In other words, *anyone* can create a
`SealedSecret` containing any `Secret` they like (provided the namespace/name matches). It is up to
your existing config management workflow, cluster RBAC rules, etc to ensure that only the intended
`SealedSecret` is uploaded to the cluster" ([Sealed Secrets][sealed]).

The sealing key is renewed and kept. "Sealing keys are automatically renewed every 30 days", and
"old keys are not deleted when new keys are generated", so older SealedSecrets can still be
decrypted. A backup of the keys "should be kept omg-safe!", and "After sealing key renewal you
should recreate your backup". Renewal is "**not a substitute** for rotating your actual secrets".
A leak is permanent: "If you store anything in a version control storage, and in a public one in
particular, you must assume you cannot ever delete that information"; "*If* a sealing key somehow
leaks out of the cluster you must consider all your `SealedSecret` resources encrypted with that key
as compromised" ([Sealed Secrets][sealed]). AWS's EKS best practices list Sealed Secrets beside
external stores as an alternative to Kubernetes Secrets, "allowing you to safely store sealed
secrets in source control systems like Git", and say that external stores "offer features such as
fine grained access controls, strong encryption, and automatic rotation of secrets that are not
available with Kubernetes Secrets" ([EKS best practices][eks-bp]).

**SOPS.** SOPS "generates a random 256 bit data key and asks each KMS and PGP master key to encrypt
the data key", and stores the results in the file. Each value is encrypted "with AES256_GCM using
the data key and a 256 bit random initialization vector. Each file uses a single data key to encrypt
all values of a document, but each value receives a unique initialization vector". Key names stay
readable and are used as additional authenticated data: "We expect that keys do not carry sensitive
information, and keeping them in cleartext allows for better diff". "SOPS computes a MAC on all the
values to ensure that no value has been added or removed fraudulently." "Any valid KMS or PGP
master key can later decrypt the data key and access the data", unless key groups are set, in which
case a threshold "Determines the number of key groups that must be present each to decrypt the
file's key". The options `encrypted_regex` and `unencrypted_suffix` choose which values are
encrypted, so a file can hold some values in the clear; by default those are still covered by the
MAC ([SOPS reference][sops-ref]).

A SOPS file can be decrypted at the moment of use. Given a path in SOPS's own syntax, a list of keys
each in brackets and quotes, `sops decrypt --extract` "can extract a specific part of a YAML or JSON
document", and prints it to standard output by default ([SOPS operations][sops-ops]). `exec-env`
"can be used to ensure that the decrypted contents are available only to this process and never
written to disk"; `exec-file` passes a file through a FIFO, so that "the plaintext secrets never
touch the disk, and the child process can only read the secrets once", and with `--no-fifo` it uses
"a traditional temporary file that will get cleaned up once the process is finished executing"
([SOPS advanced][sops-adv]). For the age keys of one person, SOPS looks for `keys.txt` under the
user's configuration directory: on macOS, `$XDG_CONFIG_HOME/sops/age/keys.txt`, then "Falls back to
$HOME/Library/Application Support/sops/age/keys.txt if $XDG_CONFIG_HOME isn't set". The variables
`SOPS_AGE_KEY_FILE`, `SOPS_AGE_KEY` and `SOPS_AGE_KEY_CMD` override that ([SOPS age][sops-age]).
Flux, by contrast, decrypts when it applies, as the [ecosystem section](#the-kubernetes-ecosystem)
says, and leaves an ordinary Kubernetes Secret.

## Delivering a credential to an existing MCP server

Second round. How existing stdio MCP servers take a credential, and how the tools that run them
deliver one.

**The protocol.** "Implementations using an STDIO transport SHOULD NOT follow this specification,
and instead retrieve credentials from the environment." The page uses the word "environment" once
and does not define it further ([MCP authorization][mcp-auth]).

**Servers.** The READMEs of several servers that take one token were read. GitHub's server
documents its personal access token only as the variable `GITHUB_PERSONAL_ACCESS_TOKEN`, "(it takes
precedence over OAuth)", with no flag and no file form for it; for a GitHub App's private key it has
a path option, "Preferred over GITHUB_APP_PRIVATE_KEY: keeps the key off the command line and out
of the environment" ([GitHub MCP server][srv-github]). Three accept a file beside the variable.
Brave's: "`BRAVE_API_KEY_FILE`: Path to a file containing your Brave Search API key. When set, this
takes precedence over `BRAVE_API_KEY`. Useful for Docker secrets and similar mounted-secret setups"
([Brave MCP server][srv-brave]). Grafana's `GRAFANA_SERVICE_ACCOUNT_TOKEN_FILE`: "The file is read
fresh on every request, so rotated tokens are picked up automatically without restarting the
server"; surrounding whitespace is trimmed, and where both are set "the inline token takes
precedence" ([Grafana MCP server][srv-grafana]). A community GitLab server: "Local desktop:
`GITLAB_PERSONAL_ACCESS_TOKEN` (or `GITLAB_PERSONAL_ACCESS_TOKEN_FILE`) is the simplest"
([GitLab MCP server][srv-gitlab]).

Other servers take a whole configuration or credential file. MongoDB's can "Store configuration in
a JSON file and load it using the `MDB_MCP_CONFIG` environment variable", with `chmod 600` on the
file; it still says "We strongly recommend using environment variables for sensitive
configuration", and warns that "Command-line arguments are visible in process listings"
([MongoDB MCP server][srv-mdb]).
The Kubernetes server lists among its sensitive values "Path to a kubeconfig, which usually contains
tokens or client keys"; left empty, "client-go uses in-cluster config or `$KUBECONFIG` /
`~/.kube/config`" ([Kubernetes MCP server][srv-k8s]). AWS's API server: "If
`AWS_API_MCP_PROFILE_NAME` is not specified, the system follows boto3's default credential selection
order" ([AWS API MCP server][srv-aws]), which lists the shared credentials file after the
environment variables ([boto3 credentials][boto3-creds]).

**Runners.** Docker's MCP catalog pairs each secret a server declares with a variable, as in the
GitHub entry, `name: github.personal_access_token` with `env: GITHUB_PERSONAL_ACCESS_TOKEN`
([Docker catalog entry][docker-mcp-github]); its contributing guide shows every secret with an
`env` field, and the catalog is run with `docker mcp gateway run`
([Docker catalog guide][docker-mcp-contrib]). ToolHive gives a stored secret to a server it runs as
a container with `thv run --secret <SECRET_NAME>,target=<ENV_VAR_NAME>`: "The secret value is
injected into the container as an environment variable" ([ToolHive secrets][toolhive]).

**Files elsewhere.** AWS's EKS best practices say "Use volume mounts instead of environment
variables The values of environment variables can unintentionally appear in logs. Secrets mounted
as volumes are instantiated as tmpfs volumes (a RAM backed file system) that are automatically
removed from the node when the pod is deleted" ([EKS best practices][eks-bp]). "Compose delivers
each secret by bind-mounting a single file into the container", under `/run/secrets/<secret_name>`
([Compose how-to][compose-howto]). In Swarm "the decrypted secret is mounted into the container in
an in-memory filesystem", but "Microsoft Windows has no built-in driver for managing RAM disks, so
within running Windows containers, secrets are persisted in clear text to the container's root disk.
However, the secrets are explicitly removed when a container stops" ([Swarm secrets][swarm]).
systemd, which starts processes, gives each service its credentials as files: "Service credentials
are accessible to service code as regular files, the path to access them is derived from the
environment variable $CREDENTIALS_DIRECTORY". "Access to credentials is restricted to the service's
user. Unlike environment variables the credential data is not propagated down the process tree."
They "are acquired at the moment of service activation, and released on service deactivation. They
are immutable during the service runtime", and "are placed in non-swappable memory. (If permissions
allow it, via ramfs.)" The page gives three problems with environment variables: they are
"inherited down the process tree, have size limitations, and issues with binary data"
([systemd credentials][systemd-cred]). The MCP registry's `server.json` can describe an input whose
value is a path: `format` takes `filepath`, "which should be interpreted as a file on the user's
filesystem", a separate field from `isSecret` ([MCP registry schema][mcp-registry]).

## A list of readers on the secret

Third round. The sections above describe grants made to the reader's identity. This one asks
whether a platform also writes on the secret itself who may read it, and what follows where it
does.

**AWS: a resource-based policy.** A Secrets Manager secret may carry a policy of its own: "In a
resource-based policy, you specify who can access the secret and the actions they can perform on
the secret." AWS gives two uses: to "Grant access to a single secret to multiple users and roles"
and to "Grant access to users or roles in other AWS accounts". Such a policy can cover any action
of Secrets Manager and can hold Deny statements: the page's examples for VPC endpoints are a Deny
with the principal `*`. AWS states no preference between the two places: "To grant permission to
retrieve secret values, you can attach policies to secrets or identities", and it refers the reader
to IAM's general page to choose ([resource policies][sm-rp]). The check found no preference on the
other Secrets Manager pages it opened either; it did not open every page of the guide.

Within one account the two places are combined as a union. "AWS evaluates all the permissions
granted by the policies for at least one Allow within the same account. An explicit deny in any of
the policies overrides the allow", and "If either the identity-based policy or the resource-based
policy within the same account allows the request and the other doesn't, the request is still
allowed" ([policy evaluation][iam-eval]). Secrets Manager "evaluates the resource-based policy
attached to the secret and all identity-based policies" ([examining policies][sm-examine]). So
within one account a grant on the secret is a second way in. How far it reaches depends on whom it
names: "If a resource-based policy grants permission directly to the IAM user or the session
principal that is making the request, then an implicit deny in an identity-based policy, a
permissions boundary, or a session policy does not impact the final decision", while
"Resource-based policies that grant permissions to an IAM role ARN are limited by an implicit deny
in a permissions boundary or session policy" ([deny and allow][iam-denyallow]). An explicit deny in
any policy still wins, and an organization's control policies still apply. The page is about IAM in
general, and warns that some services differ.

Across accounts the rule is an intersection: "you must allow access both in a resource policy and
in an identity policy." There is a third gate. "You must also allow the identity to use the KMS key
that the secret is encrypted with. This is because you can't use the AWS managed key
(aws/secretsmanager) for cross-account access. Instead, you must encrypt your secret with a KMS key
that you create, and then attach a key policy to it", and "To read the secret value, the principal
needs kms:Decrypt" ([cross-account access][sm-xacct]). The identity policy in the caller's account
must allow `kms:Decrypt` on the key as well. An account that a resource policy names passes the
access on to its own principals by identity policies, "up to the maximum permissions that are
allowed by the resource-owning account": "The trusted AWS account can delegate access only to the
extent that it has been granted access; it cannot delegate more access than the account itself has
been granted" ([cross-account resource access][iam-xacct]). That page's worked example is S3, not
Secrets Manager.

AWS guards the list on the secret. "Resource-based policies granting
secretsmanager:PutResourcePolicy permission gives principals, even those in other accounts, the
ability to modify your resource-based policies. This permission lets principals escalate existing
permissions like obtaining full administrative access to secrets"
([cross-account access][sm-xacct]). `PutResourcePolicy` takes an optional `BlockPublicPolicy`,
which says "whether to block resource-based policies that allow broad access to the secret, for
example those that use a wildcard for the principal"; "By default, public policies aren't
blocked". The guard covers only
the policy on the secret: AWS says to inspect separately the identity-based policies of associated
principals and the resource-based policies of associated resources, "for example, AWS KMS keys"
([PutResourcePolicy][sm-putrp]). `ValidateResourcePolicy` "performs three checks": a call to
Zelkova, "an automated reasoning engine, to ensure your resource policy does not allow broad access
to your secret"; a check of syntax; and a check that the policy "does not lock out a caller"
([ValidateResourcePolicy][sm-valrp]). "When you attach a resource-based policy to a secret in the
console", Secrets Manager runs them ([resource policies][sm-rp]).

**Kubernetes: no list on the Secret.** Who may read a Secret is kept outside it. A rule can be
limited to named objects with `resourceNames`: "When specified, requests can be restricted to
individual instances of a resource". A separate RoleBinding "holds a list of subjects (users,
groups, or service accounts), and a reference to the role being granted", and "Permissions are
purely additive" ([RBAC][k8s-rbac]). Those rules are not the whole set of readers: whoever can
create a Pod that uses the Secret, or list Secrets, can read it, as
[the Kubernetes Secret](#the-kubernetes-secret) says. The Node authorizer states a kubelet's right
as a relationship: it may read "secrets, configmaps, persistent volume claims and persistent volumes
related to pods bound to the kubelet's node" ([Node authorization][k8s-node]).

**GCP: an allow policy on each secret.** `setIamPolicy` "Sets the access control policy on the
specified secret. Replaces any existing policy", and "Permissions on SecretVersions are enforced
according to the policy set on the associated Secret" ([setIamPolicy][gsm-setiam]). Google
recommends granting there: "Always apply permissions at the lowest level in the resource
hierarchy", and "If a principal only needs to access a single secret's value, don't grant that
principal the ability to access all secrets" ([GCP access control][gsm-iam]).

**Azure: a role assignment, which may be scoped to one secret.** A role can be assigned on one
secret, from its "Access control (IAM) tab", with a scope that ends in `/secrets/<name>`. Microsoft
advises against it: "Our recommendation is to use a vault per application per environment
(Development, Pre-Production, and Production) with roles assigned at the key vault scope. Assigning
roles on individual keys, secrets and certificates is not recommended." The exceptions are where
"Individual secrets require individual user access" and where "Individual secrets must be shared
between multiple applications". Its FAQ says that object-scope assignments do not isolate
application teams, because "any administrative operations like network access control, monitoring,
and objects management require vault level permissions" ([Key Vault RBAC][akv-rbac]).

## What Kubernetes gives every container

Third round. What a container receives that its Pod did not ask for, and how Kubernetes withholds
it.

**An identity and its token.** "If you do not specify a ServiceAccount when you create a Pod,
Kubernetes automatically assigns the ServiceAccount named default in that namespace"
([service account configuration][k8s-sa-config]). The ServiceAccount admission controller, "active
(and it is by default on most distributions)", mutates each new Pod, "adding an extra volume that
contains a token for API access", and mounts it in each container at
`/var/run/secrets/kubernetes.io/serviceaccount`, skipping a container that already mounts that
path. The kubelet fetches the token with the TokenRequest API; it "expires either when the pod is
deleted or after a defined lifespan (by default, that is 1 hour)", the kubelet refreshes it before
then, and "The token is bound to the specific Pod and has the kube-apiserver as its audience". The
page calls such tokens "a lower risk to information security" because they expire. The volume also
carries a bundle of certificate authorities and the namespace
([service account administration][k8s-sa-admin]). Under RBAC, "The default service accounts in each
namespace get no permissions by default other than the default API discovery permissions"
([service accounts][k8s-sa]). A Pod that names another ServiceAccount gets that account's token,
with whatever that account was granted.

**Withholding the token.** It can be withheld for every Pod that runs as a ServiceAccount, by
`automountServiceAccountToken: false` on the ServiceAccount, or for one Pod. "If both the
ServiceAccount and the Pod's .spec specify a value for automountServiceAccountToken, the Pod spec
takes precedence" ([service account configuration][k8s-sa-config]). So the token is withheld where
the Pod sets false, or where the Pod leaves the field out and the ServiceAccount sets false; the
admission controller's page says it more loosely, "Provided that neither the ServiceAccount's
automountServiceAccountToken field nor the Pod's automountServiceAccountToken field is set to
false" ([service account administration][k8s-sa-admin]), and the check confirmed the Pod's
precedence in the API reference and in the plugin's source. The security checklist says "Service
account tokens should not be mounted into pods that do not require them", set "either within the
service account to apply throughout the namespace or specifically for a pod"
([security checklist][k8s-checklist]). The RBAC good practices, under hardening, say "Kubernetes
defaults to providing access which may not be required in every cluster", "Avoid the default
auto-mounting of service account tokens by setting automountServiceAccountToken: false", and
"workloads which require service account tokens can still mount them"
([RBAC good practices][k8s-rbac-gp]).

**Variables for every Service.** "A list of all services that were running when a Container was
created is available to that Container as environment variables. This list is limited to services
within the same namespace as the new Container's Pod and Kubernetes control plane services", in
forms such as `FOO_SERVICE_HOST` and `FOO_SERVICE_PORT` ([container environment][k8s-cont-env]).
Kubernetes "also supports and provides variables that are compatible with Docker Engine's "legacy
container links" feature" ([Service][k8s-service]). The Pod field `enableServiceLinks` turns this
off; it "indicates whether information about services should be injected into pod's environment
variables, matching the syntax of Docker links. Optional: Defaults to true" ([Pod API][k8s-pod]).
The ServiceAccount has no such field: its fields are `apiVersion`, `automountServiceAccountToken`,
`imagePullSecrets`, `kind`, `metadata` and `secrets` ([ServiceAccount API][k8s-sa-api]). The reasons
given for turning it off are "possible clashing with expected program ones, too many variables to
process, only using DNS, etc" ([connecting applications][k8s-connect]). The proposal that added the
field, KEP-2360, says of the old behaviour "There is no way to disable this", lists "Accidental
coupling.", images that "explicitly fail at startup time if certain service links are defined", and
a "Performance penalty in starting up pods for namespaces with many services", and says "DNS is to
be preferred over service links" ([KEP-2360][kep-2360]).

One set stays when the field is false: "only the kubernetes service in the
kl.masterServiceNamespace should be injected. The latter is needed in order to preserve Kubernetes
variables such as KUBERNETES_SERVICE_HOST since a lot of code depends on it" ([KEP-2360][kep-2360]).
A Kubernetes contributor confirms it in issue 121787: "even with `enableServiceLinks: false` the
environment variables for the "kubernetes" service are defined" ([issue 121787][k8s-121787]). Code
in a Pod finds the API server through `KUBERNETES_SERVICE_HOST` and
`KUBERNETES_SERVICE_PORT_HTTPS`, and reads the token at
`/var/run/secrets/kubernetes.io/serviceaccount/token` ([API from a Pod][k8s-api-pod]).

**Why the defaults stay.** Maintainers declined to make `enableServiceLinks` false by default while
agreeing the behaviour is bad: "This one is hard. Changing defaults is a breaking change"; "I'm not
saying I disagree. This is bad. What I am saying is that changing it would be a breaking change";
"Doing an optional admission controller is the only non-breaking way to do this"; and "I dislike
this feature as much as anyone (probably more), but we don't break users". The issue was closed with
"If we ever do a core/v2 API we can change the default to false"; the same maintainer called a new
API version "The *real* answer", and pointed to mutating admission policies and webhooks, which a
cluster's administrators can use already ([issue 121787][k8s-121787]).

**Files over variables.** The security checklist says "Pods needing secrets should have these
automatically mounted through volumes, preferably stored in memory like with the emptyDir.medium
option", and "the environment variable method might be more prone to leakage due to crash dumps in
logs and the non-confidential nature of environment variable in Linux, as opposed to the permission
mechanism on files" ([security checklist][k8s-checklist]).

## An Agent's own model credential

Third round. What the model providers offer in place of a key that never expires, and how the tools
a developer runs find it.

**Anthropic's workload identity federation.** A workload "presents a signed JWT from your identity
provider", and the exchange "returns a short-lived Anthropic access token bound to a service
account in your organization. There are no static secrets to mint, store in CI, rotate, or leak"
([Anthropic federation][anth-wif]). "The SDK reads these variables to perform a federated token
exchange with no constructor arguments": a rule, an organization, a service account and a
workspace, each by ID, and either `ANTHROPIC_IDENTITY_TOKEN_FILE` or `ANTHROPIC_IDENTITY_TOKEN`,
which holds "The literal JWT as a string". Of the file: "The SDK re-reads this file on every
exchange so that projected tokens that rotate on disk are always current"
([federation reference][anth-wif-ref]). The token lives "the lesser of (a) the rule's
`token_lifetime_seconds` (default 3,600 seconds) and (b) twice the remaining lifetime of the IdP JWT
you presented. The result is never less than 60 seconds" ([Anthropic federation][anth-wif]); a
rule's lifetime may be set from 60 to 86,400 seconds ([federation reference][anth-wif-ref]). By
default an identity JWT is used once: "the SDK re-reads the file on every exchange, so the file must
hold a new token before each refresh. A refresh that re-reads an unrotated token, or a restarted
process that re-presents a token it already exchanged, is rejected the same way". The check on a
JWT's ID can be turned off for an issuer "as a last resort" ([Anthropic federation][anth-wif]).

A token can be limited to inference. The scope `workspace:inference` grants "Messages (including
streaming and token counting), Models, and the OpenAI-compatible chat endpoint. Use this for
workloads that only need to call Claude and never need to manage Files, Skills, or other
resources"; "A federation rule's `oauth_scope` is a ceiling"; "Effective permissions are the
intersection of the rule's scope and the service account's role"; and "A request to an endpoint
outside the token's scope returns HTTP 403" ([federation reference][anth-wif-ref]). An issuer need
not be public: `inline` mode is "to upload the key set for issuers that are not reachable from the
public internet (for example, a private Kubernetes cluster)", and "For any other
standards-compliant provider (such as SPIFFE or Okta), select **Custom OIDC**". In `inline` mode
"there is no automatic key refresh"; JWTs must use asymmetric algorithms, carry `kid`, `sub`, `iat`
and `exp`, and stay within the issuer's maximum lifetime, "1 hour by default"
([Anthropic federation][anth-wif]).

The SDK's order of precedence is: a constructor argument; `ANTHROPIC_API_KEY` or
`ANTHROPIC_AUTH_TOKEN`, which "Shadows federation entirely"; `ANTHROPIC_PROFILE`; the federation
variables; the active profile. "A variable that is set to an empty string can still occupy its slot
in the credential precedence chain. Unset unused credential variables rather than blanking them"
([federation reference][anth-wif-ref]); "a leftover key in the environment silently shadows
federation" ([Anthropic federation][anth-wif]). A profile is two files: `configs/<profile>.json`,
"Non-secret. Safe to commit or bake into an image", and `credentials/<profile>.json`, which holds
"the cached `access_token`, `expires_at`, and (for interactive login) `refresh_token`" and is
"Secret. Written by the SDK with mode `0600`". "Claude Code and the Claude Agent SDK honor this same
resolution order", which the check read as the order in which the active profile's name is found
([federation reference][anth-wif-ref]).

**Claude Code.** Its order is: a cloud provider, where `CLAUDE_CODE_USE_BEDROCK`,
`CLAUDE_CODE_USE_VERTEX` or `CLAUDE_CODE_USE_FOUNDRY` is set; `ANTHROPIC_AUTH_TOKEN`;
`ANTHROPIC_API_KEY`; `apiKeyHelper`, "Use this for dynamic or rotating credentials, such as
short-lived tokens fetched from a vault"; `CLAUDE_CODE_OAUTH_TOKEN`; Anthropic profile and
federation credentials; and the subscription login of `/login`. "A profile that `ant auth login`
wrote ranks here only when you name it in `ANTHROPIC_PROFILE`; otherwise it ranks below `/login`."
The setup-token is "a one-year OAuth token", and "It can only make model requests"; it needs a Pro,
Max, Ultra, Team or Enterprise plan ([Claude Code authentication][cc-authn]). On Bedrock, "When
`AWS_BEARER_TOKEN_BEDROCK` is set, Claude Code authenticates with the key and doesn't resolve the
credential chain, even if other AWS credentials are present". Credentials from the chain are reused
"until five minutes before they expire, or for one hour when they carry no expiration", a cache
that covers every method "except an Amazon Bedrock API key, which doesn't use the provider chain".
`awsCredentialExport` "runs at session start and on each credential reload", and `awsAuthRefresh`
"runs only when Claude Code detects that your AWS credentials are expired"
([Claude Code on Bedrock][cc-bedrock]). On Google Cloud, "Claude Code uses standard Google Cloud
authentication"; it "supports X.509 certificate-based Workload Identity Federation through the same
Application Default Credentials chain", with `GOOGLE_APPLICATION_CREDENTIALS` naming the
configuration file; and "When Claude Code detects that your GCP credentials are expired or cannot
be loaded, it runs the configured command to obtain new credentials before retrying the request"
([Claude Code on Vertex][cc-vertex]).

**Amazon Bedrock.** A short-term API key "Lasts up to 12 hours (or the duration of your session,
whichever is shorter). Inherits permissions from the IAM principal used to generate it. Recommended
for production use." A long-term key "Creates an IAM user with attached policies. Recommended only
for exploration." Either is set as `AWS_BEARER_TOKEN_BEDROCK` or passed in the `Authorization`
header, and "For long-running applications, call the token generator before each request. It
returns a cached token if still valid or generates a new one automatically"
([Bedrock API keys][bedrock-keys]). The Anthropic SDK's Bedrock client takes
`AWS_BEARER_TOKEN_BEDROCK`, "which the SDK detects automatically when resolving credentials from the
environment", or keys, or "the default AWS credential providers", and accepts temporary credentials
with a session token ([Claude on Bedrock][anth-bedrock]).

**Google.** A configuration file for workload identity federation "doesn't contain a private key and
doesn't need to be kept confidential". Client libraries take the outside credential from a file, a
URL, which they query "whenever they need a new credential", or an executable, which they "launch
... whenever they need a new credential". Of a file: "You must periodically update the file so that
it always contains a valid credential" ([GCP executable credentials][gcp-wif]).

**OpenAI and Codex.** OpenAI's federation exchanges "The externally issued OIDC JWT or SPIFFE
JWT-SVID". "Access tokens expire after at most one hour", "A JWT exchange token never outlives its
external subject token", and "Token exchange doesn't return a refresh token"
([OpenAI federation reference][oai-wif-ref]); the guide says to renew by exchanging a valid outside
token again. For Codex, "Codex workload identity federation is in beta and must be enabled for your
workspace"; "`OPENAI_IDENTITY_TOKEN_FILE` contains an absolute path to a current token"; and "The
presence of either variable selects WIF ahead of API keys, access tokens, and stored logins"
([OpenAI federation guide][oai-wif]). For a custom provider, Codex runs a command that fetches a
bearer token: "The auth command receives no `stdin` and must print the token to stdout". It
"refreshes proactively at `refresh_interval_ms`", or only after an authentication retry when that is
0, and the page says not to combine the command with `env_key`. For Bedrock, "If you omit `profile`,
Codex uses the standard AWS credential chain" ([Codex advanced configuration][codex-config]).

## Processes of the same user on Linux and macOS

Third round. Who can read the environment of a process that another process started, and who can
read a file that only its owner may read.

**Linux: the environment.** Of `/proc/<pid>/environ`: "Permission to access this file is governed by
a ptrace access mode PTRACE_MODE_READ_FSCREDS check"; it holds "the initial environment that was set
when the currently executing program was started via execve(2)", and "If, after an execve(2), the
process modifies its environment (e.g., by calling functions such as putenv(3) or modifying the
environ(7) variable directly), this file will not reflect those changes"
([environ file][linux-environ]). Moving what the file refers to needs `CAP_SYS_RESOURCE`. `ps`
"works by reading the virtual files in /proc", and its `e` shows the environment
([ps][linux-ps]). ptrace(2) lists "reading /proc/pid/auxv, /proc/pid/environ, or /proc/pid/stat"
among the READ-mode checks. The check denies access unless "The real, effective, and saved-set user
IDs of the target match the caller's user ID" and "the real, effective, and saved-set group IDs of
the target match the caller's group ID", or the caller has `CAP_SYS_PTRACE` in the target's user
namespace. It denies access where the target's "dumpable" attribute is other than 1 and the caller
lacks that capability. And the default security module then denies it unless "the caller's
capabilities are a superset of the target process's _permitted_ capabilities", or the caller has
that capability ([ptrace][linux-ptrace]; re-opened). So a process of the same user, with the same
groups, reads the environment of a process that is dumpable and holds no capability the reader
lacks; a process that has made itself non-dumpable is out of its reach.

Yama limits "Any operation that performs a ptrace access mode PTRACE_MODE_ATTACH check", and its
default value, 1, restricts only those; the environment's check is of READ mode
([ptrace][linux-ptrace]). Memory is different: `/proc/<pid>/mem` "is governed by a ptrace access
mode PTRACE_MODE_ATTACH_FSCREDS check" ([mem file][linux-mem]). Under Yama's default, "the target
process must be a descendant of the caller" for such a check, so a process of the same user reads
another's memory only in the cases Yama allows, such as being its ancestor, while it reads the
environment on the terms above. With Yama at 0 there is no such difference
([ptrace][linux-ptrace]). The option `hidepid` of `/proc` says "Users may not access files and
subdirectories inside any /proc/pid directories but their own" ([proc][linux-proc]): it guards
against other users only.

**Linux: a file.** "The first group of three is used when the effective user ID of the calling
process equals the owner ID of the file", and "Linux uses the fsuid instead of the effective user ID
in permission checks"; a process also needs search permission on each directory of the path
([path resolution][linux-path]; re-opened). Read together, the two checks mean that a file only its
owner may read, in a directory its owner may search, is open to every process of that user,
whatever its groups and whether or not the process that uses the file is dumpable: every process
that can read the environment, and others besides.

**Linux: a file in memory.** `memfd_create` makes a file that, "unlike a regular file, it lives in
RAM and has a volatile backing storage"; its descriptor is inherited across `fork` and kept across
`execve` unless it is closed on exec, and no privilege is needed except for huge pages. Such files
"have the same semantics as other anonymous memory allocations" ([memfd_create][linux-memfd]), so,
like other memory, they are not promised to stay out of swap. Another process reaches the
descriptors of a process through `/proc/<pid>/fd`, where "Permission to dereference or read
(readlink(2)) the symbolic links in this directory is governed by a ptrace access mode
PTRACE_MODE_READ_FSCREDS check" ([fd directory][linux-fd]), the environment's own check;
`memfd_create(2)` itself describes a second process opening "/proc/pid/fd/fd".

**macOS: the environment.** Apple's `ps` reads the arguments and environment of a process with the
sysctl `KERN_PROCARGS2` ([ps source][mac-ps]). In XNU that sysctl fails with `EINVAL` unless the
caller's effective user ID is the target's or the caller is the superuser
([kern_sysctl.c][xnu-sysctl]). It returns the environment unless the target is restricted. The
source says: "Allow reading environment variables if any of the following are true: - kernel is
DEVELOPMENT || DEBUG - target process is same as current_proc() - target process is not
cs_restricted - SIP is off - caller has an entitlement" ([kern_sysctl.c][xnu-sysctl]). The same
block is in the tag `xnu-7195.141.2` and is absent from `xnu-6153.141.1`; which macOS releases
those tags are was not verified. `cs_restricted` tests the code-signing flag `CS_RESTRICT`, a
separate bit from the hardened runtime's `CS_RUNTIME` ([kern_cs.c][xnu-cs]), so signing a program
with the hardened runtime does not by itself hide its environment from processes of the same user.

**macOS: a file in memory.** Apple's manual for `hdiutil` documents "Creating a RAM-backed device
and filesystem" with `hdiutil attach -nomount ram://N`, then `newfs_hfs`, then `mount -t hfs` on a
directory, and the example uses no `sudo` ([hdiutil][mac-hdiutil]). An example without `sudo` is
not a statement that an ordinary user can do it; whether one can is under
[Not established](#not-established).

## Delivering to an EC2 instance

Third round. What AWS says of secrets in an instance's user data and launch templates, and how an
application on an instance reads a stored secret.

**User data.** The page on instance metadata says that instance metadata and user data are "not
protected by authentication or cryptographic methods", that "Anyone who has direct access to the
instance, and potentially any software running on the instance, can view its metadata", and
"Therefore, you should not store sensitive data, such as passwords or long-lived encryption keys,
as user data" ([instance metadata][ec2-metadata]). On Linux a user data script "is copied to and run
from /var/lib/cloud/instances/instance-id/. The script is not deleted after it is run"; AWS asks
that it be deleted before an AMI is made, "Otherwise, the script will exist in this directory on any
instance launched from the AMI". The user data attribute itself "is not included in the AMI". The
log of cloud-init "captures console output", so a value lands there only if the script prints it.
"If you use an AWS API, including the AWS CLI, in a user data script, you must use an instance
profile when launching the instance. An instance profile provides the appropriate AWS credentials
required by the user data script to issue the API call" ([user data][ec2-userdata]).

**Launch templates.** "Launch templates are immutable; after you create a launch template, you
can't modify it. Instead, you can create a new version of the launch template that includes any
changes you require", and versions no longer needed can be deleted
([launch template versions][ec2-lt-versions]). "Principals who list and view launch templates should
also have the ec2:DescribeLaunchTemplateVersions permission to retrieve the entire set of attributes
that make up the launch templates", and "Because Describe actions do not support resource-level
permissions, you must specify them without conditions and the value of the resource element in the
policy must be "*"" ([launch template permissions][ec2-lt-perms]). Those attributes include
`userData` ([launch template data][ec2-lt-data]). No AWS page that the check opened warns against
secrets in a launch template by name. A third-party scanner, Prowler, does: "Secrets in user data
can be read by identities able to view launch templates", "Past template versions retain leaked
values, complicating rotation and recovery", and it recommends: "Retrieve sensitive values at
runtime from AWS Secrets Manager or SSM Parameter Store SecureString using instance roles", deleting
the versions that hold a secret and rotating it ([Prowler][prowler-lt]).

**Reading at runtime.** "Only one role can be assigned to an Amazon EC2 instance at a time, and all
applications on the instance share the same role and permissions"; with ECS on EC2, tasks can have
roles apart from the instance's ([roles for EC2][iam-ec2]). The Well-Architected pillar's step for
long-lived secrets of third parties is to "integrate Secrets Manager into your code to retrieve
third-party secrets at runtime", and "For EC2 instances or containers, AWS provides example
client-side code for retrieving secrets from Secrets Manager in several popular programming
languages" ([Well-Architected][wa-secrets]). The Workload Credentials Provider runs on EC2, from a
package for Amazon Linux 2023 or an install script that writes its token to `/var/run/awssmatoken`,
readable by the group `aws-wcp-token`. AWS says "on Amazon EC2 the domain of trust for the AWS
Workload Credentials Provider would be the same as the domain of the credentials when using roles
for Amazon EC2", and "Security conscious applications that are not already using a provider-based
solution with the Secrets Manager credentials locked down to the application should consider using
the language-specific AWS SDKs or caching solutions" ([Workload Credentials Provider][sm-wcp]).

**Templates that launch instances.** CloudFormation says: "Rather than embedding sensitive
information in your CloudFormation templates, we recommend you use dynamic references in your stack
template", and "CloudFormation never stores the actual reference value". The page calls its
practices "helpful considerations rather than prescriptions"
([CloudFormation security][cfn-security]). A `secretsmanager` reference "can be used in all
resource properties", and the resolved value "may show up in the service whose resource it's being
used in", as the [first section](#aws-secrets-manager-and-parameter-store) says.

## Patterns across the systems

These are the points on which the sections above agree, with the exceptions they record.

1. **Reference by name and version, never the value.** Configuration names a secret and, where the
   store has versions, a version or a stage; the value stays in the store. Leaving the version out
   reads the current one: AWSCURRENT in AWS, the base identifier in Azure, `latest` in GCP, the
   most recent in ASCP. AWS recommends references with no version and keeps pinned ones for testing
   and rollback; Google recommends pinning a version number so that a change can be rolled back,
   and notes that `latest` is not strongly consistent. Kubernetes holds the value in its own
   object, which a workload names with a key. Parameter Store fails a read that names a version
   that does not exist, and never falls back; it keeps 100 versions, so a pinned one can disappear.
2. **One reference form, many stores.** AWS fronts Secrets Manager with Parameter Store,
   CloudFormation, ECS and `asm-exec`; Kubernetes has one Secret kind, with the CSI driver and ESO
   in front of outside stores; ESO keeps how a store is reached apart from what is fetched. A
   pass-through that keeps nothing applies the store's own policy.
3. **Identity instead of a stored secret, where possible.** AWS roles with STS credentials, EKS
   service-account roles and Pod Identity, Kubernetes TokenRequest, GKE Workload Identity and Azure
   managed identities replace a stored credential with one the platform issues for the workload,
   short-lived and rotated. A stored secret is the fallback, in AWS's words "remove, replace, and
   rotate". Kubernetes calls its own stored tokens legacy.
4. **Reading the value is its own right.** It is separate from seeing metadata (GCP Secret Accessor
   against Viewer, Azure Secrets User against Reader) and from managing the store (AWS's two
   roles). The right to grant is kept narrow: Azure's older model, where whoever writes the vault
   can grant itself reads, is the documented flaw; RBAC reserves granting to two roles, with a
   constrained delegate. Every operation that returns the value is a read: Parameter Store
   authorizes four of them separately, and decryption is a further gate across all four, while
   metadata stays in the clear.
5. **Values delivered at use, rather than copied.** Google says to read through the API "whenever
   possible"; AWS's ASCP project calls mounts a compatibility path; Argo CD cautions against
   secrets in rendered manifests. ECS ranks a file above a variable: it recommends, over
   environment variables, a sidecar that writes the secret to a shared volume, or the application
   reading an encrypted S3 object. A copy brings the gaps of where it is copied to, and can
   outlive its source.
6. **Environment variables do not change without a restart, and they leak.** ECS, Kubernetes, the
   CSI driver and Cloud Run each say a value in the environment is set when the container or
   instance starts, and is not updated while it runs. App Service fetches its references again
   every 24 hours and at each restart; Vault Agent restarts the process. AWS, Google, Docker and
   1Password each warn that the environment is visible to the application, its logs, its debugging
   endpoints, or other processes of the same user. Injection at start is still the accepted path
   for software that can read nothing else, and the MCP specification tells stdio servers to take
   credentials there.
7. **A workload whose secret cannot be read does not start.** Kubernetes, Cloud Run and Vault Agent
   fail closed. App Service is the exception: it passes the literal reference string on.
8. **Reads are audited without the value.** CloudTrail, the Kubernetes Metadata level and Key Vault
   record who read which secret and when. Vault records a keyed hash, which lets an operator match
   a known value, and refuses a request it cannot audit. GCP and Kubernetes leave the audit of
   reads off until the operator turns it on. A local cache in the path loses the per-read record.
9. **Base64 is not encryption, and a display mark is not secrecy.** Kubernetes's base64, its
   write-only `stringData`, CloudFormation's `NoEcho` and Terraform's `sensitive` all leave the
   value readable somewhere. Each system recommends a reference to a store over a marked literal.
10. **Whoever can run code with access can read the value.** In Kubernetes, whoever can create a
    Pod that uses a Secret can see it; on a host, whoever holds the local provider's token can read
    its cache; a privileged container reads every Secret on its node; an AI agent with shell access
    reads what it can call. Every system leaves the value's protection after delivery to the
    application.
11. **Caches trade freshness for cost.** AWS recommends caching and accepts staleness (300 seconds
    in its local provider and Lambda extension); stores are eventually consistent; Kubernetes
    serves volumes from a kubelet cache; ESO and the CSI driver poll. A forced read (`refreshNow`,
    a retry after a 401) is the remedy after a refusal.
12. **Revocation stops later reads; only an issuer recalls a copy.** AWS denies sessions issued
    before a time at the verifier; Kubernetes can only delete the object a token is bound to; Vault
    revokes leases. For a stored secret, rotating it in the store is how it is revoked.
13. **Where a secret may flow is restricted.** CloudFormation allows a SecureString reference only
    in listed properties; Terraform allows ephemeral values only in listed contexts; MCP forbids a
    server to pass a client's token on or to collect a credential through form-mode elicitation.
14. **A platform that keeps values must manage keys.** Kubernetes's record (unencrypted by default,
    a local key that fails against host compromise, a KMS plugin with its own credentials,
    rewriting every Secret to encrypt it) and Pulumi's key providers show the cost. Identifiers and
    metadata are kept in the clear by design, and must not hold anything secret.
15. **For AI agents, keep the value out of the model's context, and call it best-effort.** AWS
    resolves references into a child process and blocks direct reads; HashiCorp puts the credential
    in the MCP server; Claude Code can scrub credentials from what it starts but keeps its own key.
    AWS says plainly that this is "not a security boundary".

The second round adds five.

16. **On a developer's machine, credentials are found by convention.** An ordered chain stops at
    the first source that works. A variable that is not secret says where the credential is
    (`AWS_PROFILE`, `AWS_CONFIG_FILE`, `AWS_SHARED_CREDENTIALS_FILE`, `KUBECONFIG`, or `HOME` for
    the default paths), and a file or a cache under the home directory holds it. A person signs in
    once (`aws sso login`); clients use and refresh the cached token, and new ones fail when the
    session ends. AWS advises federation and temporary credentials over long-term keys.
17. **A command as a credential source, in each client's own format.** AWS's `credential_process`
    and Kubernetes's exec plugins share a shape: a command, JSON on standard output, a non-zero
    exit for failure, and an optional expiry after which the client runs it again; Kubernetes also
    runs it again after a 401. Each prints its own client's credential, and GCP and Docker use
    further formats. Whether a named command may run is decided apart from the file that names it:
    kuberc's allowlist, GCP's opt-in variable. A local credential endpoint is guarded by a token
    against forged, proxied and off-host requests, and AWS warns that any local process can reach
    one served on 127.0.0.1.
18. **The right to arrange a read is its own right, scoped by naming the grant.** AWS's
    `iam:PassRole` and GCP's `actAs` are apart from the right to read and from the right to make
    the workload; PassRole names the roles and the service that may receive them. Kubernetes lets
    a subject grant only what it holds, unless given `bind` or `escalate`, which its good practices
    call a way around the protection, and a superuser makes the first grants. AWS's boundaries
    cap what a delegate makes but let it make access it lacks. Azure's delegate role limits the
    roles it assigns, and limits who receives them only under an added condition. In Kubernetes
    the right to make a workload carries every service account in its namespace.
19. **A file beside the variable.** Some servers read a file named by a companion `_FILE`
    variable (Brave, Grafana, a GitLab server, and official Docker images), and some read a whole
    credential file named by a path variable (a kubeconfig, AWS's shared files). A supervisor that
    starts processes, systemd, puts each credential in a file in a directory for that one service,
    names the directory in a variable, and releases it when the service stops. The file is in
    memory where the system allows it; Swarm on Windows writes it to disk and removes it at stop.
    The pages read for the two MCP runners, Docker's catalog and ToolHive, show delivery by
    variable; whether Docker's MCP gateway ever mounts a secret as a file was not established.
20. **Ciphertext beside configuration moves the risk to the decryption key.** Sealed Secrets and
    SOPS keep encrypted values in Git. Whoever holds the key reads every value sealed with it,
    history in version control cannot be deleted, renewing the key does not rotate the values, and
    decrypting at apply leaves a plaintext copy. SOPS can also decrypt at use, into one child
    process.

The third round adds five.

21. **A list of readers on the secret is a second place to grant, and is guarded as one.** In AWS
    a policy on a secret can grant a read: within one account an Allow there or on the identity is
    enough, and a grant there directly to a user or a session is not limited by a permissions
    boundary; across accounts both must allow, and a customer managed key must too. AWS warns that
    the right to write the list can escalate to full control, offers a block on broad principals
    that is off by default, and validates a policy before it is attached; both checks cover only
    the policy on the secret. A grant across accounts can be passed on only as it is or narrower.
    GCP keeps an allow policy on each secret and recommends granting there. Kubernetes keeps who
    may read in Role and RoleBinding objects, outside the Secret, and Azure in role assignments,
    which it advises against scoping to one secret.
22. **What a platform gives every workload by default, its hardening guidance withholds.**
    Kubernetes mounts a token in every container and sets variables for every Service. Its
    checklist says not to mount the token where it is not needed; the opt-out is on the identity
    and on the workload, and the workload wins. The defaults survive because changing them would
    break existing users. The token handed out by default is short-lived, bound to one Pod and one
    audience, and of low privilege. When service links are turned off, the variables of the
    `kubernetes` Service stay, because a lot of code depends on them.
23. **A short-lived model credential needs refreshing, not only setting.** A static key, or a token
    that lasts a year, can be set once at start. A federated token at Anthropic or OpenAI lasts an
    hour or less, has no refresh token, and at Anthropic is single-use per identity token by
    default; a Bedrock short-term key lasts up to 12 hours. Each provider's answer for those is a
    variable that holds no secret and points at a file that something keeps fresh, or a command
    that the tool runs again. Orders of precedence let a leftover variable, even an empty one,
    shadow the intended source, so unused credential variables are unset rather than blanked.
24. **The operating-system user is the line both kernels draw.** On Linux a process's environment
    is open to processes of the same user and groups, where the process is dumpable and holds no
    capability the reader lacks, and to a privileged caller; on macOS to processes of the same user
    and to root, unless the process is restricted by its signature while SIP is on, which the
    hardened runtime is not. A file only its owner may read is open to every process of that user.
    The environment keeps its value from the start of the process, whatever the process does
    later; a file can be replaced or removed. Under Yama's default, memory is guarded better than
    the environment.
25. **Configuration names the store and the identity, never the value.** AWS says not to store
    passwords or long-lived keys in an instance's user data, which any software on the instance may
    read. Launch templates are immutable and versioned, and the right to read them cannot be
    limited to one template. An application reads its secret at runtime under the instance's role,
    which every application on the instance shares, and CloudFormation keeps a reference in place
    of the value.

## Not established

These were claimed by a researcher and not confirmed by the fact-checker, or confirmed only in
part. They are not used above, except in the corrected form noted.

- AWS: that request parameters "are logged". The page says they "might be logged". That a grant
  over a Parameter Store path covers every parameter under it whatever is denied: the second round
  settled it in corrected form, in the [first section](#aws-secrets-manager-and-parameter-store);
  a deny of the path action on every path above a parameter does exclude it. That CloudFormation
  promises not to persist a resolved value: the page says only that logs "should" not persist it.
  That the agent hook blocks every channel: it blocks four named ones, once a plugin is installed
  (re-opened). That with single-user rotation it is the new credential
  that may be refused: the page says "the rotated credentials" (re-opened). That the ECS page on
  task definition parameters describes the JSON key and the version of `valueFrom`: they are on the
  ECS page on secrets, which is cited for them. That the ECS page on secrets in environment
  variables names the task role: the separation of the two roles is on the page on the task
  execution role, which is cited instead. That custom resources are excluded from secure references
  for security reasons: the page gives no reason. That CloudFormation takes secrets only as dynamic
  references: it also takes `NoEcho` parameters.
- Kubernetes: that Secret types differ only in their validation, and that typed validation checks
  only the presence of keys (the Docker config type also parses its value as JSON). That humans
  should have no `get`, `list` or `watch` on Secrets: the page says to restrict them. That delivery
  is scoped to the consuming process: the page scopes it to the container. That Kubernetes cannot
  detect a secret written where none is marked: no page says so.
- The ecosystem: that the CSI driver always reads under the Pod's identity: tokens "may be
  forwarded". That ESO enforces separate owners for stores and secrets: it describes personas. That
  ESO recommends a narrowly scoped role per store: the page gives it as an example, not a best
  practice. That a store's own credential is called the "secret zero" problem by ESO. That Google
  offers syncing into Kubernetes Secrets only for applications that already read them. That the CSI
  driver is privileged because it reads every store: the page gives another reason. That SOPS's
  security page names age.
- GCP: that Secret Manager never rotates values itself. The rotation page describes only
  notifications (re-opened); managed rotation for Cloud SQL credentials, which the fact-checker
  found on another page, was not re-opened here.
- Tools: that Terraform never hands an ephemeral value back to the person running it: provisioners
  can receive one, and only the rule on root outputs is confirmed (re-opened). That changing a
  Kustomize-generated Secret rolls out the workloads that use it. That Claude Code never runs a
  project's credential helper before the workspace trust prompt: the page says so for interactive
  sessions only.
- Pages not opened or not readable, from which nothing is claimed: AWS pricing for Secrets Manager;
  the condition keys of `GetSecretValue`; the source of AWS's agent toolkit and of its local
  provider; the ASCP page on Pod Identity; the KMS page on Secrets Manager; the Kubernetes
  TokenRequest API reference and the KMS v2 design proposal; ESO's generators; the GCP page on
  destroying a version and its quotas; Azure's ABAC conditions and auto-rotation tutorials; Vault's
  on-behalf-of setup; Anthropic's workload identity federation, which the third round opened;
  Pulumi ESC.

The second round's claims that the check did not confirm, with what the page does say. None is
used above.

- Parameter Store: that no label reserved by AWS means "current". The labels page says a label is
  "a user-defined alias", that labels can be moved between versions and "can't begin with a number,
  "aws", or "ssm"", but says nothing of a reserved label, and its own example shows a user label
  named "Current". That the separate decrypt gate works only with a customer managed key: the page
  says AWS does not recommend the default key where access must differ, and shows how to deny the
  default key explicitly. That Parameter Store has no selector for a key of a JSON value: no page
  says so, though `GetParameter` documents only version and label selectors.
- AWS on a developer's machine: that IMDSv2's token is no separation between local processes; the
  page on IMDSv2's defences does not discuss it. That AWS recommends turning off the IMDS fallback
  on a laptop: the page says only that doing so "enhances security on untrusted networks where the
  Amazon EC2 Instance Metadata Service may be impersonated". That a kubeconfig's `tokenFile` is
  read again periodically in every case: the page says so only where `token` is also set. That an
  exec plugin's relative command path is taken from the kubeconfig's directory, and that the
  plugin gets the parent's environment: the claim cited one page for both. The kubeconfig
  reference says a plugin's `env` is "unioned with the host's environment"; the rule on relative
  paths is on the authentication page.
- Delegation: that a permissions boundary never limits a resource policy on a secret. It does not
  for a grant to an IAM user, but it does for a grant to a role, which is how workloads act.
- Ciphertext beside configuration: that the RSA-OAEP label wraps the session key; the session key
  is wrapped with RSA-OAEP under the controller's public key, and the scope is bound into the
  label. That a compromised SOPS key can keep access after a correct rotation: the page says so
  only where the steps are done in the wrong order. That SOPS's security page calls the data only
  as strong as the weakest master key: it says "the weakest cryptographic mechanism". That Flux
  offers only key Secrets and KMS roles for decryption: it also documents Vault and Azure Key
  Vault.
- Delivery to MCP servers: that every boto3-based AWS server honours
  `AWS_SHARED_CREDENTIALS_FILE`; only one that lets boto3 find its credentials does. How many of
  the stdio servers in use accept only a variable: the survey's count was not checked server by
  server, and only the servers named above are reported. Whether the Brave and GitLab servers read
  their file once or at each request. Whether macOS gives an unprivileged process a memory-backed
  file system. Whether Docker's MCP gateway ever mounts a secret as a file.
- Pages of the second round not opened: `git-credential`'s format; the Roles Anywhere helper's
  README; the AWS CLI's cache of assumed-role credentials; the kubelet's image credential provider
  API; the credential chains of Boto3, Go and Java; Systems Manager hybrid activations; the
  reference page for `iam:PassedToService`; ECS's `RegisterTaskDefinition`; the Kubernetes
  advisory CVE-2024-3177; Vault's rule on child-token policies; the Systems Manager service
  authorization reference and the history, tier, sharing and quota pages of Parameter Store; the
  AWS blog post on Sealed Secrets; Flux's global decryption settings.

The third round's claims that the check did not confirm, with what the page does say. None is used
above, except the two settled in corrected form from re-opened pages.

- A list of readers on the secret: that requiring `BlockPublicPolicy` is AWS's one explicit
  recommendation on resource policies. The best practices page does say "In identity policies that
  allow the action PutResourcePolicy, we recommend you use BlockPublicPolicy: true"
  ([best practices][sm-bp]), but other pages recommend more: the condition keys `aws:SourceArn` and
  `aws:SourceAccount` where a resource policy names an AWS service, least privilege, and care with
  conditions on IP addresses and VPC endpoints.
- What Kubernetes gives every container: that the Pod's name and namespace reach a container
  "only" through the "opt-in" downward API. The page says "The Pod name and namespace are available
  as environment variables through the downward API", and also that "The hostname of a Container
  is the name of the Pod in which the Container is running"; it does list a container's
  environment as the Pod's definition and the image's own variables, and nothing from the node
  ([container environment][k8s-cont-env]). That user Secrets are never inherited and reach a
  container only through its own mount or variable: the good practices say a Pod can reference a
  Secret "in a variety of ways, such as in a volume mount or as an environment variable", and the
  advice to keep a Secret from sibling containers that do not need it is on that page
  ([good practices][k8s-gp]); the checklist names another route, RBAC access to Secrets for the
  Pod's service account ([security checklist][k8s-checklist]).
- An Agent's own model credential: that every Anthropic API key expires. The page offers "a preset
  (3 hours, 1 day, 7 days, or 30 days), a custom duration, or **Never** for keys you store in a
  secrets manager and rotate yourself", and Never is unavailable only where the organization sets a
  maximum ([Anthropic authentication][anth-keys]). That Google access tokens used by Vertex AI lapse
  within an hour: the page says "By default, service account access tokens expire after one hour",
  allows longer lifetimes under an organization constraint, and does not mention Vertex AI
  ([token types][gcp-tokens]); the order in which Application Default Credentials look for a
  credential is on another page, not re-opened. That Codex's access tokens last at least a day and
  that it refreshes its stored login: the first is on the page cited ("The shortest custom validity
  window is one day", and scoped tokens "must expire") ([Codex access tokens][codex-tokens]), the
  second on Codex's authentication page, which was not re-opened here.
- Processes of the same user: that every process of the same user can read a started server's
  environment, and that a file only its owner may read is readable by the same set. Both are
  settled in corrected form above from the re-opened ptrace(2) and path_resolution(7): the
  environment's check also needs the same groups, a dumpable target and a reader whose capabilities
  include the target's, while the file needs only the owner's user ID and search permission on its
  directories. That `/run/user/$UID` keeps a file off the disk: pam_systemd says the directory "is
  either created or mounted as new "tmpfs" file system with quota applied" and is removed when the
  user's last session ends ([pam_systemd][pam-systemd]), but tmpfs "is able to swap unneeded pages
  out to swap space, if swap was enabled" ([tmpfs][linux-tmpfs]), and the directory exists only for
  a login through pam_systemd. That macOS guards memory better than the environment: Apple's page
  on the hardened runtime names "process memory space tampering" among what it prevents, and says
  nothing of environment variables ([hardened runtime][apple-hardened]). That an ordinary user can
  mount the RAM device that `hdiutil` gives: mount(2)'s entry for `EPERM` implies that a user who
  has "adequate ownership and permissions" on the device and the mount point may mount
  ([mount][mac-mount]), and XNU forces `nosuid` and `nodev` on such a mount
  ([vfs_syscalls.c][xnu-vfs]), but who owns the device that `hdiutil` makes, and what a system
  policy allows, were not established. Which macOS releases the XNU tags `xnu-7195.141.2` and
  `xnu-6153.141.1` are. Whether Apple's closed-source AMFI sets `CS_RESTRICT` on Apple's own
  programs.
- Observations on one machine, which are not sources: a researcher ran commands once on macOS 26.6.2
  with SIP on. `ps -E` showed the environment of a node binary signed with a Developer ID and the
  hardened runtime, and of a Python installed by Homebrew, both of the same user, and did not show
  that of `/bin/sleep`. `hdiutil attach -nomount ram://`, `newfs_hfs` and `mount -t hfs` worked as
  an ordinary user, and the mount kept a file's owner and its mode 0600. `mount_tmpfs` failed as an
  ordinary user with "Operation not permitted"; its manual says nothing of privilege. The RAM disk
  was unmounted and detached afterwards.
- Delivering to an EC2 instance: that anyone allowed to view an instance can see its user data. The
  sentence cited, "You can view the instance user data for any instance, and you can update the
  instance user data for a stopped instance", is about the instance's state, not about who may view
  it ([user data][ec2-userdata]). Reading user data is the attribute `userData` of
  `DescribeInstanceAttribute`, a call of its own, and the page on retrieving metadata says "You can
  access EC2 instance metadata from inside of the instance itself or from the EC2 console, API,
  SDKs, or the AWS CLI" ([retrieving metadata][ec2-retrieve]).
- Pages of the third round not opened: IAM's page on evaluation across accounts; the Secrets Manager
  page on attribute-based access; AWS Organizations' resource control policies; IAM Access
  Analyzer's validation; Azure Key Vault's ABAC conditions; GCP's IAM conditions for Secret
  Manager; KEP-2360's predecessor proposal and its review thread; the kubelet's source for service
  variables; Codex's page on workload identity (it returned 404); OpenAI's page on key permissions
  (it returned 403); OpenAI's page on X.509 federation; whether OpenAI's SDK caches and exchanges
  federated tokens again by itself; whether Claude Code itself, rather than the SDK, reads
  `ANTHROPIC_IDENTITY_TOKEN_FILE` again on the SDK's schedule; the Bedrock token generator's README;
  the pages on Vertex AI's express mode; AWS's pages on the lifetimes of STS credentials; whether
  the App Sandbox stops a reader of `KERN_PROCARGS2`; whether Landlock covers
  `/proc/<pid>/environ`; the Security Hub controls on launch templates; AWS's blog post on its
  secrets agent; the page on Secrets Manager's client-side cache for Rust.

## Sources

Every page was read on 2026-11-06. The labels are the ones the text above links.

| System | Pages |
| --- | --- |
| AWS Secrets Manager | [What's in a secret][sm-whats], [GetSecretValue][sm-get], [encryption][sm-enc], [IAM policies][sm-iam], [authentication and access][sm-access], [hardcoded credentials][sm-hardcoded], [rotation functions][sm-rot-lambda], [rotation strategies][sm-rot-strategy], [Workload Credentials Provider][sm-wcp], [AI coding agents][sm-agents], [CLI exposure][sm-cli], [deleting a secret][sm-delete], [troubleshooting][sm-trouble], [quotas][sm-limits], [data protection][sm-data], [CloudTrail entries][sm-trail] |
| AWS Parameter Store and CloudFormation | [What is a parameter][ps-param], [Parameter Store references][ps-ref], [CloudFormation reference][cfn-sm], [CloudFormation SecureString][cfn-ssm], [CloudFormation parameters][cfn-params] |
| AWS delivery | [ECS secrets][ecs-secrets], [task definition parameters][ecs-taskdef], [sensitive data][ecs-sensitive], [task execution role][ecs-exec-role], [task role][ecs-task-role], [ECS roles][ecs-roles], [IRSA][eks-irsa], [Pod Identity][eks-podid], [EKS secrets][eks-secrets], [ASCP][ascp-irsa], [ASCP examples][ascp-examples], [ASCP repository][ascp-repo], [EKS best practices][eks-bp], [Lambda environment][lambda-env], [Lambda encryption][lambda-envenc], [Lambda and Secrets Manager][lambda-sm], [instance credentials][ec2-imds], [temporary credentials][sts-temp], [revoking sessions][sts-revoke], [IAM best practices][iam-bp], [Well-Architected][wa-secrets], [shared AMIs][ec2-ami] |
| Kubernetes | [Secrets][k8s-secret], [Secret API][k8s-secret-api], [ConfigMap][k8s-configmap], [good practices][k8s-gp], [distribute credentials][k8s-distribute], [encrypting data][k8s-encrypt], [KMS provider][k8s-kms], [RBAC good practices][k8s-rbac-gp], [RBAC][k8s-rbac], [kubectl][k8s-kubectl], [Node authorization][k8s-node], [authentication][k8s-authn], [service accounts][k8s-sa], [service account administration][k8s-sa-admin], [service account configuration][k8s-sa-config], [credential provider][k8s-credprov], [auditing][k8s-audit], [Kustomize][k8s-kustomize] |
| Kubernetes ecosystem | [CSI concepts][csi-concepts], [CSI limitations][csi-limits], [CSI rotation][csi-rotation], [CSI sync][csi-sync], [GKE secrets][gke-wi-secrets], [ESO][eso], [ESO overview][eso-overview], [ESO security][eso-security], [ESO multi-tenancy][eso-tenancy], [ExternalSecret][eso-es], [ESO API][eso-spec], [ESO templating][eso-templating], [Sealed Secrets][sealed], [Flux and SOPS][flux-sops], [Argo CD][argo-secrets], [Vault process supervisor][vault-supervisor], [Vault on Kubernetes][vault-k8s-compare], [VSO CSI driver][vso-csi] |
| GCP | [GCP versions][gsm-add], [GCP access][gsm-access], [GCP best practices][gsm-bp], [Cloud Run secrets][run-secrets], [GCP access control][gsm-iam], [GCP rotation][gsm-rotation], [GCP audit logging][gsm-audit], [data access logs][gcp-audit-config], [GKE workload identity][gke-wi] |
| Azure | [Key Vault objects][akv-objects], [Key Vault secrets][akv-secrets], [Key Vault references][appsvc-kv], [RBAC and access policies][akv-rbac-vs], [Key Vault RBAC][akv-rbac], [managed identities][entra-mi], [Key Vault logging][akv-logging] |
| HashiCorp Vault | [KV v2][vault-kv2], [leases][vault-lease], [tokens][vault-tokens], [response wrapping][vault-wrap], [Vault audit][vault-audit], [Vault AWS auth][vault-aws], [secure introduction][vault-intro], [Vault injector][vault-injector], [Vault agent permissions][vault-obo], [Vault AI agent identity][vault-ai] |
| Configuration tools | [Terraform sensitive data][tf-sensitive], [Terraform ephemeral][tf-ephemeral], [Terraform output][tf-output], [Terraform write-only][tf-wo], [Pulumi secrets][pulumi], [Compose secrets][compose-secrets], [Compose services][compose-services], [Compose how-to][compose-howto], [Swarm secrets][swarm], [Helm][helm-adv], [1Password references][op-ref], [op run][op-run], [1Password environment][op-env], [Doppler][doppler], [direnv][direnv] |
| MCP and agent tooling | [MCP authorization][mcp-auth], [MCP security][mcp-sec], [MCP elicitation][mcp-elicit], [MCP security best practices][mcp-bp], [MCP registry schema][mcp-registry], [Claude Code IAM][cc-iam], [Claude Code settings][cc-settings], [Claude Code environment][cc-env], [Claude Code MCP][cc-mcp], [Anthropic SDK][anthropic-sdk], [Codex authentication][codex-auth] |
| Second round: Parameter Store | [GetParameter][ps-get], [versions][ps-versions], [retrieval authorization][ps-retrieval], [restricting access][ps-restrict], [restricting decryption][ps-decrypt], [KMS and Parameter Store][kms-ps], [parameter policies][ps-policies] |
| Second round: who may give a read | [PassRole][iam-passrole], [Lambda user permissions][lambda-user], [permissions boundaries][iam-boundaries], [attaching service accounts][gcp-attach], [built-in roles][az-roles], [delegating role assignments][az-delegate] |
| Second round: a developer's machine | [credential providers][aws-chain], [SDK for Rust][rust-chain], [file location][aws-files], [access keys][aws-static], [Identity Center][aws-sso], [process credentials][aws-process], [Roles Anywhere][ra-sdk], [credential helper][ra-helper], [IMDS configuration][imdsv2], [container credentials][aws-cont], [aws-config source][rust-ecs], [kubeconfig][k8s-kubeconfig], [kuberc][kuberc], [EKS kubeconfig][eks-kubeconfig], [GCP executable credentials][gcp-wif], [docker login][docker-login] |
| Second round: ciphertext beside configuration | [SOPS reference][sops-ref], [SOPS operations][sops-ops], [SOPS advanced][sops-adv], [SOPS age][sops-age] |
| Second round: MCP servers and runners | [GitHub MCP server][srv-github], [Brave MCP server][srv-brave], [Grafana MCP server][srv-grafana], [GitLab MCP server][srv-gitlab], [MongoDB MCP server][srv-mdb], [Kubernetes MCP server][srv-k8s], [AWS API MCP server][srv-aws], [boto3 credentials][boto3-creds], [Docker catalog entry][docker-mcp-github], [Docker catalog guide][docker-mcp-contrib], [ToolHive secrets][toolhive], [systemd credentials][systemd-cred] |
| Third round: a list of readers on the secret | [resource policies][sm-rp], [policy evaluation][iam-eval], [examining policies][sm-examine], [deny and allow][iam-denyallow], [cross-account access][sm-xacct], [cross-account resource access][iam-xacct], [PutResourcePolicy][sm-putrp], [ValidateResourcePolicy][sm-valrp], [best practices][sm-bp], [setIamPolicy][gsm-setiam] |
| Third round: what Kubernetes gives every container | [security checklist][k8s-checklist], [container environment][k8s-cont-env], [Service][k8s-service], [Pod API][k8s-pod], [ServiceAccount API][k8s-sa-api], [connecting applications][k8s-connect], [KEP-2360][kep-2360], [issue 121787][k8s-121787], [API from a Pod][k8s-api-pod] |
| Third round: an Agent's own model credential | [Anthropic federation][anth-wif], [federation reference][anth-wif-ref], [Anthropic authentication][anth-keys], [Claude Code authentication][cc-authn], [Claude Code on Bedrock][cc-bedrock], [Claude Code on Vertex][cc-vertex], [Bedrock API keys][bedrock-keys], [Claude on Bedrock][anth-bedrock], [token types][gcp-tokens], [OpenAI federation reference][oai-wif-ref], [OpenAI federation guide][oai-wif], [Codex advanced configuration][codex-config], [Codex access tokens][codex-tokens] |
| Third round: processes of the same user | [environ file][linux-environ], [ps][linux-ps], [ptrace][linux-ptrace], [mem file][linux-mem], [proc][linux-proc], [path resolution][linux-path], [memfd_create][linux-memfd], [fd directory][linux-fd], [pam_systemd][pam-systemd], [tmpfs][linux-tmpfs], [ps source][mac-ps], [kern_sysctl.c][xnu-sysctl], [kern_cs.c][xnu-cs], [hdiutil][mac-hdiutil], [mount][mac-mount], [vfs_syscalls.c][xnu-vfs], [hardened runtime][apple-hardened] |
| Third round: delivering to an EC2 instance | [instance metadata][ec2-metadata], [user data][ec2-userdata], [retrieving metadata][ec2-retrieve], [launch template versions][ec2-lt-versions], [launch template permissions][ec2-lt-perms], [launch template data][ec2-lt-data], [Prowler][prowler-lt], [roles for EC2][iam-ec2], [CloudFormation security][cfn-security] |

[sm-whats]: https://docs.aws.amazon.com/secretsmanager/latest/userguide/whats-in-a-secret.html
[sm-get]: https://docs.aws.amazon.com/secretsmanager/latest/apireference/API_GetSecretValue.html
[sm-enc]: https://docs.aws.amazon.com/secretsmanager/latest/userguide/security-encryption.html
[sm-iam]:
  https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access_iam-policies.html
[sm-access]: https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access.html
[sm-hardcoded]: https://docs.aws.amazon.com/secretsmanager/latest/userguide/hardcoded.html
[sm-rot-lambda]:
  https://docs.aws.amazon.com/secretsmanager/latest/userguide/rotate-secrets_lambda-functions.html
[sm-rot-strategy]:
  https://docs.aws.amazon.com/secretsmanager/latest/userguide/rotation-strategy.html
[sm-wcp]:
  https://docs.aws.amazon.com/secretsmanager/latest/userguide/workload-credentials-provider.html
[sm-agents]:
  https://docs.aws.amazon.com/secretsmanager/latest/userguide/retrieving-secrets-ai-agents.html
[sm-cli]:
  https://docs.aws.amazon.com/secretsmanager/latest/userguide/security_cli-exposure-risks.html
[sm-delete]: https://docs.aws.amazon.com/secretsmanager/latest/userguide/manage_delete-secret.html
[sm-trouble]: https://docs.aws.amazon.com/secretsmanager/latest/userguide/troubleshoot.html
[sm-limits]: https://docs.aws.amazon.com/secretsmanager/latest/userguide/reference_limits.html
[sm-data]: https://docs.aws.amazon.com/secretsmanager/latest/userguide/data-protection.html
[sm-trail]: https://docs.aws.amazon.com/secretsmanager/latest/userguide/cloudtrail_log_entries.html
[ps-param]: https://docs.aws.amazon.com/systems-manager/latest/userguide/what-is-a-parameter.html
[ps-ref]:
  https://docs.aws.amazon.com/systems-manager/latest/userguide/integration-ps-secretsmanager.html
[cfn-sm]:
  https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/dynamic-references-secretsmanager.html
[cfn-ssm]:
  https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/dynamic-references-ssm-secure-strings.html
[cfn-params]:
  https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/parameters-section-structure.html
[ecs-secrets]:
  https://docs.aws.amazon.com/AmazonECS/latest/developerguide/secrets-envvar-secrets-manager.html
[ecs-taskdef]:
  https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_definition_parameters.html
[ecs-sensitive]:
  https://docs.aws.amazon.com/AmazonECS/latest/developerguide/specifying-sensitive-data.html
[ecs-exec-role]:
  https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_execution_IAM_role.html
[ecs-task-role]: https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-iam-roles.html
[ecs-roles]: https://docs.aws.amazon.com/AmazonECS/latest/developerguide/security-iam-roles.html
[eks-irsa]: https://docs.aws.amazon.com/eks/latest/userguide/iam-roles-for-service-accounts.html
[eks-podid]: https://docs.aws.amazon.com/eks/latest/userguide/pod-id-how-it-works.html
[eks-secrets]: https://docs.aws.amazon.com/eks/latest/userguide/manage-secrets.html
[ascp-irsa]:
  https://docs.aws.amazon.com/secretsmanager/latest/userguide/integrating_csi_driver.html
[ascp-examples]: https://docs.aws.amazon.com/secretsmanager/latest/userguide/ascp-examples.html
[ascp-repo]: https://github.com/aws/secrets-store-csi-driver-provider-aws
[eks-bp]:
  https://docs.aws.amazon.com/eks/latest/best-practices/data-encryption-and-secrets-management.html
[lambda-env]: https://docs.aws.amazon.com/lambda/latest/dg/configuration-envvars.html
[lambda-envenc]: https://docs.aws.amazon.com/lambda/latest/dg/configuration-envvars-encryption.html
[lambda-sm]: https://docs.aws.amazon.com/lambda/latest/dg/with-secrets-manager.html
[ec2-imds]:
  https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-metadata-security-credentials.html
[sts-temp]: https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp.html
[sts-revoke]: https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use_revoke-sessions.html
[iam-bp]: https://docs.aws.amazon.com/IAM/latest/UserGuide/best-practices.html
[wa-secrets]:
  https://docs.aws.amazon.com/wellarchitected/latest/security-pillar/sec_identities_secrets.html
[ec2-ami]: https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/building-shared-amis.html
[k8s-secret]: https://kubernetes.io/docs/concepts/configuration/secret/
[k8s-secret-api]:
  https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/secret-v1/
[k8s-configmap]: https://kubernetes.io/docs/concepts/configuration/configmap/
[k8s-gp]: https://kubernetes.io/docs/concepts/security/secrets-good-practices/
[k8s-distribute]:
  https://kubernetes.io/docs/tasks/inject-data-application/distribute-credentials-secure/
[k8s-encrypt]: https://kubernetes.io/docs/tasks/administer-cluster/encrypt-data/
[k8s-kms]: https://kubernetes.io/docs/tasks/administer-cluster/kms-provider/
[k8s-rbac-gp]: https://kubernetes.io/docs/concepts/security/rbac-good-practices/
[k8s-rbac]: https://kubernetes.io/docs/reference/access-authn-authz/rbac/
[k8s-kubectl]: https://kubernetes.io/docs/tasks/configmap-secret/managing-secret-using-kubectl/
[k8s-node]: https://kubernetes.io/docs/reference/access-authn-authz/node/
[k8s-authn]: https://kubernetes.io/docs/reference/access-authn-authz/authentication/
[k8s-sa]: https://kubernetes.io/docs/concepts/security/service-accounts/
[k8s-sa-admin]: https://kubernetes.io/docs/reference/access-authn-authz/service-accounts-admin/
[k8s-sa-config]:
  https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/
[k8s-credprov]: https://kubernetes.io/docs/tasks/administer-cluster/kubelet-credential-provider/
[k8s-audit]: https://kubernetes.io/docs/tasks/debug/debug-cluster/audit/
[k8s-kustomize]: https://kubernetes.io/docs/tasks/configmap-secret/managing-secret-using-kustomize/
[csi-concepts]: https://secrets-store-csi-driver.sigs.k8s.io/concepts.html
[csi-limits]: https://secrets-store-csi-driver.sigs.k8s.io/known-limitations
[csi-rotation]: https://secrets-store-csi-driver.sigs.k8s.io/topics/secret-auto-rotation
[csi-sync]: https://secrets-store-csi-driver.sigs.k8s.io/topics/sync-as-kubernetes-secret
[gke-wi-secrets]:
  https://docs.cloud.google.com/kubernetes-engine/docs/tutorials/workload-identity-secrets
[eso]: https://external-secrets.io/latest/
[eso-overview]: https://external-secrets.io/latest/introduction/overview/
[eso-security]: https://external-secrets.io/latest/guides/security-best-practices/
[eso-tenancy]: https://external-secrets.io/latest/guides/multi-tenancy/
[eso-es]: https://external-secrets.io/latest/api/externalsecret/
[eso-spec]: https://external-secrets.io/latest/api/spec/
[eso-templating]: https://external-secrets.io/latest/guides/templating/
[sealed]: https://github.com/bitnami-labs/sealed-secrets
[flux-sops]: https://fluxcd.io/flux/guides/mozilla-sops/
[argo-secrets]: https://argo-cd.readthedocs.io/en/stable/operator-manual/secret-management/
[vault-supervisor]:
  https://developer.hashicorp.com/vault/docs/agent-and-proxy/agent/process-supervisor
[vault-k8s-compare]: https://developer.hashicorp.com/vault/docs/deploy/kubernetes/comparisons
[vso-csi]: https://developer.hashicorp.com/vault/docs/deploy/kubernetes/vso/csi
[gsm-add]: https://docs.cloud.google.com/secret-manager/docs/add-secret-version
[gsm-access]: https://docs.cloud.google.com/secret-manager/docs/access-secret-version
[gsm-bp]: https://docs.cloud.google.com/secret-manager/docs/best-practices
[run-secrets]: https://docs.cloud.google.com/run/docs/configuring/services/secrets
[gsm-iam]: https://docs.cloud.google.com/secret-manager/docs/access-control
[gsm-rotation]: https://docs.cloud.google.com/secret-manager/docs/secret-rotation
[gsm-audit]: https://docs.cloud.google.com/secret-manager/docs/audit-logging
[gcp-audit-config]: https://docs.cloud.google.com/logging/docs/audit/configure-data-access
[gke-wi]: https://docs.cloud.google.com/kubernetes-engine/docs/concepts/workload-identity
[akv-objects]:
  https://learn.microsoft.com/en-us/azure/key-vault/general/about-keys-secrets-certificates
[akv-secrets]: https://learn.microsoft.com/en-us/azure/key-vault/secrets/about-secrets
[appsvc-kv]: https://learn.microsoft.com/en-us/azure/app-service/app-service-key-vault-references
[akv-rbac-vs]: https://learn.microsoft.com/en-us/azure/key-vault/general/rbac-access-policy
[akv-rbac]: https://learn.microsoft.com/en-us/azure/key-vault/general/rbac-guide
[entra-mi]:
  https://learn.microsoft.com/en-us/entra/identity/managed-identities-azure-resources/overview
[akv-logging]: https://learn.microsoft.com/en-us/azure/key-vault/general/logging
[vault-kv2]: https://developer.hashicorp.com/vault/api-docs/secret/kv/kv-v2
[vault-lease]: https://developer.hashicorp.com/vault/docs/concepts/lease
[vault-tokens]: https://developer.hashicorp.com/vault/docs/concepts/tokens
[vault-wrap]: https://developer.hashicorp.com/vault/docs/concepts/response-wrapping
[vault-audit]: https://developer.hashicorp.com/vault/docs/audit
[vault-aws]: https://developer.hashicorp.com/vault/docs/auth/aws
[vault-intro]: https://developer.hashicorp.com/vault/tutorials/app-integration/secure-introduction
[vault-injector]: https://developer.hashicorp.com/vault/docs/deploy/kubernetes/injector
[vault-obo]:
  https://developer.hashicorp.com/vault/tutorials/enterprise/secure-agent-permissions-with-vault
[vault-ai]:
  https://developer.hashicorp.com/validated-patterns/vault/ai-agent-identity-with-hashicorp-vault
[tf-sensitive]: https://developer.hashicorp.com/terraform/language/manage-sensitive-data
[tf-ephemeral]: https://developer.hashicorp.com/terraform/language/block/ephemeral
[tf-output]: https://developer.hashicorp.com/terraform/language/block/output
[tf-wo]: https://developer.hashicorp.com/terraform/language/manage-sensitive-data/write-only
[pulumi]: https://www.pulumi.com/docs/iac/concepts/secrets/
[compose-secrets]: https://docs.docker.com/reference/compose-file/secrets/
[compose-services]: https://docs.docker.com/reference/compose-file/services/
[compose-howto]: https://docs.docker.com/compose/how-tos/use-secrets/
[swarm]: https://docs.docker.com/engine/swarm/secrets/
[helm-adv]: https://helm.sh/docs/topics/advanced/
[op-ref]: https://www.1password.dev/cli/secret-reference-syntax/
[op-run]: https://www.1password.dev/cli/reference/commands/run/
[op-env]: https://www.1password.dev/cli/secrets-environment-variables/
[doppler]: https://docs.doppler.com/docs/accessing-secrets
[direnv]: https://direnv.net/man/direnv.1.html
[mcp-auth]: https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization
[mcp-sec]:
  https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization/security-considerations
[mcp-elicit]: https://modelcontextprotocol.io/specification/2026-07-28/client/elicitation
[mcp-bp]:
  https://modelcontextprotocol.io/docs/2026-07-28/tutorials/security/security_best_practices
[mcp-registry]: https://static.modelcontextprotocol.io/schemas/2025-12-11/server.schema.json
[cc-iam]: https://code.claude.com/docs/en/iam
[cc-settings]: https://code.claude.com/docs/en/settings-reference
[cc-env]: https://code.claude.com/docs/en/env-vars
[cc-mcp]: https://code.claude.com/docs/en/mcp
[anthropic-sdk]: https://github.com/anthropics/anthropic-sdk-python
[codex-auth]: https://learn.chatgpt.com/docs/auth
[ps-get]:
  https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_GetParameter.html
[ps-versions]:
  https://docs.aws.amazon.com/systems-manager/latest/userguide/sysman-paramstore-versions.html
[ps-retrieval]:
  https://docs.aws.amazon.com/systems-manager/latest/userguide/ps-retrieval-authorization.html
[ps-restrict]:
  https://docs.aws.amazon.com/systems-manager/latest/userguide/ps-restrict-parameter-access.html
[ps-decrypt]:
  https://docs.aws.amazon.com/systems-manager/latest/userguide/ps-restrict-decryption.html
[kms-ps]: https://docs.aws.amazon.com/kms/latest/developerguide/services-parameter-store.html
[ps-policies]:
  https://docs.aws.amazon.com/systems-manager/latest/userguide/parameter-store-policies.html
[iam-passrole]: https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use_passrole.html
[lambda-user]: https://docs.aws.amazon.com/lambda/latest/dg/permissions-user-function.html
[iam-boundaries]:
  https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_boundaries.html
[gcp-attach]: https://docs.cloud.google.com/iam/docs/attach-service-accounts
[az-roles]:
  https://learn.microsoft.com/en-us/azure/role-based-access-control/built-in-roles/security#key-vault-data-access-administrator
[az-delegate]:
  https://learn.microsoft.com/en-us/azure/role-based-access-control/delegate-role-assignments-overview
[aws-chain]:
  https://docs.aws.amazon.com/sdkref/latest/guide/standardized-credentials.html
[rust-chain]: https://docs.aws.amazon.com/sdk-for-rust/latest/dg/credproviders.html
[aws-files]: https://docs.aws.amazon.com/sdkref/latest/guide/file-location.html
[aws-static]: https://docs.aws.amazon.com/sdkref/latest/guide/feature-static-credentials.html
[aws-sso]: https://docs.aws.amazon.com/sdkref/latest/guide/understanding-sso.html
[aws-process]: https://docs.aws.amazon.com/sdkref/latest/guide/feature-process-credentials.html
[ra-sdk]: https://docs.aws.amazon.com/sdkref/latest/guide/access-rolesanywhere.html
[ra-helper]: https://docs.aws.amazon.com/rolesanywhere/latest/userguide/credential-helper.html
[imdsv2]:
  https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/configuring-instance-metadata-service.html
[aws-cont]: https://docs.aws.amazon.com/sdkref/latest/guide/feature-container-credentials.html
[rust-ecs]: https://docs.rs/aws-config/1.8.13/src/aws_config/ecs.rs.html
[k8s-kubeconfig]:
  https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/
[kuberc]: https://kubernetes.io/docs/reference/kubectl/kuberc/
[eks-kubeconfig]: https://docs.aws.amazon.com/eks/latest/userguide/create-kubeconfig.html
[gcp-wif]:
  https://docs.cloud.google.com/iam/docs/workload-identity-federation-with-other-providers
[docker-login]: https://docs.docker.com/reference/cli/docker/login/
[sops-ref]: https://getsops.io/docs/reference/
[sops-ops]: https://getsops.io/docs/usage/common-operations/
[sops-adv]: https://getsops.io/docs/usage/advanced/
[sops-age]: https://getsops.io/docs/usage/identities/age/
[srv-github]: https://github.com/github/github-mcp-server
[srv-brave]: https://github.com/brave/brave-search-mcp-server
[srv-grafana]: https://github.com/grafana/mcp-grafana
[srv-gitlab]: https://raw.githubusercontent.com/zereight/gitlab-mcp/main/README.md
[srv-mdb]: https://github.com/mongodb-js/mongodb-mcp-server
[srv-k8s]:
  https://raw.githubusercontent.com/containers/kubernetes-mcp-server/main/docs/configuration.md
[srv-aws]:
  https://raw.githubusercontent.com/awslabs/mcp/main/src/aws-api-mcp-server/README.md
[boto3-creds]: https://boto3.amazonaws.com/v1/documentation/api/latest/guide/credentials.html
[docker-mcp-github]:
  https://raw.githubusercontent.com/docker/mcp-registry/main/servers/github-official/server.yaml
[docker-mcp-contrib]: https://raw.githubusercontent.com/docker/mcp-registry/main/CONTRIBUTING.md
[toolhive]: https://docs.stacklok.com/toolhive/guides-cli/secrets-management
[systemd-cred]: https://systemd.io/CREDENTIALS/
[sm-rp]:
  https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access_resource-policies.html
[iam-eval]:
  https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_evaluation-logic_policy-eval-basics.html
[sm-examine]:
  https://docs.aws.amazon.com/secretsmanager/latest/userguide/determine-acccess_examine-iam-policies.html
[iam-denyallow]:
  https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_evaluation-logic_policy-eval-denyallow.html
[sm-xacct]:
  https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access_examples_cross.html
[iam-xacct]:
  https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies-cross-account-resource-access.html
[sm-putrp]:
  https://docs.aws.amazon.com/secretsmanager/latest/apireference/API_PutResourcePolicy.html
[sm-valrp]:
  https://docs.aws.amazon.com/secretsmanager/latest/apireference/API_ValidateResourcePolicy.html
[sm-bp]: https://docs.aws.amazon.com/secretsmanager/latest/userguide/best-practices.html
[gsm-setiam]:
  https://docs.cloud.google.com/secret-manager/docs/reference/rest/v1/projects.secrets/setIamPolicy
[k8s-checklist]: https://kubernetes.io/docs/concepts/security/security-checklist/
[k8s-cont-env]: https://kubernetes.io/docs/concepts/containers/container-environment/
[k8s-service]: https://kubernetes.io/docs/concepts/services-networking/service/
[k8s-pod]: https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/
[k8s-sa-api]:
  https://kubernetes.io/docs/reference/kubernetes-api/authentication-resources/service-account-v1/
[k8s-connect]:
  https://kubernetes.io/docs/concepts/services-networking/connect-applications-service/
[kep-2360]:
  https://github.com/kubernetes/enhancements/blob/master/keps/sig-apps/2360-optional-service-environment-variables/README.md
[k8s-121787]: https://github.com/kubernetes/kubernetes/issues/121787
[k8s-api-pod]: https://kubernetes.io/docs/tasks/run-application/access-api-from-pod/
[anth-wif]: https://platform.claude.com/docs/en/manage-claude/workload-identity-federation
[anth-wif-ref]: https://platform.claude.com/docs/en/manage-claude/wif-reference
[anth-keys]: https://platform.claude.com/docs/en/manage-claude/authentication
[cc-authn]: https://code.claude.com/docs/en/authentication
[cc-bedrock]: https://code.claude.com/docs/en/amazon-bedrock
[cc-vertex]: https://code.claude.com/docs/en/google-vertex-ai
[bedrock-keys]: https://docs.aws.amazon.com/bedrock/latest/userguide/api-keys.html
[anth-bedrock]: https://platform.claude.com/docs/en/build-with-claude/claude-on-amazon-bedrock
[gcp-tokens]: https://docs.cloud.google.com/docs/authentication/token-types
[oai-wif-ref]: https://developers.openai.com/api/reference/workload-identity-federation
[oai-wif]: https://developers.openai.com/api/docs/guides/workload-identity-federation
[codex-config]: https://learn.chatgpt.com/docs/config-file/config-advanced.md
[codex-tokens]: https://learn.chatgpt.com/docs/enterprise/access-tokens.md
[linux-environ]: https://man7.org/linux/man-pages/man5/proc_pid_environ.5.html
[linux-ps]: https://man7.org/linux/man-pages/man1/ps.1.html
[linux-ptrace]: https://man7.org/linux/man-pages/man2/ptrace.2.html
[linux-mem]: https://man7.org/linux/man-pages/man5/proc_pid_mem.5.html
[linux-proc]: https://man7.org/linux/man-pages/man5/proc.5.html
[linux-path]: https://man7.org/linux/man-pages/man7/path_resolution.7.html
[linux-memfd]: https://man7.org/linux/man-pages/man2/memfd_create.2.html
[linux-fd]: https://man7.org/linux/man-pages/man5/proc_pid_fd.5.html
[pam-systemd]: https://www.freedesktop.org/software/systemd/man/latest/pam_systemd.html
[linux-tmpfs]: https://docs.kernel.org/filesystems/tmpfs.html
[mac-ps]: https://raw.githubusercontent.com/apple-oss-distributions/adv_cmds/main/ps/print.c
[xnu-sysctl]:
  https://raw.githubusercontent.com/apple-oss-distributions/xnu/main/bsd/kern/kern_sysctl.c
[xnu-cs]: https://raw.githubusercontent.com/apple-oss-distributions/xnu/main/bsd/kern/kern_cs.c
[mac-hdiutil]: https://keith.github.io/xcode-man-pages/hdiutil.1.html
[mac-mount]: https://keith.github.io/xcode-man-pages/mount.2.html
[xnu-vfs]:
  https://raw.githubusercontent.com/apple-oss-distributions/xnu/main/bsd/vfs/vfs_syscalls.c
[apple-hardened]:
  https://developer.apple.com/tutorials/data/documentation/security/hardened-runtime.json
[ec2-metadata]: https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html
[ec2-userdata]: https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/user-data.html
[ec2-retrieve]:
  https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instancedata-data-retrieval.html
[ec2-lt-versions]:
  https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/manage-launch-template-versions.html
[ec2-lt-perms]:
  https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/permissions-for-launch-templates.html
[ec2-lt-data]:
  https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_ResponseLaunchTemplateData.html
[prowler-lt]: https://hub.prowler.com/check/ec2_launch_template_no_secrets
[iam-ec2]: https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use_switch-role-ec2.html
[cfn-security]:
  https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/security-best-practices.html
