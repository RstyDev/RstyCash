-- Add migration script here
CREATE TABLE IF NOT EXISTS productos (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    precio_venta real not null,
    porcentaje real NOT NULL,
    precio_costo real NOT NULL,
    tipo TEXT NOT NULL,
    marca TEXT NOT NULL,
    variedad TEXT NOT NULL,
    presentacion TEXT NOT NULL,
    size real NOT NULL,
    updated_at DATETIME NOT NULL
)

