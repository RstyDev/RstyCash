use crate::{
    mods::{
        db::map::{
            BigIntDB, CajaDB, ClienteDB, ConfigDB, MedioPagoDB, PagoDB, PesableDB, ProductoDB,
            RelacionProdProvDB, RelatedPesDB, RelatedProdDB, RelatedRubDB, RubroDB, TotalDB,
            VentaDB,
        },
        AppError, Caja, Cli, Cliente, Config, MedioPago, Pesable, Presentacion, RelacionProdProv,
        Res, Rubro, User, Valuable,
    },
    Pago, Producto, Venta,
};
use sqlx::{query_as, Pool, Sqlite};
use std::{collections::HashMap, sync::Arc};

#[derive(Clone)]
pub struct Mapper;
impl Mapper {
    pub async fn caja(db: &Pool<Sqlite>, caja: CajaDB) -> Res<Caja> {
        let mut totales = HashMap::new();
        let totales_mod: Vec<TotalDB> = query_as!(
            TotalDB,
            r#"select medio, monto as "monto: _" from totales where caja = ? "#,
            caja.id
        )
        .fetch_all(db)
        .await?;
        if totales_mod.len() > 0 {
            for tot in totales_mod {
                totales.insert(Arc::from(tot.medio), tot.monto);
            }
        } else {
            let medios_mod: Vec<MedioPagoDB> = query_as!(
                MedioPagoDB,
                r#"select id as "id:i32", medio from medios_pago"#
            )
            .fetch_all(db)
            .await?;

            for medio in medios_mod {
                totales.insert(Arc::from(medio.medio.as_str()), 0.0);
            }
        }

        Ok(Caja::build(
            caja.id,
            caja.inicio,
            caja.cierre,
            caja.ventas_totales,
            caja.monto_inicio,
            caja.monto_cierre,
            caja.cajero.map(|c| Arc::from(c)),
            totales,
        ))
    }
    pub async fn config(db: &Pool<Sqlite>, config: ConfigDB) -> Res<Config> {
        let medios: sqlx::Result<Vec<MedioPagoDB>> = sqlx::query_as!(
            MedioPagoDB,
            r#"select id as "id:_", medio from medios_pago "#
        )
        .fetch_all(db)
        .await;
        let medios = medios?
            .iter()
            .map(|model| MedioPago::build(&model.medio, model.id))
            .collect::<Vec<MedioPago>>();
        Ok(Config::build(
            config.politica,
            config.formato.as_str(),
            config.mayus.as_str(),
            config.cantidad,
            medios,
        ))
    }
    pub fn rel_prod_prov(rel: &RelacionProdProvDB) -> RelacionProdProv {
        RelacionProdProv::build(rel.proveedor, rel.codigo)
    }
    pub async fn producto(db: &Pool<Sqlite>, prod: ProductoDB) -> Res<Producto> {
        let models: sqlx::Result<Vec<BigIntDB>> = sqlx::query_as!(
            BigIntDB,
            r#"select codigo as int from codigos where producto = ? limit 5"#,
            prod.id
        )
        .fetch_all(db)
        .await;
        let models = models?;
        let rels: Vec<RelacionProdProvDB> = sqlx::query_as!(
            RelacionProdProvDB,
            r#"select id as "id:_", producto as "producto:_", proveedor as "proveedor:_", codigo as "codigo:_" from relacion_prod_prov where producto = ?"#,
            prod.id
        )
        .fetch_all(db)
        .await?;
        let rels = rels
            .iter()
            .map(|r| Mapper::rel_prod_prov(r))
            .collect::<Vec<RelacionProdProv>>();
        let mut codigos = [0, 0, 0];
        let len = models.len();
        if len > 3 || len < 1 {
            return Err(AppError::IncorrectError(String::from(
                "Error de codigos en db incorrecto",
            )));
        } else {
            for i in 0..len {
                codigos[i] = models[i].int;
            }
        }
        let presentacion = match prod.presentacion.as_str() {
            "Gr" => Presentacion::Gr(prod.size),
            "Un" => Presentacion::Un(prod.size as u16),
            "Lt" => Presentacion::Lt(prod.size),
            "Ml" => Presentacion::Ml(prod.size as u16),
            "CC" => Presentacion::CC(prod.size as u16),
            "Kg" => Presentacion::Kg(prod.size),
            a => return Err(AppError::SizeSelection(a.to_string())),
        };
        Ok(Producto::build(
            prod.id,
            codigos,
            prod.precio_venta,
            prod.porcentaje,
            prod.precio_costo,
            prod.tipo.as_str(),
            prod.marca.as_str(),
            prod.variedad.as_str(),
            presentacion,
            rels,
        ))
    }
    pub fn pesable(pesable: PesableDB, codigo: i64) -> Pesable {
        Pesable::build(
            pesable.id,
            codigo,
            pesable.precio_peso,
            pesable.porcentaje,
            pesable.costo_kilo,
            pesable.descripcion.as_str(),
        )
    }
    pub fn rubro(rubro: RubroDB, codigo: i64) -> Rubro {
        Rubro::build(rubro.id, codigo, None, rubro.descripcion.as_str())
    }
    pub async fn pago(db: &Pool<Sqlite>, pago: PagoDB) -> Res<Pago> {
        let medio: Option<MedioPagoDB> = sqlx::query_as!(
            MedioPagoDB,
            r#"select id as "id:_", medio from medios_pago where id = ? limit 1"#,
            pago.medio_pago
        )
        .fetch_optional(db)
        .await?;
        let int_id = pago.id;
        match medio {
            Some(med) => Ok(Pago::build(
                int_id,
                MedioPago::build(&med.medio, med.id),
                pago.monto,
                pago.pagado,
            )),
            None => Err(AppError::IncorrectError(String::from(
                "No se encontro el medio pago correspondiente",
            ))),
        }
    }
    pub fn cliente(cliente: ClienteDB) -> Cli {
        Cli::build(
            cliente.dni,
            Arc::from(cliente.nombre),
            cliente.activo,
            cliente.time,
            cliente.limite,
        )
    }
    pub async fn venta(db: &Pool<Sqlite>, venta: VentaDB, user: &Option<Arc<User>>) -> Res<Venta> {
        {
            let qres:Vec<RelatedProdDB>=sqlx::query_as!(RelatedProdDB,r#"select productos.id as "id:_",
                    precio as "precio: _",porcentaje as "porcentaje: _", precio_costo as "precio_costo: _", tipo, marca, variedad, presentacion, size as "size: _", cantidad as "cantidad: _"
                    from relacion_venta_prod inner join productos on relacion_venta_prod.id = productos.id where venta = ?
                     "#,venta.id).fetch_all(db).await?;
            let mut productos = Vec::new();
            for rel in qres {
                let qres: Vec<BigIntDB> = sqlx::query_as!(
                    BigIntDB,
                    r#"select codigo as int from codigos where producto = ? limit 5"#,
                    rel.id
                )
                .fetch_all(db)
                .await?;
                let rels: Vec<RelacionProdProvDB> = sqlx::query_as!(
                    RelacionProdProvDB,
                    r#"select id as "id:_", producto as "producto:_", proveedor as "proveedor:_", codigo as "codigo:_" from relacion_prod_prov where producto = ?"#,
                    rel.id
                )
                .fetch_all(db)
                .await?;
                let rels = rels
                    .iter()
                    .map(|r| Mapper::rel_prod_prov(r))
                    .collect::<Vec<RelacionProdProv>>();
                let mut codigos = [0, 0, 0];
                let len = qres.len();
                if len > 3 || len < 1 {
                    return Err(AppError::IncorrectError(String::from(
                        "Error de codigos en db incorrecto",
                    )));
                } else {
                    for i in 0..len {
                        codigos[i] = qres[i].int;
                    }
                }
                productos.push(Valuable::Prod((
                    rel.cantidad,
                    Producto::build(
                        rel.id,
                        codigos,
                        rel.precio,
                        rel.porcentaje,
                        rel.precio_costo,
                        &rel.tipo,
                        &rel.marca,
                        &rel.variedad,
                        Presentacion::build(&rel.presentacion, rel.size),
                        rels,
                    ),
                )))
            }
            let qres:Vec<RelatedPesDB>=sqlx::query_as!(RelatedPesDB,r#"select pesables.id as "id:_",
                    precio_peso as "precio_peso: _", porcentaje as "porcentaje: _", costo_kilo as "costo_kilo: _", descripcion, cantidad as "cantidad: _", updated_at
                    from relacion_venta_pes inner join pesables on relacion_venta_pes.id = pesables.id where venta = ?
                     "#,venta.id).fetch_all(db).await?;
            for rel in qres {
                let qres: Option<BigIntDB> = sqlx::query_as!(
                    BigIntDB,
                    r#"select codigo as int from codigos where pesable = ? limit 1"#,
                    rel.id
                )
                .fetch_optional(db)
                .await?;
                match qres {
                    Some(model) => productos.push(Valuable::Pes((
                        rel.cantidad,
                        Pesable::build(
                            rel.id,
                            model.int,
                            rel.precio_peso,
                            rel.porcentaje,
                            rel.costo_kilo,
                            &rel.descripcion,
                        ),
                    ))),
                    None => {
                        return Err(AppError::IncorrectError(String::from(
                            "No se encontro codigo de pesable",
                        )))
                    }
                }
            }
            let qres:Vec<RelatedRubDB>=sqlx::query_as!(RelatedRubDB,r#"select rubros.id as "id:_", descripcion, updated_at, cantidad as "cantidad: _", precio as "precio: _"
                    from relacion_venta_rub inner join rubros on relacion_venta_rub.id = rubros.id where venta = ?
                     "#,venta.id).fetch_all(db).await?;
            for rel in qres {
                let qres: Option<BigIntDB> = sqlx::query_as!(
                    BigIntDB,
                    r#"select codigo as int from codigos where pesable = ? limit 1"#,
                    rel.id
                )
                .fetch_optional(db)
                .await?;
                match qres {
                    Some(model) => productos.push(Valuable::Rub((
                        rel.cantidad,
                        Rubro::build(
                            rel.id,
                            model.int,
                            Some(rel.precio),
                            rel.descripcion.as_str(),
                        ),
                    ))),
                    None => {
                        return Err(AppError::IncorrectError(String::from(
                            "No se encontro codigo de pesable",
                        )))
                    }
                }
            }
            let qres: Vec<PagoDB> = sqlx::query_as!(
                PagoDB,
                r#"select id as "id:_", medio_pago as "medio_pago:_", monto as "monto: _", pagado as "pagado: f32",
    venta as "venta:_" from pagos where venta = ? "#,
                venta.id
            )
            .fetch_all(db)
            .await?;
            let mut pagos = Vec::new();
            for pago in qres {
                let qres: Option<MedioPagoDB> = sqlx::query_as!(
                    MedioPagoDB,
                    r#"select id as "id:_", medio from medios_pago where id = ? limit 1"#,
                    pago.medio_pago
                )
                .fetch_optional(db)
                .await?;
                let medio = match qres {
                    Some(medio_p) => MedioPago::build(medio_p.medio.as_str(), medio_p.id),
                    None => {
                        return Err(AppError::IncorrectError(String::from(
                            "no es encontro medio_pago de pago",
                        )))
                    }
                };
                pagos.push(Pago::build(pago.id, medio, pago.monto, pago.pagado))
            }
            let qres: Option<ClienteDB> = sqlx::query_as!(
                ClienteDB,
                r#"select dni as "dni: _", nombre, limite as "limite: _", activo, time from clientes where dni = ? limit 1"#,
                venta.cliente
            )
            .fetch_optional(db)
            .await?;
            let cliente = match qres {
                Some(cliente) => {
                    if cliente.dni == 1 {
                        Cliente::Final
                    } else {
                        Cliente::Regular(Cli::build(
                            cliente.dni,
                            Arc::from(cliente.nombre),
                            cliente.activo,
                            cliente.time,
                            cliente.limite,
                        ))
                    }
                }
                None => Cliente::Final,
            };
            Ok(Venta::build(
                venta.id,
                venta.monto_total,
                productos,
                pagos,
                venta.monto_pagado,
                user.clone(),
                cliente,
                venta.paga,
                venta.cerrada,
                venta.time,
            ))
        }
    }
}

pub mod map {
    use chrono::NaiveDateTime;
    use sqlx::FromRow;

    #[derive(FromRow)]
    pub struct BigIntDB {
        pub int: i64,
    }
    #[derive(FromRow)]
    pub struct IntDB {
        pub int: i32,
    }

    #[derive(FromRow)]
    pub struct DoubleDB {
        pub double: f64,
    }

    #[derive(FromRow)]
    pub struct FloatDB {
        pub float: f32,
    }

    #[derive(FromRow)]
    pub struct BoolDB {
        pub val: bool,
    }

    #[derive(FromRow)]
    pub struct StringDB {
        pub string: String,
    }

    #[derive(FromRow)]
    pub struct MedioPagoDB {
        pub id: i32,
        pub medio: String,
    }
    #[derive(FromRow)]
    pub struct CajaParcialDB {
        pub id: i32,
        pub cierre: Option<NaiveDateTime>,
        pub ventas_totales: f32,
        pub cajero: Option<String>,
    }

    #[derive(FromRow)]
    pub struct CajaDB {
        pub id: i32,
        pub inicio: NaiveDateTime,
        pub cierre: Option<NaiveDateTime>,
        pub monto_inicio: f32,
        pub monto_cierre: Option<f32>,
        pub ventas_totales: f32,
        pub cajero: Option<String>,
    }

    #[derive(FromRow, Clone)]
    pub struct ClienteDB {
        pub dni: i32,
        pub nombre: String,
        pub limite: Option<f32>,
        pub activo: bool,
        pub time: NaiveDateTime,
    }
    #[derive(FromRow)]
    pub struct ConfigDB {
        pub id: i32,
        pub politica: f32,
        pub formato: String,
        pub mayus: String,
        pub cantidad: u8,
    }
    #[derive(FromRow)]
    pub struct ProvDB {
        pub id: i32,
        pub nombre: String,
        pub contacto: Option<i32>,
        pub updated: NaiveDateTime,
    }
    #[derive(FromRow)]
    pub struct CodeDB {
        pub codigo: i64,
        pub producto: Option<i32>,
        pub pesable: Option<i32>,
        pub rubro: Option<i32>,
    }
    #[derive(FromRow)]
    pub struct PesableDB {
        pub id: i32,
        pub precio_peso: f32,
        pub porcentaje: f32,
        pub costo_kilo: f32,
        pub descripcion: String,
        pub updated_at: NaiveDateTime,
    }
    #[derive(FromRow)]
    pub struct RelatedPesDB {
        pub id: i32,
        pub precio_peso: f32,
        pub porcentaje: f32,
        pub costo_kilo: f32,
        pub descripcion: String,
        pub updated_at: NaiveDateTime,
        pub cantidad: f32,
    }
    #[derive(FromRow)]
    pub struct CodedPesDB {
        pub id: i32,
        pub precio_peso: f32,
        pub codigo: i64,
        pub porcentaje: f32,
        pub costo_kilo: f32,
        pub descripcion: String,
        pub updated_at: NaiveDateTime,
    }
    #[derive(FromRow)]
    pub struct RubroDB {
        pub id: i32,
        pub descripcion: String,
        pub updated_at: NaiveDateTime,
    }
    #[derive(FromRow)]
    pub struct RelatedRubDB {
        pub id: i32,
        pub descripcion: String,
        pub updated_at: NaiveDateTime,
        pub cantidad: u8,
        pub precio: f32,
    }
    #[derive(FromRow)]
    pub struct CodedRubDB {
        pub id: i32,
        pub descripcion: String,
        pub updated_at: NaiveDateTime,
        pub codigo: i64,
    }
    #[derive(FromRow)]
    pub struct ProductoDB {
        pub id: i32,
        pub precio_venta: f32,
        pub porcentaje: f32,
        pub precio_costo: f32,
        pub tipo: String,
        pub marca: String,
        pub variedad: String,
        pub presentacion: String,
        pub size: f32,
        pub updated_at: NaiveDateTime,
    }

    #[derive(FromRow)]
    pub struct RelatedProdDB {
        pub id: i32,
        pub precio: f32,
        pub porcentaje: f32,
        pub precio_costo: f32,
        pub tipo: String,
        pub marca: String,
        pub variedad: String,
        pub presentacion: String,
        pub size: f32,
        pub cantidad: u8,
    }
    #[derive(FromRow)]
    pub struct UserDB {
        pub id: i32,
        pub user_id: String,
        pub nombre: String,
        pub pass: i64,
        pub rango: String,
    }
    #[derive(FromRow)]
    pub struct DeudaDB {
        pub id: i32,
        pub cliente: i32,
        pub pago: i32,
        pub monto: f32,
    }
    #[derive(FromRow)]
    pub struct MovimientoDB {
        pub id: i32,
        pub caja: i32,
        pub tipo: bool,
        pub monto: f32,
        pub descripcion: Option<String>,
        pub time: NaiveDateTime,
    }
    #[derive(FromRow)]
    pub struct PagoDB {
        pub id: i32,
        pub medio_pago: i32,
        pub monto: f32,
        pub pagado: f32,
        pub venta: i32,
    }
    #[derive(FromRow)]
    pub struct RelacionProdProvDB {
        pub id: i32,
        pub producto: i32,
        pub proveedor: i32,
        pub codigo: Option<i32>,
    }
    #[derive(FromRow)]
    pub struct RelacionVentaPesDB {
        pub id: i32,
        pub venta: i32,
        pub pesable: i32,
        pub cantidad: f32,
        pub precio_kilo: f32,
    }
    #[derive(FromRow)]
    pub struct RelacionVentaProdDB {
        pub id: i32,
        pub venta: i32,
        pub producto: i32,
        pub cantidad: u8,
        pub precio: f32,
    }
    #[derive(FromRow)]
    pub struct RelacionVentaRubDB {
        pub id: i32,
        pub venta: i32,
        pub rubro: i32,
        pub cantidad: u8,
        pub precio: f32,
    }
    #[derive(FromRow)]
    pub struct VentaDB {
        pub id: i32,
        pub time: NaiveDateTime,
        pub monto_total: f32,
        pub monto_pagado: f32,
        pub cliente: i32,
        pub cerrada: bool,
        pub paga: bool,
        pub pos: bool,
    }
    #[derive(FromRow)]
    pub struct TotalDB {
        pub medio: String,
        pub monto: f32,
    }
}

// async fn test(db: &Pool<Sqlite>) {
//     let res: sqlx::Result<Option<Venta>> = query_as!(Venta, "select * from ventas")
//         .fetch_optional(db)
//         .await;
//     let res = res.unwrap().unwrap();
// }
