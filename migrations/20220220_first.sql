CREATE TABLE users (
    id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
    username text NOT NULL
);

CREATE INDEX IF NOT EXISTS users_username_idx ON users (username);

INSERT INTO users (username)
    VALUES ('dragondef');

-- Scraping Sites
CREATE TABLE scraping_sites (
    id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id integer NOT NULL,
    every_seconds integer NOT NULL,
    url text NOT NULL,
    articles_sel text NOT NULL,
    title_sel text NOT NULL,
    link_sel text NOT NULL,
    site_title text NOT NULL,
    image_sel text,
    description_sel text,
    comments_sel text,
    FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE INDEX IF NOT EXISTS scraping_sites_by_user_idx ON scraping_sites (user_id);

INSERT INTO "main"."scraping_sites" ("id", "user_id", "every_seconds", "url", "articles_sel", "title_sel", "link_sel", "site_title", "image_sel", "description_sel", "comments_sel")
    VALUES ('1', '1', '300', 'https://old.reddit.com', '.thing:not(.promoted)', 'a.title', 'a.title', 'Old Reddit', 'a img', '', 'a.comments');

INSERT INTO "main"."scraping_sites" ("id", "user_id", "every_seconds", "url", "articles_sel", "title_sel", "link_sel", "site_title", "image_sel", "description_sel", "comments_sel")
    VALUES ('2', '1', '300', 'https://readm.org/manga/16972', '.episodes-list .item', '.table-episodes-title a', '.table-episodes-title a', 'Super Cube', '', '', '');

INSERT INTO "main"."scraping_sites" ("id", "user_id", "every_seconds", "url", "articles_sel", "title_sel", "link_sel", "site_title", "image_sel", "description_sel", "comments_sel")
    VALUES ('3', '1', '300', 'https://readm.org/manga/19243', '.episodes-list .item', '.table-episodes-title a', '.table-episodes-title a', 'Level Up Alone', '', '', '');

INSERT INTO "main"."scraping_sites" ("id", "user_id", "every_seconds", "url", "articles_sel", "title_sel", "link_sel", "site_title", "image_sel", "description_sel", "comments_sel")
    VALUES ('4', '1', '300', 'https://readm.org/manga/17515', '.episodes-list .item', '.table-episodes-title a', '.table-episodes-title a', 'The tutorial tower of the advanced player', '', '', '');

INSERT INTO "main"."scraping_sites" ("id", "user_id", "every_seconds", "url", "articles_sel", "title_sel", "link_sel", "site_title", "image_sel", "description_sel", "comments_sel")
    VALUES ('5', '1', '300', 'https://readm.org/manga/16958', '.episodes-list .item', '.table-episodes-title a', '.table-episodes-title a', 'Swallowed Start', '', '', '');

INSERT INTO "main"."scraping_sites" ("id", "user_id", "every_seconds", "url", "articles_sel", "title_sel", "link_sel", "site_title", "image_sel", "description_sel", "comments_sel")
    VALUES ('6', '1', '300', 'https://readm.org/manga/20233', '.episodes-list .item', '.table-episodes-title a', '.table-episodes-title a', 'Game Invades World', '', '', '');

INSERT INTO "main"."scraping_sites" ("id", "user_id", "every_seconds", "url", "articles_sel", "title_sel", "link_sel", "site_title", "image_sel", "description_sel", "comments_sel")
    VALUES ('7', '1', '300', 'https://readm.org/manga/7236', '.episodes-list .item', '.table-episodes-title a', '.table-episodes-title a', 'The Gamer', '', '', '');

-- Site Tags
CREATE TABLE site_tags (
    id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
    site_id integer NOT NULL,
    tag text NOT NULL,
    UNIQUE (site_id, tag),
    FOREIGN KEY (site_id) REFERENCES scraping_sites (id)
);

CREATE INDEX IF NOT EXISTS site_tags_by_site_idx ON site_tags (site_id);

INSERT INTO site_tags (site_id, tag)
    VALUES (1, 'news');

-- Articles
CREATE TABLE articles (
    id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
    site_id text NOT NULL,
    date DATETIME NOT NULL,
    read_on DATETIME,
    title text NOT NULL,
    href text NOT NULL,
    description text,
    image_src text,
    comments_href text,
    UNIQUE (site_id, title),
    FOREIGN KEY (site_id) REFERENCES scraping_sites (id)
);

CREATE INDEX IF NOT EXISTS articles_sites_idx ON articles (site_id, date DESC);

-- Preferences
CREATE TABLE preferences (
    id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id text NOT NULL,
    preference text NOT NULL,
    value text NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE UNIQUE INDEX IF NOT EXISTS preferences_udx ON preferences (user_id, preference);

