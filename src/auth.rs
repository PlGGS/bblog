use cfg_if::cfg_if;
use leptos::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use sqlx::SqlitePool;
    use axum_session_auth::{SessionSqlitePool, Authentication, HasPermission};
    use bcrypt::{hash, verify, DEFAULT_COST};
    pub type AuthSession = axum_session_auth::AuthSession<User, u32, SessionSqlitePool, SqlitePool>;
        
    /// Gets pool from database
    pub fn pool(cx: Scope) -> Result<SqlitePool, ServerFnError> {
    use_context::<SqlitePool>(cx)
            .ok_or("Pool missing.")
            .map_err(|e| ServerFnError::ServerError(e.to_string()))
    }

    /// Gets the current authentication session from the application context variable cx
    pub fn auth(cx: Scope) -> Result<AuthSession, ServerFnError> {
        use_context::<AuthSession>(cx)
            .ok_or("Auth session missing.")
            .map_err(|e| ServerFnError::ServerError(e.to_string()))
    }
}}

/// Current user struct for authentication
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password_hash: String,
    pub permissions: HashSet<String>,
}

/// Default guest user struct
impl Default for User {
    fn default() -> Self {
        let permissions = HashSet::new();

        Self {
            id: 0,
            first_name: "Guest".into(),
            last_name: "Guestington".into(),
            username: "Guest".into(),
            password_hash: "".into(),
            permissions,
        }
    }
}

cfg_if! {
if #[cfg(feature = "ssr")] {
    use async_trait::async_trait;

    /// Get current user from database by id
    impl User {
        pub async fn get(id: u32, pool: &SqlitePool) -> Option<Self> {
            let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE id = ?")
                .bind(id)
                .fetch_one(pool)
                .await
                .ok()?;

            //lets just get all the tokens the user can use, we will only use the full permissions if modifing them.
            let sql_user_perms = sqlx::query_as::<_, SqlPermissionTokens>(
                "SELECT token FROM user_permissions WHERE user_id = ?;",
            )
            .bind(id)
            .fetch_all(pool)
            .await
            .ok()?;

            Some(sqluser.into_user(Some(sql_user_perms)))
        }

        /// Get current user from database by username
        pub async fn get_from_username(name: String, pool: &SqlitePool) -> Option<Self> {
            let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE username = ?")
                .bind(name)
                .fetch_one(pool)
                .await
                .ok()?;

            //lets just get all the tokens the user can use, we will only use the full permissions if modifing them.
            let sql_user_perms = sqlx::query_as::<_, SqlPermissionTokens>(
                "SELECT token FROM user_permissions WHERE user_id = ?;",
            )
            .bind(sqluser.id)
            .fetch_all(pool)
            .await
            .ok()?;

            Some(sqluser.into_user(Some(sql_user_perms)))
        }
    }

    #[derive(sqlx::FromRow, Clone)]
    pub struct SqlPermissionTokens {
        pub token: String,
    }

    #[async_trait]
    impl Authentication<User, u32, SqlitePool> for User {
        /// Authenticates current user
        async fn load_user(userid: u32, pool: Option<&SqlitePool>) -> Result<User, anyhow::Error> {
            let pool = pool.unwrap();

            User::get(userid, pool)
                .await
                .ok_or_else(|| anyhow::anyhow!("Cannot get user"))
        }

        fn is_authenticated(&self) -> bool {
            true
        }

        fn is_active(&self) -> bool {
            true
        }

        fn is_anonymous(&self) -> bool {
            false
        }
    }

    #[async_trait]
    impl HasPermission<SqlitePool> for User {
        async fn has(&self, perm: &str, _pool: &Option<&SqlitePool>) -> bool {
            self.permissions.contains(perm)
        }
    }

    #[derive(sqlx::FromRow, Clone)]
    pub struct SqlUser {
        pub id: u32,
        pub first_name: String,
        pub last_name: String,
        pub username: String,
        pub password_hash: String,
    }

    impl SqlUser {
        /// Serializes current user with permissions from database
        pub fn into_user(self, sql_user_perms: Option<Vec<SqlPermissionTokens>>) -> User {
            User {
                id: self.id,
                first_name: self.first_name,
                last_name: self.last_name,
                username: self.username,
                password_hash: self.password_hash,
                permissions: if let Some(user_perms) = sql_user_perms {
                    user_perms
                        .into_iter()
                        .map(|x| x.token)
                        .collect::<HashSet<String>>()
                } else {
                    HashSet::<String>::new()
                },
            }
        }
    }
}
}

/// Gets the currently authenticated User
#[server(GetCurrentUser, "/api")]
pub async fn get_current_user(cx: Scope) -> Result<Option<User>, ServerFnError> {
    let auth = auth(cx)?;

    Ok(auth.current_user)
}

/// Handles a login request
#[server(Login, "/api")]
pub async fn login(
    cx: Scope,
    username: String,
    password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    let pool = pool(cx)?;
    let auth = auth(cx)?;

    let user: User = User::get_from_username(username, &pool)
        .await
        .ok_or("User does not exist.")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    match verify(password, &user.password_hash)
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?
    {
        true => {
            auth.login_user(user.id);
            auth.remember_user(remember.is_some());
            leptos_axum::redirect(cx, "/");
            Ok(())
        }
        false => Err(ServerFnError::ServerError(
            "Password does not match.".to_string(),
        )),
    }
}

/// Handles a signup request
#[server(Signup, "/api")]
pub async fn signup(
    cx: Scope,
    first_name: String,
    last_name: String,
    username: String,
    password: String,
    password_confirmation: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    let pool = pool(cx)?;
    let auth = auth(cx)?;

    if password != password_confirmation {
        return Err(ServerFnError::ServerError(
            "Passwords did not match.".to_string(),
        ));
    }

    let password_hash = hash(password, DEFAULT_COST).unwrap();

    sqlx::query("INSERT INTO users (first_name, last_name, username, password_hash) 
                     VALUES (?, ?, ?, ?)")
        .bind(first_name)
        .bind(last_name)
        .bind(username.clone())
        .bind(password_hash)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    let user = User::get_from_username(username, &pool)
        .await
        .ok_or("Signup failed: User does not exist.")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    auth.login_user(user.id);
    auth.remember_user(remember.is_some());

    leptos_axum::redirect(cx, "/");

    Ok(())
}

/// Handles a logout request
#[server(Logout, "/api")]
pub async fn logout(cx: Scope) -> Result<(), ServerFnError> {
    let auth = auth(cx)?;

    auth.logout_user();
    leptos_axum::redirect(cx, "/");

    Ok(())
}
