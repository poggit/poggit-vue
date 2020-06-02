CREATE TABLE raw_file (
	id SERIAL PRIMARY KEY NOT NULL,
	mime VARCHAR(127) NOT NULL,
	filename VARCHAR(255) NOT NULL,
	source VARCHAR(127) NOT NULL,
	downloads INT NOT NULL,
	created TIMESTAMP NOT NULL,
	touch TIMESTAMP NOT NULL,
	expiry INTERVAL NOT NULL, -- use 100 YEARS if not expiring
	auth INT NOT NULL, -- repo ID to require access on
	pref_tabs BOOL NOT NULL,
	pref_pages BOOL NOT NULL,
	pref_icons BOOL NOT NULL,
	pref_dark BOOL NOT NULL
);
CREATE INDEX ON raw_file ((touch + expiry));

CREATE TYPE account_type AS ENUM(
	'Org', -- organization, not an account
	'Indirect', -- never visited Poggit directly
	'Beta', -- registered on Poggit in the beta days
	'User' -- registered on Poggit
);

CREATE TABLE account (
	id INT PRIMARY KEY, -- GitHub user ID
	cached_name VARCHAR(255) UNIQUE, -- might be outdated, check and update every time
	acc_type account_type NOT NULL,
	email VARCHAR(255) NULL, -- write down if known, ignore otherwise
	install_id INT NULL, -- app installation ID if exists
	first_login TIMESTAMP NULL, -- first time logged in, null if never logged in after migration
	last_login TIMESTAMP NULL -- last time logged in, if ever
);

CREATE TABLE login_history (
	sess CHAR(40) PRIMARY KEY, -- generated session ID
	account INT NULL REFERENCES account (id), -- the associated account, null before authentication
	ip VARCHAR(127) NOT NULL,
	auth TIMESTAMP NOT NULL, -- time of authentication
	touch TIMESTAMP NOT NULL -- time of last usage
);
CREATE INDEX ON login_history (ip);
CREATE INDEX ON login_history (auth);

CREATE TABLE repo (
	id INT PRIMARY KEY,
	owner INT NOT NULL REFERENCES account(id),
	cached_name VARCHAR(255) NOT NULL,
	private BOOL NOT NULL,
	fork BOOL NOT NULL,
	UNIQUE (owner, cached_name)
);

CREATE TABLE project (
	id SERIAL PRIMARY KEY,
	owner INT NOT NULL REFERENCES account(id),
	repo INT NOT NULL REFERENCES repo(id) ON DELETE CASCADE,
	name VARCHAR(255) NOT NULL,
	UNIQUE (owner, name)
);

CREATE TYPE build_cause AS ENUM(
	'Manual', -- manually triggered
	'Push', -- push event
	'Pr' -- pull_request event
);
CREATE TABLE build (
	id SERIAL PRIMARY KEY,
	project INTEGER NOT NULL REFERENCES project(id) ON DELETE CASCADE,
	category VARCHAR(15) NOT NULL,
	ser INTEGER NOT NULL, -- serial number of the build within the project
	branch VARCHAR(255) NOT NULL,
	sha CHAR(40) NOT NULL,
	path VARCHAR(255) NOT NULL,
	created TIMESTAMP NOT NULL,
	creator INT NOT NULL REFERENCES account(id) ON DELETE RESTRICT,
	cause build_cause NOT NULL,
	pr_number INTEGER NULL,
	pr_head INT NULL, -- might or might not reference repo(id)
	complete BOOL NOT NULL,
	phar INTEGER NULL UNIQUE REFERENCES raw_file(id) ON DELETE SET NULL, -- keep the build metadata even if the artifact expired
	raw_log INTEGER NOT NULL REFERENCES raw_file(id) ON DELETE SET NULL, -- keep the build metadata even if the logs expired
	UNIQUE (project, category, ser) -- user/project#category:ser is unique
);
