use chrono::Local;
use sqlx::{Error, PgPool};
use crate::{CreateRecon, ReconPo, ReconView, Reconciliation};


pub async fn get_recon_by_regions(pool: &PgPool, region_ids: Option<Vec<i32>>) -> Result<Vec<ReconView>,Error>{
    let region_ids = region_ids.unwrap_or(vec![]);
    let in_syntax = region_ids.iter().enumerate().map(|(i,_)| format!("${}",i+1)).collect::<Vec<_>>().join(",");
    let str = format!("
    with base as (
        select 
            r.id,
            r.po_id,
            pp.po_name po_name,
            pp.wo_id wo_id,
            pw.wo_name wo_name,
            pt.type project_type,
            pp2.status status,
            pw.survey_md_price,
            r.survey_md_check survey_md_check,
            pp.material_price,
            r.material_check material_check,
            pp.service_price,
            r.service_check service_check,
            r.last_update,
            r2.witel,
            rank() over (partition by pp2.po_id order by pp2.date  desc) as r
        from recon r
        left join project_po pp on r.po_id = pp.id 
        left join project_wo pw on pp.wo_id = pw.id
        left join project_type pt on pw.project_type_id = pt.id
        left join progression_po pp2 on pp2.po_id = r.po_id
        left join region2 r2 on pw.region_id = r2.id
    )
    select 
        id, po_id, po_name, wo_name, project_type, status, survey_md_price, survey_md_check, material_price, material_check, service_check, service_price, last_update, r
    from base
    where r = 1 and witel in ({})", in_syntax);
    // println!("str query: {str}");

    let mut query = sqlx::query_as::<_, ReconView>(&str);
    for id in region_ids {
        query = query.bind(id);
    }
    query.fetch_all(pool).await

}

pub async fn get_recon_all(pool: &PgPool) -> Result<Vec<ReconView>,Error>{
    let str = format!("
    with base as (
        select 
            r.id,
            r.po_id,
            pp.po_name po_name,
            pp.wo_id wo_id,
            pw.wo_name wo_name,
            pt.type project_type,
            pp2.status status,
            pw.survey_md_price,
            r.survey_md_check survey_md_check,
            pp.material_price,
            r.material_check material_check,
            pp.service_price,
            r.service_check service_check,
            r.last_update,
            r2.witel,
            rank() over (partition by pp2.po_id order by pp2.date  desc) as r
        from recon r
        left join project_po pp on r.po_id = pp.id 
        left join project_wo pw on pp.wo_id = pw.id
        left join project_type pt on pw.project_type_id = pt.id
        left join progression_po pp2 on pp2.po_id = r.po_id
        left join region2 r2 on pw.region_id = r2.id
    )
    select 
        id, po_id, po_name, wo_name, project_type, status, survey_md_price, survey_md_check, material_price, material_check, service_check, service_price, last_update, r
    from base
    where r = 1");
        
    sqlx::query_as::<_, ReconView>(&str)
    .fetch_all(pool)
    .await
}

pub async fn check_recon(pool: &PgPool, param: &CreateRecon) -> Result<Reconciliation, Error> {
    sqlx::query_as!(Reconciliation,
        "
        SELECT 
        *
        FROM public.recon
        WHERE po_id = $1",
        param.po_id
    )
    .fetch_one(pool)
    .await
}

pub async fn get_po_recon_all(pool: &PgPool) -> Result<Vec<ReconPo>, Error> {
    sqlx::query_as!(ReconPo,
        "
        select 
            pp2.id po_id,
            pw.id wo_id,
            pp2.po_name,
            pw.wo_name,
            pw.wo_name || ' | ' || pp2.po_name project_name,
            pw.survey_md_price,
            false survey_md_check,
            pp2.material_price,
            false material_check,
            pp2.service_price,
            false service_check,
            status
        from progression_po pp 
        left join project_po pp2 on pp.po_id = pp2.id
        left join project_wo pw on pp2.wo_id = pw.id
        where status = 'Done';
"
    )
    .fetch_all(pool)
    .await
}

pub async fn get_po_recon_by_regions(pool: &PgPool, region_ids: Option<Vec<i32>>) -> Result<Vec<ReconPo>, Error> {
    let region_ids = region_ids.unwrap_or(vec![]);
    let in_syntax = region_ids.iter().enumerate().map(|(i,_)| format!("${}",i+1)).collect::<Vec<_>>().join(",");
    let str = format!("
        select 
            pp2.id po_id,
            pw.id wo_id,
            pp2.po_name,
            pw.wo_name,
            pw.wo_name || ' | ' || pp2.po_name project_name,
            pw.survey_md_price,
            false survey_md_check,
            pp2.material_price,
            false material_check,
            pp2.service_price,
            false service_check,
            status
        from progression_po pp 
        left join project_po pp2 on pp.po_id = pp2.id
        left join project_wo pw on pp2.wo_id = pw.id
        left join region2 r on pw.region_id = r.id 
        where status = 'Done' and r.witel in ({});
", in_syntax);

// println!("sql: {str}");

let mut sql = sqlx::query_as::<_, ReconPo>(&str);
for region_id in region_ids {
    sql = sql.bind(region_id);
}
sql
    .fetch_all(pool)
    .await
}

pub async fn check_recon_by_id(pool: &PgPool, id: i32) -> Result<Reconciliation,Error>{
    sqlx::query_as!(Reconciliation,
        "
        SELECT 
            *
        FROM public.recon 
        WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn insert_recon(pool: &PgPool, param: &CreateRecon) -> Result<Reconciliation,Error>{
    sqlx::query_as!(Reconciliation, "
        INSERT INTO public.recon 
            (po_id,survey_md_check,material_check,service_check)
        VALUES 
            ($1, $2, $3, $4) 
        returning *",
        param.po_id, param.survey_md_check, param.material_check, param.service_check
    )
        .fetch_one(pool)
        .await
}

pub async fn update_recon(pool: &PgPool, param: &Reconciliation) -> Result<Reconciliation,Error>{
    let now = Local::now().format("%F %T").to_string();
    sqlx::query_as!(Reconciliation,
        "
        UPDATE public.recon
        SET
            po_id=$1,
            survey_md_check=$2,
            material_check=$3,
            service_check=$4,
            last_update=$6
        WHERE id = $5
        RETURNING *
        ",
        param.po_id, param.survey_md_check, param.material_check, param.service_check, param.id, now
    )
    .fetch_one(pool)
    .await
}

pub async fn delete_recon(pool: &PgPool, id: i32) -> Result<Reconciliation,Error>{
    sqlx::query_as!(Reconciliation,"
        DELETE FROM public.recon WHERE id = $1
        returning *", id)
        .fetch_one(pool)
        .await
}