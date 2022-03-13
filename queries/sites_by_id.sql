-- ./sites_by_id.sql
SELECT
    s.*,
    '' || (
        SELECT
            group_concat (st.tag, ' ')
        FROM
            site_tags st
        WHERE
            st.site_id == s.id) AS "tags"
FROM
    scraping_sites s
WHERE
    s.id = $1
    AND s.user_id = $2;

