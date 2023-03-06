use crate::tcp_sink::TcpWriter;
use std::sync::Arc;
use std::sync::Mutex;
use tracing::subscriber::set_global_default;
use tracing::Level;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Registry;

/// Compose multiple layers into a `tracing`'s subscriber.
///
/// # Implementation Notes
///
/// We are using `impl Subscriber` as return type to avoid having to
/// spell out the actual type of the returned subscriber, which is
/// indeed quite complex.
/// We need to explicitly call out that the returned subscriber is
/// `Send` and `Sync` to make it possible to pass it to `init_subscriber`
/// later on.

// name is the name of the subscriber
// env_filter is the level of log (INFO, TRACE ...)
// sink is where the output is redirected
pub fn get_subscriber_with_elk<Sink>(
    name: String,
    env_filter: Level,
    sink: Sink,
    elk_adress: (String, u16),
) -> impl Subscriber + Sync + Send
where
    // This "weird" syntax is a higher-ranked trait bound (HRTB)
    // It basically means that Sink implements the `MakeWriter`
    // trait for all choices of the lifetime parameter `'a`
    // Check out https://doc.rust-lang.org/nomicon/hrtb.html
    // for more details.
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let elk_adress_string = format!("{}:{}", elk_adress.0, elk_adress.1);
    let tcp_writer = TcpWriter::new(elk_adress_string).unwrap();
    let writer_mutex = Arc::new(Mutex::new(tcp_writer));
    let writer_closure = move || {
        writer_mutex
            .lock()
            .unwrap()
            .stream
            .try_clone()
            .expect("Failed to get TCP Stream")
    };

    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter.as_str()));
    let file_layer = BunyanFormattingLayer::new(name.clone(), sink);
    let elk_layer = BunyanFormattingLayer::new(name, writer_closure);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(file_layer)
        .with(elk_layer)
}

pub fn get_subscriber_without_elk<Sink>(
    name: String,
    env_filter: Level,
    sink: Sink,
) -> impl Subscriber + Sync + Send
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter.as_str()));
    let file_layer = BunyanFormattingLayer::new(name.clone(), sink);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(file_layer)
}

/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // Redirect all `log`'s events to our subscriber
    LogTracer::init().expect("Failed to set logger");
    // Use this subscriber as the default for the entire program
    set_global_default(subscriber).expect("Failed to set subscriber");
}
