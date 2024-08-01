-- Add migration script here
CREATE TABLE IF NOT EXISTS codigos (
                codigo integer primary key not null,
                producto integer,
                pesable integer,
                rubro integer,
                foreign key (producto) references productos(id),
                foreign key (pesable) references pesables(id),
                foreign key (rubro) references rubros(id)
            )