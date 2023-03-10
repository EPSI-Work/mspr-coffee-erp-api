
Setup Firebase : 
- create a project
- Authentification / Add email/password auth

- create Firestore database / test mode / europe
- rules / delete : 
    : if
  request.time < timestamp.date(2023, 3, 17);
    
GitFlow : 
branch: main, dev, ft/test

Get Firebase Token : 
curl -X POST \  'https://www.googleapis.com/identitytoolkit/v3/relyingparty/verifyPassword?key=$API_KEY' \
  -H 'content-type: application/json' \
  -d '{ "email":"test@test.com", "password":"testeuh", "returnSecureToken":true }'

Add secret to Github Repo : 
GCP_SERVICE_ACCOUNT_KEY, GCP_PROJECT_ID and FIREBASE_ADMIN_KEY

lock branch main

export FIRESTORE_EMULATOR_HOST=localhost:8080
firebase emulators:exec --project mspr-epsi-coffee 'cargo test'
firebase emulators:exec --project mspr-epsi-coffee 'cargo tarpaulin --out Xml --output-dir coverage --fail-under 50'


cargo run --bin erp-import -- --file-path import/import-products.json --firebase-id mspr-epsi-coffee --firebase-token firebase-token/firebase-adminsdk-sa.json


security : 
cargo deny check
cargo outdated
cargo udeps
cargo audit
cargo pants




 // let file_appender = tracing_appender::rolling::hourly("logs", "log");
    // let (file_sink, _guard) = tracing_appender::non_blocking(file_appender);

    // let subscriber = get_subscriber_with_elk(
    //     "zero2prod".into(),
    //     Level::INFO.into(),
    //     std::io::stdout,
    //     (
    //         configuration.logstach.host.expose_secret().to_string(),
    //         configuration.logstach.port,
    //     ),
    // );

    // let tracer = stdout::new_pipeline().install_simple();
    // // Create a tracing layer with the configured tracer
    // let telemetry = tracing_opentelemetry::layer::<_>().with_tracer(tracer);
