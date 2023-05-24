use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use actix_files::Files;
        use actix_web::*;
        use leptos::*;
        use leptos_actix::{generate_route_list, LeptosRoutes};
        use sqlx::{SqlitePool, migrate::MigrateDatabase, Sqlite};
        use bblog::app::*;
        use std::env;

        #[tokio::main]
        async fn main() -> std::io::Result<()> {
            if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
                println!("Creating database {}", DB_URL);
                match Sqlite::create_database(DB_URL).await {
                    Ok(_) => println!("Create db success"),
                    Err(error) => panic!("error: {}", error),
                }
            } else {
                println!("Database \"{}\" exists.", DB_URL.to_string());
            }

            let db = SqlitePool::connect(DB_URL).await.unwrap();

            let mut current_dir = env::current_dir().expect("Failed to get the current directory");
            current_dir.push("migrations");
            let migration_dir = current_dir;

            let migration_results = sqlx::migrate::Migrator::new(migration_dir)
                .await
                .unwrap()
                .run(&db)
                .await;

            match migration_results {
                Ok(_) => println!("Sqlite migration success!"),
                Err(error) => {
                    panic!("error: {}", error);
                }
            }

            let conf = get_configuration(None).await.unwrap();
            let addr = conf.leptos_options.site_addr;
            let routes = generate_route_list(|cx| view! { cx, <App/> });

            HttpServer::new(move || {
                let leptos_options = &conf.leptos_options;
                let site_root = &leptos_options.site_root;

                App::new()
                    .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
                    .leptos_routes(
                        leptos_options.to_owned(),
                        routes.to_owned(),
                        |cx| view! { cx, <App/> },
                    )
                    .service(Files::new("/", site_root))
                //.wrap(middleware::Compress::default())
            })
            .bind(&addr)?
            .run()
            .await

            // let app = Router::new()
            //     .route("/api/*fn_name", get(server_fn_handler).post(server_fn_handler))
            //     .leptos_routes_with_handler(routes, get(leptos_routes_handler) )
            //     .fallback(file_and_error_handler)
            //     .layer(AuthSessionLayer::<User, i64, SessionSqlitePool, SqlitePool>::new(Some(pool.clone()))
            //                 .with_config(auth_config))
            //     .layer(SessionLayer::new(session_store))
            //     .layer(Extension(pool))
            //     .with_state(leptos_options);

            // // run our app with hyper
            // // `axum::Server` is a re-export of `hyper::Server`
            // log!("listening on http://{}", &addr);
            // axum::Server::bind(&addr)
            //     .serve(app.into_make_service())
            //     .await
            //     .unwrap();
        }
    }
    else {
        pub fn main() {
            
        }
    }
}
