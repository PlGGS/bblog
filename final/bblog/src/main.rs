use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use axum::{
            response::{Response, IntoResponse},
            routing::get,
            extract::{Path, State, RawQuery},
            http::{Request, header::HeaderMap},
            body::Body as AxumBody,
            Router,
        };
        use bblog::app::DB_URL;
        use bblog::app::App;
        use bblog::auth::*;
        use bblog::state::AppState;
        use bblog::fallback::file_and_error_handler;
        use leptos_axum::{generate_route_list, LeptosRoutes, handle_server_fns_with_context};
        use leptos::{log, view, provide_context, get_configuration};
        use sqlx::{SqlitePool, sqlite::SqlitePoolOptions, migrate::MigrateDatabase, Sqlite};
        use axum_session::{SessionConfig, SessionLayer, SessionStore};
        use axum_session_auth::{AuthSessionLayer, AuthConfig, SessionSqlitePool};

        async fn server_fn_handler(State(app_state): State<AppState>, auth_session: AuthSession, path: Path<String>, headers: HeaderMap, raw_query: RawQuery, request: Request<AxumBody>) -> impl IntoResponse {
            log!("{:?}", path);

            handle_server_fns_with_context(path, headers, raw_query, move |cx| {
                provide_context(cx, auth_session.clone());
                provide_context(cx, app_state.pool.clone());
            }, request).await
        }

        async fn leptos_routes_handler(auth_session: AuthSession, State(app_state): State<AppState>, req: Request<AxumBody>) -> Response{
                let handler = leptos_axum::render_app_to_stream_with_context(app_state.leptos_options.clone(),
                move |cx| {
                    provide_context(cx, auth_session.clone());
                    provide_context(cx, app_state.pool.clone());
                },
                |cx| view! { cx, <App/> }
            );
            handler(req).await.into_response()
        }

        #[tokio::main]
        async fn main() {
            simple_logger::init_with_level(log::Level::Info).expect("couldn't initialize logging");

            if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
                println!("Creating database at: \"{}\"", DB_URL);
                match Sqlite::create_database(DB_URL).await {
                    Ok(_) => println!("Create db success"),
                    Err(error) => panic!("error: {}", error),
                }
            } else {
                println!("Database at: \"{}\" exists.", DB_URL);
            }

            let pool = SqlitePoolOptions::new()
                .connect(DB_URL)
                .await
                .expect("Could not make pool.");

            let session_config = SessionConfig::default().with_table_name("axum_sessions");
            let auth_config = AuthConfig::<i64>::default();
            let session_store = SessionStore::<SessionSqlitePool>::new(Some(pool.clone().into()), session_config);
            session_store.initiate().await.unwrap();

            sqlx::migrate!()
                .run(&pool)
                .await
                .expect("could not run SQLx migrations");

            bblog::app::register_server_functions();

            // Setting this to None means we'll be using cargo-leptos and its env vars
            let conf = get_configuration(None).await.unwrap();
            let leptos_options = conf.leptos_options;
            let addr = leptos_options.site_addr;
            let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

            let app_state = AppState{
                leptos_options,
                pool: pool.clone(),
            };

            let app = Router::new()
                .route("/api/*fn_name", get(server_fn_handler).post(server_fn_handler))
                .leptos_routes_with_handler(routes, get(leptos_routes_handler) )
                .fallback(file_and_error_handler)
                .layer(AuthSessionLayer::<User, i64, SessionSqlitePool, SqlitePool>::new(Some(pool.clone()))
                .with_config(auth_config))
                .layer(SessionLayer::new(session_store))
                .with_state(app_state);

            log!("listening on http://{}", &addr);
            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
    }
    else {
        pub fn main() {

        }
    }
}
