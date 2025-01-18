use sqlx::{Error, PgPool};
use crate::{CreateProjectPO, ProjectPO, ProjectPOView};


pub async fn get_project_po_by_regions(pool: &PgPool, region_ids: Option<Vec<i32>>) -> Result<Vec<ProjectPOView>,Error>{
    let region_ids = region_ids.unwrap_or(vec![]);
    let in_syntax = region_ids.iter().enumerate().map(|(i,_)| format!("${}",i+1)).collect::<Vec<_>>().join(",");
    let str = format!("
    select
        wp.id,
        wp.po_code,
        wp.po_name,
        wp.wo_id,
        wp2.wo_name,
        pt.type project_type,
        pt.id project_type_id,
        wp.po_doc,
        wp.cable cable,
        wp.pole pole,
        wp.port port,
        wp2.region_id region_id,
        concat(r.name, ', ', r3.name, ', ', r2.name) location,
        ut.team_id team_id,
        t.team_name team,
        wp.user_id user_id,
        un.full_name pic,
        wp.material_price,
        wp.service_price
    from project_po wp
    left join project_wo wp2 on wp.wo_id = wp2.id
    left join project_type pt on wp2.project_type_id = pt.id
    left join region2 r on wp2.region_id = r.id
    left join region2 r2 on r2.id = r.regional
    left join region2 r3 on r3.id = r.witel
    left join users_new un on wp.user_id = un.id
    left join user_team ut on wp.user_id = ut.user_id 
    left join team t on ut.team_id  = t.id
    where r.witel in ({})", in_syntax);
    // println!("str query: {str}");

    let mut query = sqlx::query_as::<_, ProjectPOView>(&str);
    for id in region_ids {
        query = query.bind(id);
    }
    query.fetch_all(pool).await

}

pub async fn get_project_po_all(pool: &PgPool) -> Result<Vec<ProjectPOView>,Error>{
    let str = format!("
    select
        wp.id,
        wp.po_code,
        wp.po_name,
        wp.wo_id,
        wp2.wo_name,
        pt.type project_type,
        pt.id project_type_id,
        wp.po_doc,
        wp.cable cable,
        wp.pole pole,
        wp.port port,
        wp2.region_id region_id,
        concat(r.name, ', ', r3.name, ', ', r2.name) location,
        ut.team_id team_id,
        t.team_name team,
        wp.user_id user_id,
        un.full_name pic,
        wp.material_price,
        wp.service_price
    from project_po wp
    left join project_wo wp2 on wp.wo_id = wp2.id
    left join project_type pt on wp2.project_type_id = pt.id
    left join region2 r on wp2.region_id = r.id
    left join region2 r2 on r2.id = r.regional
    left join region2 r3 on r3.id = r.witel
    left join users_new un on wp.user_id = un.id
    left join user_team ut on wp.user_id = ut.user_id 
    left join team t on ut.team_id  = t.id");
        
    sqlx::query_as::<_, ProjectPOView>(&str)
    .fetch_all(pool)
    .await
}

pub async fn check_project_po(pool: &PgPool, param: &CreateProjectPO) -> Result<ProjectPO,Error> {
    sqlx::query_as!(ProjectPO,
        "
        SELECT 
        *
        FROM public.project_po 
        WHERE po_name = $1",
        param.po_name
    )
    .fetch_one(pool)
    .await
}

pub async fn check_project_po_by_id(pool: &PgPool, id: i32) -> Result<ProjectPO,Error>{
    sqlx::query_as!(ProjectPO,
        "
        SELECT 
            *
        FROM public.project_po 
        WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn insert_project_po(pool: &PgPool, param: &CreateProjectPO) -> Result<ProjectPO,Error>{
    sqlx::query_as!(ProjectPO, "
        INSERT INTO public.project_po 
            (po_name,user_id,cable,pole,port,material_price,service_price,po_doc,wo_id)
        VALUES 
            ($1, $2, $3, $4, $5, $6, $7, $8, $9) 
        returning *",
        param.po_name, param.pic, param.cable, param.pole, param.port, param.material_price, param.service_price, param.po_doc, param.wo_id
    )
        .fetch_one(pool)
        .await
}

pub async fn update_project_po(pool: &PgPool, param: &ProjectPO) -> Result<ProjectPO,Error>{
    sqlx::query_as!(ProjectPO,
        "
        UPDATE public.project_po
        SET
            po_name = $2,
            user_id = $3,
            cable = $4,
            pole = $5,
            port = $6,
            po_doc = $7,
            material_price = $8,
            service_price = $9,
            wo_id = $10
        WHERE id = $1
        RETURNING *
        ",
        param.id, param.po_name, param.user_id, param.cable, param.pole, param.port, param.po_doc, param.material_price, param.service_price, param.wo_id
    )
    .fetch_one(pool)
    .await
}

pub async fn delete_project_po(pool: &PgPool, id: i32) -> Result<ProjectPO,Error>{
    sqlx::query_as!(ProjectPO,"
        DELETE FROM public.project_po WHERE id = $1
        returning *", id)
        .fetch_one(pool)
        .await
}