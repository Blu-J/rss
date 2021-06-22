SELECT s.id,
    m.title,
    m.category,
    s.rss_feed,
    (
        SELECT count(*)
        FROM items i
        WHERE s.id = i.subscription_id
            and not exists (
                select 1
                from user_item_reads
                WHERE user_id = $1
            )
    ) as "items_not_read:i64"
FROM subscriptions s
    INNER JOIN user_subscription_metas m ON m.user_id = $1
    AND s.id = m.subscription_id
    AND s.id = $2;