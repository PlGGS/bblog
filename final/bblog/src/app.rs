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
            _ = GetUser::register();
            _ = NewPostDraft::register();
            _ = DeletePost::register();
            _ = Login::register();
            _ = Logout::register();
            _ = Signup::register();
            _ = GetCurrentUser::register();
        }

        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
        pub struct Post {
            id: u32,
            user_id: u32,
            series_id: u32,
            title: String,
            tagline: String,
            content: String,
            created_at: String,
            updated_at: String,
            draft_saved: bool,
            posted: bool
        }

        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
        pub struct User {
            id: u32,
            first_name: String,
            last_name: String,
            username: String,
            password_hash: String,
            created_at: String,
            updated_at: String,
        }
    }
    else {
        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub struct Post {
            id: u32,
            user_id: u32,
            series_id: u32,
            title: String,
            tagline: String,
            content: String,
            created_at: String,
            updated_at: String,
            draft_saved: bool,
            posted: bool
        }

        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub struct User {
            id: u32,
            first_name: String,
            last_name: String,
            username: String,
            password_hash: String,
            created_at: String,
            updated_at: String,
        }
    }
}

#[server(GetPosts, "/api")]
pub async fn get_posts(cx: Scope, posts_type: PostsType) -> Result<Vec<Post>, ServerFnError> {
    use futures::TryStreamExt;
    let user = get_current_user(cx).await?;
    let pool = pool(cx)?;

    let user_id = match user {
        Some(user) => user.id,
        None => -1, //TODO maybe handle this differently in the future
    };

    let mut posts = Vec::new();

    let query;
    match posts_type {
        PostsType::All => {
            query = "SELECT * FROM posts";
        },
        PostsType::Subscriptions => {
            query = "SELECT p.* FROM posts p
                        JOIN user_subscription subscriber ON p.user_id = subscriber.subscription_user_id
                        WHERE subscriber.user_id = ?"
        },
        PostsType::Recommended => {
            query = "SELECT * FROM posts";
        }
    }

    let mut rows = sqlx::query_as::<_, Post>(query)
        .bind(user_id)
        .fetch(&pool);
    while let Some(row) = rows.try_next()
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))? {
            
        posts.push(row);
    }

    Ok(posts)
}

#[server(GetUser, "/api")]
pub async fn get_user(cx: Scope, id: u32) -> Result<User, ServerFnError> {
    let pool = pool(cx)?;

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
                .bind(id)
                .fetch_one(&pool)
                .await
                .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    
    Ok(user)
}

#[server(NewPostDraft, "/api")]
pub async fn new_post_draft(cx: Scope, series_id: String, title: String, content: String) -> Result<(), ServerFnError> {
    let user = get_current_user(cx).await?;
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

    let current_user = create_resource(
        cx,
        move || {(
            login.version().get(),
            signup.version().get(),
            logout.version().get(),
        )},
        move |_| get_current_user(cx),
    );
    provide_meta_context(cx);

    view! {
        cx,
        <Link rel="favicon" type_="image/ico" href="/favicon.ico"/>
        <Stylesheet id="leptos" href="/pkg/bblog.css"/>
        <Router>
            <header>
                <div class="top-bar">
                    <A href="/"><h1 class="left-align">"BBlog"</h1></A>
                    <nav>
                        <Transition
                            fallback=move || view! {cx, <span>"Loading..."</span>}
                        >
                        {move || {
                            current_user.read(cx).map(|current_user| match current_user {
                                Err(e) => view! {cx,
                                    <A href="/signup"><p>"Signup"</p></A>
                                    <A href="/login"><p>"Login"</p></A>
                                    <span>{format!("Login error: {}", e.to_string())}</span>
                                }.into_view(cx),
                                Ok(None) => view! {cx,
                                    <A href="/signup"><p>"Signup"</p></A>
                                    <A href="/login"><p>"Login"</p></A>
                                    <div class="circle">
                                        <A href="/profile">
                                            <img src="/profile-img.jpg" alt="profile-img"/>
                                        </A>
                                    </div>
                                }.into_view(cx),
                                Ok(Some(current_user)) => view! {cx,
                                    <A href="/settings">"Settings"</A>
                                    <span>{format!("Hi, {}!", current_user.first_name)}</span>
                                }.into_view(cx)
                            })
                        }}
                        </Transition>
                    </nav>
                </div>
                <hr/>
            </header>
            <main>
                <Routes>
                    <Route path="/u/:user_id" view=move |cx| view! {
                        cx,
                        <h2>"User profiles coming soon..."</h2>
                    }/>
                    <Route path="/post/:post_id" view=move |cx| view! {
                        cx,
                        <h2>"Posts coming soon..."</h2>
                    }/>
                    <Route path="/signup" view=move |cx| view! {
                        cx,
                        <Signup action=signup/>
                    }/>
                    <Route path="/login" view=move |cx| view! {
                        cx,
                        <Login action=login />
                    }/>
                    <Route path="/settings" view=move |cx| view! {
                        cx,
                        <h1>"Settings"</h1>
                        <Logout action=logout />
                    }/>
                    <Route path="/" view=|cx| view! {
                        cx,
                        <ErrorBoundary fallback=|cx, errors| view!{cx, <ErrorTemplate errors=errors/>}>
                            <Posts posts_type=PostsType::All/>
                        </ErrorBoundary>
                    }/>
                </Routes>
            </main>
        </Router>
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PostsType {
    All,
    Subscriptions,
    Recommended
}

#[component]
pub fn Posts(cx: Scope, posts_type: PostsType) -> impl IntoView {
    let posts = create_resource(
        cx,
        move || (),
        move |_| get_posts(cx, posts_type),
    );

    view! {
        cx,
        <div>
            {move || {
                match posts_type {
                    PostsType::All => view! { cx, 
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
                                                                <div>
                                                                    <PostCard post=post />
                                                                </div>
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
                                    <p>
                                        {existing_posts}
                                    </p>
                                }
                            }
                        }
                        </Transition>
                    }.into_view(cx),
                    PostsType::Subscriptions => {
                        view! { cx, <div></div> }.into_view(cx)
                    },
                    PostsType::Recommended => {
                        view! { cx, <div></div> }.into_view(cx)
                    },
                }
            }
        }
        </div>
    }
}

