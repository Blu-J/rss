SELECT s.id as "id?",
    title,
    category,
    rss_feed,
    null as "unreads?:i64"
FROM subscriptions s
WHERE id = ?;