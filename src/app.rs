use crate::auth::*;
use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        pub fn register_server_functions() {
            _ = GetAllPosts::register();
            _ = GetUserPosts::register();
            _ = GetPost::register();
            _ = GetUserFromID::register();
            _ = GetUserFromUsername::register();
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

/// Gets a post entry from the posts table in the database
#[server(GetPost, "/api")]
pub async fn get_post(cx: Scope, id: u32) -> Result<Post, ServerFnError> {
    let pool = pool(cx)?;

    let post = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = ?")
                .bind(id)
                .fetch_one(&pool)
                .await
                .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    
    Ok(post)
}

/// Gets all posts entries from the posts table in the database
#[server(GetAllPosts, "/api")]
pub async fn get_all_posts(cx: Scope) -> Result<Vec<Post>, ServerFnError> {
    use futures::TryStreamExt;
    let pool = pool(cx)?;
    let mut posts = Vec::new();
    let mut rows = sqlx::query_as::<_, Post>("SELECT * FROM posts")
        .fetch(&pool);

    while let Some(row) = rows.try_next()
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))? {
            
        posts.push(row);
    }

    Ok(posts)
}

/// Gets all post entries for a specific user based on their id from the posts table in the database
#[server(GetUserPosts, "/api")]
pub async fn get_user_posts(cx: Scope, user_id: u32) -> Result<Vec<Post>, ServerFnError> {
    use futures::TryStreamExt;
    let pool = pool(cx)?;
    let mut posts = Vec::new();
    let mut rows = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE user_id = ?")
        .bind(user_id)
        .fetch(&pool);
    while let Some(row) = rows.try_next()
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))? {
            
        posts.push(row);
    }

    Ok(posts)
}

/// Gets all post entries from a specific user's subscriptions based on their id from the posts table in the database
#[server(GetSubscriptionsPosts, "/api")]
pub async fn get_subscriptions_posts(cx: Scope, user_id: u32) -> Result<Vec<Post>, ServerFnError> {
    use futures::TryStreamExt;
    let pool = pool(cx)?;
    let mut posts = Vec::new();
    let mut rows = sqlx::query_as::<_, Post>("SELECT p.* FROM posts p
                                                JOIN user_subscription subscriber ON p.user_id = subscriber.subscription_user_id
                                                WHERE subscriber.user_id = ?")
        .bind(user_id)
        .fetch(&pool);
    while let Some(row) = rows.try_next()
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))? {
            
        posts.push(row);
    }

    Ok(posts)
}

/// Gets a specific user based on their id from the users table in the database
#[server(GetUserFromID, "/api")]
pub async fn get_user_from_id(cx: Scope, id: u32) -> Result<User, ServerFnError> {
    let pool = pool(cx)?;

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
                .bind(id)
                .fetch_one(&pool)
                .await
                .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    
    Ok(user)
}

/// Gets a specific user based on their username from the users table in the database
#[server(GetUserFromUsername, "/api")]
pub async fn get_user_from_username(cx: Scope, username: String) -> Result<User, ServerFnError> {
    let pool = pool(cx)?;

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
                .bind(username)
                .fetch_one(&pool)
                .await
                .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    
    Ok(user)
}

/// Inserts a new post into the posts table in the database with its posted flag set to false, to save it as a draft that only the current user can see
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

/// Deletes a post based on its id from the posts table in the database
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

/// Main app component for rendering all routes
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
                            current_user.read(cx).map(|user| match user {
                                Err(e) => view! {cx,
                                    <A href="/signup"><p>"Signup"</p></A>
                                    <A href="/login"><p>"Login"</p></A>
                                    <span>{format!("Login error: {}", e.to_string())}</span>
                                }.into_view(cx),
                                Ok(None) => view! {cx,
                                    <A href="/signup"><p>"Signup"</p></A>
                                    <A href="/login"><p>"Login"</p></A>
                                }.into_view(cx),
                                Ok(Some(user)) => view! {cx,
                                    <A href="/settings">"Settings"</A>
                                    <div class="circle">
                                        <A href=format!("/u/{}", user.username)>
                                            <img src="/profile-img.jpg" alt="profile-img"/>
                                        </A>
                                    </div>
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
                    <Route path="" view=|cx| view! {
                        cx,
                        <AllPosts />
                    }/>
                    <Route path="/u/:username" view=move |cx| view! {
                        cx,
                        <UserProfile />
                    }/>
                    <Route path="/post/:post_id" view=move |cx| view! {
                        cx,
                        <Post/>
                    }/>
                    <Route path="/signup" view=move |cx| view! {
                        cx,
                        <Signup action=signup />
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
                </Routes>
            </main>
        </Router>
    }
}

/// PostList containing every post in the database
#[component]
pub fn AllPosts(cx: Scope) -> impl IntoView {
    let posts: Resource<(), Result<Vec<Post>, ServerFnError>> = create_resource(
        cx,
        move || (),
        move |_| get_all_posts(cx),
    );

    view! {
        cx,
        //TODO only pull so many and add buttons at the bottom to load the next and previous batch of posts
        <PostList posts=posts />
    }
}

/// PostList containing every post by a specified user based on their id in the database
#[component]
pub fn UserPosts(cx: Scope, id: u32) -> impl IntoView {
    let posts: Resource<(), Result<Vec<Post>, ServerFnError>> = create_resource(
        cx,
        move || (),
        move |_| get_user_posts(cx, id),
    );

    view! {
        cx,
        <PostList posts=posts />
    }
}

/// Generic PostList component for rendering a vector of posts as PostCard components
#[component]
pub fn PostList(cx: Scope, posts: Resource<(), Result<Vec<Post>, ServerFnError>>) -> impl IntoView {
    view! {
        cx,
        <div>
            <Transition fallback=move || view! {cx, <div/> }>
                {move || {
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
                                            <PostCard post=post />
                                        }
                                    })
                                    .collect_view(cx)
                            }
                        }
                    })
                    .unwrap_or_default()
                }}
            </Transition>
        </div>
    }
}

