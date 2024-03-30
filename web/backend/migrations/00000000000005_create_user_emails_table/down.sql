-- Drop the trigger
DROP TRIGGER IF EXISTS email_unique_trigger ON user_emails;

-- Drop the trigger function
DROP FUNCTION IF EXISTS check_email_unique();

-- Drop the table
DROP TABLE IF EXISTS user_emails;