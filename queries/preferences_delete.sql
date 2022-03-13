DELETE FROM preferences
WHERE user_id = $1
    AND preference = $2;

