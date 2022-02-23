use crate::{
    helper,
    model::{Group, GroupID},
    Result,
};
use tokio_postgres::Client;

/// 添加分组
pub(crate) async fn create(client: &Client, name: String) -> Result<GroupID> {
    let group_uuid = helper::uuid();
    super::insert(
        client,
        "INSERT INTO groups (name, url) VALUES($1, $2) RETURNING id",
        &[&name, &group_uuid],
        "添加分组失败",
    )
    .await
}

/// 修改分组名
pub(crate) async fn change_name(client: &Client, name: String, id: i32) -> Result<u64> {
    super::execute(
        client,
        "UPDATE groups SET name=$1 WHERE id=$2",
        &[&name, &id],
    )
    .await
}

/// 修改分组URL
pub(crate) async fn change_url(client: &Client, id: i32) -> Result<u64> {
    let group_uuid = helper::uuid();
    super::execute(
        client,
        "UPDATE groups SET url=$1 WHERE id=$2",
        &[&group_uuid, &id],
    )
    .await
}

/// 删除或还原分组
pub(crate) async fn del_or_restore(client: &Client, id: i32, is_del: bool) -> Result<u64> {
    super::del_or_restore(client, "groups", is_del, &id).await
}

/// 所有分组
pub(crate) async fn all(client: &Client) -> Result<Vec<Group>> {
    super::query(
        client,
        "SELECT id,name,url,is_del FROM groups WHERE is_del=FALSE ORDER BY id",
        &[],
    )
    .await
}

/// 查找指定ID的分组
pub(crate) async fn find(client: &Client, id: i32) -> Result<Group> {
    super::query_row(
        client,
        "SELECT id,name,url,is_del FROM groups WHERE is_del=FALSE AND id=$1",
        &[&id],
    )
    .await
}
