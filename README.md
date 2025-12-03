# Code As Infra
This repository is a collection of home made tools that can potentially replace the regular tools I use in my web infrastructure.

The current industry standard is to use a "infrastructure as code" solution like Ansible for example where the tools are defined in sets of roles, combined & deployed via playbooks, and configured in huge inventories of YML config files. All of that is great but sometimes one of the core tools requires hundreds of line of configuration or uses a tech stack that is (in my opinion) not suited for production usage due to a lower general reliability like all of these python based tools.

The projects in the repository try to invert that "infra as code" approach and adopt a "code as infra" one instead where the tools are actually libraries that can then be imported and configured in code and finally compiled into binaries that are tailored for the environments they're meant to be deployed into.

As configuration is done in code, the compiler catches misconfiguration, and once compiled the resulting infrastructure is a set of binaries that are all ready to be deployed & run.

## Projects
- infrastructure
  - [reverse-proxy](/infrastructure/reverse-proxy/src/main.rs), a reverse proxy that relies on Cloudflare's pingora and that is fully capable of replacing `nginx` with TLS support, compression using gzip, rate limiting, referer filtering, etc... Refer to the `main.rs` file for an example of how to import & configure a proxy server.
  - `monitoring`: to be done, offers monitoring utilities similar to [munin-monitoring](https://munin-monitoring.org/)
  - `caching`: to be done, offers request caching utilities similar to [varnish-cache](https://varnish-cache.org/)