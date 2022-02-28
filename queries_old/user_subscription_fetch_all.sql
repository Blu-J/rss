SELECT s.id,
    m.title,
    m.category,
    s.rss_feed
FROM subscriptions s
    INNER JOIN user_subscription_metas m ON m.user_id = $1
    AND s.id = m.subscription_id;