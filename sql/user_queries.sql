INSERT INTO users (user_name, password_hash, user_role) VALUE ($1,$2,$3);

SELECT user_role from users WHERE user_name = $1;