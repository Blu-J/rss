SELECT s.id,
    s.rss_feed
FROM subscriptions s
WHERE s.id = $1;