#[component]
pub fn PostCard(cx: Scope, post: Post) -> impl IntoView {
    let post_route = String::from("/post/") + post.id.to_string().as_str();
    
    view! {
        cx,
        <div class="card">
            <A href=post_route>
                <div class="container">
                    <div class="text-content">
                        <h4 class="title"><b>{post.title}</b></h4> 
                        <p>{post.tagline}</p>
                        <div class="author">
                            <div class="author-circle">
                                <img src="/profile-img.jpg" alt="Avatar"/>
                            </div>
                            <div>
                                <UserFirstAndLastName id=post.user_id.clone() />
                            </div>
                        </div>
                    </div>
                    <div class="post-image">
                        <img src="/profile-img.jpg" alt="Avatar"/>
                    </div>
                </div>
            </A>
        </div>
    }
}

#[component]
pub fn UserFirstAndLastName(cx: Scope, id: u32) -> impl IntoView {
    let user = create_resource(
        cx,
        move || (),
        move |_| get_user(cx, id)
    );

    view! {
        cx,
        <Transition fallback=move || view! {cx, <div/> }>
            {move || {
                let existing_user = {
                    move || {
                        user.read(cx).map(move |user| match user {
                            Err(e) => {
                                view! { cx, <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view(cx)
                            }
                            Ok(user) => {
                                view! { cx, {user.first_name}" "{user.last_name} }.into_view(cx)
                            }
                        })
                        .unwrap_or_default()
                    }
                };

                view! {
                    cx,
                    <p>{existing_user}</p>
                }
            }
        }
        </Transition>
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
                "Username: "
                <input type="text" placeholder="User ID" maxlength="32" name="username" class="auth-input" />
            </label>
            <br/>
            <label>
                "Password: "
                <input type="password" placeholder="Password" name="password" class="auth-input" />
            </label>
            <br/>
            <label>
                "Remember me? "
                <input type="checkbox" name="remember" class="auth-input" />
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
                "First name: "
                <input type="text" placeholder="First" maxlength="32" name="first_name" class="auth-input" />
            </label>
            <br/>
            <label>
                "Last name: "
                <input type="text" placeholder="Last" maxlength="32" name="last_name" class="auth-input" />
            </label>
            <br/>
            <label>
                "Username: "
                <input type="text" placeholder="Username" maxlength="32" name="username" class="auth-input" />
            </label>
            <br/>
            <label>
                "Password: "
                <input type="password" placeholder="Password" name="password" class="auth-input" />
            </label>
            <br/>
            <label>
                "Confirm Password: "
                <input type="password" placeholder="Password again" name="password_confirmation" class="auth-input" />
            </label>
            <br/>
            <label>
                "Remember me? "
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
