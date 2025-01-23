use sqlx::{Error, PgPool};
use crate::{BillingHighlightView, ProgressionPOHighlightView, ProgressionWOHighlightView, ProjectHighlightView, ReconHighlightView, UserManagementHighlightView};


pub async fn get_project_highlight_all(pool: &PgPool) -> Result<ProjectHighlightView,Error> {
    let str = format!("
        select 
            coalesce(count(1),0) total_project,
            coalesce(sum(pp.cable),0) cable,
            coalesce(sum(pw.survey_md_price),0) survey_md_price,
            coalesce(sum(pp.material_price),0) material_price,
            coalesce(sum(pp.service_price),0) service_price
        from project_wo pw
        left join project_po pp on pw.id = pp.wo_id
    ");
    // println!("str query: {str}");

    sqlx::query_as::<_, ProjectHighlightView>(&str)
    .fetch_one(pool).await
}

pub async fn get_project_highlight_by_regions(pool: &PgPool, region_ids: Option<Vec<i32>>) -> Result<ProjectHighlightView,Error> {
    let region_ids = region_ids.unwrap_or(vec![]);
    let in_syntax = region_ids.iter().enumerate().map(|(i,_)| format!("${}",i+1)).collect::<Vec<_>>().join(",");
    let str = format!("
        select 
            coalesce(count(1),0) total_project,
            coalesce(sum(pp.cable),0) cable,
            coalesce(sum(pw.survey_md_price),0) survey_md_price,
            coalesce(sum(pp.material_price),0) material_price,
            coalesce(sum(pp.service_price),0) service_price
        from project_wo pw
        left join project_po pp on pw.id = pp.wo_id
        left join region2 r on pw.region_id = r.id 
        where r.witel in ({});
    ", in_syntax);
    // println!("str query: {str}");

    let mut sql = sqlx::query_as::<_, ProjectHighlightView>(&str);

    for r in region_ids {
        sql = sql.bind(r);
    }

    sql.fetch_one(pool).await
}

pub async fn get_progression_po_highlight_all(pool: &PgPool) -> Result<ProgressionPOHighlightView,Error> {
    let str = format!("
        select 
            count(1) total_progress,
            coalesce(sum(pp2.cable),0) cable,
            coalesce(sum(pp2.pole),0) pole,
            coalesce(sum(pp2.port),0) port
        from progression_po pp
        left join project_po pp2 on pp2.id = pp.po_id
    ");
    // println!("str query: {str}");

    sqlx::query_as::<_, ProgressionPOHighlightView>(&str)
    .fetch_one(pool).await
}

pub async fn get_progression_po_highlight_by_regions(pool: &PgPool, region_ids: Option<Vec<i32>>) -> Result<ProgressionPOHighlightView,Error> {
    let region_ids = region_ids.unwrap_or(vec![]);
    let in_syntax = region_ids.iter().enumerate().map(|(i,_)| format!("${}",i+1)).collect::<Vec<_>>().join(",");
    let str = format!("
        select 
            count(1) total_progress,
            coalesce(sum(pp2.cable),0) cable,
            coalesce(sum(pp2.pole),0) pole,
            coalesce(sum(pp2.port),0) port
        from progression_po pp
        left join project_po pp2 on pp2.id = pp.po_id
        left join project_wo pw on pp2.wo_id = pw.id
        left join region2 r on pw.region_id = r.id 
        where r.witel in ({});
    ", in_syntax);
    // println!("str query: {str}");

    let mut sql = sqlx::query_as::<_, ProgressionPOHighlightView>(&str);

    for r in region_ids {
        sql = sql.bind(r);
    }

    sql.fetch_one(pool).await
}

pub async fn get_progression_wo_highlight_all(pool: &PgPool) -> Result<ProgressionWOHighlightView,Error> {
    let str = format!("
        select 
            count(1) total_progress,
            coalesce(sum(case when trim(lower(pw.survey_homepas)) = 'done' then 1 else 0 end),0) survey_homepas,
            coalesce(sum(pw.valid_homepas),0) valid_homepas,
            coalesce(sum(case when trim(lower(pw.submit_vermit)) = 'done' then 1 else 0 end),0) submit_vermit,
            coalesce(sum(pw.valid_vermit),0) valid_vermit
        from progression_wo pw
        left join project_wo pw2 on pw2.id = pw.wo_id
    ");
    // println!("str query: {str}");

    sqlx::query_as::<_, ProgressionWOHighlightView>(&str)
    .fetch_one(pool).await
}

pub async fn get_progression_wo_highlight_by_regions(pool: &PgPool, region_ids: Option<Vec<i32>>) -> Result<ProgressionWOHighlightView,Error> {
    let region_ids = region_ids.unwrap_or(vec![]);
    let in_syntax = region_ids.iter().enumerate().map(|(i,_)| format!("${}",i+1)).collect::<Vec<_>>().join(",");
    let str = format!("
        select 
            count(1) total_progress,
            coalesce(sum(case when trim(lower(pw.survey_homepas)) = 'done' then 1 else 0 end, 0) survey_homepas,
            coalesce(sum(pw.valid_homepas, 0) valid_homepas,
            coalesce(sum(case when trim(lower(pw.submit_vermit)) = 'done' then 1 else 0 end, 0) submit_vermit,
            coalesce(sum(pw.valid_vermit), 0) valid_vermit
        from progression_wo pw
        left join project_wo pw2 on pw2.id = pw.wo_id
        left join region2 r on pw2.region_id = r.id 
        where r.witel in ({});
    ", in_syntax);
    // println!("str query: {str}");

    let mut sql = sqlx::query_as::<_, ProgressionWOHighlightView>(&str);

    for r in region_ids {
        sql = sql.bind(r);
    }

    sql.fetch_one(pool).await
}

pub async fn get_recon_highlight_all(pool: &PgPool) -> Result<ReconHighlightView,Error> {
    let str = format!("
        with base as (
            select 
                count(1) total_recon,
                coalesce(sum(case when r.survey_md_check = true then pw.survey_md_price else 0 end),0) survey_md_price,
                coalesce(sum(case when r.material_check = true then pp.material_price else 0 end),0) material_price,
                coalesce(sum(case when r.service_check = true then pp.service_price else 0 end),0) service_price
            from recon r
            left join project_po pp on r.po_id = pp.id
            left join project_wo pw on pp.wo_id = pw.id
        )
        select 
            *,
            survey_md_price + material_price + service_price total_price
        from base;
    ");
    // println!("str query: {str}");

    sqlx::query_as::<_, ReconHighlightView>(&str)
    .fetch_one(pool).await
}

pub async fn get_recon_highlight_by_regions(pool: &PgPool, region_ids: Option<Vec<i32>>) -> Result<ReconHighlightView,Error> {
    let region_ids = region_ids.unwrap_or(vec![]);
    let in_syntax = region_ids.iter().enumerate().map(|(i,_)| format!("${}",i+1)).collect::<Vec<_>>().join(",");
    let str = format!("
        with base as (
            select 
                count(1) total_recon,
                coalesce(sum(case when r.survey_md_check = true then pw.survey_md_price else 0 end),0) survey_md_price,
                coalesce(sum(case when r.material_check = true then pp.material_price else 0 end),0) material_price,
                coalesce(sum(case when r.service_check = true then pp.service_price else 0 end),0) service_price
            from recon r
            left join project_po pp on r.po_id = pp.id
            left join project_wo pw on pp.wo_id = pw.id
            left join region2 r2 on pw.region_id = r.id 
            where r2.witel in ({})
        )
        select 
            *,
            survey_md_price + material_price + service_price total_price
        from base;
    ", in_syntax);
    // println!("str query: {str}");

    let mut sql = sqlx::query_as::<_, ReconHighlightView>(&str);

    for r in region_ids {
        sql = sql.bind(r);
    }

    sql.fetch_one(pool).await
}

pub async fn get_billing_highlight_all(pool: &PgPool) -> Result<BillingHighlightView,Error> {
    let str = format!("
        with base as (
            select 
                coalesce(count(1),0) total_billing,
                coalesce(sum(case when b.status = 'RECON' then 1 else 0 end),0) recon,
                coalesce(sum(case when b.status = 'IN BILLING' then 1 else 0 end),0) in_billing,
                coalesce(sum(case when b.status = 'PAID' then 1 else 0 end),0) paid
            from billing b
            left join recon r on r.id = b.recon_id
            left join project_po pp on pp.id = r.po_id
            left join project_wo pw on pw.id = pp.wo_id
        )
        select 
            *
        from base;
    ");

    sqlx::query_as::<_, BillingHighlightView>(&str)
    .fetch_one(pool).await
}

pub async fn get_billing_highlight_by_regions(pool: &PgPool, region_ids: Option<Vec<i32>>) -> Result<BillingHighlightView,Error> {
    let region_ids = region_ids.unwrap_or(vec![]);
    let in_syntax = region_ids.iter().enumerate().map(|(i,_)| format!("${}",i+1)).collect::<Vec<_>>().join(",");
    let str = format!("
        with base as (
            select 
                coalesce(count(1),0) total_billing,
                coalesce(sum(case when b.status = 'RECON' then 1 else 0 end),0) recon,
                coalesce(sum(case when b.status = 'IN BILLING' then 1 else 0 end),0) in_billing,
                coalesce(sum(case when b.status = 'PAID' then 1 else 0 end),0) paid
            from billing b
            left join recon r on r.id = b.recon_id
            left join project_po pp on pp.id = r.po_id
            left join project_wo pw on pw.id = pp.wo_id
            left join region2 r2 on r2.id = pw.region_id
            where r2.witel in ({})
        )
        select 
            *
        from base;
    ", in_syntax);
    // println!("str query: {str}");

    let mut sql = sqlx::query_as::<_, BillingHighlightView>(&str);

    for r in region_ids {
        sql = sql.bind(r);
    }

    sql.fetch_one(pool).await
}

pub async fn get_user_management_highlight_all(pool: &PgPool) -> Result<UserManagementHighlightView,Error> {
    let str = format!("
        select 
            count(1) total_user,
            sum(case when r.role = 'admin' then 1 else 0 end) ho,
            sum(case when r.role = 'pm' then 1 else 0 end) pm,
            sum(case when r.role = 'tl' then 1 else 0 end) tl
        from users_new u
        left join role r on u.role_id = r.id
    ");

    sqlx::query_as::<_, UserManagementHighlightView>(&str)
    .fetch_one(pool).await
}

pub async fn get_user_management_highlight_by_regions(pool: &PgPool, region_ids: Option<Vec<i32>>) -> Result<UserManagementHighlightView,Error> {
    let region_ids = region_ids.unwrap_or(vec![]);
    let in_syntax = region_ids.iter().enumerate().map(|(i,_)| format!("${}",i+1)).collect::<Vec<_>>().join(",");
    let str = format!("
        select 
            count(1) total_user,
            sum(case when r.role = 'admin' then 1 else 0 end) ho,
            sum(case when r.role = 'pm' then 1 else 0 end) pm,
            sum(case when r.role = 'tl' then 1 else 0 end) tl
        from users_new u
        left join role r on u.role_id = r.id
        left join user_area_assigned uaa on u.id = uaa.user_id 
        left join region2 r2 on uaa.area_assigned_id = r2.id
        where r2.witel in ({});
    ", in_syntax);
    // println!("str query: {str}");

    let mut sql = sqlx::query_as::<_, UserManagementHighlightView>(&str);

    for r in region_ids {
        sql = sql.bind(r);
    }

    sql.fetch_one(pool).await
}