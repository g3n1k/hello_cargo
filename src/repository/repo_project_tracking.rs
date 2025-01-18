use sqlx::{postgres::PgQueryResult, PgPool, Result};

use crate::{CreateProjectTracking, ProjectTracking, UpdateProjectTracking};

pub async fn get_project_tracking_by_project_id(pool: &PgPool, project_id: i32) -> Result<Vec<ProjectTracking>>{
    let str = format!("
    SELECT 
        id, project_id ,stage
    FROM public.project_tracking
    where project_id = $1");
    let query = sqlx::query_as::<_, ProjectTracking>(&str)
        .bind(project_id);
    query.fetch_all(pool).await
}

pub async fn get_project_tracking(pool: &PgPool) -> Result<Vec<ProjectTracking>>{
    let str = format!("
    SELECT 
        id, project_id ,stage
    FROM public.project_tracking");
    let query = sqlx::query_as::<_, ProjectTracking>(&str);
    query.fetch_all(pool).await
}

pub async fn check_project_tracking(pool: &PgPool, param: &CreateProjectTracking) -> Result<ProjectTracking>{
    let str = format!("
    SELECT 
        id, project_id ,stage
    FROM public.project_tracking
    WHERE project_id = $1 and stage = $2");
    let query = sqlx::query_as::<_, ProjectTracking>(&str)
        .bind(param.project_id)
        .bind(&param.stage);
    query.fetch_one(pool).await
}

pub async fn check_project_tracking_by_id(pool: &PgPool, id: i32) -> Result<ProjectTracking>{
    let str = format!("
    SELECT 
        id, project_id ,stage
    FROM public.project_tracking
    WHERE id = $1");

    sqlx::query_as::<_, ProjectTracking>(&str)
        .bind(id)
        .fetch_one(pool).await
}

pub async fn create_project_tracking(pool: &PgPool, param: &CreateProjectTracking) -> Result<ProjectTracking> {
    sqlx::query_as!(ProjectTracking, "
        INSERT INTO public.project_tracking
            (project_id, stage) 
        VALUES 
            ($1, $2) 
        returning *",
            param.project_id, param.stage
        )
        .fetch_one(pool)
        .await
}

pub async fn update_project_tracking(pool: &PgPool, param: &UpdateProjectTracking) -> Result<ProjectTracking> {
    sqlx::query_as!(ProjectTracking, "
        UPDATE public.project_tracking
        SET 
            project_id=$1, 
            stage=$2
        WHERE id=$3
        returning *",
            param.project_id, param.stage, param.id
        )
        .fetch_one(pool)
        .await
}

pub async fn delete_project_tracking(pool: &PgPool, id: i32) -> Result<PgQueryResult> {
    sqlx::query!("DELETE FROM public.project_tracking WHERE id = $1", id)
        .execute(pool)
        .await
}