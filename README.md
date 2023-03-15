
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

|| Tested/Total Lines:
|| src/configuration.rs: 24/24
|| src/entity/pagination.rs: 17/17
|| src/error/api_error.rs: 6/14
|| src/main.rs: 0/7
|| src/repository/products.rs: 17/17
|| src/repository/resellers.rs: 9/10
|| src/repository/users.rs: 11/12
|| src/routes/health_check.rs: 2/2
|| src/routes/products.rs: 52/59
|| src/startup.rs: 28/28
|| src/tcp_sink.rs: 7/9
|| src/telemetry.rs: 13/13
|| 
87.74% coverage, 186/212 lines covered



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
