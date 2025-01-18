use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;

use super::{DataResponse, MessageResponse};


#[derive(Serialize, Deserialize)]
pub struct DeleteSurveyMD {
    id: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct GetSurveyMD {
    region_id: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateSurveyMD {
    id: Option<i32>,
    project_id: Option<i32>,
    survey_homepas: Option<String>,
    valid_homepas: Option<i16>,
    submit_vermit: Option<String>,
    valid_vermit: Option<i16>,
    status: Option<String>,
    issue: Option<String>,
    attachment_path: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SurveyMD {
    project_id: Option<i32>,
    survey_homepas: Option<String>,
    valid_homepas: Option<i16>,
    submit_vermit: Option<String>,
    valid_vermit: Option<i16>,
    status: Option<String>,
    issue: Option<String>,
    attachment_path: Option<String>,
}

pub async fn create_survey_md(req: web::Json<SurveyMD>, pool: web::Data<PgPool>) -> impl Responder {
    let project_id: i32 = req.project_id.clone().unwrap_or_default();

    // Query the database for the user
    let project = sqlx::query_as!(SurveyMD,
        "
        SELECT 
             project_id, survey_homepas, valid_homepas, submit_vermit, valid_vermit, status, issue, attachment_path
        FROM public.survey_md 
        WHERE project_id = $1",
        project_id
    )
    .fetch_one(&**pool)
    .await;

    if let Ok(_) = project {
        return HttpResponse::Conflict().json(MessageResponse {
            success: false,
            message: "Survey MD already exist".to_string(),
        });
    }

    let insert = sqlx::query!("
    INSERT INTO public.survey_md 
        (project_id, survey_homepas, valid_homepas, submit_vermit, valid_vermit, status, issue, attachment_path) 
    VALUES 
        ($1, $2, $3, $4, $5, $6, $7, $8) 
    returning id",
    project_id, req.survey_homepas, req.valid_homepas, req.submit_vermit, req.valid_vermit, req.status, req.issue, req.attachment_path    
    )
    .fetch_one(&**pool)
    .await;
    
    match insert {
        Ok(_id) => {
            HttpResponse::Ok().json(MessageResponse {
                success: true,
                message: "Insert data success".to_string(),
            })
        },
        Err(_e) => {
            HttpResponse::InternalServerError().json(MessageResponse {
                success: false,
                message: "Insert data failed".to_string(),
            })
        },
    }
}

pub async fn get_survey_md(req: web::Query<GetSurveyMD>, pool: web::Data<PgPool>) -> impl Responder {
    // Query the database for the user
    let survey = sqlx::query_as!(SurveyMD,
        "select
            project_id, survey_homepas, valid_homepas, submit_vermit, valid_vermit, status, issue, attachment_path
        FROM public.survey_md 
        where project_id in (select id from public.project where region_id = $1)",
        req.region_id
    )
    .fetch_all(&**pool)
    .await;

    if let Ok(project) = survey {
        let empty: Vec<SurveyMD> = vec![];
        let data = serde_json::to_value(project).unwrap_or(json!(empty));
        HttpResponse::Ok().json(DataResponse {
            success: true,
            data,
        })
    } else {
        let empty: Vec<SurveyMD> = vec![];
        let data = json!(empty);
        HttpResponse::Ok().json(DataResponse {
            success: true,
            data,
        })
    }
}

pub async fn update_survey_md(req: web::Query<UpdateSurveyMD>, pool: web::Data<PgPool>) -> impl Responder {
    // Query the database for the user
    let update = sqlx::query!("
    UPDATE public.survey_md 
    SET
        project_id=$2, 
        survey_homepas=$3, 
        valid_homepas=$4, 
        submit_vermit=$5, 
        valid_vermit=$6, 
        status=$7, 
        issue=$8, 
        attachment_path=$9
    WHERE
        id=$1
    returning id",
    req.id, req.project_id, req.survey_homepas, req.valid_homepas, req.submit_vermit, req.valid_vermit, req.status, req.issue, req.attachment_path    
    )
    .fetch_one(&**pool)
    .await;

    match update {
        Ok(_id) => {
            HttpResponse::Ok().json(MessageResponse {
                success: true,
                message: "Update data success".to_string(),
            })
        },
        Err(_e) => {
            HttpResponse::InternalServerError().json(MessageResponse {
                success: false,
                message: "Update data failed".to_string(),
            })
        },
    }
}

pub async fn delete_survey_md(req: web::Query<DeleteSurveyMD>, pool: web::Data<PgPool>) -> impl Responder {
    // Query the database for the user
    let update = sqlx::query!("
    DELETE FROM public.survey_md 
    WHERE id=$1
    returning id",
    req.id
    )
    .fetch_one(&**pool)
    .await;

    match update {
        Ok(_id) => {
            HttpResponse::Ok().json(MessageResponse {
                success: true,
                message: "Update data success".to_string(),
            })
        },
        Err(_e) => {
            HttpResponse::InternalServerError().json(MessageResponse {
                success: false,
                message: "Update data failed".to_string(),
            })
        },
    }
}