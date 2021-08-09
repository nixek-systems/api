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

### Daemon Authentication

Users create "authentication tokens" which may optionally be scoped to specific
clusters and roles.

For now, they're nothing fancy (just a shared secret). We can improve this
later.

This token will be passed to a nixek daemon and let it join any cluster+role
within the scope of that token.

In the future, it may make sense to implement something more robust (i.e. make
the authentication token a bootstrapping token instead), but for now, just
having what amounts to bearer token auth seems like a reasonably expedient
option.

### Data storage

The boring answer would be postgres. I'm still thinking about this one.
DynamoDB seems awful tempting.
Realistically, it'll probably be postgres. For everything.

[goals]: ./goals.md
