use crate::mods::{
    crear_file,
    db::{
        fresh,
        map::{
            BigIntDB, ClienteDB, CodeDB, CodedPesDB, CodedRubDB, IntDB, ProductoDB, ProvDB,
            RelacionProdProvDB, StringDB, UserDB,
        },
        Mapper,
    },
    get_hash, leer_file, AppError, Caja, Cli, Config, Db, Movimiento, Pago, Pesable, Presentacion,
    Producto, Proveedor, Rango, RelacionProdProv, Res, Rubro, User, Valuable, Venta,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::{collections::HashSet, sync::Arc};
use tauri::async_runtime::{self, block_on};
use Valuable as V;

use super::{proveedor::ProveedorSH, venta::VentaSH, Cliente, UserSH};

const CUENTA: &str = "Cuenta Corriente";
#[derive(Clone)]
pub struct Sistema {
    user: Option<Arc<User>>,
    db: Arc<Pool<Sqlite>>,
    clientes: Vec<Cli>,
    caja: Caja,
    configs: Config,
    ventas: Ventas,
    proveedores: Vec<Proveedor>,
    relaciones: Vec<RelacionProdProv>,
    stash: Vec<Venta>,
    registro: Vec<Venta>,
}
#[derive(Serialize, Deserialize)]
pub struct SistemaSH {
    user: UserSH,
    caja: Caja,
    clientes: Vec<Cli>,
    configs: Config,
    ventas: [VentaSH; 2],
    proveedores: Vec<ProveedorSH>,
}
#[derive(Clone)]
pub struct Ventas {
    pub a: Venta,
    pub b: Venta,
}

impl<'a> Sistema {
    pub fn access(&self) {
        if self.user.is_none() {
            panic!("Sesión no iniciada");
        }
    }
    pub fn agregar_cliente(&self, cliente: Cli) -> Res<Cli> {
        async_runtime::block_on(async { Cli::new_to_db(self.db(), cliente).await })
    }
    pub fn agregar_pago(&mut self, pago: Pago, pos: bool) -> Res<f32> {
        let res;
        if pos {
            if !pago.medio_pago().desc().as_ref().eq("Efectivo")
                && self.ventas.a.monto_pagado() + pago.monto() > self.ventas.a.monto_total()
            {
                return Err(AppError::AmountError {
                    a_pagar: self.ventas.a.monto_total() - self.ventas.a.monto_pagado(),
                    pagado: pago.monto(),
                });
            } else {
                res = self.ventas.a.agregar_pago(pago);
            }
        } else {
            if !pago.medio_pago().desc().as_ref().eq("Efectivo")
                && self.ventas.b.monto_pagado() + pago.monto() > self.ventas.b.monto_total()
            {
                return Err(AppError::AmountError {
                    a_pagar: self.ventas.b.monto_total() - self.ventas.b.monto_pagado(),
                    pagado: pago.monto(),
                });
            } else {
                res = self.ventas.b.agregar_pago(pago);
            }
        }
        println!("{:#?}", res);
        if let Ok(a) = res {
            if a <= 0.0 {
                self.cerrar_venta(pos)?
            }
        }
        println!("Aca esta la caja {:#?} -----****", self.caja);
        res
    }
    pub fn agregar_usuario(&self, user: User) -> Res<String> {
        async_runtime::block_on(async { user.new_to_db(self.db()).await })?;
        Ok(String::from("Usuario agregado correctamente"))
    }
    #[cfg(test)]
    pub fn test(user: Option<Arc<User>>, db: Arc<Pool<Sqlite>>) -> Res<Sistema> {
        let w1 = Arc::clone(&db);
        async_runtime::block_on(async { fresh(w1.as_ref()).await });
        let configs =
            async_runtime::block_on(async { Config::get_or_def(&db.as_ref()).await }).unwrap();
        let caja =
            async_runtime::block_on(async { Caja::new(&db.as_ref(), Some(0.0), &configs).await })?;
        let w2 = Arc::clone(&db);
        let w3 = Arc::clone(&db);
        let r2 = Arc::clone(&db);
        let sis = Sistema {
            user,
            db,
            caja,
            configs,
            ventas: Ventas {
                a: async_runtime::block_on(async {
                    Venta::get_or_new(None, w2.as_ref(), true).await
                })?,
                b: async_runtime::block_on(async {
                    Venta::get_or_new(None, w3.as_ref(), false).await
                })?,
            },
            proveedores: Vec::new(),
            relaciones: Vec::new(),
            stash: Vec::new(),
            registro: Vec::new(),
            clientes: Vec::new(),
        };
        async_runtime::block_on(async { Sistema::procesar_test(Arc::clone(&w2)).await })?;
        Ok(sis)
    }
    pub fn new(db: Arc<Pool<Sqlite>>) -> Res<Sistema> {
        async_runtime::block_on(async {
            let qres: Option<IntDB> =
                match sqlx::query_as!(IntDB, r#"select id as "int:_" from productos limit 1"#)
                    .fetch_optional(db.as_ref())
                    .await
                {
                    Ok(a) => a,
                    Err(e) => loop {
                        println!("{e}")
                    },
                };
            if qres.is_none() {
                fresh(db.as_ref()).await
            }
        });
        let path_proveedores = "Proveedores.json";
        let path_relaciones = "Relaciones.json";
        let mut relaciones = Vec::new();
        leer_file(&mut relaciones, path_relaciones)?;
        let mut proveedores: Vec<Proveedor> = Vec::new();
        leer_file(&mut proveedores, path_proveedores)?;
        let aux = Arc::clone(&db);
        let db = Arc::clone(&db);
        let configs = async_runtime::block_on(async { Config::get_or_def(db.as_ref()).await })?;
        let caja =
            async_runtime::block_on(async { Caja::new(aux.as_ref(), Some(0.0), &configs).await })?;
        let stash = Vec::new();
        let registro = Vec::new();
        println!("{:#?}", caja);
        let w1 = Arc::clone(&db);
        
        let mut sis = Sistema {
            user: None,
            db,
            caja,
            configs,
            ventas: Ventas {
                a: async_runtime::block_on(async {
                    Venta::get_or_new(None, w1.as_ref(), true).await
                })?,
                b: async_runtime::block_on(async {
                    Venta::get_or_new(None, w1.as_ref(), false).await
                })?,
            },
            proveedores: proveedores.clone(),
            relaciones,
            stash,
            registro,
            clientes: Vec::new(),
        };
        sis.clientes = block_on(sis.get_clientes())?;
        async_runtime::block_on(async {
            Sistema::procesar(Arc::clone(&sis.db), sis.proveedores.clone()).await
        })?;
        Ok(sis)
    }
    fn generar_reporte_caja(&self) {
        println!("{:#?}", self.caja);
        println!("Faltante");
    }
    pub fn user(&self) -> Option<Arc<User>> {
        match &self.user {
            Some(a) => Some(Arc::clone(a)),
            None => None,
        }
    }

    pub fn cancelar_venta(&mut self, pos: bool) -> Res<()> {
        if pos {
            self.ventas.a.empty();
        } else {
            self.ventas.b.empty();
        }
        Ok(())
    }
    pub fn cerrar_caja(&mut self, monto_actual: f32) -> Res<()> {
        self.caja.set_cajero(self.user().unwrap().nombre());
        let db = Arc::clone(&self.db);
        async_runtime::block_on(self.caja.set_n_save(db.as_ref(), monto_actual))?;
        self.generar_reporte_caja();
        self.caja = async_runtime::block_on(async {
            Caja::new(self.db.as_ref(), Some(monto_actual), &self.configs).await
        })?;
        Ok(())
    }
    pub fn eliminar_usuario(&self, user: User) -> Res<()> {
        async_runtime::block_on(async { Db::eliminar_usuario(user, self.db.as_ref()).await })?;
        Ok(())
    }

    pub fn caja(&self) -> &Caja {
        &self.caja
    }
    #[cfg(test)]
    async fn procesar_test(db: Arc<Pool<Sqlite>>) -> Res<()> {
        use tauri::async_runtime::JoinHandle;

        let read_db2 = Arc::clone(&db);
        let db2 = db.clone();
        let a: JoinHandle<Result<(), AppError>> = async_runtime::spawn(async move {
            let medios = [CUENTA, "Efectivo", "Crédito", "Débito"];
            for i in 0..medios.len() {
                sqlx::query("insert into medios_pago values (?, ?)")
                    .bind(i as i64)
                    .bind(medios[i])
                    .execute(db2.as_ref())
                    .await?;
            }
            return Ok(());
        });
        let qres: Option<IntDB> =
            sqlx::query_as!(IntDB, r#"select id as "int:_" from users limit 1"#)
                .fetch_optional(read_db2.as_ref())
                .await?;
        if qres.is_none() {
            sqlx::query("insert into users values (?, ?, ?, ?)")
                .bind("test")
                .bind(get_hash("9876"))
                .bind(Rango::Admin.to_string())
                .bind("Admin")
                .execute(db.as_ref())
                .await?;
        }
        Ok(())
    }
    async fn procesar(db: Arc<Pool<Sqlite>>, proveedores: Vec<Proveedor>) -> Res<()> {
        let path_productos = "Productos.json";
        let path_configs = "Configs.json";
        let path_pesables = "Pesables.json";
        let mut configs = Vec::<Config>::new();
        leer_file(&mut configs, path_configs)?;
        if configs.len() == 0 {
            configs.push(Config::default());
            crear_file(path_configs, &mut configs)?;
        }
        let mut productos: Vec<Producto> = Vec::new();
        let mut rubros: Vec<Rubro> = Vec::new();
        let path_rubros = "Rubros.json";
        let mut pesables: Vec<Pesable> = Vec::new();

        leer_file(&mut rubros, path_rubros)?;
        leer_file(&mut pesables, path_pesables)?;
        leer_file(&mut productos, path_productos)?;

        let mut rubros_valuable: Vec<Valuable> =
            rubros.iter().map(|a| V::Rub((0, a.to_owned()))).collect();
        let mut pesables_valuable: Vec<Valuable> = pesables
            .iter()
            .map(|a| V::Pes((0.0, a.to_owned())))
            .collect();
        let mut valuables: Vec<Valuable> = productos
            .clone()
            .iter()
            .map(|a| V::Prod((0, a.to_owned())))
            .collect();
        valuables.append(&mut pesables_valuable);
        valuables.append(&mut rubros_valuable);
        let write_db2 = Arc::clone(&db);
        let read_db2 = Arc::clone(&db);
        let medios = [CUENTA, "Efectivo", "Crédito", "Débito"];
        for i in 0..medios.len() {
            let qres: Option<IntDB> = sqlx::query_as!(
                IntDB,
                r#"select id as "int:_" from medios_pago where medio = ? limit 1"#,
                medios[i]
            )
            .fetch_optional(db.as_ref())
            .await?;
            if qres.is_none() {
                sqlx::query("insert into medios_pago values (?, ?)")
                    .bind(i as i64)
                    .bind(medios[i])
                    .execute(db.as_ref())
                    .await?;
            }
        }
        let qres: Option<IntDB> =
            sqlx::query_as!(IntDB, r#"select id as "int:_" from users limit 1"#)
                .fetch_optional(read_db2.as_ref())
                .await?;
        if qres.is_none() {
            sqlx::query("insert into users (user_id, nombre, pass, rango) values (?, ?, ?, ?)")
                .bind("admin")
                .bind("Admin")
                .bind(get_hash("1234"))
                .bind(Rango::Admin.to_string())
                .execute(write_db2.as_ref())
                .await?;
            //eprintln!("Aca esta el largo de valuables {}",valuables.len());
            Db::cargar_todos_los_provs(proveedores, write_db2.as_ref()).await?;
            Db::cargar_todos_los_valuables(valuables, write_db2.as_ref()).await?;
            //Db::cargar_todas_las_relaciones_prod_prov(relaciones, write_db2.as_ref()).await?;
        }
        Ok(())
    }
    pub fn get_logged_state(&self) -> bool {
        self.user.is_some()
    }

    pub async fn get_clientes(&self) -> Res<Vec<Cli>> {
        let qres: Vec<ClienteDB> = sqlx::query_as!(ClienteDB, r#"select id as "id:_", nombre, dni as "dni:_", limite as "limite:_", activo, time from clientes "#)
            .fetch_all(self.db())
            .await?;
        Ok(qres
            .iter()
            .map(|cli| {
                Cli::build(
                    cli.id,
                    Arc::from(cli.nombre.to_owned()),
                    cli.dni,
                    cli.activo,
                    cli.time,
                    cli.limite,
                )
            })
            .collect::<Vec<Cli>>())
    }
    pub async fn try_login(&mut self, user: User) -> Res<Rango> {
        let id = user.id();
        let pass = user.pass();
        //println!("{:#?}",user);
        let qres: Option<UserDB> = sqlx::query_as!(
            UserDB,
            r#"select user_id as "user_id:_",nombre as "nombre:_",pass as "pass:_",rango as "rango:_",id as "id:_" from users where user_id = ? and pass = ? limit 1"#,
            id,
            pass
        )
        .fetch_optional(self.db())
        .await?;
        match qres {
            None => {
                let qres: Option<IntDB> = sqlx::query_as!(
                    IntDB,
                    r#"select id as "int:_" from users where user_id = ?"#,
                    id
                )
                .fetch_optional(self.db.as_ref())
                .await?;
                match qres {
                    Some(_) => Err(AppError::IncorrectError("Contraseña".to_string())),
                    None => Err(AppError::IncorrectError("Usuario".to_string())),
                }
            }
            Some(user_db) => {
                let rango = match user_db.rango.as_str() {
                    "Admin" => Rango::Admin,
                    "Cajero" => Rango::Cajero,
                    _ => panic!("No existe"),
                };
                self.user = Some(Arc::from(User::build(
                    Arc::from(user_db.user_id),
                    Arc::from(user_db.nombre),
                    *user.pass(),
                    rango,
                )));
                self.ventas = Ventas {
                    a: Venta::get_or_new(Some(self.arc_user()), &self.db, true).await?,
                    b: Venta::get_or_new(Some(self.arc_user()), &self.db, false).await?,
                };
                Ok(self.arc_user().rango().clone())
            }
        }
    }
    pub async fn val_filtrado(
        &self,
        filtro: &str,
        db: &Pool<Sqlite>,
    ) -> Result<Vec<Valuable>, AppError> {
        let mut res: Vec<Valuable> = Vec::new();
        match filtro.parse::<i64>() {
            Ok(code) => {
                let qres: Option<CodeDB> =
                    sqlx::query_as!(CodeDB, r#"select codigo as "codigo:_", producto as "producto:_", pesable as "pesable:_", rubro as "rubro:_" from codigos where codigo = ?"#, code)
                        .fetch_optional(db)
                        .await?;
                match qres {
                    None => return Ok(res),
                    Some(code) => {
                        if code.producto.is_some() {
                            res.push(V::Prod((0, Producto::fetch_code(code, db).await?)))
                        } else if code.pesable.is_some() {
                            res.push(V::Pes((0.0, Pesable::fetch_code(code, db).await?)))
                        } else if code.rubro.is_some() {
                            res.push(V::Rub((0, Rubro::fetch_code(code, db).await?)));
                        }
                    }
                }
            }
            Err(_) => {
                let filtros = filtro
                    .split(' ')
                    .filter(|p| p.len() > 0)
                    .collect::<Vec<&str>>();
                let mut query = String::from(
                    r#"select id, precio_venta, porcentaje, precio_costo, tipo, marca, variedad, presentacion, size, updated_at from productos where ((tipo like ?) or (marca like ?) or (variedad like ?) or (presentacion like ?) or (size like ?))"#,
                );
                let row =
                    " and ((tipo like ?) or (marca like ?) or (variedad like ?) or (presentacion like ?) or (size like ?))";
                for _ in 1..filtros.len() {
                    query.push_str(row);
                }
                query.push_str(" limit 15");
                let mut qres = sqlx::query_as(query.as_ref());
                for filtro in &filtros {
                    let filtro = format!("%{filtro}%");
                    qres = qres
                        .bind(filtro.to_owned())
                        .bind(filtro.to_owned())
                        .bind(filtro.to_owned())
                        .bind(filtro.to_owned());
                }
                let qres: Vec<ProductoDB> = qres.fetch_all(db).await?;
                for prod in qres {
                    let rels: Vec<RelacionProdProvDB> = sqlx::query_as!(
                        RelacionProdProvDB,
                        r#"select id as "id:_", producto as "producto:_",proveedor as "proveedor:_", codigo as "codigo:_" from relacion_prod_prov where producto = ?"#,
                        prod.id
                    )
                    .fetch_all(db)
                    .await?;
                    let codes: Vec<BigIntDB> = sqlx::query_as!(
                        BigIntDB,
                        r#"select codigo as "int:_" from codigos where producto = ?"#,
                        prod.id
                    )
                    .fetch_all(db)
                    .await?;
                    let mut codigos = [0, 0, 0];
                    if codes.len() > 3 || codes.len() < 1 {
                        return Err(AppError::IncorrectError(String::from(
                            "Mas de 3 codigos para el mismo prod en db",
                        )));
                    } else {
                        for i in 0..codes.len() {
                            codigos[i] = codes[i].int;
                        }
                    }
                    let rels = rels
                        .iter()
                        .map(|r| Mapper::rel_prod_prov(r))
                        .collect::<Vec<RelacionProdProv>>();
                    res.push(V::Prod((
                        0,
                        Producto::build(
                            prod.id,
                            codigos,
                            prod.precio_venta,
                            prod.porcentaje,
                            prod.precio_costo,
                            prod.tipo.as_str(),
                            prod.marca.as_str(),
                            prod.variedad.as_str(),
                            Presentacion::build(prod.presentacion.as_str(), prod.size),
                            rels,
                        ),
                    )));
                }
                let mut query=String::from("select id, precio_peso, codigo, porcentaje, costo_kilo, descripcion, updated_at from pesables inner join codigos on pesables.id = codigos.pesable where (descripcion like ?)");
                let row = " and (descripcion like ?)";
                for _ in 1..filtros.len() {
                    query.push_str(row);
                }
                query.push_str(" limit 15");
                let mut qres = sqlx::query_as(query.as_str());
                for filtro in &filtros {
                    let filtro = format!("%{filtro}%");
                    qres = qres.bind(filtro.to_owned());
                }
                let qres: Vec<CodedPesDB> = qres.fetch_all(db).await?;
                res.append(
                    &mut qres
                        .iter()
                        .map(|pes| {
                            V::Pes((
                                0.0,
                                Pesable::build(
                                    pes.id,
                                    pes.codigo,
                                    pes.precio_peso,
                                    pes.porcentaje,
                                    pes.costo_kilo,
                                    pes.descripcion.as_str(),
                                ),
                            ))
                        })
                        .collect::<Vec<V>>(),
                );
                let mut query=String::from("select id, descripcion, updated_at, codigo from rubros inner join codigos on rubros.id = codigos.rubro where (descripcion like ?)");
                for _ in 1..filtros.len() {
                    query.push_str(row);
                }
                query.push_str(" limit 15");
                let mut qres = sqlx::query_as(query.as_str());
                for filtro in filtros {
                    let filtro = format!("%{filtro}%");
                    qres = qres.bind(filtro);
                }
                let qres: Vec<CodedRubDB> = qres.fetch_all(db).await?;
                res.append(
                    &mut qres
                        .iter()
                        .map(|rub| {
                            V::Rub((
                                0,
                                Rubro::build(rub.id, rub.codigo, None, rub.descripcion.as_str()),
                            ))
                        })
                        .collect::<Vec<V>>(),
                );
            }
        }
        Ok(res
            .iter()
            .cloned()
            .take(*self.configs.cantidad_productos() as usize)
            .collect())
    }
    pub fn cerrar_sesion(&mut self) {
        self.user = None;
    }

    fn splitx(filtro: &str) -> Res<(f32, &str)> {
        let partes = filtro.split('*').collect::<Vec<&str>>();
        match partes.len() {
            1 => Ok((1.0, partes[0])),
            2 => Ok((partes[0].parse::<f32>()?, partes[1])),
            _ => Err(AppError::ParseError),
        }
    }
    pub async fn proveedores(&self) -> Res<Vec<Proveedor>> {
        let qres: Vec<ProvDB> = sqlx::query_as!(
            ProvDB,
            r#"select id as "id:_", nombre, contacto as "contacto:_", updated from proveedores"#
        )
        .fetch_all(self.db.as_ref())
        .await?;
        Ok(qres
            .iter()
            .map(|prov| {
                Proveedor::build(
                    prov.id,
                    prov.nombre.as_str(),
                    prov.contacto.map(|c| c as i64),
                )
            })
            .collect::<Vec<Proveedor>>())
    }
    pub fn configs(&self) -> &Config {
        &self.configs
    }

    pub fn eliminar_pago(&mut self, pos: bool, id: i32) -> Res<Vec<Pago>> {
        let res;
        if pos {
            self.ventas.a.eliminar_pago(id, &self.db)?;
            res = self.venta(pos).pagos()
        } else {
            self.ventas.b.eliminar_pago(id, &self.db)?;
            res = self.venta(pos).pagos()
        }

        Ok(res)
    }
    pub fn set_configs(&mut self, configs: Config) {
        self.configs = configs;
        async_runtime::block_on(async {
            sqlx::query("update config set cantidad = ?, mayus = ?, formato = ?, politica = ?")
                .bind(self.configs.cantidad_productos())
                .bind(self.configs.modo_mayus().to_string())
                .bind(self.configs.formato().to_string())
                .bind(self.configs.politica())
                .execute(self.db())
                .await
                .unwrap();
        });
    }
    pub fn pagar_deuda_especifica(&self, cliente: i64, venta: Venta) -> Res<Venta> {
        async_runtime::block_on(async {
            Cli::pagar_deuda_especifica(cliente, &self.db, venta, &self.user).await
        })
    }
    pub fn pagar_deuda_general(&self, cliente: i64, monto: f32) -> Res<f32> {
        async_runtime::block_on(async { Cli::pagar_deuda_general(cliente, &self.db, monto).await })
    }
    // pub async fn get_cliente(&self, id: i64) -> Res<Cliente> {
    //     let model = CliDB::Entity::find_by_id(id)
    //         .one(self.db.as_ref())
    //         .await?
    //         .unwrap();
    //     Ok(Mapper::map_model_cli(model).await)
    // }
    pub async fn agregar_producto(&self, mut prod: Producto) -> Res<String> {
        prod.new_to_db(&self.db).await?;
        Ok(String::from("Producto agregado correctamente"))
    }
    pub fn agregar_pesable(&self, mut pesable: Pesable, db: &Pool<Sqlite>) -> Res<String> {
        Ok(async_runtime::block_on(async {
            pesable.new_to_db(db).await
        })?)
    }
    pub fn agregar_rubro(&self, rubro: Rubro, db: &Pool<Sqlite>) -> Res<String> {
        async_runtime::block_on(async { rubro.new_to_db(db).await })?;
        Ok(String::from("Rubro agregado exitosamente"))
    }
    pub fn agregar_proveedor(&mut self, proveedor: Proveedor) -> Res<()> {
        async_runtime::block_on(async { Proveedor::new_to_db(proveedor, self.db()).await })?;
        Ok(())
    }
    pub async fn agregar_producto_a_venta(&mut self, prod: V, pos: bool) -> Res<()> {
        let existe = match &prod {
            Valuable::Prod((_, prod)) => {
                let qres: Option<IntDB> = sqlx::query_as!(
                    IntDB,
                    r#"select id as "int:_" from productos where id = ? "#,
                    *prod.id()
                )
                .fetch_optional(self.db())
                .await?;
                qres.is_some()
            }
            Valuable::Pes((_, pes)) => {
                let qres: Option<IntDB> = sqlx::query_as!(
                    IntDB,
                    r#"select id as "int:_" from pesables where id = ? "#,
                    *pes.id()
                )
                .fetch_optional(self.db())
                .await?;
                qres.is_some()
            }
            Valuable::Rub((_, rub)) => {
                let qres: Option<IntDB> = sqlx::query_as!(
                    IntDB,
                    r#"select id as "int:_" from rubros where id = ? "#,
                    *rub.id()
                )
                .fetch_optional(self.db())
                .await?;
                qres.is_some()
            }
        };
        let result;

        if existe {
            if pos {
                result = Ok(self
                    .ventas
                    .a
                    .agregar_producto(prod, &self.configs().politica()))
            } else {
                result = Ok(self
                    .ventas
                    .b
                    .agregar_producto(prod, &self.configs().politica()))
            }
        } else {
            return Err(AppError::NotFound {
                objeto: String::from("producto"),
                instancia: prod.descripcion(&self.configs()),
            });
        }

        result
    }
    pub fn descontar_producto_de_venta(
        &mut self,
        index: usize,
        pos: bool,
    ) -> Result<Venta, AppError> {
        Ok(if pos {
            self.ventas
                .a
                .restar_producto(index, &self.configs().politica())?
        } else {
            self.ventas
                .b
                .restar_producto(index, &self.configs().politica())?
        })
    }
    pub fn incrementar_producto_a_venta(
        &mut self,
        index: usize,
        pos: bool,
    ) -> Result<Venta, AppError> {
        let result;
        if pos {
            result = self
                .ventas
                .a
                .incrementar_producto(index, &self.configs().politica());
        } else {
            result = self
                .ventas
                .b
                .incrementar_producto(index, &self.configs().politica());
        }

        result
    }
    pub fn eliminar_producto_de_venta(
        &mut self,
        index: usize,
        pos: bool,
    ) -> Result<Venta, AppError> {
        let result;
        if pos {
            if self.ventas.a.productos().len() > 1 {
                result = self
                    .ventas
                    .a
                    .eliminar_producto(index, &self.configs().politica());
            } else {
                self.ventas.a.empty();
                result = Ok(self.ventas.a.clone());
            }
        } else {
            if self.ventas.b.productos().len() > 1 {
                result = self
                    .ventas
                    .b
                    .eliminar_producto(index, &self.configs().politica());
            } else {
                self.ventas.b.empty();
                result = Ok(self.ventas.b.clone());
            }
        }

        result
    }
    pub fn venta(&self, pos: bool) -> Venta {
        if pos {
            self.ventas.a.clone()
        } else {
            self.ventas.b.clone()
        }
    }
    pub fn ventas(&self) -> Ventas {
        self.ventas.clone()
    }
    pub fn filtrar_marca(&self, filtro: &str) -> Res<Vec<String>> {
        async_runtime::block_on(async {
            let qres: Vec<StringDB> = sqlx::query_as!(
                StringDB,
                "select marca as string from productos where marca like ? order by marca asc",
                filtro
            )
            .fetch_all(self.db())
            .await?;
            Ok(qres
                .iter()
                .map(|s| s.string.to_owned())
                .collect::<HashSet<String>>()
                .iter()
                .cloned()
                .collect::<Vec<String>>())
        })
    }
    // pub fn get_deuda_cliente(&self, cliente: Cli)->Res<f64>{

    // }
    pub fn filtrar_tipo_producto(&self, filtro: &str) -> Res<Vec<String>> {
        async_runtime::block_on(async {
            let qres: Vec<StringDB> = sqlx::query_as!(
                StringDB,
                "select marca as string from productos where tipo like ? order by tipo asc",
                filtro
            )
            .fetch_all(self.db())
            .await?;
            Ok(qres
                .iter()
                .map(|d| d.string.to_owned())
                .collect::<HashSet<String>>()
                .iter()
                .cloned()
                .collect::<Vec<String>>())
        })
    }
    pub fn db(&self) -> &Pool<Sqlite> {
        &self.db
    }
    fn set_venta(&mut self, pos: bool, venta: Venta) {
        if pos {
            self.ventas.a = venta;
        } else {
            self.ventas.b = venta;
        }
    }
    fn cerrar_venta(&mut self, pos: bool) -> Res<()> {
        async_runtime::block_on(async { self.venta(pos).guardar(pos, self.db()).await })?;
        self.registro.push(self.venta(pos).clone());
        println!("{:#?}", self.venta(pos));
        async_runtime::block_on(async {
            self.update_total(self.venta(pos).monto_total(), &self.venta(pos).pagos())
                .await
        })?;
        self.set_venta(
            pos,
            async_runtime::block_on(async {
                Venta::get_or_new(Some(self.arc_user()), self.db(), pos).await
            })?,
        );
        Ok(())
    }
    pub fn hacer_ingreso(&self, monto: f32, descripcion: Option<Arc<str>>) -> Res<()> {
        let mov = Movimiento::Ingreso { descripcion, monto };
        async_runtime::block_on(async { self.caja.hacer_movimiento(mov, &self.db).await })
    }
    pub fn hacer_egreso(&self, monto: f32, descripcion: Option<Arc<str>>) -> Res<()> {
        let mov = Movimiento::Egreso { descripcion, monto };
        async_runtime::block_on(async { self.caja.hacer_movimiento(mov, &self.db).await })
    }
    pub fn get_deuda(&self, cliente: Cli) -> Res<f32> {
        async_runtime::block_on(async { cliente.get_deuda(&self.db).await })
    }
    pub fn get_deuda_detalle(&self, cliente: Cli) -> Res<Vec<Venta>> {
        async_runtime::block_on(async { cliente.get_deuda_detalle(&self.db, self.user()).await })
    }
    pub fn eliminar_valuable(&self, val: V) {
        let _res = async_runtime::block_on(async { val.eliminar(self.db.as_ref()).await });
    }
    pub fn editar_valuable(&self, val: V) {
        let _res = async_runtime::block_on(async { val.editar(self.db.as_ref()).await });
    }
    pub fn arc_user(&self) -> Arc<User> {
        Arc::clone(&self.user.as_ref().unwrap())
    }
    pub fn stash_sale(&mut self, pos: bool) -> Res<()> {
        self.stash.push(self.venta(pos));
        self.set_venta(
            pos,
            async_runtime::block_on(async {
                Venta::get_or_new(Some(self.arc_user()), self.db(), pos).await
            })?,
        );
        Ok(())
    }
    pub fn set_cantidad_producto_venta(
        &mut self,
        index: usize,
        cantidad: f32,
        pos: bool,
    ) -> Res<Venta> {
        if index < self.venta(pos).productos().len() {
            if pos {
                self.ventas
                    .a
                    .set_cantidad_producto(index, cantidad, &self.configs.politica())
            } else {
                self.ventas
                    .b
                    .set_cantidad_producto(index, cantidad, &self.configs.politica())
            }
        } else {
            Err(AppError::NotFound {
                objeto: String::from("Producto"),
                instancia: index.to_string(),
            })
        }
    }
    pub fn set_cliente(&mut self, id: i64, pos: bool) -> Res<()> {
        if pos {
            async_runtime::block_on(async { self.ventas.a.set_cliente(id, &self.db).await })
        } else {
            async_runtime::block_on(async { self.ventas.b.set_cliente(id, &self.db).await })
        }
    }
    pub fn unstash_sale(&mut self, pos: bool, index: usize) -> Res<()> {
        if index < self.stash.len() {
            if self.venta(pos).productos().len() > 0 {
                self.stash.push(self.venta(pos).to_owned())
            }
            let venta = self.stash.remove(index);
            self.set_venta(pos, venta);
            Ok(())
        } else {
            Err(AppError::SaleSelection.into())
        }
    }
    pub fn stash(&self) -> &Vec<Venta> {
        &self.stash
    }
    pub async fn update_total(&mut self, monto: f32, pagos: &Vec<Pago>) -> Res<()> {
        self.caja.update_total(&self.db, monto, pagos).await
    }
    pub fn to_shared(&self) -> SistemaSH {
        SistemaSH {
            user: self.user.clone().unwrap().to_shared(),
            caja: self.caja.to_shared_complete(),
            configs: self.configs.to_shared_complete(),
            clientes: self.clientes.clone(),
            ventas: [self.ventas.a.to_shared(), self.ventas.b.to_shared()],
            proveedores: self
                .proveedores
                .iter()
                .map(|prov| prov.to_shared_complete())
                .collect::<Vec<ProveedorSH>>(),
        }
    }
}
