use sqlx::{Error, PgPool};
use crate::{DailyProgressionView, ProgressionUpdatesView, RegionalDistributionView, SummaryProgressionView, SummaryProjectView};


pub async fn get_summary_project_all(pool: &PgPool) -> Result<Vec<SummaryProjectView>,Error> {
    let str = format!("
        with base as (
            select 
                count(1) total_project,
                sum(pw.survey_md_price + pp.material_price + pp.service_price) total_price,
                case when b.status = 'PAID' then 'PAID' else 'UNPAID' end status
            from project_wo pw
            left join project_po pp on pw.id = pp.wo_id
            left join recon r on r.po_id = pp.id
            left join billing b on r.id = b.recon_id
            group by case when b.status = 'PAID' then 'PAID' else 'UNPAID' end
        )
        select 'estimated total priced' name, 'Rp. ' || round(sum(total_price::numeric / 1000000.0),1) || 'M' value1, '-' value2 from base
        union all
        select 'total project' name, sum(total_project)::varchar value1, '-' value2 from base
        union all
        select 'total unpaid' name, total_project::varchar value1, 'Rp. ' || round(total_price::numeric / 1000000.0,1) || 'M' value2 from base where status = 'UNPAID'
        union all
        select 'total paid' name, total_project::varchar value1, 'Rp. ' || round(total_price::numeric / 1000000.0,1) || 'M' value2 from base where status = 'PAID'
    ");
    // println!("str query: {str}");

    sqlx::query_as::<_, SummaryProjectView>(&str)
    .fetch_all(pool).await
}

pub async fn get_summary_progression_all(pool: &PgPool) -> Result<Vec<SummaryProgressionView>,Error> {
    let str = format!("
        with po as (
            select 
                date, 
                po_id, 
                status,
                rank() over(partition by po_id order by date desc) r
            from progression_po pp
        ),
        jml as (
            select
                count(1) value,
                p.status
            from project_po pp 
            left join (select * from po where r = 1) p on pp.id = p.po_id
            group by status
        ),
        status as (
            select * from status
        )
        select
            s.status name,
            coalesce(j.value, 0) value
        from status s
        left join jml j on s.status = j.status
    ");
    // println!("str query: {str}");

    sqlx::query_as::<_, SummaryProgressionView>(&str)
    .fetch_all(pool).await
}

pub async fn get_daily_progression_all(pool: &PgPool) -> Result<Vec<DailyProgressionView>,Error> {
    let str = format!("
        with dates as (
            SELECT generate_series(
                date_trunc('month', CURRENT_DATE),
                (date_trunc('month', CURRENT_DATE) + interval '1 month' - interval '1 day'),
                interval '1 day'
            )::date::varchar AS tanggal
        )
        select 
            d.tanggal date,
            count(pw.id) wo,
            count(pp.id) po,
            count(pw.id) + count(pp.id) total
        from dates d
        left join progression_wo pw on pw.date = d.tanggal
        left join progression_po pp on pp.date = d.tanggal
        group by d.tanggal
        order by d.tanggal desc
        limit 5;
    ");
    // println!("str query: {str}");

    sqlx::query_as::<_, DailyProgressionView>(&str)
    .fetch_all(pool).await
}

pub async fn get_progression_updates_all(pool: &PgPool) -> Result<Vec<ProgressionUpdatesView>,Error> {
    let str = format!("
        with base as (
            (select pw2.wo_name project, description update, date from progression_wo pw left join project_wo pw2 on pw.wo_id = pw2.id)
            union all
            (
	            select pw.wo_name || ' | ' || pp2.po_name project, description update, date from progression_po pp 
	            left join project_po pp2 on pp.po_id = pp2.id 
	            left join project_wo pw on pw.id = pp2.wo_id
            )
        )
        select * from base order by date desc limit 5;
    ");
    // println!("str query: {str}");

    sqlx::query_as::<_, ProgressionUpdatesView>(&str)
    .fetch_all(pool).await
}

pub async fn get_regional_distribution_all(pool: &PgPool) -> Result<Vec<RegionalDistributionView>,Error> {
    let str = format!("
        with base as (
            select
                r2.name name,
                count(1) value
            from project_wo pw
            left join region2 r on pw.region_id = r.id
            left join region2 r2 on r.regional = r2.id
            group by r2.name
        ),
        regional as (
            select
                id,
                name
            from region2
            where level = 'REGIONAL'
        )
        select
            r.name regional,
            coalesce(b.value, 0) value
        from regional r
        left join base b on r.name = b.name;
    ");
    // println!("str query: {str}");

    sqlx::query_as::<_, RegionalDistributionView>(&str)
    .fetch_all(pool).await
}