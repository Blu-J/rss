-- ./site_tags_upsert.sql
INSERT INTO site_tags (site_id, tag)
    VALUES (?, ?)
ON CONFLICT
    DO NOTHING
RETURNING
    id;

