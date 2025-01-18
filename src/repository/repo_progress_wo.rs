use sqlx::{postgres::PgQueryResult, PgPool, Result};

use crate::{CreateProgressionWO, ProgressionWO, ProgressionWOView, UserCredential};


pub async fn get_progression_wo(pool: &PgPool) -> Result<Vec<ProgressionWOView>>{
    let str = format!("
    select 
        p.id,
        p.wo_id,
        p2.wo_code wo_code,
        p2.wo_name wo_name,
        p.date,
        p2.region_id ,
        r.name || ', ' || r2.name || ', ' || r1.name location,
        r1.name regional,
        r2.name witel,
        r.name psa,
        p.survey_homepas,
        p.valid_homepas,
        p.submit_vermit,
        p.valid_vermit, 
        p.description,
        p.status,
        p.attachment_path
    from public.progression_wo p
    left join project_wo p2 on p.wo_id = p2.id 
    left join project_type p3 on p2.project_type_id = p3.id
    left join users_new u on p2.user_id = u.id
    left join user_team ut on u.id = ut.user_id
    left join team t on ut.team_id = t.id
    left join region2 r on p2.region_id = r.id
    left join region2 r1 on r.regional = r1.id
    left join region2 r2 on r.witel = r2.id");
    let query = sqlx::query_as::<_, ProgressionWOView>(&str);
    query.fetch_all(pool).await
}

pub async fn get_progression_wo_by_wo_id(pool: &PgPool, wo_id: i32) -> Result<Vec<ProgressionWOView>>{
    let str = format!("
    select 
        p.id,
        p.wo_id,
        p2.wo_code wo_code,
        p2.wo_name wo_name,
        p.date,
        p2.region_id ,
        r.name || ', ' || r2.name || ', ' || r1.name location,
        r1.name regional,
        r2.name witel,
        r.name psa,
        p.survey_homepas,
        p.valid_homepas,
        p.submit_vermit,
        p.valid_vermit, 
        p.description,
        p.status,
        p.attachment_path
    from public.progression_wo p
    left join project_wo p2 on p.wo_id = p2.id 
    left join project_type p3 on p2.project_type_id = p3.id
    left join users_new u on p2.user_id = u.id
    left join user_team ut on u.id = ut.user_id
    left join team t on ut.team_id = t.id
    left join region2 r on p2.region_id = r.id
    left join region2 r1 on r.regional = r1.id
    left join region2 r2 on r.witel = r2.id
    where p.wo_id={wo_id}");
    let query = sqlx::query_as::<_, ProgressionWOView>(&str);
    query.fetch_all(pool).await
}

pub async fn get_progression_wo_by_region_ids(pool: &PgPool, user_info: UserCredential) -> Result<Vec<ProgressionWOView>>{
    let region_ids = user_info.region_id.unwrap_or(vec![]);
    let in_syntax = region_ids.iter().enumerate().map(|(i,_)| format!("${}",i+1)).collect::<Vec<_>>().join(",");
    let region_level = user_info.region_level.unwrap_or_default().to_lowercase();
    let str = format!("
    select 
        p.id,
        p.wo_id,
        p2.wo_code wo_code,
        p2.wo_name wo_name,
        p.date,
        p2.region_id ,
        r.name || ', ' || r2.name || ', ' || r1.name location,
        r1.name regional,
        r2.name witel,
        r.name psa,
        p.survey_homepas,
        p.valid_homepas,
        p.submit_vermit,
        p.valid_vermit, 
        p.description,
        p.status,
        p.attachment_path
    from public.progression_wo p
    left join project_wo p2 on p.wo_id = p2.id 
    left join project_type p3 on p2.project_type_id = p3.id
    left join users_new u on p2.user_id = u.id
    left join user_team ut on u.id = ut.user_id
    left join team t on ut.team_id = t.id
    left join region2 r on p2.region_id = r.id
    left join region2 r1 on r.regional = r1.id
    left join region2 r2 on r.witel = r2.id
    where r.{region_level} in ({in_syntax})");
    // println!("query: {str}");
    let mut query = sqlx::query_as::<_, ProgressionWOView>(&str);

    for id in region_ids {
        query = query.bind(id);
    }

    query.fetch_all(pool).await
}

pub async fn check_progress_wo(pool: &PgPool, wo_id: Option<i32>, date: Option<String>) -> Result<ProgressionWO>{
    let str = format!("
    SELECT 
        *
    FROM public.progression_wo
    WHERE wo_id = $1 and date = $2");
    let query = sqlx::query_as::<_, ProgressionWO>(&str)
        .bind(wo_id)
        .bind(date);
    query.fetch_one(pool).await
}

pub async fn check_progress_wo_by_id(pool: &PgPool, id: i32) -> Result<ProgressionWO>{
    let str = format!("
    SELECT 
        *
    FROM public.progression_wo
    WHERE id = $1");
    let query = sqlx::query_as::<_, ProgressionWO>(&str)
        .bind(id);
    query.fetch_one(pool).await
}

pub async fn create_progress_wo(pool: &PgPool, param: &CreateProgressionWO) -> Result<ProgressionWO> {
    sqlx::query_as!(ProgressionWO, "
        INSERT INTO public.progression_wo
            (wo_id, date, description, attachment_path, status, survey_homepas, valid_homepas, submit_vermit, valid_vermit) 
        VALUES 
            ($1, $2, $3, $4, $5, $6, $7, $8, $9) 
        returning *",
            param.wo_id, param.date, param.description, param.attachment_path, param.status, param.survey_homepas, param.valid_homepas, param.submit_vermit, param.valid_vermit
        )
        .fetch_one(pool)
        .await
}

pub async fn update_progress_wo(pool: &PgPool, param: &ProgressionWO) -> Result<ProgressionWO> {
    sqlx::query_as!(ProgressionWO, "
        UPDATE public.progression_wo
        SET 
            description=$1, 
            attachment_path=$2, 
            status=$3, 
            survey_homepas=$4, 
            valid_homepas=$5, 
            submit_vermit=$6, 
            valid_vermit=$7,
            wo_id=$9
        WHERE id = $8
        returning *",
            param.description, param.attachment_path, param.status, param.survey_homepas, param.valid_homepas, param.submit_vermit, param.valid_vermit, param.id, param.wo_id
        )
        .fetch_one(pool)
        .await
}

pub async fn delete_progress_wo(pool: &PgPool, id: i32) -> Result<PgQueryResult> {
    sqlx::query!("DELETE FROM public.progression_wo WHERE id = $1", id)
        .execute(pool)
        .await
}