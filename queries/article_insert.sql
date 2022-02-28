INSERT INTO articles (
        site_id,
        date,
        title,
        href,
        description,
        image_src,
        comments_href
    )
VALUES (?, ?, ?, ?, ?, ?, ?) ON CONFLICT DO NOTHING;
;