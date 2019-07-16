# ultra-megatron
The goal of this project is to build a modern, general purpose and configurable
load balancer that easily supports Layer 4 Network Load Balancing.

It supports a few modes of operations as well as other universal of backend
health checking for availability and configuration hot reloading.

Configuration is driven via a configuration file but this could easily be
extended to support other configuration sources such as a key value stores etc.

The load balancer runs as a proxy by default since it's likely the most common
use case but startup configuration facilitates running it in more advanced modes
such as Passthrough or Direct Server Return (DSR).

## Modes
### 1. Proxy Mode
The client’s TCP connection is terminated at the load balancer in proxy mode.
The load balancer the copies over the payload and initiates another TCP stream
to one of backed servers in it's inventory. This connection persists for the
length of the TCP session as established by the client.

![binary tree](/res/img/lb-proxy-mode.png)

### 2. Passthrough Mode
### 3. DSR Mode
<!-- - Stats page (at /stats) with basic connection/bytes counters and backend server pool statuses
- Dynamic configuration re-loading of backend servers and associated weights. Configuration is loaded via a .toml file (see sample.toml for a full example).
- Tcp-based health checking of backend servers at a configured interval. If a server fails its health check it will be automatically removed from selection and added back once its health checks are successful. -->
