use sqlx::{PgPool, Result};

use crate::{GetPSA, Region, Regional, UserCredential, Witel, PSA};




pub async fn get_region_by(pool: &PgPool, regional: i32, witel: i32, psa: i32) -> Result<Region> {
    sqlx::query_as!(Region,
        "
        SELECT 
            id
        FROM public.region2 
        WHERE regional = $1 and witel = $2 and psa = $3",
        regional, witel, psa
    )
    .fetch_one(pool)
    .await
}

pub async fn get_regional_by_region_ids(pool: &PgPool, region_ids: Option<Vec<i32>>) -> Result<Vec<Regional>> {
    let region_ids = region_ids.unwrap_or(vec![]);
    let in_syntax = region_ids.iter().enumerate().map(|(i,_)| format!("${}",i+1)).collect::<Vec<_>>().join(",");
    let str = format!("
        select
            distinct r2.id as id,
            r2.name regional
        from public.region2 r
        left join region2 r2 on r.regional = r2.id
        where case 
            when r.level = 'WITEL' then r.witel in ({}) 
            else r.psa in ({})
        end", in_syntax, in_syntax);
    // println!("query: {str}");

    let mut query = sqlx::query_as::<_,Regional>(&str);

    for id in region_ids {
        query = query.bind(id);
    }

    query.fetch_all(pool).await
}

pub async fn get_regional_all(pool: &PgPool) -> Result<Vec<Regional>> {
    let str = format!("
        select
            distinct
            r2.id as id,
            r2.name regional
        from public.region2 r
        left join region2 r2 on r.regional = r2.id");

    let query = sqlx::query_as::<_,Regional>(&str);
    query.fetch_all(pool).await
}

pub async fn get_witel_by_region_ids(pool: &PgPool, region_ids: Option<Vec<i32>>, regional: i32) -> Result<Vec<Witel>> {
    let region_ids = region_ids.unwrap_or(vec![]);
    let in_syntax = region_ids.iter().enumerate().map(|(i,_)| format!("${}", i+1)).collect::<Vec<_>>().join(",");
    let str = format!("
        SELECT 
            r2.name as witel,
            r2.id as id
        FROM public.region2 r1
        left join region2 r2 on r1.witel = r2.id
        where r1.regional = {}
        and case 
            when r1.level = 'WITEL' then r1.witel in ({}) 
            else r1.psa in ({})
        end", regional, in_syntax, in_syntax);
    let mut query = sqlx::query_as::<_, Witel>(&str);

    for id in region_ids {
        query = query.bind(id);
    }

    query.fetch_all(pool).await
}

pub async fn get_witel_all(pool: &PgPool, regional: i32) -> Result<Vec<Witel>> {
    let str = format!(" 
        SELECT 
          distinct
            r2.name as witel,
            r2.id as id
        FROM public.region2 r1
        left join region2 r2 on r1.witel = r2.id
        where r1.regional = {}  AND r2.name IS NOT NULL
  AND r2.id IS NOT NULL", regional);
    
    let query = sqlx::query_as::<_, Witel>(&str);
    query.fetch_all(pool).await
}

pub async fn get_psa_by_region_ids(pool: &PgPool, user_info: UserCredential, param: GetPSA) -> Result<Vec<PSA>> {
    let region_ids = user_info.region_id.unwrap_or(vec![]);
    let in_syntax = region_ids.iter().enumerate().map(|(i,_)| format!("${}", i+1)).collect::<Vec<_>>().join(",");
    let str = format!("
        SELECT 
            r2.name as psa,
            r2.id as id
        FROM public.region2 r1
        left join region2 r2 on r1.psa = r2.id
        where r1.regional = {} and r1.witel = {} and r1.level = 'PSA'
        and case 
            when 'WITEL' = '{}' then r1.witel in ({}) 
            else r1.psa in ({})
        end", param.regional, param.witel, user_info.region_level.unwrap_or_default(), in_syntax, in_syntax);
    let mut query = sqlx::query_as::<_, PSA>(&str);

    for id in region_ids {
        query = query.bind(id);
    }

    query.fetch_all(pool).await
}

pub async fn get_psa_all(pool: &PgPool, param: GetPSA) -> Result<Vec<PSA>> {
    let str = format!("
        SELECT 
            r2.name as psa,
            r2.id as id
        FROM public.region2 r1
        left join region2 r2 on r1.psa = r2.id
        where r1.regional = {} and r1.witel = {} and r1.level = 'PSA'", 
        param.regional, param.witel);
    
    let query = sqlx::query_as::<_, PSA>(&str);
    query.fetch_all(pool).await
}