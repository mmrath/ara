
-- password is `password12`
INSERT INTO public.app_user(
	first_name, last_name, username, email, phone_number, active, created_by, updated_by, version)
	VALUES ( 'System', 'Admin', 'system', 'system@localhost', null, true, 'SYSTEM', 'SYSTEM', 1);


INSERT INTO public.user_credential(
    id, password_hash, invalid_attempts, locked, activated, updated_at, version ) SELECT u.id, '$argon2id$v=19$m=4096,t=192,p=8$/tNluCOWugt3ORI/Oj4QW4gs9Ky2mH15dHREcmpADLM$vV+6kV0WvPq//5L+8+j2I7kfVo55wIcjp3yW2jn7IKU', 0, false, true, current_timestamp, 1 from app_user u where email = 'test@test.com';
