use chrono::Local;
use sqlx::{Error, PgPool};
use crate::{Billing, BillingView, CreateBilling};


pub async fn get_billing_by_regions(pool: &PgPool, region_ids: Option<Vec<i32>>) -> Result<Vec<BillingView>,Error>{
    let region_ids = region_ids.unwrap_or(vec![]);
    let in_syntax = region_ids.iter().enumerate().map(|(i,_)| format!("${}",i+1)).collect::<Vec<_>>().join(",");
    let str = format!("
    with base as (
	select
	        b.id,
	        pw.id wo_id,
	        pw.wo_code wo_code,
	        pw.wo_name wo_name,
	        pp.id po_id,
	        pp.po_code po_code,
	        pp.po_name po_name,
	        pw.wo_name || ' | ' || pp.po_name project_name,
	        pt.id project_type_id,
	        pt.type project_type,
	        pw.region_id region_id,
	        r2.name || ', ' ||  r3.name || ', ' || r1.name location,
			r2.name regional,
			r3.name witel,
			r1.name psa,
	        t.team_name team_name,
	        t.id team_id,
	        un.username pic,
	        un.id user_id,
	        pw2.survey_homepas,
	        sum(pw2.valid_homepas) valid_homepas,
	        pw2.submit_vermit,
	        sum(pw2.valid_vermit) valid_vermit,
	        pp.cable cable,
	        pp.pole pole,
	        pp.port port,
	        pw.survey_md_price,
	        pp.material_price,
	        pp.service_price,
	        case when r.survey_md_check = true then pw.survey_md_price else 0 end + 
	        case when r.material_check = true then pp.material_price else 0 end + 
	        case when r.service_check = true then pp.service_price else 0 end total_price,
	        r.survey_md_check,
	        r.material_check,
	        r.service_check,
	        '-' phase,        
	        pp2.status po_status,
	        pp2.description,
	        b.status status,
	        b.update_time last_update,
	        rank() over(partition by pp2.po_id order by pp2.date desc) pp_rank,
	        rank() over(partition by pw2.wo_id order by pw2.date desc) pw_rank
	    from billing b
	    left join recon r on b.recon_id = r.id
	    left join project_po pp on r.po_id = pp.id
	    left join project_wo pw on pp.wo_id = pw.id
	    left join users_new un on pw.user_id = un.id
	    left join user_team ut on un.id = ut.user_id
	    left join team t on t.id = ut.team_id
	    left join project_type pt on pw.project_type_id = pt.id
	    left join region2 r1 on r1.id = pw.region_id
	    left join region2 r2 on r2.id = r1.regional
	    left join region2 r3 on r3.id = r1.witel
	    left join progression_wo pw2 on pw.id = pw2.wo_id
	    left join progression_po pp2 on pp.id = pp2.po_id
	    group by b.id,
	        pw.id,
	        pw.wo_code,
	        pw.wo_name,
	        pp.id,
	        pp.po_code,
	        pp.po_name,
	        pt.id,
	        pt.type,
	        pw.region_id,
	        r2.name || ', ' ||  r3.name || ', ' || r1.name,
			r2.name,
			r3.name,
			r1.name,
	        t.team_name,
	        t.id,
	        un.username,
	        un.id,
	        pw2.survey_homepas,
	        pw2.submit_vermit,
	        pp.cable,
	        pp.pole,
	        pp.port,
	        pw.survey_md_price,
	        pp.material_price,
	        pp.service_price,
	        r.survey_md_check,
	        r.material_check,
	        r.service_check,
	        pp2.status,
	        pp2.description,
	        b.status,
	        pp2.date,
	        pp2.po_id,
	        pw2.wo_id,
	        pw2.date)
select 
	id, wo_id, wo_code, wo_name, po_id, po_code, po_name, project_name, project_type_id, project_type, region_id, location, regional, witel, psa, team_name, team_id, pic, user_id, survey_homepas, valid_homepas, submit_vermit, valid_vermit, cable, pole, port, survey_md_price, material_price, service_price, total_price, survey_md_check, material_check, service_check, base.description, phase, po_status, status, last_update
from base
where pp_rank = 1 and pw_rank = 1 and r1.witel in ({})", in_syntax);
    // println!("str query: {str}");

    let mut query = sqlx::query_as::<_, BillingView>(&str);
    for id in region_ids {
        query = query.bind(id);
    }
    query.fetch_all(pool).await

}

pub async fn get_billing_all(pool: &PgPool) -> Result<Vec<BillingView>,Error>{
    let str = format!("
    with base as (
	select
	        b.id,
	        pw.id wo_id,
	        pw.wo_code wo_code,
	        pw.wo_name wo_name,
	        pp.id po_id,
	        pp.po_code po_code,
	        pp.po_name po_name,
	        pw.wo_name || ' | ' || pp.po_name project_name,
	        pt.id project_type_id,
	        pt.type project_type,
	        pw.region_id region_id,
	        r2.name || ', ' ||  r3.name || ', ' || r1.name location,
			r2.name regional,
			r3.name witel,
			r1.name psa,
	        t.team_name team_name,
	        t.id team_id,
	        un.username pic,
	        un.id user_id,
	        pw2.survey_homepas,
	        sum(pw2.valid_homepas) valid_homepas,
	        pw2.submit_vermit,
	        sum(pw2.valid_vermit) valid_vermit,
	        pp.cable cable,
	        pp.pole pole,
	        pp.port port,
	        pw.survey_md_price,
	        pp.material_price,
	        pp.service_price,
	        case when r.survey_md_check = true then pw.survey_md_price else 0 end + 
	        case when r.material_check = true then pp.material_price else 0 end + 
	        case when r.service_check = true then pp.service_price else 0 end total_price,
	        r.survey_md_check,
	        r.material_check,
	        r.service_check,
	        '-' phase,        
	        pp2.status po_status,
	        pp2.description,
	        b.status status,
	        b.update_time last_update,
	        rank() over(partition by pp2.po_id order by pp2.date desc) pp_rank,
	        rank() over(partition by pw2.wo_id order by pw2.date desc) pw_rank
	    from billing b
	    left join recon r on b.recon_id = r.id
	    left join project_po pp on r.po_id = pp.id
	    left join project_wo pw on pp.wo_id = pw.id
	    left join users_new un on pw.user_id = un.id
	    left join user_team ut on un.id = ut.user_id
	    left join team t on t.id = ut.team_id
	    left join project_type pt on pw.project_type_id = pt.id
	    left join region2 r1 on r1.id = pw.region_id
	    left join region2 r2 on r2.id = r1.regional
	    left join region2 r3 on r3.id = r1.witel
	    left join progression_wo pw2 on pw.id = pw2.wo_id
	    left join progression_po pp2 on pp.id = pp2.po_id
	    group by b.id,
	        pw.id,
	        pw.wo_code,
	        pw.wo_name,
	        pp.id,
	        pp.po_code,
	        pp.po_name,
	        pt.id,
	        pt.type,
	        pw.region_id,
	        r2.name || ', ' ||  r3.name || ', ' || r1.name,
			r2.name,
			r3.name,
			r1.name,
	        t.team_name,
	        t.id,
	        un.username,
	        un.id,
	        pw2.survey_homepas,
	        pw2.submit_vermit,
	        pp.cable,
	        pp.pole,
	        pp.port,
	        pw.survey_md_price,
	        pp.material_price,
	        pp.service_price,
	        r.survey_md_check,
	        r.material_check,
	        r.service_check,
	        pp2.status,
	        pp2.description,
	        b.status,
	        pp2.date,
	        pp2.po_id,
	        pw2.wo_id,
	        pw2.date)
select 
	id, wo_id, wo_code, wo_name, po_id, po_code, po_name, project_name, project_type_id, project_type, region_id, location, regional, witel, psa, team_name, team_id, pic, user_id, survey_homepas, valid_homepas, submit_vermit, valid_vermit, cable, pole, port, survey_md_price, material_price, service_price, total_price, survey_md_check, material_check, service_check, base.description, phase, po_status, status, last_update
from base
where pp_rank = 1 and pw_rank = 1;");
        
    sqlx::query_as::<_, BillingView>(&str)
    .fetch_all(pool)
    .await
}

pub async fn check_billing_by_id(pool: &PgPool, id: i32) -> Result<Billing,Error>{
    sqlx::query_as!(Billing,
        "
        SELECT 
            *
        FROM public.billing
        WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn check_billing(pool: &PgPool, param: &CreateBilling) -> Result<Billing,Error>{
    sqlx::query_as!(Billing,
        "
        SELECT 
            *
        FROM public.billing
        WHERE recon_id = $1 and status = $2",
        param.recon_id, param.status
    )
    .fetch_one(pool)
    .await
}

pub async fn update_billing(pool: &PgPool, param: &Billing) -> Result<Billing,Error>{
    let now = Local::now().format("%F %T").to_string();
    sqlx::query_as!(Billing,
        "
        UPDATE public.billing
        SET
            status=$2,
            update_time=$3
        WHERE id = $1
        RETURNING *
        ",
        param.id, param.status, now
    )
    .fetch_one(pool)
    .await
}

pub async fn insert_billing(pool: &PgPool, param: &CreateBilling) -> Result<Billing,Error>{
    sqlx::query_as!(Billing, "
        INSERT INTO public.billing 
            (recon_id, status)
        VALUES 
            ($1, $2) 
        returning *",
        param.recon_id, param.status
    )
        .fetch_one(pool)
        .await
}