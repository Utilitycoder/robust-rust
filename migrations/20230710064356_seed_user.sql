-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
    'ddf8994f-d522-4659-8d02-c1d479057be6',
    'admin',
    '$argon2id$v=19$m=4096,t=3,p=1$mCtoUCDJnzgrD9XuG60AIA$inM+EWltZekuCtI2QI4o5FdJxR9wZS6Nos4xic8fY4M'
);