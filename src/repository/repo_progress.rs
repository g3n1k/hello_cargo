use sqlx::{PgPool, Result};

use crate::{dto_progression::ProgressionForm, UserCredential};


pub async fn get_progression(pool: &PgPool) -> Result<Vec<ProgressionForm>>{
    let str = format!("
    select 
        p.id,
        p.project_id,
        case when p.progress_type='po' then p2.po_id else p2.wo_id end project_name,
        p.date,
        t.team_name team,
        u.username pic,
        p2.region_id ,
        r1.name regional,
        r2.name witel,
        r.name psa,
        u.phone_number phone_number,
        p.description,
        p.status,
        p.phase,
        p.progress_type,
        p.attachment_path
    from public.progression p
    left join project p2 on p.project_id = p2.id 
    left join users_new u on p2.user_id = u.id
    left join user_team ut on u.id = ut.user_id
    left join team t on ut.team_id = t.id
    left join region2 r on p2.region_id = r.id
    left join region2 r1 on r.regional = r1.id
    left join region2 r2 on r.witel = r2.id");
    let query = sqlx::query_as::<_, ProgressionForm>(&str);
    query.fetch_all(pool).await
}

pub async fn get_progression_by_project_id(pool: &PgPool, project_id: i32) -> Result<Vec<ProgressionForm>>{
    let str = format!("
    select 
        p.id,
        p.project_id,
        case when p.progress_type='po' then p2.po_id else p2.wo_id end project_name,
        p.date,
        t.team_name team,
        u.username pic,
        p2.region_id ,
        r1.name regional,
        r2.name witel,
        r.name psa,
        u.phone_number phone_number,
        p.description,
        p.status,
        p.phase,
        p.progress_type,
        p.attachment_path
    from public.progression p
    left join project p2 on p.project_id = p2.id 
    left join users_new u on p2.user_id = u.id
    left join user_team ut on u.id = ut.user_id
    left join team t on ut.team_id = t.id
    left join region2 r on p2.region_id = r.id
    left join region2 r1 on r.regional = r1.id
    left join region2 r2 on r.witel = r2.id 
    where p.project_id={project_id}");
    let query = sqlx::query_as::<_, ProgressionForm>(&str);
    query.fetch_all(pool).await
}

pub async fn get_progression_by_region_ids(pool: &PgPool, user_info: UserCredential) -> Result<Vec<ProgressionForm>>{
    let region_ids = user_info.region_id.unwrap_or(vec![]);
    let in_syntax = region_ids.iter().enumerate().map(|(i,_)| format!("${}",i+1)).collect::<Vec<_>>().join(",");
    let region_level = user_info.region_level.unwrap_or_default().to_lowercase();
    let str = format!("
    select 
        p.id,
        p.project_id,
        case when p.progress_type='po' then p2.po_id else p2.wo_id end project_name,
        p.date,
        t.team_name team,
        u.username pic,
        p2.region_id ,
        r1.name regional,
        r2.name witel,
        r.name psa,
        u.phone_number phone_number,
        p.description,
        p.status,
        p.phase,
        p.progress_type,
        p.attachment_path
    from public.progression p
    left join project p2 on p.project_id = p2.id 
    left join users_new u on p2.user_id = u.id
    left join user_team ut on u.id = ut.user_id
    left join team t on ut.team_id = t.id
    left join region2 r on p2.region_id = r.id
    left join region2 r1 on r.regional = r1.id
    left join region2 r2 on r.witel = r2.id 
    where r.{region_level} in ({in_syntax})");
    // println!("query: {str}");
    let mut query = sqlx::query_as::<_, ProgressionForm>(&str);

    for id in region_ids {
        query = query.bind(id);
    }

    query.fetch_all(pool).await
}