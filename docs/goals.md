## nixek system management - goals

This document attempts to provide a high level description of what space nixek
systems management (nixek systems) lives in, what's immediately in scope, and
what will be possible longer term.

Note, this entire document is "brainstorm" quality. Massive grains of salt for
all of this.

### Vague hand-wavy mission

Nixek systems management aims to make it easy to configure, update, and manage
computers, whether a fleet of a thousand servers, or a single developer's
laptop.

Our mission is to help users securely manage servers.

### Ideals

Free software and user freedom is good. All code, both server and client, is
open source.

Probably no profit. Definitely not for open source / hobbyist users. TBD
on big evil megacorps.

With this out of the way, on to technical goals.

### Specific technical goals - MVP

This section will focus on the specific technical goals for nixek systems management's "MVP".

Note, the MVP specifically only targets NixOS.

#### Can create clusters

Computers running NixOS can easily be configured to securely connect to the "nixek systems
management server" with at most a couple lines of configuration.
Computers may join a "cluster" of machines, and may have a "role" within that cluster.

The list of machines that are currently online (as best the server knows
anyway) can be viewed in a web interface, as can the machine's role.

#### Can update configuration of machines

A user may associate a desired configuration, in the form of a nixos
configuration expression, with a role. This configuration will be applied to
any machines that take on that role.

#### Update rollout

A user may also configure a role with a "canary rollout", where if multiple
machines exist within a role, any new configuration is applied to a single
machine first before rolling it out further.

Additional configuration of rollout (i.e. "n at once" or manually setting
percents, etc) is not configurable in the MVP.

Alerts about failed rollouts also probably won't make the MVP. Just read the
logs for now.

### Specific technical goals - Post MVP

#### Healthchecks

During the MVP, a rollout succeeding or failing will just be "did nixos-rebuild
switch exit 0 or non-0". This isn't good enough for a lot of things.
Configurable healthchecks will be necessary.

#### Automatic rollback

Per the title. Nixos makes this relatively easy in most cases.

### Specific technical goals -- Someday

Goals that aren't expected to happen in any specific timeframe, but are in
scope for the project as I envision it now.

#### Secrets management for machines

As the title. Secrets management is hard, and it dovetails with configuration
management decently well.

For now, users will just have to trust the service to not leak their secrets,
or otherwise manage to get secrets onto the machine.

#### Mergable role heirarchies

For now, users can do this with nix themselves since the nix language has
robust capabilities for this. It would be nice to make it easier to do with a
nice UX though.

This one's pretty tentative.

#### "zero trust" mode

Everything so far requires trusting the server (after all, it can serve
arbitrary nixos configuration, which is root). It should be possible to do a
zero-trust mode, where effectively the nixek-daemon users run is given a list
of trusted keys, and the user signs configurations before sending them to the
nixek systems management api. The daemon can then reject all configurations
that aren't signed, and now even a malicious server couldn't exploit the
client.
