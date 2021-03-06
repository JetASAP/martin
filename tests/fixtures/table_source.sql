DROP TABLE IF EXISTS table_source;
CREATE TABLE table_source(gid serial PRIMARY KEY, geom geometry(GEOMETRY, 4326));
INSERT INTO table_source(geom) values (GeomFromEWKT('SRID=4326;POINT(0 0)'));
INSERT INTO table_source(geom) values (GeomFromEWKT('SRID=4326;POINT(-2 2)'));
