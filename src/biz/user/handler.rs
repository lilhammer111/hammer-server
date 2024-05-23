use actix_web::{Error, get, HttpMessage, HttpRequest, HttpResponse, post, web};
use crate::AppState;
use deadpool_postgres::{Client as PgClient};
use log::debug;
use crate::biz::user::communicator::{UserReq, UserResp};
use crate::biz::user::recorder::{query_account_by_id, update_account};
use crate::infra::error::biz::BizKind::{ClaimsNotFound};
use crate::infra::error::error::{Kind, ServerError};
use crate::infra::error::error::Kind::BizError;
use crate::infra::middleware::jwt::Claims;


#[get("")]
async fn get_user_info(req: HttpRequest, app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let pg_client: PgClient = app_state.pool.get().await?;

    let user_id = req.extensions()
        .get::<Claims>()
        .ok_or_else(|| {
            ServerError::build()
                .belong(BizError(ClaimsNotFound))
                .because(Box::new("unknown".to_string()))
                .done()
        })?
        .sub;

    let queried_user = query_account_by_id(&pg_client, user_id).await?;

    let resp: UserResp = queried_user.into();

    Ok(
        HttpResponse::Ok()
            .json(
                resp
            )
    )
}

#[post("")]
async fn update_user_info(req: HttpRequest, app_state: web::Data<AppState>, req_body: web::Json<UserReq>) -> Result<HttpResponse, Error> {
    debug!("update user req body: {:?}", req_body);

    let pg_client: PgClient = app_state.pool.get().await?;

    let info_to_update = req_body.into_inner();

    let user_id = req.extensions()
        .get::<Claims>()
        .ok_or_else(|| BizError::JwtError)?
        .sub;

    let updated_user_info = update_account(&pg_client, user_id, info_to_update).await?;

    let resp: UserResp = updated_user_info.into();

    Ok(
        HttpResponse::Ok()
            .json(
                resp
            )
    )
}
