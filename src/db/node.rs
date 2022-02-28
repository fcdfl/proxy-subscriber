use tokio_postgres::Client;

use crate::{form, model, Result};

use super::{Paginate, DEFAULT_PAGE_SIZE};

pub async fn create(client: &Client, nc: form::NodeCreate) -> Result<model::NodeID> {
    let sql = r#"INSERT INTO nodes
        (group_id, name, scheme, host, port, password, path, uuid, alter_id, cipher, username,is_del)
        VALUES
        ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, FALSE)
        RETURNING id"#;
    super::insert(
        client,
        sql,
        &[
            &nc.group_id,
            &nc.name,
            &nc.scheme,
            &nc.host,
            &nc.port,
            &nc.password,
            &nc.path,
            &nc.uuid,
            &nc.alter_id,
            &nc.cipher,
            &nc.username,
        ],
        "节点添加失败",
    )
    .await
}

pub async fn list(client: &Client, page: u32) -> Result<Paginate<Vec<model::Node>>> {
    let sql_base = "SELECT id, group_id, name, scheme, host, port, password, path, uuid, alter_id, cipher, username,is_del FROM nodes WHERE is_del=FALSE";
    let sql = format!(
        "{} ORDER BY id DESC LIMIT {} OFFSET {}",
        sql_base,
        DEFAULT_PAGE_SIZE,
        page * DEFAULT_PAGE_SIZE as u32
    );
    let count_sql = "SELECT COUNT(*) FROM nodes WHERE is_del=FALSE";
    super::paginate(client, &sql, count_sql, &[], page).await
}

pub async fn find(client: &Client, id: i32) -> Result<model::Node> {
    let sql = "SELECT id, group_id, name, scheme, host, port, password, path, uuid, alter_id, cipher, username,is_del FROM nodes WHERE is_del=FALSE AND id=$1";
    super::query_row(client, sql, &[&id]).await
}

pub async fn update(client: &Client, id: i32, nc: form::NodeCreate) -> Result<bool> {
    let sql = r#"UPDATE nodes SET
            group_id=$1, name=$2, scheme=$3, host=$4, port=$5, password=$6, path=$7, uuid=$8, alter_id=$9, cipher=$10, username=$11 
            WHERE id=$12
        "#;
    let n = super::execute(
        client,
        sql,
        &[
            &nc.group_id,
            &nc.name,
            &nc.scheme,
            &nc.host,
            &nc.port,
            &nc.password,
            &nc.path,
            &nc.uuid,
            &nc.alter_id,
            &nc.cipher,
            &nc.username,
            &id,
        ],
    )
    .await?;
    Ok(n > 0)
}
pub async fn del_or_restore(client: &Client, id: i32, is_del: bool) -> Result<bool> {
    let n = super::del_or_restore(client, "nodes", is_del, &id).await?;
    Ok(n > 0)
}
pub async fn subscriber(client: &Client, uuid: String) -> Result<Vec<model::Node>> {
    let sql = "SELECT n.id, group_id, n.name, scheme, host, port, password, path, uuid, alter_id, cipher, username,n.is_del
                FROM nodes AS n 
                INNER JOIN groups AS g
                ON n.group_id=g.id
                WHERE g.is_del=FALSE 
                AND n.is_del=FALSE
                AND g.url=$1";
    super::query(client, sql, &[&uuid]).await
}
