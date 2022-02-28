CREATE TABLE subscriptions (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    rss_feed TEXT NOT NULL,
    unique (rss_feed)
);
CREATE TABLE items (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    subscription_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    link TEXT NOT NULL,
    pub_date INTEGER NOT NULL,
    author TEXT,
    description TEXT,
    comments TEXT,
    contents TEXT,
    FOREIGN KEY (subscription_id) REFERENCES subscriptions (id),
    unique(subscription_id, title)
);
CREATE TABLE users (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    salt TEXT not NULL,
    username TEXT not null,
    salted_password TEXT not null
);
CREATE TABLE user_subscription_metas (
    subscription_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    category TEXT NOT NULL,
    title TEXT NOT NULL,
    FOREIGN KEY (subscription_id) REFERENCES subscriptions (id),
    FOREIGN KEY (user_id) REFERENCES users (id),
    PRIMARY KEY (subscription_id, user_id)
);
CREATE TABLE user_item_reads (
    item_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    read_on INTEGER NOT NULL,
    FOREIGN KEY (item_id) REFERENCES items (id),
    FOREIGN KEY (user_id) REFERENCES users (id),
    PRIMARY KEY (item_id, user_id)
);
CREATE INDEX IF NOT EXISTS items_idx ON items (subscription_id, pub_date DESC);
CREATE INDEX IF NOT EXISTS users_username_idx ON users (username);
insert into subscriptions (id, rss_feed)
values (1, 'https://swordscomic.com/comic/feed/'),
    (
        2,
        'http://www.questionablecontent.net/QCRSS.xml'
    ),
    (3, 'http://xkcd.com/rss.xml'),
    (4, 'http://feeds.feedburner.com/zeefeed'),
    (5, 'http://oglaf.com/feeds/rss/'),
    -- (6, 'http://what-if.xkcd.com/feed.atom'),
    (7, 'http://theoatmeal.com/feed/rss'),
    (8, 'http://www.smbc-comics.com/rss.php'),
    (9, 'http://feeds2.feedburner.com/rsspect/fJur'),
    -- (10, 'http://jvns.ca/atom.xml'),
    -- (11, 'https://hugotunius.se/feed.xml'),
    -- (12, 'https://nullprogram.com/feed/'),
    (13, 'http://bartoszmilewski.wordpress.com/feed/'),
    -- (14, 'http://blog.8thlight.com/feed/atom.xml'),
    -- (15, 'https://code.facebook.com/posts/rss'),
    -- (16, 'http://martinfowler.com/bliki/bliki.atom'),
    (17, 'http://lambda-the-ultimate.org/rss.xml'),
    (
        18,
        'http://techblog.netflix.com/feeds/posts/default'
    ),
    (19, 'http://feeds.feedburner.com/codinghorror/'),
    (20, 'http://codeascraft.etsy.com/feed/'),
    (21, 'https://lobste.rs/rss'),
    (22, 'http://news.ycombinator.com/rss'),
    -- (23, 'http://scotch.io/feed'),
    (24, 'http://feeds.feedburner.com/Bludice'),
    (25, 'http://www.smashingmagazine.com/feed/');
insert into users(id, salt, username, salted_password)
values (
        1,
        'ufRK8ESE2V2N67VXUTzg',
        'test',
        'b645fff053639ea122db01b434502e5b8a96cc4912444978097f00c10da28084'
    );
INSERT INTO user_subscription_metas (user_id, subscription_id, category, title)
VALUES (1, 1, 'comics', 'Swords Comics'),
    (
        1,
        2,
        'comics',
        'Questionable Content'
    ),
    (1, 3, 'comics', 'xkcd'),
    (
        1,
        4,
        'comics',
        'http://feeds.feedburner.com/zeefeed'
    ),
    (1, 5, 'comics', 'oglaf'),
    -- (1, 6, 'comics', 'what if xkcd'),
    (1, 7, 'comics', 'oatmeal'),
    (1, 8, 'comics', 'smbc'),
    -- (1, 12, 'programming', 'nullprogram'),
    (1, 13, 'programming', 'bartoszmilewski'),
    -- (1, 14, 'programming', 'blog.8thlight'),
    -- (1, 15, 'programming', 'code facebook'),
    -- (1, 16, 'programming', 'martinfowler'),
    (1, 17, 'programming', 'lambda the ultimate'),
    (
        1,
        18,
        'programming',
        'techblog netflix'
    ),
    (1, 19, 'programming', 'feeds feedburner'),
    (1, 20, 'programming', 'codeascraft etsy'),
    (1, 21, 'meta feeds', 'lobsters'),
    (1, 22, 'meta feeds', 'news ycombinator'),
    -- (1, 23, 'javascript blogs', 'scotch'),
    (1, 24, 'javascript blogs', 'feedburner'),
    (1, 25, 'style', 'smashingmagazine');