use sqlx::{postgres::PgQueryResult, PgPool, Result};

use crate::{CreateProgressionPO, ProgressionPO, ProgressionPOView, UserCredential};


pub async fn get_progression_po(pool: &PgPool) -> Result<Vec<ProgressionPOView>>{
    let str = format!("
    select 
        p.id,
        p.po_id,
        p2.po_code,
        p2.po_name,
        p.date,
        p3.region_id ,
        r1.name || ', ' || r2.name || ', ' || r.name location,
        r1.name regional,
        r2.name witel,
        r.name psa,
        u.username pic,
        t.team_name team,
        p.description,
        p.status,
        p.phase,
        p.attachment_path
    from public.progression_po p
    left join project_po p2 on p.po_id = p2.id 
    left join project_wo p3 on p2.wo_id = p3.id 
    left join users_new u on p2.user_id = u.id
    left join user_team ut on u.id = ut.user_id
    left join team t on ut.team_id = t.id
    left join region2 r on p3.region_id = r.id
    left join region2 r1 on r.regional = r1.id
    left join region2 r2 on r.witel = r2.id");
    let query = sqlx::query_as::<_, ProgressionPOView>(&str);
    query.fetch_all(pool).await
}

pub async fn get_progression_po_by_project_id(pool: &PgPool, project_id: i32) -> Result<Vec<ProgressionPOView>>{
    let str = format!("
    select 
        p.id,
        p.po_id,
        p2.po_code,
        p2.po_name,
        p.date,
        p3.region_id ,
        r1.name || ', ' || r2.name || ', ' || r.name location,
        r1.name regional,
        r2.name witel,
        r.name psa,
        u.username pic,
        t.team_name team,
        p.description,
        p.status,
        p.phase,
        p.attachment_path
    from public.progression_po p
    left join project_po p2 on p.po_id = p2.id 
    left join project_wo p3 on p2.wo_id = p3.id 
    left join users_new u on p2.user_id = u.id
    left join user_team ut on u.id = ut.user_id
    left join team t on ut.team_id = t.id
    left join region2 r on p3.region_id = r.id
    left join region2 r1 on r.regional = r1.id
    left join region2 r2 on r.witel = r2.id
    where p.project_id={project_id}");
    let query = sqlx::query_as::<_, ProgressionPOView>(&str);
    query.fetch_all(pool).await
}

pub async fn get_progression_po_by_region_ids(pool: &PgPool, user_info: UserCredential) -> Result<Vec<ProgressionPOView>>{
    let region_ids = user_info.region_id.unwrap_or(vec![]);
    let in_syntax = region_ids.iter().enumerate().map(|(i,_)| format!("${}",i+1)).collect::<Vec<_>>().join(",");
    let region_level = user_info.region_level.unwrap_or_default().to_lowercase();
    let str = format!("
    select 
        p.id,
        p.po_id,
        p2.po_code,
        p2.po_name,
        p.date,
        p3.region_id ,
        r1.name || ', ' || r2.name || ', ' || r.name location,
        r1.name regional,
        r2.name witel,
        r.name psa,
        u.username pic,
        t.team_name team,
        p.description,
        p.status,
        p.phase,
        p.attachment_path
    from public.progression_po p
    left join project_po p2 on p.po_id = p2.id 
    left join project_wo p3 on p2.wo_id = p3.id 
    left join users_new u on p2.user_id = u.id
    left join user_team ut on u.id = ut.user_id
    left join team t on ut.team_id = t.id
    left join region2 r on p3.region_id = r.id
    left join region2 r1 on r.regional = r1.id
    left join region2 r2 on r.witel = r2.id
    where r.{region_level} in ({in_syntax})");
    // println!("query: {str}");
    let mut query = sqlx::query_as::<_, ProgressionPOView>(&str);

    for id in region_ids {
        query = query.bind(id);
    }

    query.fetch_all(pool).await
}

pub async fn check_progress_po(pool: &PgPool, param: &CreateProgressionPO) -> Result<ProgressionPO>{
    let str = format!("
    SELECT 
        *
    FROM public.progression_po
    WHERE po_id = $1 and date = $2");
    let query = sqlx::query_as::<_, ProgressionPO>(&str)
        .bind(param.po_id)
        .bind(&param.date);
    query.fetch_one(pool).await
}

pub async fn check_progress_po_by_id(pool: &PgPool, id: i32) -> Result<ProgressionPO>{
    let str = format!("
    SELECT 
        *
    FROM public.progression_po
    WHERE id = $1");
    let query = sqlx::query_as::<_, ProgressionPO>(&str)
        .bind(id);
    query.fetch_one(pool).await
}

pub async fn create_progress_po(pool: &PgPool, param: &CreateProgressionPO) -> Result<ProgressionPO> {
    sqlx::query_as!(ProgressionPO, "
        INSERT INTO public.progression_po
            (po_id, date, description, attachment_path, status, phase) 
        VALUES 
            ($1, $2, $3, $4, $5, $6) 
        returning *",
            param.po_id, param.date, param.description, param.attachment_path, param.status, param.phase
        )
        .fetch_one(pool)
        .await
}

pub async fn update_progress_po(pool: &PgPool, param: &ProgressionPO) -> Result<ProgressionPO> {
    sqlx::query_as!(ProgressionPO, "
        UPDATE public.progression_po
        SET 
            description=$1, 
            attachment_path=$2, 
            status=$3, 
            phase=$4
        where id = $5
        returning *",
            param.description, param.attachment_path, param.status, param.phase, param.id
        )
        .fetch_one(pool)
        .await
}

pub async fn delete_progress_po(pool: &PgPool, id: i32) -> Result<PgQueryResult> {
    sqlx::query!("DELETE FROM public.progression_po WHERE id = $1", id)
        .execute(pool)
        .await
}