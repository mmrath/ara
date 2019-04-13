
-- password is `password12`
INSERT INTO public.app_user(
	first_name, last_name, username, email, phone_number, password_hash, activated, locked, failed_login_count, version)
	VALUES ( 'Test', 'User', 'testuser', 'test@test.com', null, '$argon2id$v=19$m=4096,t=192,p=8$/tNluCOWugt3ORI/Oj4QW4gs9Ky2mH15dHREcmpADLM$vV+6kV0WvPq//5L+8+j2I7kfVo55wIcjp3yW2jn7IKU', true, false, 0, 1);

