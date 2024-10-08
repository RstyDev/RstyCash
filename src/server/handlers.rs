use actix_web::{get, web::{self, Data}, App, Error, HttpResponse, HttpServer, Responder};

#[get("/admin/get_productos")]
async fn get_productos(data: Data<Arc<Pool<Sqlite>>>)->impl Responder{
    let db = data.into_inner();
    let res=query_as!(BigIntDB,"select id as int from productos").fetch_all(db.as_ref().as_ref()).await.expect("Error DB");
    format!("{:#?}",res.iter().map(|x|x.int).collect::<Vec<i64>>()[5])
}