# solid_auth

In Solid.rs, the authentication flow should work with [Decentralized Identifiers (DIDs)](https://www.w3.org/TR/did-core/) without a username or password. 


The Solid specification describes possible technologies for authentication and authorization:

- [WebID](https://www.w3.org/wiki/WebID)
- [Web Access Control (WAC)](https://github.com/solid/web-access-control-spec#example-wac-document)
- [OpenID Connect (OIDC)](https://openid.net/connect/)

> **WebID** gives one way to log into an internet service. Instead of using a password, for example, the member refers to another web address which can vouch for it. WebID is not a specific service or product. Instead WebID is a suggested method for internet services and members to know who they are communicating with.

> **Web Access Control (WAC)** is a decentralized cross-domain access control system. The main concepts should be familiar to developers, as they are similar to access control schemes used in many file systems. It's concerned with giving access to agents (users, groups and more) to perform various kinds of operations (read, write, append, etc) on resources.

A Solid data pod MUST conform to the Web Access Control specification

> **OpenID Connect (OIDC)** is an authentication layer on top of OAuth 2.0, an authorization framework. The standard is controlled by the OpenID Foundation.

> **OAuth** is an open standard for access delegation, commonly used as a way for Internet users to grant websites or applications access to their information on other websites but without giving them the passwords.