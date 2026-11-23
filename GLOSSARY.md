# Clerk CLI

Clerk CLI administers users, organizations, and SAML single sign-on in a Clerk
Instance. It also requests sign-in tokens and JWTs for selected users.

## Language

### Users and Organizations

**Clerk Instance**:
A Clerk environment with its own Users, Organizations, and sign-in settings.
Its Secret Key identifies it for administrative requests.

**Secret Key**:
The credential the CLI uses to authenticate administrative requests to Clerk.

**User**:
A Clerk account with its own sign-in identity. A User can belong to multiple
Organizations through separate Organization Memberships.

**Primary Email Address**:
The email address Clerk designates as primary for a User. With no primary
address designated, the CLI uses the first email address on record.

**Organization**:
A group in a Clerk Instance that has its own members and their assigned roles.

**Organization Slug**:
A readable identifier for an Organization. The CLI accepts the slug or the
Clerk-assigned identifier for direct lookup.
_Avoid_: Organization name

**Organization Membership**:
The relationship that makes a User a member of an Organization and assigns
their Organization Role there.

**Organization Role**:
The access role assigned to a User for a particular Organization Membership.

### Single Sign-On

**Single Sign-On (SSO)**:
Sign-in that uses an external Identity Provider to authenticate a User for an
application. The CLI manages SAML configuration for this sign-in method.

**SAML Connection**:
A configuration that links sign-in for a domain to a SAML Identity Provider.
The CLI creates each connection for a selected Organization.

**Connection Domain**:
The sign-in domain associated with a SAML Connection.

**Connection Status**:
The active or inactive state Clerk records for a SAML Connection. The CLI
can update that state while changing the connection settings.

**Identity Provider**:
The service that verifies a User's identity during SAML sign-in.

**Identity Provider Settings**:
The SAML settings Clerk uses from an Identity Provider. They can be supplied
through provider metadata or as individual values such as the entity identifier,
sign-in address, and signing certificate.

**Service Provider Details**:
The SAML settings Clerk provides to an Identity Provider. They identify Clerk
and specify where to send sign-in responses and find Clerk's metadata.

### Sign-In and Tokens

**Impersonation**:
Signing in as a chosen User through a link carrying a Sign-In Token. The CLI
requests and displays the link.

**Sign-In Token**:
A time-limited credential Clerk issues for one User. Redeeming it can establish
a Session as that User.
_Avoid_: JWT

**Session**:
Clerk's record of a User's sign-in state. The CLI requests JWTs for a Session.

**Active Session**:
A Session Clerk reports as active. For JWT requests without an Organization
selection, the CLI reuses an available Active Session or creates one through
sign-in.

**JWT**:
A signed token that can be used to authenticate API requests. The CLI requests
it for a User's Session using a selected JWT Template.

**JWT Template**:
A named Clerk configuration for issuing a JWT, including the token's lifetime.

**Organization-Scoped JWT**:
A JWT requested for a specific Organization. The CLI uses the User's email to
start a fresh Session and selects the Organization before requesting the token.

**Frontend API**:
The Clerk Instance's interface for client sign-in. The CLI uses it to establish
Sessions from Sign-In Tokens and to request Organization-Scoped JWTs.