/// Displays info about a Post alongside a thumbnail to entice a user to read it
#[component]
pub fn PostCard(cx: Scope, post: Post) -> impl IntoView {
    let post_route = String::from("/post/") + post.id.to_string().as_str();
    
    view! {
        cx,
        <div class="card">
            <div class="container">
                <div class="text-content">
                    <A class="text-content-link" href=post_route.clone()>
                        <h4 class="title"><b>{post.title}</b></h4> 
                        <p>{post.tagline}</p>
                    </A>
                    <AuthorLink id=post.user_id />
                </div>
                <A class="post-image-link" href=post_route>
                    <div class="post-image">
                        <img src="/profile-img.jpg" alt="Avatar"/>
                    </div>
                </A>
            </div>
        </div>
    }
}

/// Displays a user's profile picture alongside a link to their profile, most commonly used for displaying authors in PostCard components
#[component]
pub fn AuthorLink(cx: Scope, id: u32) -> impl IntoView {
    let user = create_resource(
        cx,
        move || (),
        move |_| get_user_from_id(cx, id)
    );

    view! {
        cx,
        <Transition fallback=move || view! {cx, {} }>
            {move || {
                user.read(cx).map(move |user| match user {
                    Err(e) => {
                        view! { cx, <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view(cx)
                    }
                    Ok(user) => {
                        view! { 
                            cx, 
                            <div class="author">
                                <div class="author-circle">
                                    <A href=format!("/u/{}", user.username)><img src="/profile-img.jpg" alt="Avatar"/></A>
                                </div>
                                <p>
                                    <UserProfileLink id=user.id />
                                </p>
                            </div>
                        }.into_view(cx)
                    }
                })
                .unwrap_or_default()
            }}
        </Transition>
    }
}

/// Displays a user's first and last name based on their id
#[component]
pub fn UserFirstAndLastName(cx: Scope, id: u32) -> impl IntoView {
    let user = create_resource(
        cx,
        move || (),
        move |_| get_user_from_id(cx, id)
    );

    view! {
        cx,
        <Transition fallback=move || view! {cx, {} }>
            {move || {
                user.read(cx).map(move |user| match user {
                    Err(e) => {
                        view! { cx, <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view(cx)
                    }
                    Ok(user) => {
                        view! { cx, {user.first_name}" "{user.last_name} }.into_view(cx)
                    }
                })
                .unwrap_or_default()
            }}
        </Transition>
    }
}

/// Displays a Post based on the post_id in the URL route
#[component]
pub fn Post(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    
    let get_post_id = move || params.with(|params| params.get("post_id").cloned().unwrap_or_default().parse::<u32>().unwrap_or_default());
    let post_id: u32 = get_post_id();
    
    let post = create_resource(
        cx,
        move || (),
        move |_| get_post(cx, post_id),
    );

    view! {
        cx,
        <div>
            <Transition fallback=move || view! {cx, <p>"Loading..."</p> }>
                {move || {
                    post.read(cx).map(move |post| match post {
                        Err(e) => {
                            view! { cx, <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view(cx)
                        }
                        Ok(post) => {
                            view! {
                                cx,
                                <div class="post">
                                    <h1>{post.title}</h1>
                                    //TODO make series component and get the series <h2>{post.series}</h2>
                                    <h4><UserProfileLink id=post.user_id /></h4>
                                    //TODO make date component and get post date here <h4>{post.title}</h4>
                                    <p>{post.content}</p>
                                </div>
                            }
                            .into_view(cx)
                        }
                    })
                    .unwrap_or_default()
                }}
            </Transition>
        </div>
    }
}

/// Displays a user's first and last name as a link to their profile page
#[component]
pub fn UserProfileLink(cx: Scope, id: u32) -> impl IntoView {
    let user = create_resource(
        cx,
        move || (),
        move |_| get_user_from_id(cx, id)
    );

    view! {
        cx,
        <Transition fallback=move || view! {cx, {} }>
            {move || {
                user.read(cx).map(move |user| match user {
                    Err(e) => {
                        view! { cx, <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view(cx)
                    }
                    Ok(user) => {
                        view! { cx, <A href=format!("/u/{}", user.username)><UserFirstAndLastName id=user.id /></A> }.into_view(cx)
                    }
                })
                .unwrap_or_default()
            }}
        </Transition>
    }
}

/// Displays a user's information alongside their most recent posts
#[component]
pub fn UserProfile(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    
    let get_username = move || params.with(|params| params.get("username").cloned().unwrap_or_default());
    let username: String = get_username();
    
    let user = create_resource(
        cx,
        move || (),
        move |_| get_user_from_username(cx, username.clone()),
    );

    view! {
        cx,
        <div>
            <Transition fallback=move || view! {cx, <p>"Loading..."</p> }>
                {move || {
                    user.read(cx).map(move |user| match user {
                        Err(e) => {
                            view! { cx, <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view(cx)
                        }
                        Ok(user) => {
                            view! {
                                cx,
                                <div class="user">
                                    <h1><UserFirstAndLastName id=user.id /></h1>
                                    //TODO make profile picture component and get it here next to their name in flex box
                                    <UserPosts id=user.id />
                                </div>
                            }
                            .into_view(cx)
                        }
                    })
                    .unwrap_or_default()
                }}
            </Transition>
        </div>
    }
}

/// Displays a login page for logging a user into BBlog
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

/// Displays a login page for creating an account on BBlog
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

/// Displays a logout page for logging a user out of BBlog
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
