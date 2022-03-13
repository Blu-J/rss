-- ./sites_delete.sql
DELETE FROM scraping_sites
WHERE id = $1
    AND user_id = $2;

