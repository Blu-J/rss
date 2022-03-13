INSERT INTO preferences (user_id, preference, value)
    VALUES ($1, $2, $3)
ON CONFLICT
    DO UPDATE SET
        value = excluded.value;

