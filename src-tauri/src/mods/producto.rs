use crate::mods::{
    db::{
        map::{BigIntDB, CodeDB, IntDB, ProductoDB},
        Mapper,
    },
    redondeo, AppError, Presentacion, Res, ValuableTrait,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, Pool, Sqlite};
use tauri::async_runtime::block_on;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Producto {
    id: i32,
    codigos_de_barras: [i64; 3],
    precio_venta: f32,
    porcentaje: f32,
    precio_costo: f32,
    tipo_producto: Arc<str>,
    marca: Arc<str>,
    variedad: Arc<str>,
    presentacion: Presentacion,
    proveedores: Vec<RelacionProdProv>,
}
#[derive(Serialize, Deserialize)]
pub struct ProductoSH {
    id: i32,
    codigo_de_barras: [u8; 8],
    precio_venta: f32,
    tipo_producto: Arc<str>,
    marca: Arc<str>,
    variedad: Arc<str>,
    presentacion: Presentacion,
}
#[derive(Serialize, Deserialize)]
pub struct ProductoSHC {
    id: i32,
    codigos_de_barras: [[u8; 8]; 3],
    precio_venta: f32,
    porcentaje: f32,
    precio_costo: f32,
    tipo_producto: Arc<str>,
    marca: Arc<str>,
    variedad: Arc<str>,
    presentacion: Presentacion,
    proveedores: Vec<RelacionProdProv>,
}
impl Producto {
    pub async fn new_to_db(&mut self, db: &Pool<Sqlite>) -> Res<()> {
        let mut query = String::from(r#"select id as "int:_" from codigos where codigo = ?"#);
        let row = " or codigo = ?";
        for i in 1..3 {
            if self.codigos_de_barras[i] > 0 {
                query.push_str(row);
            }
        }
        let mut qres = sqlx::query_as(query.as_str());
        for code in &self.codigos_de_barras {
            if *code > 0 {
                qres = qres.bind(code);
            }
        }
        let qres: Option<IntDB> = qres.fetch_optional(db).await?;
        if let Some(res) = qres {
            return Err(AppError::ExistingError {
                objeto: "Codigo".to_string(),
                instancia: res.int.to_string(),
            });
        }
        let cant = self.presentacion.get_cantidad();
        let pres = self.presentacion.get_string();
        let variedad = self.variedad.as_ref();
        let marca = self.marca.as_ref();
        let tipo = self.tipo_producto.as_ref();
        let qres:Option<IntDB>=sqlx::query_as!(IntDB,
            r#"select id as "int:_" from productos where tipo = ? and marca = ? and variedad = ? and presentacion = ? and size = ?"#,tipo,marca,variedad,pres,cant)
            .fetch_optional(db).await?;
        match qres {
            None => {
                let prod_qres =
                    sqlx::query("insert into productos values (?, ?, ?, ?, ?, ?, ?, ?, ?)")
                        .bind(self.precio_venta)
                        .bind(self.porcentaje)
                        .bind(self.precio_costo)
                        .bind(self.tipo_producto.as_ref())
                        .bind(self.marca.as_ref())
                        .bind(self.variedad.as_ref())
                        .bind(self.presentacion.get_string())
                        .bind(self.presentacion.get_cantidad())
                        .bind(Utc::now().naive_local())
                        .execute(db)
                        .await?;
                let mut query = String::from("insert into codigos values (?, ?)");
                let row = ", (?, ?)";
                for i in 1..3 {
                    if self.codigos_de_barras[i] > 0 {
                        query.push_str(row);
                    }
                }
                let mut qres = sqlx::query(query.as_ref());
                for code in &self.codigos_de_barras {
                    if *code > 0 {
                        qres = qres.bind(*code);
                    }
                }
                qres.execute(db).await?;

                let mut query = String::from("insert into relacion_prod_prov values (?, ?, ?)");
                let row = ", (?, ?, ?)";
                for _ in 1..self.proveedores.len() {
                    query.push_str(row);
                }
                let mut qres = sqlx::query(query.as_ref());
                for i in 0..self.proveedores.len() {
                    qres = qres
                        .bind(prod_qres.last_insert_rowid())
                        .bind(self.proveedores[i].proveedor())
                        .bind(self.proveedores[i].codigo_interno());
                }
                qres.execute(db).await?;
                self.id = prod_qres.last_insert_rowid() as i32;
                Ok(())
            }
            Some(_) => {
                return Err(AppError::ExistingError {
                    objeto: String::from("Producto"),
                    instancia: format!(
                        "{} {} {} {} {}",
                        self.tipo_producto,
                        self.marca,
                        self.variedad,
                        self.presentacion.get_cantidad(),
                        self.presentacion.get_string()
                    ),
                })
            }
        }
    }
    pub fn build(
        id: i32,
        codigos_de_barras: [i64; 3],
        precio_venta: f32,
        porcentaje: f32,
        precio_costo: f32,
        tipo_producto: &str,
        marca: &str,
        variedad: &str,
        presentacion: Presentacion,
        proveedores: Vec<RelacionProdProv>,
    ) -> Producto {
        Producto {
            id,
            codigos_de_barras,
            precio_venta,
            porcentaje,
            precio_costo,
            tipo_producto: Arc::from(tipo_producto),
            marca: Arc::from(marca),
            variedad: Arc::from(variedad),
            presentacion,
            proveedores,
        }
    }
    pub fn id(&self) -> &i32 {
        &self.id
    }
    pub fn codigos_de_barras(&self) -> &[i64; 3] {
        &self.codigos_de_barras
    }
    pub fn precio_venta(&self) -> &f32 {
        &self.precio_venta
    }
    pub fn porcentaje(&self) -> &f32 {
        &self.porcentaje
    }
    pub fn precio_costo(&self) -> &f32 {
        &self.precio_costo
    }
    pub fn tipo_producto(&self) -> Arc<str> {
        Arc::clone(&self.tipo_producto)
    }
    pub fn marca(&self) -> Arc<str> {
        Arc::clone(&self.marca)
    }
    pub fn variedad(&self) -> Arc<str> {
        Arc::clone(&self.variedad)
    }
    pub fn presentacion(&self) -> &Presentacion {
        &self.presentacion
    }
    pub fn proveedores(&self) -> &Vec<RelacionProdProv> {
        &self.proveedores
    }
    pub fn nombre_completo(&self) -> String {
        format!(
            "{} {} {} {}",
            self.marca, self.tipo_producto, self.variedad, self.presentacion
        )
    }
    pub fn rm_code(&mut self, i: usize) {
        for j in i..2 {
            self.codigos_de_barras[j] = self.codigos_de_barras[j + 1];
        }
        self.codigos_de_barras[2] = 0;
    }

    // pub fn unifica_codes(&mut self) {
    //     let mut i=0;
    //     while i<self.codigos_de_barras.len(){
    //         let act=self.codigos_de_barras[i];
    //         let mut j=i+1;
    //         while j<self.codigos_de_barras.len(){
    //             if act==self.codigos_de_barras[j]{
    //                 self.codigos_de_barras.remove(j);
    //             }else{
    //                 j+=1;
    //             }
    //         }
    //         i+=1;
    //     }
    // }
    pub async fn eliminar(self, db: &Pool<Sqlite>) -> Res<()> {
        let qres: Option<BigIntDB> = sqlx::query_as!(
            BigIntDB,
            "select id as int from productos where id = ?",
            self.id
        )
        .fetch_optional(db)
        .await?;
        match qres {
            Some(model) => {
                sqlx::query("delete from productos where id = ?")
                    .bind(model.int)
                    .execute(db)
                    .await?;
                Ok(())
            }
            None => Err(AppError::NotFound {
                objeto: String::from("Producto"),
                instancia: self.id.to_string(),
            }),
        }
    }
    pub async fn fetch_code(code: CodeDB, db: &Pool<Sqlite>) -> Res<Producto> {
        let prod = code.producto.clone().unwrap();
        let qres:ProductoDB=query_as!(ProductoDB,
            r#"select id as "id:_", precio_venta as "precio_venta:_", porcentaje as "porcentaje:_", precio_costo as "precio_costo:_",
             tipo, marca, variedad, presentacion, size as "size:_", updated_at from productos where id = ?"#,
            prod).fetch_one(db).await?;
        Mapper::producto(db, qres).await
    }
    #[cfg(test)]
    pub fn desc(&self) -> String {
        format!(
            "{} {} {} {} {}",
            self.tipo_producto,
            self.marca,
            self.variedad,
            self.presentacion.get_cantidad(),
            self.presentacion.get_string()
        )
    }
    pub async fn editar(self, db: &Pool<Sqlite>) -> Res<()> {
        let qres: Option<BigIntDB> = sqlx::query_as!(
            BigIntDB,
            "select id as int from productos where id = ?",
            self.id
        )
        .fetch_optional(db)
        .await?;
        match qres {
            Some(model) => {
                if self.precio_venta != self.precio_costo * (1.0 + self.porcentaje / 100.0) {
                    return Err(AppError::IncorrectError(String::from(
                        "Cálculo de precio incorrecto",
                    )));
                }
                sqlx::query(
                    "update productos set precio_venta = ?, porcentaje = ?, precio_costo = ?, tipo = ?, marca = ?, variedad = ?, presentacion = ?, size = ?, updated_at = ? where id = ?")
                    .bind(self.precio_venta).bind(self.porcentaje).bind(self.precio_costo).bind(self.tipo_producto.as_ref()).bind(self.marca.as_ref()).bind(self.variedad.as_ref()).bind(self.presentacion.get_string()).bind(self.presentacion.get_cantidad()).bind(Utc::now().naive_local()).bind(model.int).execute(db).await?;
                Ok(())
            }
            None => Err(AppError::NotFound {
                objeto: String::from("Producto"),
                instancia: self.id.to_string(),
            }),
        }
    }
    pub fn to_shared(&self, codigo: i64) -> Res<ProductoSH> {
        Ok(ProductoSH {
            id: self.id,
            codigo_de_barras: match self.codigos_de_barras.iter().find(|cod| **cod == codigo) {
                Some(&a) => a.to_be_bytes(),
                None => {
                    return Err(AppError::IncorrectError(String::from(
                        "Codigo no encontrado",
                    )))
                }
            },
            precio_venta: self.precio_venta,
            tipo_producto: self.tipo_producto.clone(),
            marca: self.marca.clone(),
            variedad: self.variedad.clone(),
            presentacion: self.presentacion.clone(),
        })
    }
    pub fn to_shared_complete(&self) -> ProductoSHC {
        ProductoSHC {
            id: self.id,
            codigos_de_barras: [
                self.codigos_de_barras[0].to_be_bytes(),
                self.codigos_de_barras[1].to_be_bytes(),
                self.codigos_de_barras[2].to_be_bytes(),
            ],
            precio_venta: self.precio_venta,
            porcentaje: self.porcentaje,
            precio_costo: self.precio_costo,
            tipo_producto: self.tipo_producto.clone(),
            marca: self.marca.clone(),
            variedad: self.variedad.clone(),
            presentacion: self.presentacion.clone(),
            proveedores: self.proveedores.clone(),
        }
    }
    pub async fn from_shared(producto: ProductoSH, db: &Pool<Sqlite>)->Res<Self>{
        let qres = sqlx::query_as!(ProductoDB,r#"select id as "id:_",
        precio_venta as "precio_venta:_",
        porcentaje as "porcentaje:_",
        precio_costo as "precio_costo:_",
        tipo,
        marca,
        variedad,
        presentacion,
        size as "size:_",
        updated_at from productos where id = ?"#,producto.id).fetch_one(db).await?;
        Mapper::producto(db, qres).await
    }
    pub fn from_shared_complete(producto: ProductoSHC) -> Self {
        Producto {
            id: producto.id,
            codigos_de_barras: [
                i64::from_be_bytes(producto.codigos_de_barras[0]),
                i64::from_be_bytes(producto.codigos_de_barras[1]),
                i64::from_be_bytes(producto.codigos_de_barras[2]),
            ],
            precio_venta: producto.precio_venta,
            porcentaje: producto.porcentaje,
            precio_costo: producto.precio_costo,
            tipo_producto: producto.tipo_producto,
            marca: producto.marca,
            variedad: producto.variedad,
            presentacion: producto.presentacion,
            proveedores: producto.proveedores,
        }
    }
}

impl PartialEq for Producto {
    fn eq(&self, other: &Self) -> bool {
        let mut esta = false;
        for code in &self.codigos_de_barras {
            if other.codigos_de_barras.contains(&code) {
                esta = true;
            }
        }

        esta
    }
}

impl ValuableTrait for Producto {
    fn redondear(&self, politica: &f32) -> Producto {
        Producto {
            id: self.id,
            codigos_de_barras: self.codigos_de_barras.clone(),
            precio_venta: redondeo(politica, self.precio_venta),
            porcentaje: self.porcentaje,
            precio_costo: self.precio_costo,
            tipo_producto: self.tipo_producto.clone(),
            marca: self.marca.clone(),
            variedad: self.variedad.clone(),
            presentacion: self.presentacion.clone(),
            proveedores: self.proveedores.clone(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RelacionProdProv {
    proveedor: i32,
    codigo_interno: Option<i32>,
}

impl RelacionProdProv {
    pub fn build(proveedor: i32, codigo_interno: Option<i32>) -> Self {
        RelacionProdProv {
            proveedor,
            codigo_interno,
        }
    }
    pub fn proveedor(&self) -> &i32 {
        &self.proveedor
    }
    pub fn codigo_interno(&self) -> Option<i32> {
        self.codigo_interno
    }
}
