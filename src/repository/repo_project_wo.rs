use sqlx::{Error, PgPool};
use crate::{CreateProjectWO, ProjectWO, ProjectWOView};


pub async fn get_project_wo_by_regions(pool: &PgPool, region_ids: Option<Vec<i32>>) -> Result<Vec<ProjectWOView>,Error>{
    let region_ids = region_ids.unwrap_or(vec![]);
    let in_syntax = region_ids.iter().enumerate().map(|(i,_)| format!("${}",i+1)).collect::<Vec<_>>().join(",");
    let str = format!("
    select
        wp.id,
        wp.wo_code,
        wp.wo_name,
        pt.type project_type,
        pt.id project_type_id,
        r.id region_id,
        r2.id regional_id,
        r3.id witel_id,
        r.id psa_id,
        wp.user_id user_id,
        ut.team_id team_id,
        concat(r.name, ', ', r3.name, ', ', r2.name) location,
        r2.name regional,
        r3.name witel,
        r.name psa,
        un.full_name pic,
        t.team_name team,
        wp.wo_doc,
        wp.survey_md_doc,
        wp.survey_md_price
    from project_wo wp
    left join project_type pt on wp.project_type_id = pt.id
    left join region2 r on wp.region_id = r.id
    left join region2 r2 on r2.id = r.regional
    left join region2 r3 on r3.id = r.witel
    left join users_new un on wp.user_id = un.id
    left join user_team ut on wp.user_id = ut.user_id 
    left join team t on ut.team_id  = t.id
    where r.witel in ({})", in_syntax);
    // println!("str query: {str}");

    let mut query = sqlx::query_as::<_, ProjectWOView>(&str);
    for id in region_ids {
        query = query.bind(id);
    }
    query.fetch_all(pool).await

}

pub async fn get_project_wo_all(pool: &PgPool) -> Result<Vec<ProjectWOView>,Error>{
    let str = format!("
    select
        wp.id,
        wp.wo_code,
        wp.wo_name,
        pt.type project_type,
        pt.id project_type_id,
        r.id region_id,
        r2.id regional_id,
        r3.id witel_id,
        r.id psa_id,
        wp.user_id user_id,
        ut.team_id team_id,
        concat(r.name, ', ', r3.name, ', ', r2.name) location,
        r2.name regional,
        r3.name witel,
        r.name psa,
        un.full_name pic,
        t.team_name team,
        wp.wo_doc,
        wp.survey_md_doc,
        wp.survey_md_price
    from project_wo wp
    left join project_type pt on wp.project_type_id = pt.id
    left join region2 r on wp.region_id = r.id
    left join region2 r2 on r2.id = r.regional
    left join region2 r3 on r3.id = r.witel
    left join users_new un on wp.user_id = un.id
    left join user_team ut on wp.user_id = ut.user_id 
    left join team t on ut.team_id  = t.id");
        
    sqlx::query_as::<_, ProjectWOView>(&str)
    .fetch_all(pool)
    .await
}

pub async fn check_project_wo(pool: &PgPool, param: &CreateProjectWO) -> Result<ProjectWO,Error> {
    sqlx::query_as!(ProjectWO,
        "
        SELECT 
        *
        FROM public.project_wo 
        WHERE wo_name = $1",
        param.wo_name
    )
    .fetch_one(pool)
    .await
}

pub async fn check_project_wo_by_id(pool: &PgPool, id: i32) -> Result<ProjectWO,Error>{
    sqlx::query_as!(ProjectWO,
        "
        SELECT 
            *
        FROM public.project_wo 
        WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn insert_project_wo(pool: &PgPool, param: &CreateProjectWO) -> Result<ProjectWO,Error>{
    sqlx::query_as!(ProjectWO, "
        INSERT INTO public.project_wo 
            (wo_name,project_type_id,region_id,user_id,survey_md_doc,wo_doc,survey_md_price)
        VALUES 
            ($1, $2, $3, $4, $5, $6, $7) 
        returning *",
        param.wo_name, param.project_type_id, param.region_id, param.pic, param.survey_md_doc, param.wo_doc, param.survey_md_price
    )
        .fetch_one(pool)
        .await
}

pub async fn update_project_wo(pool: &PgPool, param: &ProjectWO) -> Result<ProjectWO,Error>{
    sqlx::query_as!(ProjectWO,
        "
        UPDATE public.project_wo
        SET
            wo_name = $2,
            project_type_id = $3,
            region_id = $4,
            user_id = $5,
            survey_md_doc = $6,
            wo_doc = $7,
            survey_md_price = $8
        WHERE id = $1
        RETURNING *
        ",
        param.id, param.wo_name, param.project_type_id, param.region_id, param.user_id, param.survey_md_doc, param.wo_doc, param.survey_md_price
    )
    .fetch_one(pool)
    .await
}

pub async fn delete_project_wo(pool: &PgPool, id: i32) -> Result<ProjectWO,Error>{
    sqlx::query_as!(ProjectWO,"
        DELETE FROM public.project_wo WHERE id = $1
        returning *", id)
        .fetch_one(pool)
        .await
}