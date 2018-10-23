//! Db executor actor
use actix::prelude::*;
use actix_web::*;
use diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use models;
use schema;

pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

#[derive(Deserialize, Debug)]
pub struct CreateBuild {
}

impl Message for CreateBuild {
    type Result = Result<models::Build, diesel::result::Error>;
}

impl Handler<CreateBuild> for DbExecutor {
    type Result = Result<models::Build, diesel::result::Error>;

    fn handle(&mut self, _msg: CreateBuild, _: &mut Self::Context) -> Self::Result {
        use self::schema::builds::dsl::*;
        let conn = &self.0.get().unwrap();
        diesel::insert_into(builds)
            .default_values()
            .get_result::<models::Build>(conn)
    }
}


#[derive(Deserialize, Debug)]
pub struct CreateBuildRef {
    pub data : models::NewBuildRef,
}

impl Message for CreateBuildRef {
    type Result = Result<models::BuildRef, diesel::result::Error>;
}

impl Handler<CreateBuildRef> for DbExecutor {
    type Result = Result<models::BuildRef, diesel::result::Error>;

    fn handle(&mut self, msg: CreateBuildRef, _: &mut Self::Context) -> Self::Result {
        use self::schema::build_refs::dsl::*;
        let conn = &self.0.get().unwrap();
        diesel::insert_into(build_refs)
            .values(&msg.data)
            .get_result::<models::BuildRef>(conn)
    }
}

#[derive(Deserialize, Debug)]
pub struct LookupBuild {
    pub id: i32
}

impl Message for LookupBuild {
    type Result = Result<models::Build, diesel::result::Error>;
}

impl Handler<LookupBuild> for DbExecutor {
    type Result = Result<models::Build, diesel::result::Error>;

    fn handle(&mut self, msg: LookupBuild, _: &mut Self::Context) -> Self::Result {
        use schema::builds::dsl::*;
        let conn = &self.0.get().unwrap();
        builds.filter(id.eq(msg.id)).get_result::<models::Build>(conn)
    }
}
