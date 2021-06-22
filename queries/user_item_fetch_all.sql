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
WHERE NOT EXISTS (
        SELECT 1
        from user_item_reads r
        WHERE r.user_id = $1
            AND r.item_id = i.id
    )
    AND EXISTS (
        SELECT 1
        from user_subscription_metas m
        WHERE m.user_id = $1
            AND m.subscription_id = i.subscription_id
    );