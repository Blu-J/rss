-- SQLite
SELECT href, description, title, image_src, comments_href
FROM articles
INNER JOIN scraping_sites 
    ON scraping_sites.id = articles.site_id AND scraping_sites.user_id = ?;