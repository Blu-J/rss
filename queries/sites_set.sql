-- ./sites_set.sql
INSERT INTO scraping_sites (id, user_id, every_seconds, url, articles_sel, title_sel, link_sel, site_title, image_sel, description_sel, comments_sel)
    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
ON CONFLICT (id)
    DO UPDATE SET
        user_id = excluded.user_id, every_seconds = excluded.every_seconds, url = excluded.url, articles_sel = excluded.articles_sel, title_sel = excluded.title_sel, link_sel = excluded.link_sel, site_title = excluded.site_title, image_sel = excluded.image_sel, description_sel = excluded.description_sel, comments_sel = excluded.comments_sel
    RETURNING
        id;

