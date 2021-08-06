## nixek systems management - design

This document outlines design choices in building nixek systems management.

For background on what it wants to accomplish, see [goals.md][goals]

### High level design

XXX: diagram

### API

buncha grpc

### Dashboard

MVP is just a list of clusters w/ online nodes, their roles, and what version
of software they're running.

### Daemon Bootstrapping Security

Users create "bootstrapping tokens" which may optionally be scoped to specific
clusters and roles. They may also be single use, or unlimited use.

For now, they're nothing fancy (just a shared secret). We can improve this
later.

This token will be passed to a nixek daemon and let it join any cluster+role
within the scope of that token.

On joining the cluster, the node will generate a TLS key, and request a client
certificate from the server. Effectively, the bootstrap token is exchanged for
a TLS certificate, allowing all future communication to occur using mutual TLS.

Sufficient information to identify the host will be encoded in the CN, and it
will be validated appropriately before issuing the client certificate.

### Data storage

The boring answer would be postgres. I'm still thinking about this one.
DynamoDB seems awful tempting.
Realistically, it'll probably be postgres. For everything.

[goals]: ./goals.md
