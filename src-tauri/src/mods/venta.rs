use crate::mods::db::map::{BigIntDB, ClienteDB, IntDB, VentaDB};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, Pool, Sqlite};
use std::sync::Arc;

use Valuable as V;
const CUENTA: &str = "Cuenta Corriente";

use crate::mods::db::Mapper;

use super::{
    lib::debug,
    redondeo, AppError, Cli, Cliente,
    Cuenta::{Auth, Unauth},
    Pago, Res, User, UserSH, UserSHC, Valuable,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Venta {
    id: i32,
    monto_total: f32,
    productos: Vec<Valuable>,
    pagos: Vec<Pago>,
    monto_pagado: f32,
    vendedor: Option<Arc<User>>,
    cliente: Cliente,
    paga: bool,
    cerrada: bool,
    time: NaiveDateTime,
}
#[derive(Serialize, Deserialize)]
pub struct VentaSH {
    id: i32,
    monto_total: f32,
    productos: Vec<Valuable>,
    pagos: Vec<Pago>,
    monto_pagado: f32,
    vendedor: Option<Arc<UserSH>>,
    cliente: Cliente,
    paga: bool,
    cerrada: bool,
    time: NaiveDateTime,
}
#[derive(Serialize, Deserialize)]
pub struct VentaSHC {
    id: i32,
    monto_total: f32,
    productos: Vec<Valuable>,
    pagos: Vec<Pago>,
    monto_pagado: f32,
    vendedor: Option<Arc<UserSHC>>,
    cliente: Cliente,
    paga: bool,
    cerrada: bool,
    time: NaiveDateTime,
}
impl<'a> Venta {
    pub async fn new(vendedor: Option<Arc<User>>, db: &Pool<Sqlite>, pos: bool) -> Res<Venta> {
        let time = Utc::now().naive_local();
        let qres = query_as!(
            IntDB,
            r#"select dni as "int:_" from clientes where nombre = ?"#,
            "Final"
        )
        .fetch_one(db)
        .await;

        let id= match query(
            "insert into ventas (time, monto_total, monto_pagado, cliente, cerrada, paga, pos ) values (?, ?, ?, ?, ?, ?, ?)").bind(time).bind(0.0).bind(0.0).bind(qres.unwrap().int).bind(false).bind(false).bind(pos).execute(db).await{
                Ok(a) => a.last_insert_rowid() as i32,
                Err(e) => {println!("Aca el error de insert venta {e}");0},
            };

        let cliente = Cliente::new(None);
        Ok(Venta {
            monto_total: 0.0,
            productos: Vec::new(),
            pagos: Vec::new(),
            monto_pagado: 0.0,
            vendedor,
            id,
            paga: false,
            cliente,
            cerrada: false,
            time,
        })
    }
    pub async fn get_or_new(
        vendedor: Option<Arc<User>>,
        db: &Pool<Sqlite>,
        pos: bool,
    ) -> Res<Venta> {
        let qres: Option<VentaDB> = sqlx::query_as!(
            VentaDB,
            r#"select id as "id:_", time, monto_total as "monto_total:_", monto_pagado as "monto_pagado:_", cliente as "cliente:_", cerrada, paga, pos from ventas where pos = ? and cerrada = ?"#,
            pos,
            false
        )
        .fetch_optional(db)
        .await?;
        match qres {
            Some(model) => match model.cerrada {
                true => Venta::new(vendedor, db, pos).await,
                false => Mapper::venta(db, model, &vendedor).await,
            },
            None => Venta::new(vendedor, db, pos).await,
        }
    }
    pub fn build(
        id: i32,
        monto_total: f32,
        productos: Vec<Valuable>,
        pagos: Vec<Pago>,
        monto_pagado: f32,
        vendedor: Option<Arc<User>>,
        cliente: Cliente,
        paga: bool,
        cerrada: bool,
        time: NaiveDateTime,
    ) -> Venta {
        Venta {
            id,
            monto_total,
            productos,
            pagos,
            monto_pagado,
            vendedor,
            paga,
            cliente,
            cerrada,
            time,
        }
    }
    pub fn id(&self) -> &i32 {
        &self.id
    }
    pub fn empty(&mut self) {
        self.monto_pagado = 0.0;
        self.productos.clear();
        self.monto_total = 0.0;
        self.pagos.clear();
    }
    pub fn monto_total(&self) -> f32 {
        self.monto_total
    }
    pub fn productos(&self) -> Vec<Valuable> {
        self.productos.clone()
    }
    pub fn cerrada(&self) -> bool {
        self.cerrada
    }
    pub fn pagos(&self) -> Vec<Pago> {
        self.pagos.clone()
    }
    pub fn monto_pagado(&self) -> f32 {
        self.monto_pagado
    }
    pub fn set_cantidad_producto(
        &mut self,
        index: usize,
        cantidad: f32,
        politica: &f32,
    ) -> Res<Self> {
        let producto = self.productos.remove(index);
        let producto = match producto {
            Valuable::Prod((_, prod)) => Valuable::Prod((cantidad as u8, prod)),
            Valuable::Pes((_, pes)) => Valuable::Pes((cantidad, pes)),
            Valuable::Rub((_, rub)) => Valuable::Rub((cantidad as u8, rub)),
        };
        self.productos.insert(index, producto);
        self.update_monto_total(politica);
        Ok(self.clone())
    }
    pub fn agregar_pago(&mut self, pago: Pago) -> Res<Venta> {
        let mut es_cred: bool = false;
        match pago.medio_pago().desc().as_ref() {
            CUENTA => match &self.cliente {
                Cliente::Final => {
                    return Err(AppError::IncorrectError(String::from(
                        "No esta permitido cuenta corriente en este cliente",
                    )))
                }
                Cliente::Regular(cli) => match cli.limite() {
                    Auth(_) => {
                        self.pagos.push(pago.clone());
                    }
                    Unauth => {
                        return Err(AppError::IncorrectError(String::from(
                            "No esta permitido cuenta corriente en este cliente",
                        )))
                    }
                },
            },
            _ => {
                let mut pago = pago.clone();
                pago.set_pagado(pago.monto());
                self.pagos.push(pago);
            }
        }

        self.monto_pagado += pago.monto();
        let res = self.monto_total - self.monto_pagado;
        if res <= 0.0 {
            self.cerrada = true;
        }

        for pago in &self.pagos {
            if pago.medio().eq_ignore_ascii_case(CUENTA) {
                es_cred = true;
                break;
            }
        }
        if self.cerrada && !es_cred {
            self.paga = true;
        }
        Ok(self.clone())
    }
    pub fn agregar_producto(&mut self, producto: Valuable, politica: &f32) {
        let mut esta = false;
        for i in 0..self.productos.len() {
            if producto == self.productos[i] {
                let mut prod = self.productos.remove(i);
                match &prod {
                    V::Pes(a) => prod = V::Pes((a.0 + 1.0, a.1.clone())),
                    V::Prod(a) => prod = V::Prod((a.0 + 1, a.1.clone())),
                    V::Rub(a) => self.productos.push(V::Rub(a.clone())),
                }
                self.productos.insert(i, prod);
                esta = true;
            }
        }
        if !esta {
            let prod = match producto {
                V::Pes(a) => V::Pes((a.0, a.1.clone())),
                V::Prod(a) => V::Prod((a.0, a.1.clone())),
                V::Rub(a) => V::Rub((a.0, a.1.clone())),
            };
            self.productos.push(prod);
        }
        self.update_monto_total(politica);
    }
    fn update_monto_total(&mut self, politica: &f32) {
        self.monto_total = 0.0;
        for i in &self.productos {
            match &i {
                V::Pes(a) => self.monto_total += redondeo(politica, a.0 * a.1.precio_peso()),
                V::Prod(a) => self.monto_total += a.1.precio_venta() * a.0 as f32,
                V::Rub(a) => self.monto_total += a.1.monto().unwrap() * a.0 as f32,
            }
        }
    }
    pub fn eliminar_pago(&mut self, mut pago: Pago) -> Res<()> {
        let mut esta = false;
        for i in 0..self.pagos.len() {
            if self.pagos[i] == pago {
                pago = self.pagos.remove(i);
                esta = true;
                break;
            }
        }
        if !esta {
            return Err(AppError::IncorrectError(String::from(
                "Error de id de pago",
            )));
        }
        self.monto_pagado -= pago.monto();
        Ok(())
    }
    pub fn restar_producto(&mut self, index: usize, politica: &f32) -> Result<Venta, AppError> {
        let mut prod = self.productos.remove(index);
        match &prod {
            V::Pes(a) => {
                if a.0 > 1.0 {
                    prod = V::Pes((a.0 - 1.0, a.1.clone()))
                }
            }
            V::Prod(a) => {
                if a.0 > 1 {
                    prod = V::Prod((a.0 - 1, a.1.clone()))
                }
            }
            V::Rub(a) => {
                if a.0 > 1 {
                    prod = V::Rub((a.0 - 1, a.1.clone()))
                }
            }
        }
        self.productos.insert(index, prod);
        self.update_monto_total(politica);
        Ok(self.clone())
    }
    pub fn incrementar_producto(
        &mut self,
        index: usize,
        politica: &f32,
    ) -> Result<Venta, AppError> {
        let mut prod = self.productos.remove(index);
        match &prod {
            V::Pes(a) => prod = V::Pes((a.0 + 1.0, a.1.clone())),
            V::Prod(a) => prod = V::Prod((a.0 + 1, a.1.clone())),
            V::Rub(a) => prod = V::Rub((a.0 + 1, a.1.clone())),
        }
        self.productos.insert(index, prod);
        self.update_monto_total(politica);
        Ok(self.clone())
    }
    pub async fn set_cliente(&mut self, id: i32, db: &Pool<Sqlite>) -> Res<()> {
        if id == 0 {
            self.cliente = Cliente::Final;
            Ok(())
        } else {
            let qres: Option<ClienteDB> =
                sqlx::query_as!(ClienteDB, r#"select dni as "dni:_", nombre, limite as "limite:_", activo, time from clientes where dni = ? limit 1"#, id)
                    .fetch_optional(db)
                    .await?;
            match qres {
                Some(model) => {
                    self.cliente = Cliente::Regular(Cli::build(
                        model.dni,
                        Arc::from(model.nombre),
                        model.activo,
                        model.time,
                        model.limite,
                    ));
                    Ok(())
                }
                None => Err(AppError::NotFound {
                    objeto: String::from("Cliente"),
                    instancia: id.to_string(),
                }),
            }
        }
    }
    pub fn eliminar_producto(&mut self, index: usize, politica: &f32) -> Result<Venta, AppError> {
        self.productos.remove(index);
        self.update_monto_total(politica);
        Ok(self.clone())
    }
    pub async fn guardar(&self, pos: bool, db: &Pool<Sqlite>) -> Res<()> {
        let qres: Option<BigIntDB> = sqlx::query_as!(
            BigIntDB,
            "select id as int from ventas where id = ?",
            self.id
        )
        .fetch_optional(db)
        .await?;
        match qres {
            Some(_) => {
                let paga;
                let cliente;
                match &self.cliente {
                    Cliente::Final => {
                        paga = true;
                        cliente = 1;
                    }
                    Cliente::Regular(cli) => {
                        paga = self.paga;
                        cliente = *cli.dni();
                    }
                }
                sqlx::query("update ventas set time = ?, monto_total = ?, monto_pagado = ?, cliente = ?, cerrada = ?, paga = ?, pos = ? where id = ?")
                .bind(Utc::now().naive_local()).bind(self.monto_total)
                .bind(self.monto_pagado).bind(cliente).bind(self.cerrada).bind(paga).bind(pos).bind(self.id).execute(db).await?;
            }
            None => {
                let paga;
                let cliente;
                match &self.cliente {
                    Cliente::Final => {
                        paga = true;
                        cliente = 1;
                    }
                    Cliente::Regular(cli) => {
                        paga = self.paga;
                        cliente = *cli.dni();
                    }
                }
                sqlx::query("insert into ventas (time, monto_total, monto_pagado, cliente, cerrada, paga, pos)
                values (?, ?, ?, ?, ?, ?, ?)").bind(Utc::now().naive_local()).bind(self.monto_total).bind(self.monto_pagado).bind(cliente).bind(self.cerrada)
                .bind(paga).bind(pos).execute(db).await?;
            }
        }
        let mut pagos_sql = String::from(
            "INSERT INTO pagos (medio_pago, monto, pagado, venta) VALUES (?, ?, ?, ?)",
        );
        let mut estados = (false, false, false);

        let mut venta_prod_sql = String::from("INSERT INTO relacion_venta_prod (venta, producto, cantidad, precio, pos) VALUES (?, ?, ?, ?, ?)");
        let mut venta_pes_sql = String::from("INSERT INTO relacion_venta_pes (venta, pesable, cantidad, precio_kilo) VALUES (?, ?, ?, ?, ?)");
        let mut venta_rub_sql = String::from("INSERT INTO relacion_venta_rub (venta, rubro, cantidad, precio) VALUES (?, ?, ?, ?, ?)");
        let row = ", (?, ?, ?, ?)";
        let row5 = ", (?, ?, ?, ?, ?)";
        for _ in 1..self.pagos.len() {
            pagos_sql.push_str(row);
        }
        self.productos
            .iter()
            .enumerate()
            .for_each(|(i, prod)| match prod {
                Valuable::Prod(_) => {
                    estados = match estados {
                        (_, b, c) => (true, b, c),
                    };
                    if i > 0 {
                        venta_prod_sql.push_str(row5)
                    }
                }
                Valuable::Pes(_) => {
                    if i > 0 {
                        estados = match estados {
                            (a, _, c) => (a, true, c),
                        };
                        if i > 0 {
                            venta_pes_sql.push_str(row5)
                        }
                    }
                }
                Valuable::Rub(_) => {
                    if i > 0 {
                        estados = match estados {
                            (a, b, _) => (a, b, true),
                        };
                        if i > 0 {
                            venta_rub_sql.push_str(row5)
                        }
                    }
                }
            });
        // for prod in &self.productos {
        //     match prod {
        //         Valuable::Prod(_) => {
        //             estados = match estados{(_,b,c)=>(true,b,c)};
        //             venta_prod_sql.push_str(row5)
        //         },
        //         Valuable::Pes(_) => {
        //             estados = match estados{(a,_,c)=>(a,true,c)};
        //             venta_pes_sql.push_str(row5)
        //         },
        //         Valuable::Rub(_) => {
        //             estados = match estados{(a,b,_)=>(a,b,true)};
        //             venta_rub_sql.push_str(row5)
        //         },
        //     }
        // }
        let mut pagos_query = sqlx::query(pagos_sql.as_str());
        let mut prod_query = sqlx::query(venta_prod_sql.as_str());
        let mut pes_query = sqlx::query(venta_pes_sql.as_str());
        let mut rub_query = sqlx::query(venta_rub_sql.as_str());
        for pago in &self.pagos {
            let aux = pagos_query;
            pagos_query = aux
                .bind(*pago.medio_pago().id())
                .bind(pago.monto())
                .bind(*pago.pagado())
                .bind(self.id);
        }
        for prod in &self.productos {
            match prod {
                Valuable::Prod((c, p)) if estados.0 => {
                    let aux = prod_query;
                    prod_query = aux
                        .bind(self.id)
                        .bind(*p.id())
                        .bind(*c)
                        .bind(*p.precio_venta())
                        .bind(pos);
                }
                Valuable::Pes((c, p)) if estados.1 => {
                    let aux = pes_query;
                    pes_query = aux
                        .bind(self.id)
                        .bind(*p.id())
                        .bind(*c)
                        .bind(*p.precio_peso())
                        .bind(pos);
                }
                Valuable::Rub((c, r)) if estados.2 => {
                    let aux = rub_query;
                    rub_query = aux
                        .bind(self.id)
                        .bind(*r.id())
                        .bind(*c)
                        .bind(r.monto())
                        .bind(pos);
                }
                _ => (),
            }
        }
        pagos_query.execute(db).await?;
        if estados.0 {
            if let Err(e) = prod_query.execute(db).await {
                debug(&e, 454, "venta");
            }
        }
        if estados.1 {
            pes_query.execute(db).await?;
        }
        if estados.2 {
            rub_query.execute(db).await?;
        }
        Ok(())
    }
    pub fn to_shared(&self) -> VentaSH {
        VentaSH {
            id: self.id,
            monto_total: self.monto_total,
            productos: self.productos.clone(),
            pagos: self.pagos.clone(),
            monto_pagado: self.monto_pagado,
            vendedor: self
                .vendedor
                .clone()
                .map(|v| Arc::from(v.as_ref().to_shared())),
            cliente: self.cliente.clone(),
            paga: self.paga,
            cerrada: self.cerrada,
            time: self.time,
        }
    }
    pub fn to_shared_complete(&self) -> VentaSHC {
        VentaSHC {
            id: self.id,
            monto_total: self.monto_total,
            productos: self.productos.clone(),
            pagos: self.pagos.clone(),
            monto_pagado: self.monto_pagado,
            vendedor: self
                .vendedor
                .clone()
                .map(|v| Arc::from(v.as_ref().to_shared_complete())),
            cliente: self.cliente.clone(),
            paga: self.paga,
            cerrada: self.cerrada,
            time: self.time,
        }
    }
    pub fn from_shared_complete(venta: VentaSHC) -> Self {
        Venta {
            id: venta.id,
            monto_total: venta.monto_total,
            productos: venta.productos,
            pagos: venta.pagos,
            monto_pagado: venta.monto_pagado,
            vendedor: venta
                .vendedor
                .map(|u| Arc::from(User::from_shared_complete(u.as_ref().clone()))),
            cliente: venta.cliente,
            paga: venta.paga,
            cerrada: venta.cerrada,
            time: venta.time,
        }
    }
}
