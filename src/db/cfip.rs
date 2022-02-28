use tokio_postgres::Client;

use crate::{error::AppError, model::Cfips, Result};

pub enum CfipList {
    Single(Cfips),
    Muilte(Vec<Cfips>),
}

pub async fn get(client: &Client, cfs: &Vec<String>) -> Result<CfipList> {
    if cfs.is_empty() {
        return Err(AppError::invalid_param());
    }
    if cfs.len() == 1 {
        let item = super::query_row(
            client,
            "SELECT id,ip,label,code,is_del FROM cfips WHERE code=$1 AND is_del=FALSE",
            &[&cfs[0]],
        )
        .await?;
        return Ok(CfipList::Single(item));
    }
    let list = super::query(
        client,
        "SELECT id,ip,label,code,is_del FROM cfips WHERE code = ANY($1) AND is_del=FALSE",
        &[cfs],
    )
    .await?;
    Ok(CfipList::Muilte(list))
}
