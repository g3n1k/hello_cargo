use sqlx::{Error, PgPool};

use crate::{model::ProjectView, CreateProject, DeleteProject, Project, UpdateProject};




pub async fn get_project_by_regions(pool: &PgPool, region_ids: Option<Vec<i32>>) -> Result<Vec<ProjectView>,Error>{
    let region_ids = region_ids.unwrap_or(vec![]);
    let in_syntax = region_ids.iter().enumerate().map(|(i,_)| format!("${}",i+1)).collect::<Vec<_>>().join(",");
    let str = format!("
    SELECT 
        p.id, 
        p.wo_id,
        p.project_type_id,
        pt.type as project_type,
        p.po_id,
        p.region_id,
        r2.name as regional,
        r3.name as witel,
        r1.name as psa,
        r2.regional as regional_id,
        r3.witel as witel_id,
        r1.psa as psa_id,
        p.unit,
        p.cable_volume,
        p.pole,
        p.port,
        p.service_price,
        p.material_price,
        p.inc_material_price,
        p.deploy_service_price,
        p.description,
        p.attachment_path,
        p.service,
        p.material,
        p.status,
        p.phase,
        p.survey_md_doc,
        p.wo_doc,
        p.po_doc,
        p.user_id as user_id,
        un.full_name as pic
    FROM public.project p
    LEFT JOIN public.project_type pt ON p.project_type_id = pt.id
    LEFT JOIN public.region2 r1 ON r1.id = p.region_id 
    LEFT JOIN public.region2 r2 ON r2.id = r1.regional 
    LEFT JOIN public.region2 r3 ON r3.id = r1.witel
    left join public.users_new un on p.user_id = un.id
    where r1.witel in ({})", in_syntax);
    // println!("str query: {str}");

    let mut query = sqlx::query_as::<_, ProjectView>(&str);
    for id in region_ids {
        query = query.bind(id);
    }
    query.fetch_all(pool).await

}

pub async fn get_project_all(pool: &PgPool) -> Result<Vec<ProjectView>,Error>{
    let str = format!("
    SELECT 
        p.id, 
        p.wo_id,
        p.project_type_id,
        pt.type as project_type,
        p.po_id,
        p.region_id,
        r2.name as regional,
        r3.name as witel,
        r1.name as psa,
        r2.regional as regional_id,
        r3.witel as witel_id,
        r1.psa as psa_id,
        p.unit,
        p.cable_volume,
        p.pole,
        p.port,
        p.service_price,
        p.material_price,
        p.inc_material_price,
        p.deploy_service_price,
        p.description,
        p.attachment_path,
        p.service,
        p.material,
        p.status,
        p.phase,
        p.survey_md_doc,
        p.wo_doc,
        p.po_doc,
        p.user_id as user_id,
        un.full_name as pic
        FROM public.project p
        LEFT JOIN public.project_type pt ON p.project_type_id = pt.id
        LEFT JOIN public.region2 r1 ON r1.id = p.region_id 
        LEFT JOIN public.region2 r2 ON r2.id = r1.regional 
        LEFT JOIN public.region2 r3 ON r3.id = r1.witel
        left join public.users_new un on p.user_id = un.id");
        
    sqlx::query_as::<_, ProjectView>(&str)
    .fetch_all(pool)
    .await
}

pub async fn check_project(pool: &PgPool, param: &CreateProject) -> Result<Project,Error>{
    sqlx::query_as!(Project,
        "
        SELECT 
        id, po_id ,wo_id ,project_type_id ,region_id, unit ,cable_volume ,pole ,port ,service_price ,material_price ,description ,attachment_path ,service ,material ,status 
        FROM public.project 
        WHERE wo_id = $1 and po_id = $2",
        param.wo_id, param.po_id
    )
    .fetch_one(pool)
    .await
}

pub async fn check_project_by_id(pool: &PgPool, id: i32) -> Result<Project,Error>{
    sqlx::query_as!(Project,
        "
        SELECT 
            id, po_id ,wo_id ,project_type_id ,region_id, unit ,cable_volume ,pole ,port ,service_price ,material_price ,description ,attachment_path ,service ,material ,status 
        FROM public.project 
        WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn insert_project(pool: &PgPool, param: &CreateProject, region_id: i32) -> Result<DeleteProject,Error>{
    sqlx::query_as!(DeleteProject, "
        INSERT INTO public.project 
            (po_id ,wo_id ,project_type_id ,region_id, unit ,cable_volume ,pole ,port ,service_price ,material_price ,deploy_service_price ,description ,attachment_path ,service ,material ,status, user_id, phase, survey_md_doc, wo_doc, po_doc) 
        VALUES 
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21) 
        returning id",
        param.po_id, param.wo_id, param.project_type_id, region_id, param.unit, param.cable_volume, param.pole, param.port, param.service_price, param.material_price, param.deploy_service_price, param.description, param.attachment_path, param.service, param.material, param.status, param.pic, param.phase, param.survey_md_doc, param.wo_doc, param.po_doc
    )
        .fetch_one(pool)
        .await
}

pub async fn update_project(pool: &PgPool, param: &UpdateProject, region_id: i32) -> Result<DeleteProject,Error>{
    sqlx::query_as!(DeleteProject,
        "
        UPDATE public.project
        SET
            po_id = $2,
            wo_id = $3,
            project_type_id = $4,
            region_id = $5,
            unit = $6,
            cable_volume = $7,
            pole = $8,
            port = $9,
            service_price = $10,
            material_price = $11,
            deploy_service_price = $12,
            description = $13,
            attachment_path = $14,
            service = $15,
            material = $16,
            status = $17,
            user_id = $18,
            phase = $19,
            survey_md_doc = $20,
            wo_doc = $21,
            po_doc = $22
        WHERE id = $1
        RETURNING id
        ",
        param.id,
        param.po_id,
        param.wo_id,
        param.project_type_id,
        region_id,
        param.unit,
        param.cable_volume,
        param.pole,
        param.port,
        param.service_price,
        param.material_price,
        param.deploy_service_price,
        param.description,
        param.attachment_path,
        param.service,
        param.material,
        param.status,
        param.pic,
        param.phase,
        param.survey_md_doc,
        param.wo_doc,
        param.po_doc
    )
    .fetch_one(pool)
    .await
}

pub async fn delete_project(pool: &PgPool, id: i32) -> Result<DeleteProject,Error>{
    sqlx::query_as!(DeleteProject,"
        DELETE FROM public.project WHERE id = $1
        returning id", id)
        .fetch_one(pool)
        .await
}