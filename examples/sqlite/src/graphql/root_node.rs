use super::entities;
use sea_orm::prelude::*;
#[derive(async_graphql :: InputObject, Debug)]
pub struct PaginationInput {
    pub limit: usize,
    pub page: usize,
}
#[derive(async_graphql :: SimpleObject, Debug)]
#[graphql(concrete(name = "PaginatedTracksResult", params(entities::tracks::Model)))]
#[graphql(concrete(name = "PaginatedEmployeesResult", params(entities::employees::Model)))]
#[graphql(concrete(name = "PaginatedPlaylistsResult", params(entities::playlists::Model)))]
#[graphql(concrete(name = "PaginatedGenresResult", params(entities::genres::Model)))]
#[graphql(concrete(name = "PaginatedCustomersResult", params(entities::customers::Model)))]
#[graphql(concrete(name = "PaginatedAlbumsResult", params(entities::albums::Model)))]
#[graphql(concrete(
    name = "PaginatedMediaTypesResult",
    params(entities::media_types::Model)
))]
#[graphql(concrete(name = "PaginatedInvoicesResult", params(entities::invoices::Model)))]
#[graphql(concrete(name = "PaginatedArtistsResult", params(entities::artists::Model)))]
#[graphql(concrete(
    name = "PaginatedInvoiceItemsResult",
    params(entities::invoice_items::Model)
))]
#[graphql(concrete(
    name = "PaginatedPlaylistTrackResult",
    params(entities::playlist_track::Model)
))]
pub struct PaginatedResult<T: async_graphql::ObjectType> {
    pub data: Vec<T>,
    pub pages: usize,
    pub current: usize,
}
pub struct QueryRoot;
#[async_graphql::Object]
impl QueryRoot {
    async fn tracks<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::tracks::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::tracks::Model> {
        println!("filters: {:?}", filters);
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt =
            entities::tracks::Entity::find().filter(entities::tracks::filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::tracks::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::tracks::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn employees<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::employees::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::employees::Model> {
        println!("filters: {:?}", filters);
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt = entities::employees::Entity::find()
            .filter(entities::employees::filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::employees::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::employees::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn playlists<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::playlists::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::playlists::Model> {
        println!("filters: {:?}", filters);
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt = entities::playlists::Entity::find()
            .filter(entities::playlists::filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::playlists::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::playlists::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn genres<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::genres::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::genres::Model> {
        println!("filters: {:?}", filters);
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt =
            entities::genres::Entity::find().filter(entities::genres::filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::genres::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::genres::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn customers<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::customers::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::customers::Model> {
        println!("filters: {:?}", filters);
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt = entities::customers::Entity::find()
            .filter(entities::customers::filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::customers::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::customers::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn albums<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::albums::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::albums::Model> {
        println!("filters: {:?}", filters);
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt =
            entities::albums::Entity::find().filter(entities::albums::filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::albums::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::albums::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn media_types<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::media_types::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::media_types::Model> {
        println!("filters: {:?}", filters);
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt = entities::media_types::Entity::find()
            .filter(entities::media_types::filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::media_types::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::media_types::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn invoices<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::invoices::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::invoices::Model> {
        println!("filters: {:?}", filters);
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt = entities::invoices::Entity::find()
            .filter(entities::invoices::filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::invoices::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::invoices::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn artists<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::artists::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::artists::Model> {
        println!("filters: {:?}", filters);
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt =
            entities::artists::Entity::find().filter(entities::artists::filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::artists::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::artists::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn invoice_items<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::invoice_items::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::invoice_items::Model> {
        println!("filters: {:?}", filters);
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt = entities::invoice_items::Entity::find()
            .filter(entities::invoice_items::filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::invoice_items::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::invoice_items::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn playlist_track<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::playlist_track::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::playlist_track::Model> {
        println!("filters: {:?}", filters);
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt = entities::playlist_track::Entity::find()
            .filter(entities::playlist_track::filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::playlist_track::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::playlist_track::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
}