use sqlx::{Error, PgPool};

use crate::{UserCredential, UserDetail, PIC};



pub async fn get_user_area_all(pool: &PgPool) -> Result<Vec<UserDetail>,Error>{
    sqlx::query_as!(
        UserDetail,
        r#"
        SELECT 
             u.id,
            u.username,
            u.full_name,
            u.email,
            u.phone_number,
            u.employee_id,
            u.status,
            u.last_login,
            u.date_of_birth,
            u.join_date, 
            u.gender,
            u.address,
            u.profile_picture,
            u.notes,
            u.created_at,
            u.updated_at,
            r.role,
            t.team_name,
            array_agg(DISTINCT aa.name) as assigned_areas
        FROM users_new u
        LEFT JOIN role r ON u.role_id = r.id
        LEFT JOIN team t ON u.team_id = t.id
        LEFT JOIN user_area_assigned uaa ON u.id = uaa.user_id
        LEFT JOIN region2 aa ON uaa.area_assigned_id = aa.id
        GROUP BY u.id, r.role, t.team_name
        ORDER BY u.id
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn get_pic(pool: &PgPool, team_id: i32) -> Result<Vec<PIC>,Error>{
    sqlx::query_as!(
        PIC,
        r#"
        SELECT 
            u.id id,
            u.username pic
        FROM user_team ut 
        left join users_new u on u.id = ut.user_id 
        where ut.team_id = $1;
        "#,
        team_id
    )
    .fetch_all(pool)
    .await
}

pub async fn get_user_area_by_regions(pool: &PgPool, region_id: Option<Vec<i32>>) -> Result<Vec<UserDetail>,Error>{
    let region_ids = region_id.unwrap_or(vec![]);
    let in_syntax = region_ids.iter().map(|_x| "?").collect::<Vec<_>>();
    let in_syntax = in_syntax.join(",");
    let str = format!("
    SELECT 
         u.id,
        u.username,
        u.full_name,
        u.email,
        u.phone_number,
        u.employee_id,
        u.status,
        u.last_login,
        u.date_of_birth,
        u.join_date, 
        u.gender,
        u.address,
        u.profile_picture,
        u.notes,
        u.created_at,
        u.updated_at,
        r.role,
        t.team_name,
        array_agg(DISTINCT aa.name) as assigned_areas
    FROM users_new u
    LEFT JOIN role r ON u.role_id = r.id
    LEFT JOIN team t ON u.team_id = t.id
    LEFT JOIN user_area_assigned uaa ON u.id = uaa.user_id
    LEFT JOIN region2 aa ON uaa.area_assigned_id = aa.id
    WHERE uaa.area_assigned_id in ({})
    GROUP BY u.id, r.role, t.team_name
    ORDER BY u.id", in_syntax);

    let mut query = sqlx::query_as::<_, UserDetail>(&str);
    for id in region_ids {
        query = query.bind(id);
    }
    query.fetch_all(pool).await
}

pub async fn get_user_credential(pool: &PgPool, user_id: i32) -> Result<UserCredential,Error>{
    sqlx::query_as!(
        UserCredential,
        r#"
        select 
        	u.id as user_id,
        	r.role role,
            r2.level region_level,
        	array_agg(DISTINCT uaa.area_assigned_id) region_id
        from public.users_new u
        left join role r on u.role_id = r.id
        left join user_area_assigned uaa on u.id = uaa.user_id
        left join region2 r2 on uaa.area_assigned_id = r2.id
        where u.id = $1
      	group by u.id,r.role, r2.level;
        "#, user_id
    )
    .fetch_one(pool)
    .await
}