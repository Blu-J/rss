-- SQLite
SELECT href,
    description,
    title,
    image_src,
    comments_href,
    date as 'date:u32',
    (
        SELECT site_title
        FROM scraping_sites
        WHERE id = site_id
    ) as 'site_title'
FROM articles
    INNER JOIN scraping_sites ON scraping_sites.id = articles.site_id
    AND scraping_sites.user_id = ?
ORDER BY date DESC;