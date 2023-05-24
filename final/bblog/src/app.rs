use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::prelude::*;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{Connection, SqliteConnection};

        pub async fn db() -> Result<SqliteConnection, ServerFnError> {
            SqliteConnection::connect("sqlite:BBlog.db").await.map_err(|e| ServerFnError::ServerError(e.to_string()))
        }

        pub fn register_server_functions() {
            _ = GetPosts::register();
            _ = NewPost::register();
        }

        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
        pub struct Post {
            post_uuid: String,
            user_google_uuid: String,
            series_uuid: String,
            title: String,
            content: String,
            created_at: String,
            updated_at: String,
            draft_saved: bool,
            posted: bool
        }
    } else {
        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub struct Post {
            post_uuid: String,
            user_google_uuid: String,
            series_uuid: String,
            title: String,
            content: String,
            created_at: String,
            updated_at: String,
            draft_saved: bool,
            posted: bool
        }
    }
}

#[server(GetPosts, "/api")]
pub async fn get_posts(cx: Scope) -> Result<Vec<Post>, ServerFnError> {
    // print api request path
    //TODO figure out why this isn't working
    let req = use_context::<actix_web::HttpRequest>(cx);
    if let Some(req) = req {
        println!("req.path = {:#?}", req.path());
    }
    
    use futures::TryStreamExt;
    let mut conn = db().await?;

    let mut posts = Vec::new();
    let mut rows = sqlx::query_as::<_, Post>("SELECT * FROM posts").fetch(&mut conn);
    while let Some(row) = rows.try_next()
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))? {
        posts.push(row);
    }

    Ok(posts)
}

#[server(NewPost, "/api")]
pub async fn new_post(user_google_id: String, series_uuid: String, title: String, content: String) -> Result<(), ServerFnError> {
    let mut conn = db().await?;

    let post_uuid: Uuid = Uuid::new_v4();
    let current_time: String = get_current_timestamp();

    match sqlx::query
    (format!("INSERT INTO posts (post_uuid, user_google_id, series_uuid, title, content, created_at, updated_at, draft_saved, posted) 
    VALUES ({}, $1, $2, $3, $4, {}, {}, 1, 0)", post_uuid, current_time, current_time).as_str())
        .bind(user_google_id)
        .bind(series_uuid)
        .bind(title)
        .bind(content)
        .execute(&mut conn)
        .await {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

fn get_current_timestamp() -> String {
    return Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,
        <Link rel="favicon" type_="image/ico" href="/favicon.ico"/>
        <Stylesheet id="leptos" href="/pkg/bblog.css"/>
        <Title text="BBlog"/>
        <Router>
            <Routes>
                //I use page components to house additional components, because I want a dynamic header
                <Route path="" view=|cx| view! { cx, <HomePage/> }/>
            </Routes>
        </Router>
    }
}

/// Homepage for viewing posts from users you've subscribed to
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // // Creates a reactive value to update the button
    // let (count, set_count) = create_signal(cx, 0);
    // let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <header>
            <h1>"BBlog (update this to actual header component)"</h1>
        </header>
        <main>
            // <button on:click=on_click>"Click Me: " {count}</button>
            <Posts/>
        </main>
    }
}

#[component]
pub fn Posts(cx: Scope) -> impl IntoView {
    let posts = create_resource(
        cx,
        move || (),
        move |_| get_posts(cx),
    );

    view! {
        cx,
        <div>
            <Transition fallback=move || view! {cx, <p>"Loading..."</p> }>
                {move || {
                    let existing_posts = {
                        move || {
                            posts.read(cx).map(move |posts| match posts {
                                Err(e) => {
                                    view! { cx, <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view(cx)
                                }
                                Ok(posts) => {
                                    if posts.is_empty() {
                                        view! { cx, <p>"No posts found."</p> }.into_view(cx)
                                    } else {
                                        posts
                                            .into_iter()
                                            .map(move |post| {
                                                view! {
                                                    cx,
                                                    <li>
                                                        {post.title}
                                                    </li>
                                                    <li>
                                                        {post.content}
                                                    </li>
                                                    <li>
                                                        {post.created_at}
                                                    </li>
                                                }
                                            })
                                            .collect_view(cx)
                                    }
                                }
                            })
                            .unwrap_or_default()
                        }
                    };

                    view! {
                        cx,
                        <ul>
                            {existing_posts}
                        </ul>
                    }
                }
            }
            </Transition>
        </div>
    }
}
