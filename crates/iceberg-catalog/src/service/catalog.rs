use iceberg::spec::TableMetadata;
use iceberg_ext::catalog::rest::{
    CommitTableResponse, CommitTransactionRequest, CreateTableRequest,
};
use std::collections::{HashMap, HashSet};
use utoipa::ToSchema;

use crate::SecretIdent;

use super::{
    storage::StorageProfile, NamespaceIdentUuid, ProjectIdent, TableIdentUuid, WarehouseIdent,
    WarehouseStatus,
};
use crate::api::iceberg::v1::{
    CreateNamespaceRequest, CreateNamespaceResponse, GetNamespaceResponse, ListNamespacesQuery,
    ListNamespacesResponse, NamespaceIdent, Result, TableIdent, UpdateNamespacePropertiesRequest,
    UpdateNamespacePropertiesResponse,
};

#[async_trait::async_trait]
pub trait Transaction<D>
where
    Self: Sized + Send + Sync,
{
    type Transaction<'a>
    where
        Self: 'a;

    async fn begin_write(db_state: D) -> Result<Self>;

    async fn begin_read(db_state: D) -> Result<Self>;

    async fn commit(self) -> Result<()>;

    async fn rollback(self) -> Result<()>;

    fn transaction(&mut self) -> Self::Transaction<'_>;
}

#[derive(Debug)]
pub struct CreateTableResult {
    pub table_metadata: TableMetadata,
}

#[derive(Debug)]
pub struct LoadTableResult {
    pub table_id: TableIdentUuid,
    pub namespace_id: NamespaceIdentUuid,
    pub table_metadata: TableMetadata,
    pub metadata_location: Option<String>,
    pub storage_secret_ident: Option<SecretIdent>,
    pub storage_profile: StorageProfile,
}

#[derive(Debug)]
pub struct GetTableMetadataResult {
    pub table: TableIdent,
    pub table_id: TableIdentUuid,
    pub warehouse_id: WarehouseIdent,
    pub location: String,
    pub metadata_location: Option<String>,
    pub storage_secret_ident: Option<SecretIdent>,
    pub storage_profile: StorageProfile,
}

#[derive(Debug)]
pub struct GetStorageConfigResult {
    pub storage_profile: StorageProfile,
    pub storage_secret_ident: Option<SecretIdent>,
}

/// Extends the `CommitTableResponse` with the storage config.
#[derive(Debug)]
pub struct CommitTableResponseExt {
    pub commit_response: CommitTableResponse,
    pub storage_config: GetStorageConfigResult,
    pub previous_table_metadata: TableMetadata,
}

#[derive(Debug, Clone)]
pub struct GetWarehouseResponse {
    /// ID of the warehouse.
    pub id: WarehouseIdent,
    /// Name of the warehouse.
    pub name: String,
    /// Project ID in which the warehouse is created.
    pub project_id: ProjectIdent,
    /// Storage profile used for the warehouse.
    pub storage_profile: StorageProfile,
    /// Storage secret ID used for the warehouse.
    pub storage_secret_id: Option<SecretIdent>,
    /// Whether the warehouse is active.
    pub status: WarehouseStatus,
}

