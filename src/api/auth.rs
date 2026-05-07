use dioxus::prelude::*;

// Our `auth` argument is a function argument hoisted to the argument list in the proc macro
#[post("/api/user/login", header: TypedHeader<Cookie>, auth: Session)]
pub async fn sign_in() -> ServerFnResult<()> {
    auth.login_user(2);
    Ok(())
}

#[get("/api/headers", headers: HeaderMap)]
async fn get_headers() -> ServerFnResult<String> {
    Ok(format!("{:#?}", headers))
}
