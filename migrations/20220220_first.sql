CREATE TABLE users (
    id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
    username text NOT NULL,
    salt text NOT NULL
);

CREATE INDEX IF NOT EXISTS users_username_idx ON users (username);

INSERT INTO users (id, username, salt)
    VALUES (0, 'dragondef', 'random_salt');


CREATE TABLE scraping_sites (
    id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
    username_x_sites text NOT NULL,
    every_seconds number NOT NULL,
    url text NOT NULL,
    articles_sel text NOT NULL,
    title_sel text NOT NULL,
    link_sel text NOT NULL,
    description_sel text
);

CREATE INDEX IF NOT EXISTS scraping_sites_x_user_idx ON scraping_sites (id, username_x_sites);

INSERT INTO scraping_sites (username_x_sites, every_seconds, url, articles_sel, title_sel, link_sel, description_sel)
    VALUES ('dragondefXSite', 300, 'https://old.reddit.com', '.thing:not(.promoted)', 'a.title', 'a.title', NULL);


CREATE TABLE articles (
    id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
    username_x_article text NOT NULL,
    date number not null,
    title text NOT NULL,
    href text NOT NULL,
    description text,
    UNIQUE(username_x_article, title)
);

CREATE INDEX IF NOT EXISTS articles_x_user_idx ON articles (id, username_x_article);


-- CREATE TABLE items (
--     id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
--     subscription_id integer NOT NULL,
--     title text NOT NULL,
--     link text NOT NULL,
--     pub_date integer NOT NULL,
--     author text,
--     description text,
--     comments text,
--     contents text,
--     FOREIGN KEY (subscription_id) REFERENCES subscriptions (id),
--     UNIQUE (subscription_id, title)
-- );
-- CREATE TABLE users (
--     id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
--     salt text NOT NULL,
--     username text NOT NULL,
--     salted_password text NOT NULL
-- );
-- CREATE TABLE user_subscription_metas (
--     subscription_id integer NOT NULL,
--     user_id integer NOT NULL,
--     category text NOT NULL,
--     title text NOT NULL,
--     FOREIGN KEY (subscription_id) REFERENCES subscriptions (id),
--     FOREIGN KEY (user_id) REFERENCES users (id),
--     PRIMARY KEY (subscription_id, user_id)
-- );
-- CREATE TABLE user_item_reads (
--     item_id integer NOT NULL,
--     user_id integer NOT NULL,
--     read_on integer NOT NULL,
--     FOREIGN KEY (item_id) REFERENCES items (id),
--     FOREIGN KEY (user_id) REFERENCES users (id),
--     PRIMARY KEY (item_id, user_id)
-- );
-- CREATE INDEX IF NOT EXISTS items_idx ON items (subscription_id, pub_date DESC);
-- CREATE INDEX IF NOT EXISTS users_username_idx ON users (username);
-- INSERT INTO subscriptions (id, rss_feed)
--     VALUES (1, 'https://swordscomic.com/comic/feed/'), (2, 'http://www.questionablecontent.net/QCRSS.xml'), (3, 'http://xkcd.com/rss.xml'), (4, 'http://feeds.feedburner.com/zeefeed'), (5, 'http://oglaf.com/feeds/rss/'),
--     -- (6, 'http://what-if.xkcd.com/feed.atom'),
--     (7, 'http://theoatmeal.com/feed/rss'), (8, 'http://www.smbc-comics.com/rss.php'), (9, 'http://feeds2.feedburner.com/rsspect/fJur'),
--     -- (10, 'http://jvns.ca/atom.xml'),
--     -- (11, 'https://hugotunius.se/feed.xml'),
--     -- (12, 'https://nullprogram.com/feed/'),
--     (13, 'http://bartoszmilewski.wordpress.com/feed/'),
--     -- (14, 'http://blog.8thlight.com/feed/atom.xml'),
--     -- (15, 'https://code.facebook.com/posts/rss'),
--     -- (16, 'http://martinfowler.com/bliki/bliki.atom'),
--     (17, 'http://lambda-the-ultimate.org/rss.xml'), (18, 'http://techblog.netflix.com/feeds/posts/default'), (19, 'http://feeds.feedburner.com/codinghorror/'), (20, 'http://codeascraft.etsy.com/feed/'), (21, 'https://lobste.rs/rss'), (22, 'http://news.ycombinator.com/rss'),
--     -- (23, 'http://scotch.io/feed'),
--     (24, 'http://feeds.feedburner.com/Bludice'), (25, 'http://www.smashingmagazine.com/feed/');
-- INSERT INTO users (id, salt, username, salted_password)
--     VALUES (1, 'ufRK8ESE2V2N67VXUTzg', 'test', 'b645fff053639ea122db01b434502e5b8a96cc4912444978097f00c10da28084');
-- INSERT INTO user_subscription_metas (user_id, subscription_id, category, title)
--     VALUES (1, 1, 'comics', 'Swords Comics'), (1, 2, 'comics', 'Questionable Content'), (1, 3, 'comics', 'xkcd'), (1, 4, 'comics', 'http://feeds.feedburner.com/zeefeed'), (1, 5, 'comics', 'oglaf'),
--     -- (1, 6, 'comics', 'what if xkcd'),
--     (1, 7, 'comics', 'oatmeal'), (1, 8, 'comics', 'smbc'),
--     -- (1, 12, 'programming', 'nullprogram'),
--     (1, 13, 'programming', 'bartoszmilewski'),
--     -- (1, 14, 'programming', 'blog.8thlight'),
--     -- (1, 15, 'programming', 'code facebook'),
--     -- (1, 16, 'programming', 'martinfowler'),
--     (1, 17, 'programming', 'lambda the ultimate'), (1, 18, 'programming', 'techblog netflix'), (1, 19, 'programming', 'feeds feedburner'), (1, 20, 'programming', 'codeascraft etsy'), (1, 21, 'meta feeds', 'lobsters'), (1, 22, 'meta feeds', 'news ycombinator'),
--     -- (1, 23, 'javascript blogs', 'scotch'),
--     (1, 24, 'javascript blogs', 'feedburner'), (1, 25, 'style', 'smashingmagazine');
