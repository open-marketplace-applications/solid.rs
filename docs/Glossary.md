# Glossary

[This Terminology](https://www.w3.org/TR/ldp/#terms) is based on W3C's Architecture of the World Wide Web [WEBARCH] and Hyper-text Transfer Protocol ([RFC7230], [RFC7231], [RFC7232]).

## Link
A relationship between two resources when one resource (representation) refers to the other resource by means of a URI [WEBARCH].

## Linked Data
As defined by Tim Berners-Lee [LINKED-DATA].

## Client
A program that establishes connections for the purpose of sending one or more HTTP requests [RFC7230].

## Server
A program that accepts connections in order to service HTTP requests by sending HTTP responses.
The terms "client" and "server" refer only to the roles that these programs perform for a particular connection. The same program might act as a client on some connections and a server on others.

HTTP enables the use of intermediaries to satisfy requests through a chain of connections. There are three common forms of HTTP intermediary: proxy, gateway, and tunnel. In some cases, a single intermediary might act as an origin server, proxy, gateway, or tunnel, switching behavior based on the nature of each request. [RFC7230].

## Linked Data Platform Resource (LDPR)
A HTTP resource whose state is represented in any way that conforms to the simple lifecycle patterns and conventions in section 4. Linked Data Platform Resources.

## Linked Data Platform RDF Source (LDP-RS)
An LDPR whose state is fully represented in RDF, corresponding to an RDF graph. See also the term RDF Source from [rdf11-concepts].

## Linked Data Platform Non-RDF Source (LDP-NR)
An LDPR whose state is not represented in RDF. For example, these can be binary or text documents that do not have useful RDF representations.

## Linked Data Platform Container (LDPC)
A LDP-RS representing a collection of linked documents (RDF Document [rdf11-concepts] or information resources [WEBARCH]) that responds to client requests for creation, modification, and/or enumeration of its linked members and documents, and that conforms to the simple lifecycle patterns and conventions in section 5.  Linked Data Platform Containers.

## Linked Data Platform Basic Container (LDP-BC)
An LDPC that defines a simple link to its contained documents (information resources) [WEBARCH].

## Linked Data Platform Direct Container (LDP-DC)
An LDPC that adds the concept of membership, allowing the flexibility of choosing what form its membership triples take, and allows members to be any resources [WEBARCH], not only documents.

## Linked Data Platform Indirect Container (LDP-IC)
An LDPC similar to a LDP-DC that is also capable of having members whose URIs are based on the content of its contained documents rather than the URIs assigned to those documents.

## Membership
The relationship linking an LDPC and its member LDPRs, which can be different resources than its contained documents. The LDPC often assists with managing the membership triples, whether or not the LDPC's URI occurs in them.

## Membership triples
A set of triples that lists an LDPC's members. A LDPC's membership triples all have one of the following patterns:
membership-constant-URI	membership-predicate	member-derived-URI
member-derived-URI	membership-predicate	membership-constant-URI
The difference between the two is simply which position member-derived-URI occupies, which is usually driven by the choice of membership-predicate. Most predicates have a natural forward direction inherent in their name, and existing vocabularies contain useful examples that read naturally in each direction. ldp:member and dcterms:isPartOf are representative examples.
Each linked container exposes properties (see section 5.2.1 General) that allow clients to determine which pattern it uses, what the actual membership-predicate and membership-constant-URI values are, and (for containers that allow the creation of new members) what value is used for the member-derived-URI based on the client's input to the creation process.

## Membership predicate
The predicate of all an LDPC's membership triples.
Containment
The relationship binding an LDPC to LDPRs whose lifecycle it controls and is aware of. The lifecycle of the contained LDPR is limited by the lifecycle of the containing LDPC; that is, a contained LDPR cannot be created (through LDP-defined means) before its containing LDPC exists.

## Containment triples
A set of triples, maintained by the LDPC, that lists documents created by the LDPC but not yet deleted. These triples always have the form: ( LDPC URI, ldp:contains , document-URI ).

## Minimal-container triples
The portion of an LDPC's triples that would be present when the container is empty. Currently, this definition is equivalent to all the LDPC's triples minus its containment triples, and minus its membership triples (if either are considered part of its state), but if future versions of LDP define additional classes of triples then this definition would expand to subtract out those classes as well.

## LDP-server-managed triples
The portion of an LDP's triples whose behavior is constrained directly by this specification; for example, membership triples and containment triples. This portion of resources' content does not include constraints imposed outside of LDP, for example by other specifications that the server happens to support, or by server implementation decisions.