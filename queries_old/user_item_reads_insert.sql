INSERT
    OR IGNORE INTO user_item_reads (item_id, user_id, read_on)
SELECT i.id,
    $1,
    $3
FROM items i
WHERE pub_date <= $2
    AND exists (
        SELECT 1
        FROM user_subscription_metas m
        WHERE i.subscription_id = m.subscription_id
            AND m.user_id = $1
            AND (
                $4 IS NULL
                OR m.subscription_id = $4
            )
            AND (
                $5 IS NULL
                OR m.category = $4
            )
    )