#[async_trait::async_trait]
#[allow(clippy::module_name_repetitions)]
pub trait Catalog
where
    Self: Clone + Send + Sync + 'static,
{
    type Transaction: Transaction<Self::State>;
    type State: Clone + Send + Sync + 'static;

    async fn list_namespaces(
        warehouse_id: &WarehouseIdent,
        query: &ListNamespacesQuery,
        catalog_state: Self::State,
    ) -> Result<ListNamespacesResponse>;

    async fn create_warehouse_profile<'a>(
        warehouse_name: String,
        project_id: ProjectIdent,
        storage_profile: StorageProfile,
        storage_secret_id: Option<SecretIdent>,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<WarehouseIdent>;

    async fn create_namespace<'a>(
        warehouse_id: &WarehouseIdent,
        request: CreateNamespaceRequest,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<CreateNamespaceResponse>;

    async fn get_namespace_metadata<'a>(
        warehouse_id: &WarehouseIdent,
        namespace: &NamespaceIdent,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<GetNamespaceResponse>;

    /// Return Err only on unexpected errors, not if the namespace does not exist.
    /// If the namespace does not exist, return Ok(false).
    ///
    /// We use this function also to handle the `namespace_exists` endpoint.
    async fn namespace_ident_to_id(
        warehouse_id: &WarehouseIdent,
        namespace: &NamespaceIdent,
        catalog_state: Self::State,
    ) -> Result<Option<NamespaceIdentUuid>>;

    async fn drop_namespace<'a>(
        warehouse_id: &WarehouseIdent,
        namespace: &NamespaceIdent,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<()>;

    async fn update_namespace_properties<'a>(
        warehouse_id: &WarehouseIdent,
        namespace: &NamespaceIdent,
        request: UpdateNamespacePropertiesRequest,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<UpdateNamespacePropertiesResponse>;

    async fn create_table<'a>(
        namespace_id: &NamespaceIdentUuid,
        table: &TableIdent,
        table_id: &TableIdentUuid,
        request: CreateTableRequest,
        // Metadata location may be none if stage-create is true
        metadata_location: Option<&String>,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<CreateTableResult>;

    async fn list_tables(
        warehouse_id: &WarehouseIdent,
        namespace: &NamespaceIdent,
        include_staged: bool,
        catalog_state: Self::State,
    ) -> Result<HashMap<TableIdentUuid, TableIdent>>;

    /// Return Err only on unexpected errors, not if the table does not exist.
    /// If include_staged is true, also return staged tables.
    /// If the table does not exist, return Ok(None).
    ///
    /// We use this function also to handle the `table_exists` endpoint.
    async fn table_ident_to_id(
        warehouse_id: &WarehouseIdent,
        table: &TableIdent,
        include_staged: bool,
        catalog_state: Self::State,
    ) -> Result<Option<TableIdentUuid>>;

    /// Same as `table_ident_to_id`, but for multiple tables.
    async fn table_idents_to_ids(
        warehouse_id: &WarehouseIdent,
        tables: HashSet<&TableIdent>,
        include_staged: bool,
        catalog_state: Self::State,
    ) -> Result<HashMap<TableIdent, Option<TableIdentUuid>>>;

    async fn load_table(
        warehouse_id: &WarehouseIdent,
        table: &TableIdent,
        catalog_state: Self::State,
    ) -> Result<LoadTableResult>;

    /// Get table metadata by table id.
    /// If include_staged is true, also return staged tables,
    /// i.e. tables with no metadata file yet.
    async fn get_table_metadata_by_id(
        warehouse_id: &WarehouseIdent,
        table: &TableIdentUuid,
        include_staged: bool,
        catalog_state: Self::State,
    ) -> Result<GetTableMetadataResult>;

    /// Get table metadata by location.
    async fn get_table_metadata_by_s3_location(
        warehouse_id: &WarehouseIdent,
        location: &str,
        include_staged: bool,
        catalog_state: Self::State,
    ) -> Result<GetTableMetadataResult>;

    /// Rename a table. Tables may be moved across namespaces.
    async fn rename_table<'a>(
        warehouse_id: &WarehouseIdent,
        source_id: &TableIdentUuid,
        source: &TableIdent,
        destination: &TableIdent,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<()>;

    /// Drop a table.
    /// Should drop staged and non-staged tables.
    ///
    /// Consider in your implementation to implement an UNDROP feature.
    async fn drop_table<'a>(
        warehouse_id: &WarehouseIdent,
        table_id: &TableIdentUuid,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<()>;

    /// Commit changes to a table.
    /// The table might be staged or not.
    async fn commit_table_transaction<'a>(
        warehouse_id: &WarehouseIdent,
        request: CommitTransactionRequest,
        table_ids: &HashMap<TableIdent, TableIdentUuid>,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<Vec<CommitTableResponseExt>>;

    /// Return a list of all project ids in the catalog
    async fn list_projects(catalog_state: Self::State) -> Result<HashSet<ProjectIdent>>;

    /// Return a list of all warehouse in a project
    async fn list_warehouses(
        project_id: &ProjectIdent,
        include_inactive: bool,
        // If None, return all warehouses in the project
        // If Some, return only the warehouses in the set
        warehouse_id_filter: Option<&HashSet<WarehouseIdent>>,
        catalog_state: Self::State,
    ) -> Result<HashSet<GetWarehouseResponse>>;

    /// Get the warehouse metadata.
    async fn get_warehouse<'a>(
        warehouse_id: &WarehouseIdent,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<GetWarehouseResponse>;

    /// Delete a warehouse.
    async fn delete_warehouse<'a>(
        warehouse_id: &WarehouseIdent,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<()>;

    /// Rename a warehouse.
    async fn rename_warehouse<'a>(
        warehouse_id: &WarehouseIdent,
        new_name: &str,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<()>;

    /// Set the status of a warehouse.
    async fn set_warehouse_status<'a>(
        warehouse_id: &WarehouseIdent,
        status: WarehouseStatus,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<()>;

    async fn update_storage_profile<'a>(
        warehouse_id: &WarehouseIdent,
        storage_profile: StorageProfile,
        storage_secret_id: Option<SecretIdent>,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<()>;
}
