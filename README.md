# ultra-megatron
The goal of this project is to build a load balancer that easily supports Layer
4 Network Load Balancing but is still modern and general purpose.

It supports a few modes of operations but some of the features are universal,
namely health checking backends for availability and hot reloading of the load
balancer configuration.

Backend configuration can be hot-reloaded, there is no need to restart or even
reload the process. Simply update the configuration file and save it. The change
will be noticed and the backend server configuration reloaded making the
addition or removal of load balanced servers simple.

Right now the only available configuration source is via a configuration file,
but this could easily be extended to support other configuration sources such as
a key value store, for example.

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
