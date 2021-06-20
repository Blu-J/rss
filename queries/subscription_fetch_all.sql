SELECT s.id as "id?",
    title,
    category,
    rss_feed,
    (
        SELECT count(*)
        FROM items i
        WHERE s.id = i.subscription_id
            and i.is_read = false
    ) as "unreads?:i64"
FROM subscriptions s;