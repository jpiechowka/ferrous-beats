use anyhow::Context;
use hickory_resolver::config::{ResolverConfig, ResolverOpts};
use hickory_resolver::TokioAsyncResolver;
use once_cell::sync::OnceCell;
use reqwest::dns::{Addrs, Name, Resolve, Resolving};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{debug, info, instrument};

#[derive(Debug, Default, Clone)]
pub struct CloudflareDoHResolver {
    /// Since we might not have been called in the context of a
    /// Tokio Runtime in initialization, so we must delay the actual
    /// construction of the resolver.
    state: Arc<OnceCell<TokioAsyncResolver>>,
}

impl Resolve for CloudflareDoHResolver {
    #[instrument(skip(self))]
    fn resolve(&self, name: Name) -> Resolving {
        debug!(
            "Resolving IP address for hostname using DoH: {}",
            name.as_str()
        );
        let doh_resolver = self.clone();
        Box::pin(async move {
            let resolver = doh_resolver
                .state
                .get_or_try_init(new_cloudflare_doh_resolver)
                .context("Failed to get or initialize resolver")?;
            let lookup = resolver
                .lookup_ip(name.as_str())
                .await
                .context(format!("IP lookup failed for hostname: {}", name.as_str()))?;
            let addrs: Addrs = Box::new(lookup.into_iter().map(|addr| SocketAddr::new(addr, 0)));

            Ok(addrs)
        })
    }
}

#[instrument(ret(level = "debug"))]
fn new_cloudflare_doh_resolver() -> Result<TokioAsyncResolver, anyhow::Error> {
    info!("Creating DNS over HTTPS (DoH) resolver using Cloudflare for DNS resolution");
    let doh_config = ResolverConfig::cloudflare_https();
    let doh_opts = ResolverOpts::default();
    Ok(TokioAsyncResolver::tokio(doh_config, doh_opts))
}
