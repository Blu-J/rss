CREATE TABLE subscriptions (
    id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
    title text NOT NULL,
    category text NOT NULL,
    rss_feed text NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS subscriptions_title_cat_udx ON subscriptions (title, category);
CREATE TABLE items (
    id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
    subscription_id integer NOT NULL,
    title text NOT NULL,
    link text NOT NULL,
    pub_date INTEGER NOT NULL,
    author text,
    description text,
    comments text,
    contents text,
    is_read bool default false,
    FOREIGN KEY (subscription_id) REFERENCES subscriptions (id)
);
CREATE UNIQUE INDEX IF NOT EXISTS items_link_udx ON items (title);
CREATE UNIQUE INDEX IF NOT EXISTS items_idx ON items (subscription_id, pub_date DESC);