-- Add migration script here
CREATE TABLE IF NOT EXISTS clientes (
            dni integer PRIMARY KEY not null,
            nombre text not null,
            limite real,
            activo boolean not null,
            time datetime not null
        )