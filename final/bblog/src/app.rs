use crate::{auth::*, error_template::ErrorTemplate};
use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::SqlitePool;

        pub const DB_URL: &str = "sqlite://bblog.db";

        pub fn pool(cx: Scope) -> Result<SqlitePool, ServerFnError> {
        use_context::<SqlitePool>(cx)
                .ok_or("Pool missing.")
                .map_err(|e| ServerFnError::ServerError(e.to_string()))
        }

        pub fn auth(cx: Scope) -> Result<AuthSession, ServerFnError> {
            use_context::<AuthSession>(cx)
                .ok_or("Auth session missing.")
                .map_err(|e| ServerFnError::ServerError(e.to_string()))
        }

        pub fn register_server_functions() {
            _ = GetPosts::register();
            _ = NewPostDraft::register();
            _ = DeletePost::register();
            _ = Login::register();
            _ = Logout::register();
            _ = Signup::register();
            _ = GetUser::register();
        }

        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
        pub struct Post {
            id: String,
            user_id: String,
            series_id: String,
            title: String,
            content: String,
            created_at: String,
            updated_at: String,
            draft_saved: bool,
            posted: bool
        }
    }
    else {
        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub struct Post {
            id: String,
            user_id: String,
            series_id: String,
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
    use futures::TryStreamExt;
    let pool = pool(cx)?;

    let mut posts = Vec::new();
    let mut rows = sqlx::query_as::<_, Post>("SELECT * FROM posts").fetch(&pool);

        while let Some(row) = rows.try_next()
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))? {
        posts.push(row);
    }

    Ok(posts)
}

#[server(NewPostDraft, "/api")]
pub async fn new_post_draft(cx: Scope, series_id: String, title: String, content: String) -> Result<(), ServerFnError> {
    let user = get_user(cx).await?;
    let pool = pool(cx)?;

    let user_id = match user {
        Some(user) => user.id,
        None => -1, //TODO maybe handle this differently in the future
    };

    match sqlx::query
    ("INSERT INTO posts (user_id, series_id, title, content, draft_saved, posted) 
          VALUES (?, ?, ?, ?, 1, 0)")
        .bind(user_id)
        .bind(series_id)
        .bind(title)
        .bind(content)
        .execute(&pool)
        .await {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[server(DeletePost, "/api")]
pub async fn delete_post(cx: Scope, id: u16) -> Result<(), ServerFnError> {
    let pool = pool(cx)?;

    sqlx::query("DELETE FROM posts WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map(|_| ())
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let login = create_server_action::<Login>(cx);
    let logout = create_server_action::<Logout>(cx);
    let signup = create_server_action::<Signup>(cx);

    let user = create_resource(
        cx,
        move || {
            (
                login.version().get(),
                signup.version().get(),
                logout.version().get(),
            )
        },
        move |_| get_user(cx),
    );
    provide_meta_context(cx);

    view! {
        cx,
        <Link rel="favicon" type_="image/ico" href="/favicon.ico"/>
        <Stylesheet id="leptos" href="/pkg/bblog.css"/>
        <Router>
            <header>
                <A href="/"><h1>"BBlog"</h1></A>
                <Transition
                    fallback=move || view! {cx, <span>"Loading..."</span>}
                >
                {move || {
                    user.read(cx).map(|user| match user {
                        Err(e) => view! {cx,
                            <A href="/signup">"Signup"</A>", "
                            <A href="/login">"Login"</A>", "
                            <span>{format!("Login error: {}", e.to_string())}</span>
                        }.into_view(cx),
                        Ok(None) => view! {cx,
                            <A href="/signup">"Signup"</A>", "
                            <A href="/login">"Login"</A>", "
                            <span>"Logged out."</span>
                        }.into_view(cx),
                        Ok(Some(user)) => view! {cx,
                            <A href="/settings">"Settings"</A>", "
                            <span>{format!("Hi, {}!", user.first_name)}</span>
                        }.into_view(cx)
                    })
                }}
                </Transition>
            </header>
            <hr/>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! {
                        cx,
                        <ErrorBoundary fallback=|cx, errors| view!{cx, <ErrorTemplate errors=errors/>}>
                            <Posts/>
                        </ErrorBoundary>
                    }/> //Route
                    <Route path="signup" view=move |cx| view! {
                        cx,
                        <Signup action=signup/>
                    }/>
                    <Route path="login" view=move |cx| view! {
                        cx,
                        <Login action=login />
                    }/>
                    <Route path="settings" view=move |cx| view! {
                        cx,
                        <h1>"Settings"</h1>
                        <Logout action=logout />
                    }/>
                </Routes>
            </main>
        </Router>
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

#[component]
pub fn Login(
    cx: Scope,
    action: Action<Login, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
        cx,
        <ActionForm action=action>
            <h1>"Log In"</h1>
            <label>
                "User ID:"
                <input type="text" placeholder="User ID" maxlength="32" name="username" class="auth-input" />
            </label>
            <br/>
            <label>
                "Password:"
                <input type="password" placeholder="Password" name="password" class="auth-input" />
            </label>
            <br/>
            <label>
                <input type="checkbox" name="remember" class="auth-input" />
                "Remember me?"
            </label>
            <br/>
            <button type="submit" class="button">"Log In"</button>
        </ActionForm>
    }
}

#[component]
pub fn Signup(
    cx: Scope,
    action: Action<Signup, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
        cx,
        <ActionForm action=action>
            <h1>"Sign Up"</h1>
            <label>
                "First name:"
                <input type="text" placeholder="First" maxlength="32" name="first_name" class="auth-input" />
            </label>
            <br/>
            <label>
                "Last name:"
                <input type="text" placeholder="Last" maxlength="32" name="last_name" class="auth-input" />
            </label>
            <br/>
            <label>
                "Username:"
                <input type="text" placeholder="Username" maxlength="32" name="username" class="auth-input" />
            </label>
            <br/>
            <label>
                "Password:"
                <input type="password" placeholder="Password" name="password" class="auth-input" />
            </label>
            <br/>
            <label>
                "Confirm Password:"
                <input type="password" placeholder="Password again" name="password_confirmation" class="auth-input" />
            </label>
            <br/>
            <label>
                "Remember me?"
                <input type="checkbox" name="remember" class="auth-input" />
            </label>

            <br/>
            <button type="submit" class="button">"Sign Up"</button>
        </ActionForm>
    }
}

#[component]
pub fn Logout(
    cx: Scope,
    action: Action<Logout, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! {
        cx,
        <div id="loginbox">
            <ActionForm action=action>
                <button type="submit" class="button">"Log Out"</button>
            </ActionForm>
        </div>
    }
}
