SELECT
    st.*,
    s.site_title
FROM
    site_tags AS st
    JOIN scraping_sites AS s ON st.site_id = s.id
        AND s.user_id = $1;

