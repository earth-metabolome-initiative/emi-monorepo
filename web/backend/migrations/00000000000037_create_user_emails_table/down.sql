-- Drop the primary_email_trigger trigger
DROP TRIGGER IF EXISTS primary_email_trigger ON user_emails;

-- Drop the update_primary_email function
DROP FUNCTION IF EXISTS update_primary_email();

-- Drop the trigger
DROP TRIGGER IF EXISTS email_unique_trigger ON user_emails;

-- Drop the trigger function
DROP FUNCTION IF EXISTS check_email_unique();

-- Drop the table
DROP TABLE IF EXISTS user_emails;
