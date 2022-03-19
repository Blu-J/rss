-- SQLite
SELECT
    href,
    description,
    title,
    image_src,
    comments_href,
    date AS 'date:u32',
    (
        SELECT
            site_title
        FROM
            scraping_sites
        WHERE
            id = articles.site_id) AS 'site_title'
FROM
    articles
    INNER JOIN scraping_sites ON scraping_sites.id = articles.site_id
        AND scraping_sites.user_id = $1
    INNER JOIN site_tags AS tags ON tags.tag = $2
        AND tags.site_id = scraping_sites.id
    ORDER BY
        date DESC;

