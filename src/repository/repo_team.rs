use sqlx::{PgPool, Result};

use crate::Team;



pub async fn get_team_by_regions(pool: &PgPool, region_ids: Option<Vec<i32>>) -> Result<Vec<Team>>{
    let region_ids = region_ids.unwrap_or(vec![]);
    let in_syntax = region_ids.iter().enumerate().map(|(i,_)| format!("${}",i+1)).collect::<Vec<_>>().join(",");
    let str = format!("
    select
        t.id as id,
        t.team_name as team_name
    from public.team_area ta
    left join team t on t.id = ta.team_id 
    left join region2 r on ta.region_id = r.id 
    where r.id in ({})", in_syntax);
    // println!("str query: {str}");

    let mut query = sqlx::query_as::<_, Team>(&str);
    for id in region_ids {
        query = query.bind(id);
    }
    query.fetch_all(pool).await
}

pub async fn get_team_all(pool: &PgPool) -> Result<Vec<Team>>{
    let str = format!("select * from public.team");
    let query = sqlx::query_as::<_, Team>(&str);
    query.fetch_all(pool).await
}