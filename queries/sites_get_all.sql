-- ./sites_get_all.sql
-- SQLite
SELECT
    *
FROM
    scraping_sites
WHERE
    user_id = $1;

