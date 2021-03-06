SELECT id,
    i.subscription_id,
    i.contents,
    i.title,
    i.pub_date,
    i.link,
    i.author,
    i.description,
    i.comments
FROM items i
WHERE (
        $4 IS NULL
        OR NOT EXISTS (
            SELECT 1
            from user_item_reads r
            WHERE r.user_id = $1
                AND r.item_id = i.id
        )
    )
    AND EXISTS (
        SELECT 1
        from user_subscription_metas m
        WHERE m.user_id = $1
            AND m.subscription_id = i.subscription_id
            AND (
                $2 IS NULL
                OR m.subscription_id = $2
            )
            AND (
                $3 IS NULL
                OR m.title = $3
            )
    )
ORDER BY i.pub_date desc;